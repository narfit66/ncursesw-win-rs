/*
    src/gen/hasinsfunctions.rs

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

use std::convert::TryFrom;

use ncursesw::{ChtypeChar, ComplexChar, WideString, WINDOW};
use crate::{NCurseswWinError, gen::HasHandle};

/// Does the window canvas type have ncursesw insert functions.
pub trait HasInsFunctions: HasHandle<WINDOW> {
    fn insch(&self, ch: ChtypeChar) -> result!(()) {
        Ok(ncursesw::winsch(self._handle(), ch)?)
    }

    fn insnstr(&self, str: &str, length: Option<u16>) -> result!(()) {
        Ok(ncursesw::winsnstr(self._handle(), str, option_length!(length)?)?)
    }

    fn ins_nwstr(&self, wstr: &WideString, length: Option<u16>) -> result!(()) {
        Ok(ncursesw::wins_nwstr(self._handle(), wstr, option_length!(length)?)?)
    }

    fn insstr(&self, str: &str) -> result!(()) {
        Ok(ncursesw::winsstr(self._handle(), str)?)
    }

    fn ins_wch(&self, wch: ComplexChar) -> result!(()) {
        Ok(ncursesw::wins_wch(self._handle(), wch)?)
    }

    fn ins_wstr(&self, wstr: &WideString) -> result!(()) {
        Ok(ncursesw::wins_wstr(self._handle(), wstr)?)
    }
}
