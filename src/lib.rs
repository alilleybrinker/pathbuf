// SPDX-License-Identifier: Apache-2.0

//! `pathbuf` provides a single macro, [`pathbuf!`][pathbuf], which gives a [`vec!`][std_vec]-like syntax
//! for constructing [`PathBuf`]s.
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
//! As the macro relies on [`PathBuf::push`] there is also no protection against path traversal attacks.
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

use std::path::PathBuf;

/// Creates a [`PathBuf`] containing the arguments.
///
/// `pathbuf!` allows [`PathBuf`]s to be defined with the same syntax as array expressions, like so:
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
    path: PathBuf,
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
    pub fn new<S: Into<PathBuf>>(component: S) -> Option<Self> {
        let component = Self {
            path: component.into(),
        };

        component.is_valid().then_some(component)
    }

    #[cfg(feature = "sanitise")]
    /// This will sanitise the input and therefore all inputs are valid.
    /// Unless there is a bug in the sanitisation then it would `panic`.
    ///
    /// ```
    /// # use pathbuf::PathComponent;
    /// # #[cfg(unix)]
    /// # {
    /// assert_eq!(
    ///     PathComponent::with_sanitise("/etc/shadow"),
    ///     PathComponent::new("etc_shadow").unwrap(),
    /// );
    /// # }
    /// ```
    ///
    /// The sanitisation algorithm isn't considered stable.
    /// Therefore the sanitised path could change in the future for the same input.
    pub fn with_sanitise(component: &str) -> Self {
        let sanitised_component = sanitize_filename_reader_friendly::sanitize(component);
        Self::new(sanitised_component).unwrap_or_else(|| {
            panic!(
                "Expected a sanitised path of the original path '{}'",
                component
            )
        })
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

impl PushPathComponent for PathBuf {
    fn push_component(&mut self, component: PathComponent) {
        self.push(component);
    }
}

/// Creates a [`PathBuf`] containing the arguments without allowing path traversal.
///
/// ```
/// # use std::path::PathBuf;
/// # use pathbuf::pathbuf_safe;
/// #
/// # #[cfg(unix)]
/// # {
/// let user_input = "foo.txt";
/// assert_eq!(pathbuf_safe!["tmp", user_input].unwrap(), PathBuf::from("tmp/foo.txt"));
/// let user_input = "/etc/shadow";
/// assert!(pathbuf_safe!["tmp", user_input].is_none());
/// # }
/// ```
///
/// When the first part is trusted, the `allow` keyword can be used.
/// It allows the usage of multiple components and the root.
///
/// ```
/// # use std::path::PathBuf;
/// # use pathbuf::pathbuf_safe;
/// #
/// # #[cfg(unix)]
/// # {
/// let user_input = "foo.txt";
/// assert_eq!(
///     pathbuf_safe![allow "/var/tmp", user_input].unwrap(),
///     PathBuf::from("/var/tmp/foo.txt"),
/// );
/// # }
/// ```
#[macro_export]
macro_rules! pathbuf_safe {
    ( $( $part:expr ),* ) => {{
        use std::path::PathBuf;
        use $crate::PushPathComponent;

        let mut temp = Some(PathBuf::with_capacity( $( std::mem::size_of_val($part) + )* 0));

        $(
            temp = temp.and_then(|mut tmp_path| {
                let component = $crate::PathComponent::new($part)?;
                tmp_path.push_component(component);
                Some(tmp_path)
            });
        )*

        temp
    }};
    (allow $first:expr, $( $part:expr ),* ) => {{
        use std::path::PathBuf;
        use $crate::PushPathComponent;

        let mut temp = Some(PathBuf::with_capacity( $( std::mem::size_of_val($part) + )* 0));

        temp = temp.map(|mut tmp_path| {
            tmp_path.push($first);
            tmp_path
        });
        $(
            temp = temp.and_then(|mut tmp_path| {
                let component = $crate::PathComponent::new($part)?;
                tmp_path.push_component(component);
                Some(tmp_path)
            });
        )*

        temp
    }};

    ($( $part:expr, )*) => {{
        $crate::pathbuf_safe![$($part),*]
    }};

    (allow $( $part:expr, )*) => {{
        $crate::pathbuf_safe![allow $($part),*]
    }};
}
