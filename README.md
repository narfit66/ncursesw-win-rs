ncursesw-win [![Build Status](https://travis-ci.com/narfit66/ncursesw-win-rs.svg?branch=master)](https://travis-ci.com/narfit66/ncursesw-win-rs) [![Crates.io](https://img.shields.io/crates/v/ncursesw-win.svg)](https://crates.io/crates/ncursesw-win)
============

This is a wrapper crate around the [ncursesw](https://crates.io/crates/ncursesw) crate, it's purpose is too abstracts away the raw pointers that NCurses uses and function in a more safe way, however a knowledge of how NCurses works is advised to use the true power of this library.

Please see the [README.md](https://github.com/narfit66/ncursesw-rs/blob/master/README.md) for `ncursesw` for more details.

## Inclusion

```
[dependencies]
ncursesw-win = "0.4"
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

To use menus (this is still under active development)
```
use ncurseswwin::menu::*;
```

Instead of calling `initscr()` and `endwin()` to initialise and teardown the NCurses library (these can still be called as the [ncursesw](https://crates.io/crates/ncursesw) crate is public) use the `ncursesw_entry()`. This initialises and tears down NCurses and provided the ability to catch panics in a controlled maner by passing them back to the client code as the error type `NCurseswWinError::Panic { message }`.

To create a window use `Window::new()` which will teardown correctly when going out of scope and provides all NCurses functionality associatiated with a NCurses window. Likewise a pad is created using `Pad::new()` and a panel using `Panel::new()`.

All NCurses methods associated with a `Window`, `Pad` or `RipoffWindow` use either their original ncurses name or where specificlly passed the pointer `_win_st` the 'w' has been removed, for example the ncurses 'C' function `mvwgetn_wstr(*WINDOW)` has become the method `self.mvgetn_wstr()`.

The NCurses ripoff and mouse features are encapsulated, please see example code for how to use these features.

## Examples

Examples are built by `cargo build --examples`. To run them, use `cargo run --example <EXAMPLE>`.

Current examples are [Safe Initialisation/Calling Of NCurses](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/ncursesw_entry-test.rs) (**ncursesw_entry-test**), [Ansi Border Drawing](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/border-test.rs) (**border-test**), [Unicode Border Drawing](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/border_set-test.rs) (**border_set-test**), [Unicode Box Drawing](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/box_drawing-test.rs) (**box_drawing-test**), [Mouse Events](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/mouse-test.rs) (**mouse-test**), [Ripoff Lines](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/ripoff-test.rs) (**ripoff-test**) and [Non-Blocking Get](https://github.com/narfit66/ncursesw-win-rs/blob/master/examples/getch_nonblocking-test.rs) (**getch_nonblocking-test**).

## Documentation

Please use `cargo doc --open` for this crate for the time being!.

## License

Licensed under the MIT license, see [LICENSE](LICENSE)
