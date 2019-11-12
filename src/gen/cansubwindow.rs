/*
    src/gen/cansubwindow.rs

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

use ncursesw::{Origin, Size};
use crate::NCurseswWinError;
use crate::window::*;
use crate::gen::*;

/// Is the window canvas type capable of creating a sub-window.
pub trait CanSubWindow: HasHandle + HasYXAxis + NCurseswWindow + IsWindow {
    fn subwin(&self, size: Size, origin: Origin) -> result!(Window) {
        match ncursesw::subwin(self._handle(), size, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Window::from(handle, true))
        }
    }

    /// returns the parent Window for subwindows, or None if their is no parent.
    fn getparent(&self) -> Option<Window> {
        match ncursesw::wgetparent(self._handle()) {
            None         => None,
            Some(handle) => Some(Window::from(handle, false))
        }
    }

    fn getparx(&self) -> result!(i32) {
        match ncursesw::getparx(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(x)       => Ok(x)
        }
    }

    fn getpary(&self) -> result!(i32) {
        match ncursesw::getpary(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(y)       => Ok(y)
        }
    }

    fn getparyx(&self) -> result!(Origin) {
        match ncursesw::getparyx(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(origin)  => Ok(origin)
        }
    }
}
