// SPDX-License-Identifier: Apache-2.0

//! `pathbuf` provides a single macro, [`pathbuf!`][pathbuf], which gives a [`vec!`][std_vec]-like syntax
//! for constructing [`PathBuf`][std_path_pathbuf]s.
//!
//! # Example
//!
//! ```
//! # use pathbuf::pathbuf;
//! # use std::path::Path;
//! #
//! fn do_something(dir: &Path) {
//!     let file_name = pathbuf![dir, "filename.txt"];
//!
//!     if file_name.exists() {
//!         // do something...
//!     }
//! }
//! ```
//!
//! # Security
//!
//! As the macro relies on [`std::path::PathBuf::push`] there is also no protection against path traversal attacks.
//! Therefore no path element shall be untrusted user input without validation or sanitisation.
//!
//! An example for a path traversal/override on an UNIX system:
//!
//! ```
//! # use pathbuf::pathbuf;
//! # use std::path::PathBuf;
//! #
//! # #[cfg(unix)]
//! # {
//! let user_input = "/etc/shadow";
//! assert_eq!(pathbuf!["/tmp", user_input], PathBuf::from("/etc/shadow"));
//! # }
//! ```
//!
//! [pathbuf]: macro.pathbuf.html
//! [std_vec]: https://doc.rust-lang.org/std/macro.vec.html "Documentation for std::vec (macro)"
//! [std_path_pathbuf]: https://doc.rust-lang.org/std/path/struct.PathBuf.html "Documentation for std::path::PathBuf (struct)"

/// Creates a [`PathBuf`][std_path_pathbuf] containing the arguments.
///
/// `pathbuf!` allows [`PathBuf`][std_path_pathbuf]s to be defined with the same syntax as array expressions, like so:
///
/// ```
/// # use pathbuf::pathbuf;
/// # use std::path::Path;
/// #
/// fn do_something(dir: &Path) {
///     let file_name = pathbuf![dir, "filename.txt"];
///
///     if file_name.exists() {
///         // do something...
///     }
/// }
/// ```
///
/// [std_path_pathbuf]: https://doc.rust-lang.org/std/path/struct.PathBuf.html "Documentation for std::path::PathBuf (struct)"
#[macro_export]
macro_rules! pathbuf {
    ( $( $part:expr ),* ) => {{
        use std::path::PathBuf;

        let mut temp = PathBuf::with_capacity( $( std::mem::size_of_val($part) + )* 0);

        $(
            temp.push($part);
        )*

        temp
    }};

    ($( $part:expr, )*) => ($crate::pathbuf![$($part),*])
}

#[cfg(test)]
mod tests {
    use crate::pathbuf;
    use std::path::PathBuf;

    #[test]
    fn it_works() {
        let p = pathbuf!["hello", "filename.txt"];

        let expected = {
            let mut temp = PathBuf::new();
            temp.push("hello");
            temp.push("filename.txt");
            temp
        };

        assert_eq!(p, expected);
    }
}

/// A safe wrapper for a path with only a single component.
/// This prevents path traversal attacks.
///
/// It just allows a single normal path element and no parent, root directory or prefix like `C:`.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct PathComponent {
    path: std::path::PathBuf,
}

impl PathComponent {
    /// It creates the wrapped `PathComponent` if it's valid.
    /// Otherwise it will return `None`.
    ///
    /// ```
    /// # use pathbuf::PathComponent;
    /// # #[cfg(unix)]
    /// # {
    /// let some_valid_folder: PathComponent = PathComponent::new("foo").unwrap();
    /// let some_valid_file: PathComponent = PathComponent::new("bar.txt").unwrap();
    /// assert!(PathComponent::new("/etc/shadow").is_none());
    /// # }
    /// ```
    pub fn new<S: Into<std::path::PathBuf>>(component: S) -> Option<Self> {
        let component = Self {
            path: component.into(),
        };

        component.is_valid().then_some(component)
    }

    fn is_valid(&self) -> bool {
        use std::path::Component;

        let mut components = self.path.components();
        matches!(
            (components.next(), components.next()),
            (Some(Component::Normal(_)), None)
        )
    }
}

impl std::ops::Deref for PathComponent {
    type Target = std::path::Path;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl AsRef<std::path::Path> for PathComponent {
    fn as_ref(&self) -> &std::path::Path {
        &self.path
    }
}

/// This allows to push just a [`PathComponent`] to a [`std::path::PathBuf`].
///
/// ```
/// use std::path::PathBuf;
/// # use pathbuf::{pathbuf, PathComponent, PushPathComponent};
/// # #[cfg(unix)]
/// # {
/// let mut path = PathBuf::new();
/// path.push_component(PathComponent::new("foo").unwrap());
/// path.push_component(PathComponent::new("bar.txt").unwrap());
///
/// assert_eq!(path, pathbuf!["foo", "bar.txt"])
/// # }
/// ```
pub trait PushPathComponent {
    fn push_component(&mut self, component: PathComponent);
}

impl PushPathComponent for std::path::PathBuf {
    fn push_component(&mut self, component: PathComponent) {
        self.push(component);
    }
}
