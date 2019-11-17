/*
    src/window.rs

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

use ncursesw::{stdscr, WINDOW};
use crate::gen::*;

/// A moveable window canvas.
///
/// All methods are either there original ncurses name or were specificlly passed a pointer
/// to `_win_st` the 'w' has been removed for example the ncurses function `mvwgetn_wstr()`
/// has become the method `self.mvgetn_wstr()`.
pub struct Window {
    handle:       WINDOW, // pointer to ncurses _win_st internal structure
    free_on_drop: bool    // free WINDOW handle on drop of structure
}

impl HasHandle for Window {
    fn _from(handle: WINDOW, free_on_drop: bool) -> Self {
        Self { handle, free_on_drop }
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
            if let Err(e) = ncursesw::delwin(self.handle) {
                panic!(e.to_string())
            }
        }
    }
}

/// The default window which is the stdscr created by `ncursesw_entry()`
impl Default for Window {
    fn default() -> Self {
        Self::_from(stdscr(), false)
    }
}

unsafe impl Send for Window { } // too make thread safe
unsafe impl Sync for Window { } // too make thread safe
