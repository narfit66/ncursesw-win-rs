## Issues with Travis CI

Travis CI fails to compile with extened color pairs as of compile [#41](https://travis-ci.com/narfit66/ncursesw-win-rs/builds/134301416), this was corrected in build #42 by changing `use ncurseswwin::extend::*` to `use ncurseswwin::normal::*` in `examples/box_drawing-test.rs`, both normal and extend compile and run nativly.

tarvis CI is reporting the like of
```
  = note: /home/travis/build/narfit66/ncursesw-win-rs/target/debug/deps/libncursesw-559f96d2fa9e6d54.rlib(ncursesw-559f96d2fa9e6d54.ncursesw.2jkianb1-cgu.1.rcgu.o): In function `ncursesw::shims::ncurses::init_extended_pair::hdab8794995ed6992':

          /home/travis/.cargo/git/checkouts/ncursesw-rs-b15859baf9b5f199/5a5da9c/src/shims/ncurses.rs:933: undefined reference to `init_extended_pair'

          collect2: error: ld returned 1 exit status
```
