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
use crate::pad::*;

/// is the window canvas type a pad.
pub trait IsPad: HasHandle + Drop + Sync + Send {
    /// Create a new instance of a Window that will act as a pad.
    fn new(size: Size) -> result!(Pad) {
        match ncursesw::newpad(size) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Pad::from(handle, true))
        }
    }

    #[deprecated(since = "0.3.1", note = "Use Pad::new() instead")]
    /// Create a new instance of a Window that will act as a pad.
    fn newpad(size: Size) -> result!(Pad) {
        Pad::new(size)
    }

    fn subpad(&self, size: Size, origin: Origin) -> result!(Pad) {
        match ncursesw::subpad(self._handle(), size, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Pad::from(handle, true))
        }
    }

    /// returns the parent Window for subwindows, or None if their is no parent.
    fn getparent(&self) -> Option<Pad> {
        match ncursesw::wgetparent(self._handle()) {
            None         => None,
            Some(handle) => Some(Pad::from(handle, false))
        }
    }

    /// Create a Window instance from a previous saved file.
    ///
    /// This uses the file previously generated using the Window.putwin() routine.
    fn getwin(path: &path::Path) -> result!(Pad) {
        match ncursesw::getwin(path) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Pad::from(handle, true))
        }
    }

    fn putwin(&self, path: &path::Path) -> result!(()) {
        ncursesw::putwin(self._handle(), path)?;

        Ok(())
    }

    fn overlay(&self, srcwin: &Pad) -> result!(()) {
        ncursesw::overlay(srcwin._handle(), self._handle())?;

        Ok(())
    }

    fn overwrite(&self, srcwin: &Pad) -> result!(()) {
        ncursesw::overwrite(srcwin._handle(), self._handle())?;

        Ok(())
    }

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
