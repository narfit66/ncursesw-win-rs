ncursesw-win [![Crates.io](https://img.shields.io/crates/v/ncursesw-win.svg)](https://crates.io/crates/ncursesw-win) [![Build Status](https://travis-ci.com/narfit66/ncursesw-win-rs.svg?branch=master)](https://travis-ci.com/narfit66/ncursesw-win-rs) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/narfit66/ncursesw-win-rs/blob/master/LICENSE) ![Lines of Code](https://tokei.rs/b1/github/narfit66/ncursesw-win-rs?category=code)
============

This crate is a wrapper around the [ncursesw](https://crates.io/crates/ncursesw) crate, it's purpose is too abstracts away the raw pointers that NCurses uses and function in a safe way, however a knowledge of how NCurses works is advised to use the true power of this library.

Please see the [README.md](https://github.com/narfit66/ncursesw-rs/blob/master/README.md) for `ncursesw` for more details.

## Inclusion

```
[dependencies]
ncursesw-win = "0.6"
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

For additional information on how to custom compile please see the `Building`, `Custom Build` and `Features` sections in the `ncursesw` [README.md](https://github.com/narfit66/ncursesw-rs/blob/master/README.md) crate.

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

To use menus
```
use ncurseswwin::menu::*;
```

To use forms
```
use ncurseswwin::form::*;
```

Instead of calling `initscr()` and `endwin()` to initialise and teardown the NCurses library (these can still be called as the [ncursesw](https://crates.io/crates/ncursesw) crate is public) use the `ncursesw_entry()` function. This initialises and tears down NCurses and provided the ability to catch panics in a controlled maner by passing them back to the client code as the error type `NCurseswWinError::Panic { message }`.

To create a window use `Window::new()` which will teardown correctly when going out of scope and provides all NCurses functionality associatiated with a NCurses window. Likewise a pad is created using `Pad::new()` and a panel using `Panel::new()`.

All NCurses methods associated with a `Window`, `Pad` or `RipoffWindow` use either their original NCurses name or where specificlly passed the pointer `_win_st` the 'w' has been removed, for example the NCurses 'C' function `wget_wch(*WINDOW)` has become the method `self.get_wch()` (where `self` is an instance of a `Window` for example).

The NCurses ripoff and mouse features are encapsulated, please see example code for how to use these features.

## Examples

Examples are built by `cargo build --examples`. To run them, use `cargo run --example <EXAMPLE>`.

Current examples are [Safe Initialisation/Calling Of NCurses](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/ncursesw_entry-test.rs) (**ncursesw_entry-test**), [Ansi Border Drawing](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/border-test.rs) (**border-test**), [Unicode Border Drawing](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/border_set-test.rs) (**border_set-test**), [Unicode Box Drawing](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/box_drawing-test.rs) (**box_drawing-test**), [Mouse Events](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/mouse-test.rs) (**mouse-test**), [Ripoff Lines](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/ripoff-test.rs) (**ripoff-test**), [Non-Blocking Get](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/getch_nonblocking-test.rs) (**getch_nonblocking-test**), [Screen](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/screen-test.rs) (**screen-test**) and [Menu](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/menu-test.rs) (**menu-test**).

## Documentation

Documentation for this crate can be found [here](https://docs.rs/ncursesw-win).

## License

Licensed under the MIT license, see [LICENSE](https://github.com/narfit66/ncursesw-win-rs/blob/master/LICENSE)
