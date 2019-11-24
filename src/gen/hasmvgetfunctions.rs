/*
    src/gen/hasmvgetfunctions.rs

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

use std::convert::{TryInto, TryFrom};

use ncursesw::{CharacterResult, WideChar, WideString};
use crate::{
    Origin, NonBlockingResult, Timeout, NCurseswWinError,
    gen::{HasHandle, HasYXAxis, HasNonBlocking}
};

/// Does the window canvas type have ncursesw get origin functions.
pub trait HasMvGetFunctions: HasHandle + HasYXAxis + HasNonBlocking {
    fn mvgetch(&self, origin: Origin) -> result!(CharacterResult<char>) {
        assert_origin!("mvgetch", self.size()?, origin);

        Ok(ncursesw::mvwgetch(self._handle(), origin.try_into()?)?)
    }

    fn mvgetnstr(&self, origin: Origin, length: u16) -> result!(String) {
        assert_origin_hlength!("mvgetnstr", self.size()?, origin, length);

        Ok(ncursesw::mvwgetnstr(self._handle(), origin.try_into()?, i32::try_from(length)?)?)
    }

    fn mvgetn_wstr(&self, origin: Origin, length: u16) -> result!(WideString) {
        assert_origin_hlength!("mvgetn_wstr", self.size()?, origin, length);

        Ok(ncursesw::mvwgetn_wstr(self._handle(), origin.try_into()?, i32::try_from(length)?)?)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvgetnstr() instead")]
    fn mvgetstr(&self, origin: Origin) -> result!(String) {
        assert_origin!("mvgetstr", self.size()?, origin);

        Ok(ncursesw::mvwgetstr(self._handle(), origin.try_into()?)?)
    }

    fn mvget_wch(&self, origin: Origin) -> result!(CharacterResult<WideChar>) {
        assert_origin!("mvget_wch", self.size()?, origin);

        Ok(ncursesw::mvwget_wch(self._handle(), origin.try_into()?)?)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvgetn_wstr() instead")]
    fn mvget_wstr(&self, origin: Origin) -> result!(WideString) {
        assert_origin!("mvget_wstr", self.size()?, origin);

        Ok(ncursesw::mvwget_wstr(self._handle(), origin.try_into()?)?)
    }

    nonblocking_get_with_origin!(mvgetch_nonblocking, "mvgetch_nonblocking", mvgetch, "mvwgetch", char);
    nonblocking_get_with_origin!(mvget_wch_nonblocking, "mvget_wch_nonblocking", mvget_wch, "mvwget_wch", WideChar);
}
