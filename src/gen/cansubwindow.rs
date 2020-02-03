/*
    src/gen/cansubwindow.rs

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

use std::convert::{TryFrom, TryInto};

use ncursesw::WINDOW;
use crate::{
    Origin, Size, NCurseswWinError,
    window::Window, gen::{HasHandle, HasYXAxis, NCurseswWindow, IsWindow}
};

/// Is the window canvas type capable of creating a sub-window.
pub trait CanSubWindow: HasHandle<WINDOW> + HasYXAxis + NCurseswWindow + IsWindow {
    fn subwin(&self, size: Size, origin: Origin) -> result!(Window) {
        Ok(Window::_from(self._screen(), ncursesw::subwin(self._handle(), size.try_into()?, origin.try_into()?)?, true))
    }

    /// returns the parent Window for subwindows, or None if their is no parent.
    fn getparent(&self) -> Option<Window> {
        ncursesw::wgetparent(self._handle()).map_or_else(|| None, |ptr| Some(Window::_from(self._screen(), ptr, false)))
    }

    fn getparx(&self) -> result!(u16) {
        Ok(u16::try_from(ncursesw::getparx(self._handle())?)?)
    }

    fn getpary(&self) -> result!(u16) {
        Ok(u16::try_from(ncursesw::getpary(self._handle())?)?)
    }

    fn getparyx(&self) -> result!(Origin) {
        Ok(Origin::try_from(ncursesw::getparyx(self._handle())?)?)
    }
}
