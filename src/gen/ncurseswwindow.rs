/*
    src/gen/ncurseswwindow.rs

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

use std::{convert::TryInto, os::unix::io::AsRawFd, io::Read};

use ncursesw::WINDOW;
use crate::{Screen, Origin, Size, Window, NCurseswWinError, gen::{HasHandle, IsWindow}};

/// Is the window canvas a ncursesw window type.
pub trait NCurseswWindow: HasHandle<WINDOW> + IsWindow {
    /// Create a new instance of a Window
    fn new(size: Size, origin: Origin) -> result!(Window) {
        Ok(Window::_from(None, ncursesw::newwin(size.try_into()?, origin.try_into()?)?, true))
    }

    #[deprecated(since = "0.3.1", note = "Use Window::new() instead")]
    /// Create a new instance of a Window
    fn newwin(size: Size, origin: Origin) -> result!(Window) {
        Self::new(size, origin)
    }

    /// Create a new instance of a Window
    fn new_sp(screen: &Screen, size: Size, origin: Origin) -> result!(Window) {
        Ok(Window::_from(Some(screen._handle()), ncursesw::newwin_sp(screen._handle(), size.try_into()?, origin.try_into()?)?, true))
    }

    #[deprecated(since = "0.5.0", note = "Use Window::new() instead")]
    fn newwin_sp(screen: &Screen, size: Size, origin: Origin) -> result!(Window) {
        Self::new_sp(screen, size, origin)
    }

    fn copywin(
        &self,
        dstwin: &Window,
        smin: Origin,
        dmin: Origin,
        dmax: Origin,
        overlay: bool) -> result!(())
    {
        Ok(ncursesw::copywin(self._handle(), dstwin._handle(), smin.try_into()?, dmin.try_into()?, dmax.try_into()?, overlay)?)
    }

    fn dupwin(&self) -> result!(Window) {
        Ok(Window::_from(self._screen(), ncursesw::dupwin(self._handle())?, true))
    }

    /// Create a Window instance from a previous saved file.
    ///
    /// This uses the file previously generated using the Window.putwin() routine.
    fn getwin<I: AsRawFd + Read>(file: I) -> result!(Window) {
        Ok(Window::_from(None, ncursesw::getwin(file)?, true))
    }

    fn overlay(&self, srcwin: &Window) -> result!(()) {
        Ok(ncursesw::overlay(srcwin._handle(), self._handle())?)
    }

    fn overwrite(&self, srcwin: &Window) -> result!(()) {
        Ok(ncursesw::overwrite(srcwin._handle(), self._handle())?)
    }

    fn refresh(&self) -> result!(()) {
        Ok(ncursesw::wrefresh(self._handle())?)
    }
}
