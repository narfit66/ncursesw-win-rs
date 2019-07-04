ncursesw-win
============

This is a *thin* wrapper around the `ncursesw`(https://crates.io/crates/ncursesw) crate, it's purpose is too abstracts away the raw pointers that ncurses uses and function in a more safe way, however a knowledge of how ncurses works is advised to use the true power of this library.

Please see the README.md for `ncursesw` for more details.

## Inclusion

```
[dependencies]
ncursesw-win = "0.1"
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
