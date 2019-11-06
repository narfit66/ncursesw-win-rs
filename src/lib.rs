/*
    src/lib.rs

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

extern crate ncursesw;
extern crate custom_error;
#[macro_use]
extern crate lazy_static;
extern crate strum;
extern crate strum_macros;

use std::time;

mod macros;

mod graphics;
mod inputmode;
mod mouse;
mod ncurses;
mod ncurseswwinerror;
mod pad;
mod panel;
mod ripoff;
mod traits;
mod utils;
mod window;

pub use graphics::*;
pub use inputmode::*;
pub use mouse::*;
pub use ncurses::*;
pub use ncurseswwinerror::*;
pub use pad::*;
pub use panel::*;
pub use ripoff::*;
pub use traits::*;
pub use utils::*;
pub use window::*;

pub use ncursesw::normal;
pub use ncursesw::extend;

pub use ncursesw::{
    ChtypeChar, ChtypeString, ComplexChar, ComplexString, Origin,
    Region, Size, WideChar, WideCharAndAttributes, WideString
};
pub use ncursesw::{
    AttributesColorPairSet, BaseColor, Changed, CharacterResult,
    CursorType, Justification, KeyBinding, LcCategory, Legacy,
    NCursesColorType, NCurseswError, Orientation
};
pub use ncursesw::{
    AttributesColorPairType, AttributesGeneric, AttributesType,
    ColorAttributeTypes, ColorPairColors, ColorPairType, ColorType,
    ColorsType
};
pub use ncursesw::{
    COLORS, COLOR_PAIRS, COLS, ESCDELAY, LINES, TABSIZE, baudrate,
    beep, can_change_color, def_prog_mode, def_shell_mode, define_key,
    delay_output, doupdate, flash, get_escdelay, getcchar, halfdelay,
    has_colors, has_ic, has_il, has_key, is_term_resized, key_defined,
    key_name, keybound, keyname, keyok, killchar, killwchar, longname,
    ncurses_colortype, ncurses_colortype_set, ncurses_version,
    reset_color_pairs, reset_prog_mode, reset_shell_mode, resetty,
    resize_term, resizeterm, setcchar, use_legacy_coding
};
pub use ncursesw::mouse::OriginResult;
pub use ncursesw::mouse::{
    has_mouse, mouseinterval, set_mouseinterval, mouse_trafo,
    mouse_version, has_mouse_interface
};

/// Timeout type, None represents blocking mode.
pub type Timeout = Option<time::Duration>;
