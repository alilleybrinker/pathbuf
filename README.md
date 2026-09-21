# `pathbuf`

[![CI](https://github.com/alilleybrinker/pathbuf/actions/workflows/ci.yml/badge.svg)](https://github.com/alilleybrinker/pathbuf/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/pathbuf.svg)](https://crates.io/crates/pathbuf)
[![Documentation](https://docs.rs/pathbuf/badge.svg)](https://docs.rs/pathbuf)

`pathbuf` is a simple crate which provides the `pathbuf` macro to
conveniently construct the Rust `PathBuf` type.

## Example

```rust
use pathbuf::pathbuf;
use std::path::PathBuf;

fn main() {
    let p = pathbuf!["hello", "filename.txt"];

    let expected = {
        let mut temp = PathBuf::new();
        temp.push("hello");
        temp.push("filename.txt");
        temp
    };

    assert_eq!(p, expected);
}
```

Each argument may be anything implementing `AsRef<Path>` — the same bound
`PathBuf::push` accepts — and is evaluated exactly once. The resulting
`PathBuf` is pre-allocated to fit its components.

## Minimum Supported Rust Version

`pathbuf` supports Rust 1.56 and later. Raising this is considered a
breaking change.

## License

`pathbuf` is licensed under the Apache 2.0 license, and is itself a
reproduction of the [`hc_pathbuf`][hc] crate found in Hipcheck, pulled out
into its own distinct crate for reuse.

[hc]: https://github.com/mitre/hipcheck/tree/main/libs/hc_pathbuf
