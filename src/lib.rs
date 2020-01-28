/*
    src/lib.rs

    Copyright (c) 2019, 2020 Stephen Whittle  All rights reserved.

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

extern crate libc;
extern crate ncursesw;
extern crate custom_error;
#[macro_use]
extern crate lazy_static;
extern crate strum;
extern crate strum_macros;
extern crate errno;

mod macros;

mod cstring;
/// Form module.
pub mod form;
mod funcs;
mod gen;
mod graphics;
mod inputmode;
/// Menu module.
pub mod menu;
mod mouse;
mod ncurses;
mod ncurseswwinerror;
mod nonblockingresult;
mod origin;
mod pad;
mod panels;
mod position;
mod region;
mod ripoff;
mod screen;
mod size;
mod timeout;
mod window;

pub use funcs::*;
pub use gen::*;
pub use graphics::*;
pub use inputmode::*;
pub use mouse::*;
pub use ncurses::*;
pub use ncurseswwinerror::*;
pub use nonblockingresult::*;
pub use origin::*;
pub use pad::*;
pub use panels::*;
pub use position::*;
pub use region::*;
pub use ripoff::*;
pub use screen::*;
pub use size::*;
pub use timeout::*;
pub use window::*;

pub use ncursesw::{normal, extend};
pub use ncursesw::{
    ChtypeChar, ChtypeString, ComplexChar, ComplexString,
    WideChar, WideCharAndAttributes, WideString
};
pub use ncursesw::{
    AttributesColorPairSet, BaseColor, Changed, CharacterResult,
    CursorType, Justification, KeyBinding, LcCategory, Legacy,
    NCursesColorType, NCurseswError, panels::NCurseswPanelsError,
    mouse::NCurseswMouseError, menu::NCurseswMenuError,
    form::NCurseswFormError, Orientation, SoftLabelType
};
pub use ncursesw::{
    AttributesColorPairType, AttributesGeneric, AttributesType,
    ColorAttributeTypes, ColorPairColors, ColorPairType, ColorType,
    ColorsType
};
pub use ncursesw::{
    COLORS, COLOR_PAIRS, ESCDELAY, TABSIZE, baudrate, beep,
    can_change_color, curses_version, def_prog_mode, def_shell_mode,
    define_key, delay_output, doupdate, flash, get_escdelay, getcchar,
    halfdelay, has_colors, has_ic, has_il, has_key, is_term_resized,
    key_defined, key_name, keybound, keyname, keyok, killchar, killwchar,
    longname, mcprint, ncurses_colortype, ncurses_colortype_set,
    ncurses_version, reset_color_pairs, reset_prog_mode, reset_shell_mode,
    resetty, resize_term, resizeterm, savetty, scr_dump, scr_init,
    scr_restore, scr_set, scrl, set_escdelay, set_tabsize, setcchar,
    typeahead, use_legacy_coding
};
pub use ncursesw::mouse::{
    has_mouse, mouseinterval, mouse_version, has_mouse_interface
};
