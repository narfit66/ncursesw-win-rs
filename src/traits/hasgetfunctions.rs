/*
    src/traits/hasgetfunctions.rs

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

use ncursesw::{CharacterResult, WideChar, WideString};
use crate::{NonBlockingResult, Timeout, NCurseswWinError};
use crate::traits::*;

/// Does the window canvas type have ncursesw get functions.
pub trait HasGetFunctions: HasHandle + HasNonBlocking {
    fn getch(&self) -> result!(CharacterResult<char>) {
        match ncursesw::wgetch(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(result)  => Ok(result)
        }
    }

    fn getnstr(&self, number: i32) -> result!(String) {
        match ncursesw::wgetnstr(self._handle(), number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(str)     => Ok(str)
        }
    }

    fn getn_wstr(&self, number: i32) -> result!(WideString) {
        match ncursesw::wgetn_wstr(self._handle(), number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(wstr)    => Ok(wstr)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use getnstr() instead")]
    fn getstr(&self) -> result!(String) {
        match ncursesw::wgetstr(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(str)     => Ok(str)
        }
    }

    fn get_wch(&self) -> result!(CharacterResult<WideChar>) {
        match ncursesw::wget_wch(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(result)  => Ok(result)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use getn_wstr() instead")]
    fn get_wstr(&self) -> result!(WideString) {
        match ncursesw::wget_wstr(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(wstr)    => Ok(wstr)
        }
    }

    nonblocking_get!(getch_nonblocking, getch, "wgetch", char);
    nonblocking_get!(get_wch_nonblocking, get_wch, "wget_wch", WideChar);
}
