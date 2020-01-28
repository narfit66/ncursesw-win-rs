/*
    src/window.rs

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

use std::{ptr, fmt, hash::{Hash, Hasher}};

use ncursesw::{stdscr, SCREEN, WINDOW};
use crate::gen::*;

/// A moveable window canvas.
///
/// All methods are either there original ncurses name or were specificlly passed a pointer
/// to `_win_st` the 'w' has been removed for example the ncurses function `mvwgetn_wstr()`
/// has become the method `self.mvgetn_wstr()`.
pub struct Window {
    screen:       Option<SCREEN>, // pointer to optional NCurses screen internal structure.
    handle:       WINDOW,         // pointer to NCurses _win_st internal structure.
    free_on_drop: bool            // free WINDOW handle on drop of structure.
}

impl HasHandle<WINDOW> for Window {
    fn _from(screen: Option<SCREEN>, handle: WINDOW, free_on_drop: bool) -> Self {
        if let Some(sp) = screen {
            assert!(!sp.is_null(), "Window::_from() : screen.is_null()");
        }
        assert!(!handle.is_null(), "Window::_from() : handle.is_null()");

        Self { screen, handle, free_on_drop }
    }

    fn _screen(&self) -> Option<SCREEN> {
        self.screen
    }

    fn _handle(&self) -> WINDOW {
        self.handle
    }
}

impl NCurseswWindow for Window { }
impl IsWindow for Window { }
impl BaseCanvas for Window { }
impl CanSubWindow for Window { }
impl Mouseable for Window { }
impl Moveable for Window { }
impl Derivable for Window { }
impl Scrollable for Window { }

impl HasYAxis for Window { }
impl HasYXAxis for Window { }
impl HasXAxis for Window { }
impl GraphicsTransform for Window { }
impl HasGraphicFunctions for Window { }
impl HasBackground for Window { }
impl HasAttributes for Window { }
impl HasMvAttributes for Window { }
impl HasAddFunctions for Window { }
impl HasMvAddFunctions for Window { }
impl HasDelFunctions for Window { }
impl HasMvDelFunctions for Window { }
impl HasInFunctions for Window { }
impl HasMvInFunctions for Window { }
impl HasInsFunctions for Window { }
impl HasMvInsFunctions for Window { }
impl HasNonBlocking for Window { }
impl HasGetFunctions for Window { }
impl HasMvGetFunctions for Window { }

impl Drop for Window {
    fn drop(&mut self) {
        if self.free_on_drop {
            if let Err(source) = ncursesw::delwin(self.handle) {
                panic!("{} @ {:?}", source, self)
            }
        }
    }
}

/// The default window which is the stdscr created by `ncursesw_entry()`
impl Default for Window {
    fn default() -> Self {
        Self::_from(None, stdscr(), false)
    }
}

unsafe impl Send for Window { } // too make thread safe
unsafe impl Sync for Window { } // too make thread safe

impl PartialEq for Window {
    fn eq(&self, rhs: &Self) -> bool {
        self.screen == rhs._screen() && ptr::eq(self.handle, rhs.handle)
    }
}

impl Eq for Window { }

impl Hash for Window {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.screen.hash(state);
        self.handle.hash(state);
    }
}

impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Window {{ screen: {:?}, handle: {:p}, free_on_drop: {} }}", self.screen, self.handle, self.free_on_drop)
    }
}
