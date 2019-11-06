/*
    src/traits/ispad.rs

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

use ncursesw::{ChtypeChar, ComplexChar, Origin, Size};
use crate::ncurseswwinerror::NCurseswWinError;
use crate::traits::*;
use crate::window::*;

/// is the window canvas type a pad.
pub trait IsPad: HasHandle + Drop {
    /// Create a new instance of a Window that will act as a pad.
    // TODO: change result to Pad when it's done.
    fn newpad(size: Size) -> result!(Window) {
        match ncursesw::newpad(size) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Window::from(handle, true))
        }
    }

    // TODO: change result to Pad when it's done.
    fn subpad(&self, size: Size, origin: Origin) -> result!(Window) {
        match ncursesw::subpad(self._handle(), size, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Window::from(handle, true))
        }
    }

    /// returns the parent Window for subwindows, or None if their is no parent.
    // TODO: change result to Pad when it's done.
    fn getparent(&self) -> Option<Window> {
        match ncursesw::wgetparent(self._handle()) {
            None         => None,
            Some(handle) => Some(Window::from(handle, false))
        }
    }

    /// Create a Window instance from a previous saved file.
    ///
    /// This uses the file previously generated using the Window.putwin() routine.
    // TODO: change result to Pad when it's done.
    fn getwin(path: &path::Path) -> result!(Window) {
        match ncursesw::getwin(path) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Window::from(handle, true))
        }
    }

    // TODO: change result to Pad when it's done.
    fn overlay(&self, srcwin: &Window) -> result!(()) {
        ncursesw::overlay(srcwin._handle(), self._handle())?;

        Ok(())
    }

    // TODO: change result to Pad when it's done.
    fn overwrite(&self, srcwin: &Window) -> result!(()) {
        ncursesw::overwrite(srcwin._handle(), self._handle())?;

        Ok(())
    }

    // make all the following deperciated and use Window equivilants
    // eg. pechochar() -> echochar() but still calls ncursesw::pechochar().
    // maybe move Window::echochar() to IsWindow.

    fn pechochar(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::pechochar(self._handle(), ch)?;

        Ok(())
    }

    fn pecho_wchar(&self, wch: ComplexChar) -> result!(()) {
        ncursesw::pecho_wchar(self._handle(), wch)?;

        Ok(())
    }

    fn pnoutrefresh(&self, pmin: Origin, smin: Origin, smax: Origin) -> result!(()) {
        ncursesw::pnoutrefresh(self._handle(), pmin, smin, smax)?;

        Ok(())
    }

    fn prefresh(&self, pmin: Origin, smin: Origin, smax: Origin) -> result!(()) {
        ncursesw::prefresh(self._handle(), pmin, smin, smax)?;

        Ok(())
    }
}
