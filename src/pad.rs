/*
    src/pad.rs

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

use ncursesw::{SCREEN, WINDOW};
use crate::gen::*;

/// A pad canvas.
///
/// All methods are either there original ncurses name or were specificlly passed a pointer
/// to `_win_st` the 'w' has been removed for example the ncurses function `mvwgetn_wstr()`
/// has become the method `self.mvgetn_wstr()`.
pub struct Pad {
    screen:       Option<SCREEN>, // pointer to optional NCurses _screen internal structure.
    handle:       WINDOW,         // pointer to NCurses _win_st internal structure.
    free_on_drop: bool            // free WINDOW handle on drop of structure.
}

impl HasHandle<WINDOW> for Pad {
    fn _from(screen: Option<SCREEN>, handle: WINDOW, free_on_drop: bool) -> Self {
        if let Some(sp) = screen {
            assert!(!sp.is_null(), "Pad::_from() : screen.is_null()");
        }
        assert!(!handle.is_null(), "Pad::_from() : handle.is_null()");

        Self { screen, handle, free_on_drop }
    }

    fn _screen(&self) -> Option<SCREEN> {
        self.screen
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
            if let Err(source) = ncursesw::delwin(self.handle) {
                panic!("{} @ {:?}", source, self)
            }
        }
    }
}

unsafe impl Send for Pad { } // too make thread safe
unsafe impl Sync for Pad { } // too make thread safe

impl PartialEq for Pad {
    fn eq(&self, rhs: &Self) -> bool {
        self.screen == rhs._screen() && ptr::eq(self.handle, rhs.handle)
    }
}

impl Eq for Pad { }

impl Hash for Pad {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.screen.hash(state);
        self.handle.hash(state);
    }
}

impl fmt::Debug for Pad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pad {{ screen: {:?}, handle: {:p}, free_on_drop: {} }}", self.screen, self.handle, self.free_on_drop)
    }
}
