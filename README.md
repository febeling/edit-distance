[![CI](https://github.com/febeling/edit-distance/actions/workflows/rust.yml/badge.svg)](https://github.com/febeling/edit-distance/actions/workflows/rust.yml)

# edit-distance

Calculate Levenshtein distance between two strings.

The Levenshtein edit distance is a measure for the similarity between
two strings. It's helpful for spelling correction, fuzzy completion,
type-ahead and similar use cases.

This implementation supports Unicode.

## Installation

In Cargo.toml add

```toml
[dependencies]
edit-distance = "2.1.2"
```

Then re-run `cargo build`. That fetches the dependencies and builds
the code.

## Usage

```rust
extern crate edit_distance;

edit_distance("kitten", "sitting"); // => 3
```

## Development

Test changes before comitting.

```shell
cargo clean
cargo fmt --all -- --check
cargo build
cargo test
```

## Contributing

Before sending a pull-request that goes beyond a bugfix, please open an issue to discuss. PRs without clear objective or separation of concerns aren't likely to be accepted.

## History

2024-06-26 2.1.2 Fix formatting
2024-06-26 2.1.1 Replace CI badge, maintenance  
2019-03-02 2.1.0 Optimize memory usage  
2018-01-02 2.0.1 Update dev-dependencies  
2017-07-02 2.0.0  
2015-05-01 1.0.0 Release  
2015-04-18 0.0.1 Initial upload

## Credits

Thanks to @skade for very helpful criticism of my first rust lib.

## License

APL 2.0, see LICENSE file.
