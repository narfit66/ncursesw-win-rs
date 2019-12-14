/*
    src/gen/hasinfunctions.rs

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

#![allow(deprecated)]

use std::convert::TryFrom;

use ncursesw::{ChtypeChar, ChtypeString, ComplexChar, ComplexString, WideString, WINDOW};
use crate::{NCurseswWinError, gen::HasHandle};

/// Does the window canvas type have ncursesw in functions.
pub trait HasInFunctions: HasHandle<WINDOW> {
    fn inchnstr(&self, length: u16) -> result!(ChtypeString) {
        Ok(ncursesw::winchnstr(self._handle(), i32::try_from(length)?)?)
    }

    fn inch(&self) -> ChtypeChar {
        ncursesw::winch(self._handle())
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use inchnstr() instead")]
    fn inchstr(&self) -> result!(ChtypeString) {
        Ok(ncursesw::winchstr(self._handle())?)
    }

    fn innstr(&self, length: u16) -> result!(String) {
        Ok(ncursesw::winnstr(self._handle(), i32::try_from(length)?)?)
    }

    fn innwstr(&self, length: u16) -> result!(WideString) {
        Ok(ncursesw::winnwstr(self._handle(), i32::try_from(length)?)?)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use innstr() instead")]
    fn instr(&self) -> result!(String) {
        Ok(ncursesw::winstr(self._handle())?)
    }

    fn in_wchnstr(&self, length: u16) -> result!(ComplexString) {
        Ok(ncursesw::win_wchnstr(self._handle(), i32::try_from(length)?)?)
    }

    fn in_wch(&self) -> result!(ComplexChar) {
        Ok(ncursesw::win_wch(self._handle())?)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use in_wchnstr() instead")]
    fn in_wchstr(&self) -> result!(ComplexString) {
        Ok(ncursesw::win_wchstr(self._handle())?)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use innwstr() instead")]
    fn inwstr(&self) -> result!(WideString) {
        Ok(ncursesw::winwstr(self._handle())?)
    }
}
