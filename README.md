ncursesw-win [![Build Status](https://travis-ci.com/narfit66/ncursesw-win-rs.svg?branch=master)](https://travis-ci.com/narfit66/ncursesw-win-rs) [![Crates.io](https://img.shields.io/crates/v/ncursesw-win.svg)](https://crates.io/crates/ncursesw-win)
============

This is a wrapper crate around the [ncursesw](https://crates.io/crates/ncursesw) crate, it's purpose is too abstracts away the raw pointers that ncurses uses and function in a more safe way, however a knowledge of how ncurses works is advised to use the true power of this library.

Please see the [README.md](https://github.com/narfit66/ncursesw-rs/blob/master/README.md) for `ncursesw` for more details.

## Inclusion

```
[dependencies]
ncursesw-win = "0.3"
```
Or to use the latest git version
```
[dependencies]
ncursesw-win = { git = "https://github.com/narfit66/ncursesw-win-rs" }
```

## Building

The compiled library will be built in the `target` directory.

```
cargo build
```

## How to Use

```
extern crate ncurseswwin;

use ncurseswwin::*;
```

To use attributes and color pairs
```
use ncurseswwin::normal::*; // for 'ansi' color pairs and attributes...
use ncurseswwin::extend::*; // or for 'extended' color pairs and attributes.
```

## [Documentation](https://narfit66.github.io/ncursesw-win-rs/ncursesw-win/index.html)

This at the moment is only *partial* but the end objective is to document the whole crate/library.

## License

Licensed under the MIT license, see [LICENSE](LICENSE)
