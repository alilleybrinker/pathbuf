# Changelog

## v0.3.1

- Fix compilation issue by switching from `size_of` to
  `size_of_val`.

## v0.3.0

- The `PathBuf` pre-allocation is now based on bytes, rather than
  a count of the items to be added.

## v0.2.0

- The `PathBuf` now pre-allocates its capacity by calculating the
  number of elements passed to the macro.


