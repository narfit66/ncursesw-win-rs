/*
    src/ripoff/ripoffwindow.rs

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

use std::{fmt, convert::{TryFrom, TryInto}};

use ncursesw::{SCREEN, WINDOW};
use crate::{Screen, Origin, NCurseswWinError, gen::*};

/// A ripoff line window canvas.
///
/// All methods are either there original ncurses name or were specificlly passed a pointer
/// to `_win_st` the 'w' has been removed for example the ncurses function `wget_wch(*WINDOW)`
/// has become the method `self.get_wch()`.
pub struct RipoffWindow {
    screen: Option<SCREEN>, // pointer to ncurses screen internal structure
    handle: WINDOW          // pointer to ncurses _win_st internal structure
}

impl HasHandle<WINDOW> for RipoffWindow {
    fn _from(screen: Option<SCREEN>, handle: WINDOW, _: bool) -> Self {
        assert!(screen.map_or_else(|| true, |screen| !screen.is_null()), "RipoffWindow::_from() : screen.is_null()");
        assert!(!handle.is_null(), "RipoffWindow::_from() : handle.is_null()");

        Self { screen, handle }
    }

    fn _screen(&self) -> Option<SCREEN> {
        self.screen
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
    /// The screen the window is attached to.
    pub fn screen(&self) -> Option<Screen> {
        self.screen.map_or_else(|| None, |ptr| Some(Screen::_from(ptr, false)))
    }

    /// Get the cursor column.
    pub fn column(&self) -> result!(u16) {
        Ok(u16::try_from(ncursesw::getcurx(self._handle())?)?)
    }

    /// Set the cursor column.
    pub fn set_column(&self, column: u16) -> result!(()) {
        Ok(ncursesw::wmove(self._handle(), Origin { y: 0, x: column }.try_into()?)?)
    }
}

impl Drop for RipoffWindow {
    fn drop(&mut self) {
        if let Err(source) = ncursesw::delwin(self.handle) {
            panic!("{} @ {:?}", source, self)
        }
    }
}

unsafe impl Send for RipoffWindow { } // too make thread safe
unsafe impl Sync for RipoffWindow { } // too make thread safe

impl fmt::Debug for RipoffWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RipoffWindow {{ screen: {:?}, handle: {:p} }}", self.screen, self.handle)
    }
}
