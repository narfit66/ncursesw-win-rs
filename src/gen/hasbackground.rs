/*
    src/gen/hasbackground.rs

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

use ncursesw::{ChtypeChar, ComplexChar, WINDOW};
use crate::{NCurseswWinError, gen::HasHandle};

/// Does the window canvas type have ncursesw background functions.
pub trait HasBackground: HasHandle<WINDOW> {
    fn bkgd(&self, ch: ChtypeChar) -> result!(()) {
        Ok(ncursesw::wbkgd(self._handle(), ch)?)
    }

    fn bkgdset(&self, ch: ChtypeChar) {
        ncursesw::wbkgdset(self._handle(), ch)
    }

    fn bkgrnd(&self, wch: ComplexChar) -> result!(()) {
        Ok(ncursesw::wbkgrnd(self._handle(), wch)?)
    }

    fn bkgrndset(&self, wch: ComplexChar) {
        ncursesw::wbkgrndset(self._handle(), wch)
    }

    fn getbkgd(&self) -> ChtypeChar {
        ncursesw::getbkgd(self._handle())
    }

    fn getbkgrnd(&self) -> result!(ComplexChar) {
        Ok(ncursesw::wgetbkgrnd(self._handle())?)
    }
}
