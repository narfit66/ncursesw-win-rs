/*
    src/traits/iswindow.rs

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

use std::path;

use ncursesw::{ChtypeChar, ComplexChar};
use crate::ncurseswwinerror::NCurseswWinError;
use crate::traits::*;
use crate::window::*;

/// is the window canvas type a window.
pub trait IsWindow: HasHandle + Drop {
    fn dupwin(&self) -> result!(Window) {
        match ncursesw::dupwin(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Window::from(handle, true))
        }
    }

    /// Create a Window instance from a previous saved file.
    ///
    /// This uses the file previously generated using the Window.putwin() routine.
    fn getwin(path: &path::Path) -> result!(Window) {
        match ncursesw::getwin(path) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Window::from(handle, true))
        }
    }

    fn putwin(&self, path: &path::Path) -> result!(()) {
        ncursesw::putwin(self._handle(), path)?;

        Ok(())
    }

    fn echochar(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::wechochar(self._handle(), ch)?;

        Ok(())
    }

    fn echo_wchar(&self, wch: ComplexChar) -> result!(()) {
        ncursesw::wecho_wchar(self._handle(), wch)?;

        Ok(())
    }

    fn noutrefresh(&self) -> result!(()) {
        ncursesw::wnoutrefresh(self._handle())?;

        Ok(())
    }
}
