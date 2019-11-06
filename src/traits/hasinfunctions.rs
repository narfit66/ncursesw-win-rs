/*
    src/traits/hasinfunctions.rs

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

use ncursesw::{ChtypeChar, ChtypeString, ComplexChar, ComplexString, WideString};
use crate::ncurseswwinerror::NCurseswWinError;
use crate::traits::*;

/// Does the window canvas type have ncursesw in functions.
pub trait HasInFunctions: HasHandle {
    fn inchnstr(&self, number: i32) -> result!(ChtypeString) {
        match ncursesw::winchnstr(self._handle(), number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(chstr)   => Ok(chstr)
        }
    }

    fn inch(&self) -> ChtypeChar {
        ncursesw::winch(self._handle())
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use inchnstr() instead")]
    fn inchstr(&self) -> result!(ChtypeString) {
        match ncursesw::winchstr(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(chstr)   => Ok(chstr)
        }
    }

    fn innstr(&self, number: i32) -> result!(String) {
        match ncursesw::winnstr(self._handle(), number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(str)     => Ok(str)
        }
    }

    fn innwstr(&self, number: i32) -> result!(WideString) {
        match ncursesw::winnwstr(self._handle(), number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(wstr)    => Ok(wstr)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use innstr() instead")]
    fn instr(&self) -> result!(String) {
        match ncursesw::winstr(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(str)     => Ok(str)
        }
    }

    fn in_wchnstr(&self, number: i32) -> result!(ComplexString) {
        match ncursesw::win_wchnstr(self._handle(), number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(cstr)    => Ok(cstr)
        }
    }

    fn in_wch(&self) -> result!(ComplexChar) {
        match ncursesw::win_wch(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(cc)      => Ok(cc)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use in_wchnstr() instead")]
    fn in_wchstr(&self) -> result!(ComplexString) {
        match ncursesw::win_wchstr(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(cstr)    => Ok(cstr)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use innwstr() instead")]
    fn inwstr(&self) -> result!(WideString) {
        match ncursesw::winwstr(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(wstr)    => Ok(wstr)
        }
    }
}
