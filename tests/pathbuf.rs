// SPDX-License-Identifier: Apache-2.0

//! Integration tests for the `pathbuf!` macro.
//!
//! These deliberately live outside the crate, so that the macro is exercised the same way a
//! downstream user would exercise it.

use pathbuf::pathbuf;
use std::cell::Cell;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

fn joined(parts: &[&str]) -> PathBuf {
    let mut expected = PathBuf::new();
    for part in parts {
        expected.push(part);
    }
    expected
}

#[test]
fn builds_from_str_literals() {
    assert_eq!(
        pathbuf!["hello", "filename.txt"],
        joined(&["hello", "filename.txt"])
    );
}

#[test]
fn builds_from_empty_invocation() {
    let p: PathBuf = pathbuf![];
    assert_eq!(p, PathBuf::new());
}

#[test]
fn builds_from_single_part() {
    assert_eq!(pathbuf!["only"], joined(&["only"]));
}

#[test]
fn accepts_trailing_comma() {
    assert_eq!(
        pathbuf!["hello", "filename.txt",],
        joined(&["hello", "filename.txt"])
    );
    assert_eq!(pathbuf!["only",], joined(&["only"]));
}

/// Every `AsRef<Path>` implementor that `PathBuf::push` accepts should work here too, whether
/// owned, borrowed, or a temporary.
#[test]
fn accepts_any_as_ref_path() {
    let expected = joined(&["a", "b", "c", "d", "e", "f"]);

    let owned_string = String::from("c");
    let owned_path = PathBuf::from("e");

    assert_eq!(
        pathbuf![
            "a",                 // &str
            String::from("b"),   // String (owned temporary)
            &owned_string,       // &String
            Path::new("d"),      // &Path
            &owned_path,         // &PathBuf
            OsString::from("f")  // OsString (owned temporary)
        ],
        expected
    );

    // Borrowed arguments are not consumed by the macro.
    assert_eq!(owned_string, "c");
    assert_eq!(owned_path, PathBuf::from("e"));
}

/// Argument expressions must be evaluated exactly once; an earlier implementation expanded each
/// one twice, silently duplicating side effects.
#[test]
fn evaluates_each_argument_exactly_once() {
    let calls = Cell::new(0);

    let bump = |name: &'static str| {
        calls.set(calls.get() + 1);
        name
    };

    assert_eq!(
        pathbuf![bump("a"), bump("b"), bump("c")],
        joined(&["a", "b", "c"])
    );
    assert_eq!(calls.get(), 3);
}

/// Expressions containing commas are a single `expr` fragment and must not be split up.
#[test]
fn accepts_expressions_containing_commas() {
    fn pick(first: &'static str, _second: &'static str) -> &'static str {
        first
    }

    assert_eq!(pathbuf![pick("a", "b"), "c"], joined(&["a", "c"]));
}

/// The buffer is sized up front, so building a path should not need to grow it.
#[test]
fn preallocates_enough_capacity() {
    let p = pathbuf!["hello", "sub", "dir", "filename.txt"];
    assert!(
        p.capacity() >= p.as_os_str().len(),
        "capacity {} is smaller than the finished path ({} bytes)",
        p.capacity(),
        p.as_os_str().len()
    );
}

/// `push` replaces the whole path when given an absolute component. This is the behaviour the
/// crate documents under "Security"; pin it so it cannot change silently.
#[test]
#[cfg(unix)]
fn absolute_component_replaces_path() {
    let user_input = "/etc/shadow";
    assert_eq!(pathbuf!["/tmp", user_input], PathBuf::from("/etc/shadow"));
}

#[test]
fn works_in_expression_position() {
    fn take(p: PathBuf) -> PathBuf {
        p
    }

    assert_eq!(take(pathbuf!["a", "b"]), joined(&["a", "b"]));
    assert_eq!(vec![pathbuf!["a"]], vec![joined(&["a"])]);
}

/// The macro must not depend on any name at the call site. These modules shadow `std` and drop
/// the prelude entirely; the expansion has to keep compiling regardless.
mod hygiene {
    #[allow(dead_code)]
    mod std {}

    #[test]
    fn works_with_shadowed_std() {
        let p = ::pathbuf::pathbuf!["a", "b"];
        assert_eq!(p, super::joined(&["a", "b"]));
    }

    #[no_implicit_prelude]
    mod no_prelude {
        pub(crate) fn build() -> ::std::path::PathBuf {
            ::pathbuf::pathbuf!["a", "b"]
        }

        pub(crate) fn build_empty() -> ::std::path::PathBuf {
            ::pathbuf::pathbuf![]
        }
    }

    #[test]
    fn works_without_prelude() {
        assert_eq!(no_prelude::build(), super::joined(&["a", "b"]));
        assert_eq!(no_prelude::build_empty(), ::std::path::PathBuf::new());
    }
}
