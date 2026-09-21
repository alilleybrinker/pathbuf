// SPDX-License-Identifier: Apache-2.0

//! `pathbuf` provides a single macro, [`pathbuf!`], which gives a
//! [`vec!`][std_vec]-like syntax for constructing [`PathBuf`]s.
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
//! [std_vec]: std::vec! "Documentation for std::vec (macro)"
//! [`PathBuf`]: std::path::PathBuf
//! [`PathBuf::push`]: std::path::PathBuf::push

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

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
///
/// Each argument may be anything that implements <code>[AsRef]&lt;[Path]&gt;</code>, which is the
/// same bound [`PathBuf::push`] itself accepts. Arguments are borrowed rather than consumed, and
/// each one is evaluated exactly once:
///
/// ```
/// # use pathbuf::pathbuf;
/// # use std::path::{Path, PathBuf};
/// #
/// let dir = String::from("hello");
/// let p = pathbuf![&dir, Path::new("sub"), PathBuf::from("filename.txt")];
///
/// // `dir` was only borrowed, so it is still usable here.
/// assert_eq!(p, PathBuf::from("hello").join("sub").join("filename.txt"));
/// assert_eq!(dir, "hello");
/// ```
///
/// The resulting [`PathBuf`] is pre-allocated to fit all of its components, so building it
/// performs at most one allocation.
///
/// [`PathBuf`]: std::path::PathBuf
/// [`PathBuf::push`]: std::path::PathBuf::push
/// [Path]: std::path::Path
#[macro_export]
macro_rules! pathbuf {
    ( $( $part:expr ),* $(,)? ) => {{
        // Bind every part exactly once, so that argument expressions are not evaluated twice
        // and their temporaries live until the `PathBuf` has been built.
        let parts: &[&dyn ::std::convert::AsRef<::std::path::Path>] = &[ $( &$part ),* ];

        // Size the buffer up front: each component contributes its own bytes plus, at most,
        // one separator byte. Written as a loop rather than an iterator chain so that the
        // macro does not depend on `Iterator` being in scope at the call site.
        let mut capacity = 0usize;
        for part in parts {
            capacity += ::std::convert::AsRef::as_ref(part).as_os_str().len() + 1;
        }

        let mut path = ::std::path::PathBuf::with_capacity(capacity);

        for part in parts {
            path.push(::std::convert::AsRef::as_ref(part));
        }

        path
    }};
}
