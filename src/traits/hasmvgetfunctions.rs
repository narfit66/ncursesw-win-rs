/*
    src/traits/hasmvgetfunctions.rs

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

use ncursesw::{CharacterResult, Origin, WideChar, WideString};
use crate::{NonBlockingResult, Timeout, NCurseswWinError};
use crate::traits::*;

/// Does the window canvas type have ncursesw get origin functions.
pub trait HasMvGetFunctions: HasHandle + HasYXAxis + HasNonBlocking {
    fn mvgetch(&self, origin: Origin) -> result!(CharacterResult<char>) {
        match ncursesw::mvwgetch(self._handle(), origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(result)  => Ok(result)
        }
    }

    fn mvgetnstr(&self, origin: Origin, number: i32) -> result!(String) {
        match ncursesw::mvwgetnstr(self._handle(), origin, number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(str)     => Ok(str)
        }
    }

    fn mvgetn_wstr(&self, origin: Origin, number: i32) -> result!(WideString) {
        match ncursesw::mvwgetn_wstr(self._handle(), origin, number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(wstr)    => Ok(wstr)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvgetnstr() instead")]
    fn mvgetstr(&self, origin: Origin) -> result!(String) {
        match ncursesw::mvwgetstr(self._handle(), origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(str)     => Ok(str)
        }
    }

    fn mvget_wch(&self, origin: Origin) -> result!(CharacterResult<WideChar>) {
        match ncursesw::mvwget_wch(self._handle(), origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(result)  => Ok(result)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvgetn_wstr() instead")]
    fn mvget_wstr(&self, origin: Origin) -> result!(WideString) {
        match ncursesw::mvwget_wstr(self._handle(), origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(wstr)    => Ok(wstr)
        }
    }

    nonblocking_get_with_origin!(mvgetch_nonblocking, mvgetch, "mvwgetch", char);
    nonblocking_get_with_origin!(mvget_wch_nonblocking, mvget_wch, "mvwget_wch", WideChar);
}
