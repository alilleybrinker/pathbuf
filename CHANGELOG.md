# Changelog

## Unreleased

- Fixed: arguments are now evaluated exactly once. Previously each argument
  expression was expanded twice — once to compute the capacity and once to
  push it — so any argument with a side effect ran it twice.
- Fixed: the macro now accepts everything `PathBuf::push` accepts, i.e. any
  `AsRef<Path>`. Owned values such as `String`, `PathBuf`, and `OsString`
  previously failed to compile, with an error pointing into `std::mem`.
- Fixed: the capacity calculation now measures the components themselves.
  It previously used `size_of_val`, which reports the size of the pointer
  for `&String`/`&PathBuf` rather than the length of the path, and which
  omitted the separator bytes — so the common case reallocated anyway.
- Fixed: the expansion is now fully hygienic, using absolute `::std` paths
  and no trait methods. It previously failed to compile at call sites that
  shadowed `std` or that had no prelude.
- Added: an integration test suite, covering argument kinds, evaluation
  count, trailing commas, empty invocations, hygiene, and pre-allocation.
- Added: continuous integration, running tests on Linux, macOS, and Windows
  on stable and on the minimum supported Rust version.
- Added: a documented minimum supported Rust version of 1.56, and
  `keywords`/`categories` metadata for crates.io.

## v1.0.0

- Documented that the macro offers no protection against path traversal,
  since it builds on `PathBuf::push`.

## v0.3.1

- Fix compilation issue by switching from `size_of` to
  `size_of_val`.

## v0.3.0

- The `PathBuf` pre-allocation is now based on bytes, rather than
  a count of the items to be added.

## v0.2.0

- The `PathBuf` now pre-allocates its capacity by calculating the
  number of elements passed to the macro.
