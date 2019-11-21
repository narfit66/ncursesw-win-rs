/*
    src/ripoff/ripoffwindow.rs

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

use ncursesw::{Origin, WINDOW};
use crate::{NCurseswWinError, gen::*};

/// A ripoff line window canvas.
///
/// All methods are either there original ncurses name or were specificlly passed a pointer
/// to `_win_st` the 'w' has been removed for example the ncurses function `mvwgetn_wstr()`
/// has become the method `self.mvgetn_wstr()`.
pub struct RipoffWindow {
    handle: WINDOW // pointer to ncurses _win_st internal structure
}

impl HasHandle for RipoffWindow {
    fn _from(handle: WINDOW, _: bool) -> Self {
        Self { handle }
    }

    fn _handle(&self) -> WINDOW {
        self.handle
    }
}

impl IsWindow for RipoffWindow { }
impl BaseCanvas for RipoffWindow { }
impl Mouseable for RipoffWindow { }

impl HasXAxis for RipoffWindow { }
impl HasBackground for RipoffWindow { }
impl HasAttributes for RipoffWindow { }
impl HasAddFunctions for RipoffWindow { }
impl HasDelFunctions for RipoffWindow { }
impl HasInFunctions for RipoffWindow { }
impl HasInsFunctions for RipoffWindow { }
impl HasNonBlocking for RipoffWindow { }
impl HasGetFunctions for RipoffWindow { }

impl RipoffWindow {
    /// get the cursor column on the ripoff window.
    pub fn column(&self) -> result!(i32) {
        let column = ncursesw::getcurx(self._handle())?;

        Ok(column)
    }

    /// set the cursor column on the ripoff window.
    pub fn set_column(&self, column: i32) -> result!(()) {
        ncursesw::wmove(self._handle(), Origin { y: 0, x: column })?;

        Ok(())
    }
}

impl Drop for RipoffWindow {
    fn drop(&mut self) {
        if let Err(source) = ncursesw::delwin(self.handle) {
            panic!(source.to_string())
        }
    }
}

unsafe impl Send for RipoffWindow { } // too make thread safe
unsafe impl Sync for RipoffWindow { } // too make thread safe
