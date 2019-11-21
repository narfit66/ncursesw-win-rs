/*
    src/pad.rs

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

use std::ptr;

use ncursesw::WINDOW;
use crate::gen::*;

/// A pad canvas.
///
/// All methods are either there original ncurses name or were specificlly passed a pointer
/// to `_win_st` the 'w' has been removed for example the ncurses function `mvwgetn_wstr()`
/// has become the method `self.mvgetn_wstr()`.
pub struct Pad {
    handle:       WINDOW, // pointer to ncurses _win_st internal structure
    free_on_drop: bool    // free WINDOW handle on drop of structure
}

impl HasHandle for Pad {
    fn _from(handle: WINDOW, free_on_drop: bool) -> Self {
        Self { handle, free_on_drop }
    }

    fn _handle(&self) -> WINDOW {
        self.handle
    }
}

impl IsPad for Pad { }
impl BaseCanvas for Pad { }
impl Mouseable for Pad { }
impl Scrollable for Pad { }

impl HasYAxis for Pad { }
impl HasYXAxis for Pad { }
impl HasXAxis for Pad { }
impl GraphicsTransform for Pad { }
impl HasGraphicFunctions for Pad { }
impl HasBackground for Pad { }
impl HasAttributes for Pad { }
impl HasMvAttributes for Pad { }
impl HasAddFunctions for Pad { }
impl HasMvAddFunctions for Pad { }
impl HasDelFunctions for Pad { }
impl HasMvDelFunctions for Pad { }
impl HasInFunctions for Pad { }
impl HasMvInFunctions for Pad { }
impl HasInsFunctions for Pad { }
impl HasMvInsFunctions for Pad { }
impl HasNonBlocking for Pad { }
impl HasGetFunctions for Pad { }
impl HasMvGetFunctions for Pad { }

impl Drop for Pad {
    fn drop(&mut self) {
        if self.free_on_drop {
            if let Err(e) = ncursesw::delwin(self.handle) {
                panic!(e.to_string())
            }
        }
    }
}

unsafe impl Send for Pad { } // too make thread safe
unsafe impl Sync for Pad { } // too make thread safe

impl PartialEq for Pad {
    fn eq(&self, rhs: &Self) -> bool {
        ptr::eq(self.handle, rhs.handle)
    }
}

impl Eq for Pad { }
