[![Build Status](https://travis-ci.org/febeling/edit-distance.svg)](https://travis-ci.org/febeling/edit-distance)

# Project Name

Calculate Levenshtein distance between two strings.

The edit distance of Levenshtein distance is helpful for spelling
correction, fuzzy completion and similar use cases.

## Installation

In Cargo.toml add

```
[dependencies]
edit-distance = "0.0.1"
```

Then re-run `cargo build`. That fetches the dependencies and builds
the code.

## Usage

```
extern crate edit_distance;

edit_distance("kitten", "sitting"); // => 3
```

## Contributing

1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request :D

## History

2015-04-18 0.0.1 Initial publication

## Credits

Thanks to @skade for very helpful criticism of my first rust lib.

## License

APL 2.0, see LICENSE file.
