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
use crate::ncurses::{INITSCR_CALLED, COLOR_STARTED};
use crate::ncurseswwinerror::NCurseswWinError;
use ncursesw::{
    ColorsType, ColorType, ColorAttributeTypes,
    CursorType, LcCategory, SoftLabelType
};

/// Set the locale to be used, required if using unicode representation.
pub fn setlocale(lc: LcCategory, locale: &str) -> result!(String) {
    if INITSCR_CALLED.load(Ordering::SeqCst) {
        Err(NCurseswWinError::InitscrAlreadyCalled)
    } else {
        Ok(ncursesw::setlocale(lc, locale))
    }
}

/// Set the input mode to use within ncurses.
///
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
        Err(NCurseswWinError::InitscrNotCalled)
    } else {
        match match mode {
            InputMode::Character    => ncursesw::cbreak(),
            InputMode::Cooked       => ncursesw::nocbreak(),
            InputMode::RawCharacter => ncursesw::raw(),
            InputMode::RawCooked    => ncursesw::noraw()
        } {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(_)       => Ok(())
        }
    }
}

/// Set echo on or off within ncurses.
///
/// Enables or disables the automatic echoing of input into the window as
/// the user types. Default to on, but you probably want it to be off most
/// of the time.
pub fn set_echo(echoing: bool) -> result!(()) {
    if !INITSCR_CALLED.load(Ordering::SeqCst) {
        Err(NCurseswWinError::InitscrNotCalled)
    } else {
        match if echoing {
            ncursesw::echo()
        } else {
            ncursesw::noecho()
        } {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(_)       => Ok(())
        }
    }
}

/// Control whether the ncurses translates the return key into newline on input,
///
/// This determines wether ncurses translates newline into return and line-feed on output (in either
/// case, the call addch('\n') does the equivalent of return and line feed on the virtual screen).
/// Initially, these translations do occur. If you disable then ncurses will be able to make
/// better use of the line-feed capability, resulting in faster cursor motion.
/// Also, ncurses will then be able to detect the return key.
pub fn set_newline(newline: bool) -> result!(()) {
    if !INITSCR_CALLED.load(Ordering::SeqCst) {
        Err(NCurseswWinError::InitscrNotCalled)
    } else {
        match if newline {
            ncursesw::nl()
        } else {
            ncursesw::nonl()
        } {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(_)       => Ok(())
        }
    }
}

/// Initialise ncurses internal color system.
///
/// This allows ncurses to initialise internal buffers to deal with color pairs etc.
pub fn start_color() -> result!(()) {
    if !INITSCR_CALLED.load(Ordering::SeqCst) {
        Err(NCurseswWinError::InitscrNotCalled)
    } else if COLOR_STARTED.load(Ordering::SeqCst) {
        Err(NCurseswWinError::StartColorAlreadyCalled)
    } else {
        ncursesw::start_color()?;

        COLOR_STARTED.store(true, Ordering::SeqCst);

        Ok(())
    }
}

/// Use the default terminal colors for color pair 0.
///
/// This is usually a white foreground on a black background.
pub fn use_default_colors() -> result!(()) {
    if !INITSCR_CALLED.load(Ordering::SeqCst) {
        Err(NCurseswWinError::InitscrNotCalled)
    } else if !COLOR_STARTED.load(Ordering::SeqCst) {
        Err(NCurseswWinError::StartColorNotCalled)
    } else {
        ncursesw::use_default_colors()?;

        Ok(())
    }
}

/// Use the specialifed colors (foreground and background) as the default colors for defining color pair 0.
pub fn assume_default_colors<S, C, T>(colors: S) -> result!(())
    where S: ColorsType<C, T>,
          C: ColorType<T>,
          T: ColorAttributeTypes
{
    if !INITSCR_CALLED.load(Ordering::SeqCst) {
        Err(NCurseswWinError::InitscrNotCalled)
    } else if !COLOR_STARTED.load(Ordering::SeqCst) {
        Err(NCurseswWinError::StartColorNotCalled)
    } else {
        ncursesw::assume_default_colors(colors)?;

        Ok(())
    }
}

/// Set the cursor type to display
pub fn cursor_set(cursor: CursorType) -> result!(CursorType) {
    if !INITSCR_CALLED.load(Ordering::SeqCst) {
        Err(NCurseswWinError::InitscrNotCalled)
    } else {
        let old_cursor = ncursesw::curs_set(cursor)?;

        Ok(old_cursor)
    }
}

/// Define the softlabels line at the bottom of the screen.
///
/// This will be initialised when ncurses is initialised.
pub fn slk_init(fmt: SoftLabelType) -> result!(()) {
    if INITSCR_CALLED.load(Ordering::SeqCst) {
        Err(NCurseswWinError::InitscrAlreadyCalled)
    } else {
        ncursesw::slk_init(fmt)?;

        Ok(())
    }
}
