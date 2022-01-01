/*
    src/lib.rs

    Copyright (c) 2019-2021 Stephen Whittle  All rights reserved.

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
#[macro_use]
extern crate lazy_static;
extern crate strum;
extern crate strum_macros;
extern crate errno;

mod macros;

mod cstring;
/// Extended color's, color pairs and attributes module.
pub mod extend;
/// Form module
///
/// The form library provides terminal-independent facilities for composing
/// form screens on character-cell terminals. The library includes: field
/// routines, which create and modify form fields; and form routines, which
/// group fields into forms, display forms on the screen, and handle
/// interaction with the user.
///
/// Your program should set up the locale, e.g.,
/// ```text
/// use gettextrs;
///
/// ncursesw::setlocale(LcCategory::All, "")?;
/// ```
/// so that input/output processing will work.
pub mod form;
mod funcs;
mod gen;
mod graphics;
mod inputmode;
/// Menu module
///
/// The menu library provides terminal-independent facilities for composing
/// menu systems on character-cell terminals. The library includes: item routines,
/// which create and modify menu items; and menu routines, which group items into
/// menus, display menus on the screen, and handle interaction with the user.
pub mod menu;
mod mouse;
mod ncurses;
mod ncurseswwinerror;
mod nonblockingresult;
/// Normal color's, color pairs and attributes module.
pub mod normal;
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

pub use crate::{
    funcs::*, gen::*, graphics::*, inputmode::*, mouse::*, ncurses::*,
    ncurseswwinerror::*, nonblockingresult::*, origin::*, pad::*,
    panels::*, position::*, region::*, ripoff::*, screen::*, size::*,
    timeout::*, window::*
};

pub use ncursesw::{
    ChtypeChar, ChtypeString, ComplexChar, ComplexString,
    WideChar, WideCharAndAttributes, WideString
};
pub use ncursesw::{
    AttributesColorPairSet, Changed, CharacterResult, CursorType, Justification,
    KeyBinding, Legacy, NCursesColorType, NCurseswError, panels::NCurseswPanelsError,
    mouse::NCurseswMouseError, menu::NCurseswMenuError, form::NCurseswFormError,
    Orientation, SoftLabelType
};
pub use ncursesw::{
    AttributesType, ColorAttributeTypes, ColorPairColors, ColorPairType, ColorType,
    ColorsType
};
pub use ncursesw::{
    COLORS, COLOR_PAIRS, ESCDELAY, TABSIZE, baudrate, beep,
    can_change_color, curses_version, def_prog_mode, def_shell_mode,
    define_key, delay_output, doupdate, flash, get_escdelay, getcchar,
    halfdelay, has_colors, has_ic, has_il, has_key, is_term_resized,
    key_defined, key_name, keybound, keyname, keyok, killchar, killwchar,
    longname, mcprint, ncurses_colortype, ncurses_colortype_set,
    ncurses_version, reset_prog_mode, reset_shell_mode, resetty,
    resize_term, resizeterm, savetty, scr_dump, scr_init, scr_restore,
    scr_set, scrl, set_escdelay, set_tabsize, setcchar, typeahead,
    use_legacy_coding
};
pub use ncursesw::features;
pub use ncursesw::mouse::{
    has_mouse, mouseinterval, mouse_version, has_mouse_interface
};
