/*
    src/gen/hasmvinsfunctions.rs

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

use std::convert::{TryFrom, TryInto};
use ncursesw::{ChtypeChar, ComplexChar, WideString, WINDOW};
use crate::{Origin, NCurseswWinError, gen::{HasHandle, HasYXAxis}};

/// Does the window canvas type have ncursesw insert origin functions.
pub trait HasMvInsFunctions: HasHandle<WINDOW> + HasYXAxis {
    fn mvinsch(&self, origin: Origin, ch: ChtypeChar) -> result!(()) {
        assert_origin!("mvinsch", self.size()?, origin);

        Ok(ncursesw::mvwinsch(self._handle(), origin.try_into()?, ch)?)
    }

    fn mvinsnstr(&self, origin: Origin, str: &str, length: Option<u16>) -> result!(()) {
        assert_origin!("mvinsnstr", self.size()?, origin);

        Ok(ncursesw::mvwinsnstr(self._handle(), origin.try_into()?, str, option_length!(length)?)?)
    }

    fn mvins_nwstr(&self, origin: Origin, wstr: &WideString, length: Option<u16>) -> result!(()) {
        assert_origin!("mvins_nwstr", self.size()?, origin);

        Ok(ncursesw::mvwins_nwstr(self._handle(), origin.try_into()?, wstr, option_length!(length)?)?)
    }

    fn mvinsstr(&self, origin: Origin, str: &str) -> result!(()) {
        assert_origin!("mvinsstr", self.size()?, origin);

        Ok(ncursesw::mvwinsstr(self._handle(), origin.try_into()?, str)?)
    }

    fn mvins_wch(&self, origin: Origin, wch: ComplexChar) -> result!(()) {
        assert_origin!("mvins_wch", self.size()?, origin);

        Ok(ncursesw::mvwins_wch(self._handle(), origin.try_into()?, wch)?)
    }

    fn mvins_wstr(&self, origin: Origin, wstr: &WideString) -> result!(()) {
        assert_origin!("mvins_wstr", self.size()?, origin);

        Ok(ncursesw::mvwins_wstr(self._handle(), origin.try_into()?, wstr)?)
    }
}
