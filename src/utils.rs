/*
    src/utils.rs

    Copyright (c) 2019 Stephen Whittle  All rights reserved.

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom
    the Software is furnished to do so, subject to the following conditions:
    The above copyright notice and this permission notice shall be included
    in all copies or substantial portions of the Software.
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
    THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
    IN THE SOFTWARE.
*/

use std::sync::atomic::Ordering;

use crate::inputmode::InputMode;
use ncursesw::{LcCategory, NCurseswError};
use ncursesw::gen::{ColorsType, ColorType, ColorAttributeTypes};
use crate::ncurses::{INITSCR_CALLED, COLOR_STARTED, INITSCR_NOT_CALLED, INITSCR_ALREADY_CALLED};

lazy_static! {
    static ref START_COLOR_ALREADY_CALLED: &'static str = "ncursesw::start_color() has already been called!";
    static ref START_COLOR_NOT_CALLED: &'static str = "ncursesw::start_color() has not been called!";
}

pub fn setlocale(lc: LcCategory, locale: &str) -> String {
    if INITSCR_CALLED.load(Ordering::SeqCst) {
        panic!(INITSCR_ALREADY_CALLED.to_string());
    };

    ncursesw::setlocale(lc, locale)
}

/// The terminal gets input from the user. Then it's sometimes buffered up. At
/// some point it's passed into the program's input buffer.
///
/// * Character: Input is passed in 1 character at a time, but special
///   characters (such as Ctrl+C and Ctrl+S) are automatically processed for
///   you by the terminal.
/// * Cooked: Input is passed in 1 line at a time, with the special character
///   processing mentioned above enabled.
/// * RawCharacter: Input is passed in 1 character at a time, and special
///   character sequences are not processed automatically.
/// * RawCooked: Input is passed in 1 line at a time, and special
///   character sequences are not processed automatically.
///
/// The default mode is inherited from the terminal that started the program
/// (usually Cooked), so you should _always_ set the desired mode explicitly
/// at the start of your program.
pub fn set_input_mode(mode: InputMode) -> result!(()) {
    if !INITSCR_CALLED.load(Ordering::SeqCst) {
        panic!(INITSCR_NOT_CALLED.to_string());
    };

    match mode {
        InputMode::Character    => ncursesw::cbreak(),
        InputMode::Cooked       => ncursesw::nocbreak(),
        InputMode::RawCharacter => ncursesw::raw(),
        InputMode::RawCooked    => ncursesw::noraw()
    }
}

/// Enables or disables the automatic echoing of input into the window as
/// the user types. Default to on, but you probably want it to be off most
/// of the time.
pub fn set_echo(echoing: bool) -> result!(()) {
    if !INITSCR_CALLED.load(Ordering::SeqCst) {
        panic!(INITSCR_NOT_CALLED.to_string());
    } else if echoing {
        ncursesw::echo()
    } else {
        ncursesw::noecho()
    }
}

pub fn set_newline(newline: bool) -> result!(()) {
    if !INITSCR_CALLED.load(Ordering::SeqCst) {
        panic!(INITSCR_NOT_CALLED.to_string());
    } else if newline {
        ncursesw::nl()
    } else {
        ncursesw::nonl()
    }
}

pub fn start_color() -> result!(()) {
    if !INITSCR_CALLED.load(Ordering::SeqCst) {
        panic!(INITSCR_NOT_CALLED.to_string());
    } else if COLOR_STARTED.load(Ordering::SeqCst) {
        panic!(START_COLOR_ALREADY_CALLED.to_string());
    }

    match ncursesw::start_color() {
        Err(e) => Err(e),
        Ok(_)  => {
            COLOR_STARTED.store(true, Ordering::SeqCst);
            Ok(())
        }
    }
}

pub fn use_default_colors() -> result!(()) {
    if !INITSCR_CALLED.load(Ordering::SeqCst) {
        panic!(INITSCR_NOT_CALLED.to_string());
    } else if !COLOR_STARTED.load(Ordering::SeqCst) {
        panic!(START_COLOR_NOT_CALLED.to_string());
    };

    ncursesw::use_default_colors()
}

pub fn assume_default_colors<S, C, T>(colors: S) -> result!(()) where S: ColorsType<C, T>, C: ColorType<T>, T: ColorAttributeTypes {
    if !INITSCR_CALLED.load(Ordering::SeqCst) {
        panic!(INITSCR_NOT_CALLED.to_string());
    } else if !COLOR_STARTED.load(Ordering::SeqCst) {
        panic!(START_COLOR_NOT_CALLED.to_string());
    };

    ncursesw::assume_default_colors(colors)
}
