/*
    src/traits/hasaddfunctions.rs

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

use ncursesw::{ChtypeChar, ChtypeString, ComplexChar, ComplexString, WideString};
use crate::NCurseswWinError;
use crate::traits::*;

/// Does the window canvas type have ncursesw add functions.
pub trait HasAddFunctions: HasHandle {
    fn addchnstr(&self, chstr: &ChtypeString, number: i32) -> result!(()) {
        ncursesw::waddchnstr(self._handle(), chstr, number)?;

        Ok(())
    }

    fn addch(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::waddch(self._handle(), ch)?;

        Ok(())
    }

    fn addchstr(&self, chstr: &ChtypeString) -> result!(()) {
        ncursesw::waddchstr(self._handle(), chstr)?;

        Ok(())
    }

    fn addnstr(&self, str: &str, number: i32) -> result!(()) {
        ncursesw::waddnstr(self._handle(), str, number)?;

        Ok(())
    }

    fn addnwstr(&self, wstr: &WideString, number: i32) -> result!(()) {
        ncursesw::waddnwstr(self._handle(), wstr, number)?;

        Ok(())
    }

    fn addstr(&self, str: &str) -> result!(()) {
        ncursesw::waddstr(self._handle(), str)?;

        Ok(())
    }

    fn add_wchnstr(&self, wchstr: &ComplexString, number: i32) -> result!(()) {
        ncursesw::wadd_wchnstr(self._handle(), wchstr, number)?;

        Ok(())
    }

    fn add_wch(&self, wch: ComplexChar) -> result!(()) {
        ncursesw::wadd_wch(self._handle(), wch)?;

        Ok(())
    }

    fn add_wchstr(&self, wchstr: &ComplexString) -> result!(()) {
        ncursesw::wadd_wchstr(self._handle(), wchstr)?;

        Ok(())
    }

    fn addwstr(&self, wstr: &WideString) -> result!(()) {
        ncursesw::waddwstr(self._handle(), wstr)?;

        Ok(())
    }
}
