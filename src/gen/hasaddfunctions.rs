/*
    src/gen/hasaddfunctions.rs

    Copyright (c) 2019-2022 Stephen Whittle  All rights reserved.

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
use ncursesw::{ChtypeChar, ChtypeString, ComplexChar, ComplexString, WideString, WINDOW};
use crate::{NCurseswWinError, gen::HasHandle};

/// Does the window canvas type have ncursesw add functions.
pub trait HasAddFunctions: HasHandle<WINDOW> {
    fn addchnstr(&self, chstr: &ChtypeString, length: Option<u16>) -> result!(()) {
        Ok(ncursesw::waddchnstr(self._handle(), chstr, option_length!(length)?)?)
    }

    fn addch(&self, ch: ChtypeChar) -> result!(()) {
        Ok(ncursesw::waddch(self._handle(), ch)?)
    }

    fn addchstr(&self, chstr: &ChtypeString) -> result!(()) {
        Ok(ncursesw::waddchstr(self._handle(), chstr)?)
    }

    fn addnstr<S: Into<String>>(&self, str: S, length: Option<u16>) -> result!(()) {
        Ok(ncursesw::waddnstr(self._handle(), str, option_length!(length)?)?)
    }

    fn addnwstr(&self, wstr: &WideString, length: Option<u16>) -> result!(()) {
        Ok(ncursesw::waddnwstr(self._handle(), wstr, option_length!(length)?)?)
    }

    fn addstr<S: Into<String>>(&self, str: S) -> result!(()) {
        Ok(ncursesw::waddstr(self._handle(), str)?)
    }

    fn add_wchnstr(&self, wchstr: &ComplexString, length: Option<u16>) -> result!(()) {
        Ok(ncursesw::wadd_wchnstr(self._handle(), wchstr, option_length!(length)?)?)
    }

    fn add_wch(&self, wch: ComplexChar) -> result!(()) {
        Ok(ncursesw::wadd_wch(self._handle(), wch)?)
    }

    fn add_wchstr(&self, wchstr: &ComplexString) -> result!(()) {
        Ok(ncursesw::wadd_wchstr(self._handle(), wchstr)?)
    }

    fn addwstr(&self, wstr: &WideString) -> result!(()) {
        Ok(ncursesw::waddwstr(self._handle(), wstr)?)
    }
}
