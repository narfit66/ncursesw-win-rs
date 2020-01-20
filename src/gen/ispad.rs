/*
    src/gen/ispad.rs

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

use std::{convert::TryInto, os::unix::io::AsRawFd, io::{Read, Write}};

use ncursesw::{ChtypeChar, ComplexChar, WINDOW};
use crate::{Origin, Size, Pad, NCurseswWinError, gen::HasHandle};

/// is the window canvas type a pad.
pub trait IsPad: HasHandle<WINDOW> {
    /// Create a new instance of a Window that will act as a pad.
    fn new(size: Size) -> result!(Pad) {
        Ok(Pad::_from(ncursesw::newpad(size.try_into()?)?, true))
    }

    #[deprecated(since = "0.3.1", note = "Use Pad::new() instead")]
    /// Create a new instance of a Window that will act as a pad.
    fn newpad(size: Size) -> result!(Pad) {
        Pad::new(size)
    }

    fn subpad(&self, size: Size, origin: Origin) -> result!(Pad) {
        Ok(Pad::_from(ncursesw::subpad(self._handle(), size.try_into()?, origin.try_into()?)?, true))
    }

    /// returns the parent Window for subwindows, or None if their is no parent.
    fn getparent(&self) -> Option<Pad> {
        match ncursesw::wgetparent(self._handle()) {
            None         => None,
            Some(handle) => Some(Pad::_from(handle, false))
        }
    }

    /// Create a Pad instance from a previous saved file.
    ///
    /// This uses the file previously generated using the Pad::putwin() routine.
    fn getwin<I: AsRawFd + Read>(file: I) -> result!(Pad) {
        Ok(Pad::_from(ncursesw::getwin(file)?, true))
    }

    fn putwin<O: AsRawFd + Write>(&self, file: O) -> result!(()) {
        Ok(ncursesw::putwin(self._handle(), file)?)
    }

    fn overlay(&self, srcwin: &Pad) -> result!(()) {
        Ok(ncursesw::overlay(srcwin._handle(), self._handle())?)
    }

    fn overwrite(&self, srcwin: &Pad) -> result!(()) {
        Ok(ncursesw::overwrite(srcwin._handle(), self._handle())?)
    }

    fn pechochar(&self, ch: ChtypeChar) -> result!(()) {
        Ok(ncursesw::pechochar(self._handle(), ch)?)
    }

    fn pecho_wchar(&self, wch: ComplexChar) -> result!(()) {
        Ok(ncursesw::pecho_wchar(self._handle(), wch)?)
    }

    fn pnoutrefresh(&self, pmin: Origin, smin: Origin, smax: Origin) -> result!(()) {
        Ok(ncursesw::pnoutrefresh(self._handle(), pmin.try_into()?, smin.try_into()?, smax.try_into()?)?)
    }

    fn prefresh(&self, pmin: Origin, smin: Origin, smax: Origin) -> result!(()) {
        Ok(ncursesw::prefresh(self._handle(), pmin.try_into()?, smin.try_into()?, smax.try_into()?)?)
    }
}
