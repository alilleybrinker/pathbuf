# `pathbuf`

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

## License

`pathbuf` is licensed under the Apache 2.0 license, and is itself a
reproduction of the [`hc_pathbuf`][hc] crate found in Hipcheck, pulled out
into its own distinct crate for reuse.

[hc]: https://github.com/mitre/hipcheck/tree/main/libs/hc_pathbuf

