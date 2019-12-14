/*
    src/gen/hasmvaddfunctions.rs

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

use std::convert::{TryFrom, TryInto};

use ncursesw::{ChtypeChar, ChtypeString, ComplexChar, ComplexString, WideString, WINDOW};
use crate::{Origin, NCurseswWinError, gen::{HasHandle, HasYXAxis}};

/// Does the window canvas type have ncursesw origin add functions.
pub trait HasMvAddFunctions: HasHandle<WINDOW> + HasYXAxis {
    fn mvaddchnstr(&self, origin: Origin, chstr: &ChtypeString, length: u16) -> result!(()) {
        assert_origin_hlength!("mvaddchnstr", self.size()?, origin, length);

        Ok(ncursesw::mvwaddchnstr(self._handle(), origin.try_into()?, chstr, i32::try_from(length)?)?)
    }

    fn mvaddch(&self, origin: Origin, ch: ChtypeChar) -> result!(()) {
        assert_origin!("mvaddch", self.size()?, origin);

        Ok(ncursesw::mvwaddch(self._handle(), origin.try_into()?, ch)?)
    }

    fn mvaddchstr(&self, origin: Origin, chstr: &ChtypeString) -> result!(()) {
        assert_origin!("mvaddchstr", self.size()?, origin);

        Ok(ncursesw::mvwaddchstr(self._handle(), origin.try_into()?, chstr)?)
    }

    fn mvaddnstr(&self, origin: Origin, str: &str, length: u16) -> result!(()) {
        assert_origin_hlength!("mvaddnstr", self.size()?, origin, length);

        Ok(ncursesw::mvwaddnstr(self._handle(), origin.try_into()?, str, i32::try_from(length)?)?)
    }

    fn mvaddnwstr(&self, origin: Origin, wstr: &WideString, length: u16) -> result!(()) {
        assert_origin_hlength!("mvaddnwstr", self.size()?, origin, length);

        Ok(ncursesw::mvwaddnwstr(self._handle(), origin.try_into()?, wstr, i32::try_from(length)?)?)
    }

    fn mvaddstr(&self, origin: Origin, str: &str) -> result!(()) {
        assert_origin!("mvaddstr", self.size()?, origin);

        Ok(ncursesw::mvwaddstr(self._handle(), origin.try_into()?, str)?)
    }

    fn mvadd_wchnstr(&self, origin: Origin, wchstr: &ComplexString, length: u16) -> result!(()) {
        assert_origin_hlength!("mvadd_wchnstr", self.size()?, origin, length);

        Ok(ncursesw::mvwadd_wchnstr(self._handle(), origin.try_into()?, wchstr, i32::try_from(length)?)?)
    }

    fn mvadd_wch(&self, origin: Origin, wch: ComplexChar) -> result!(()) {
        assert_origin!("mvadd_wch", self.size()?, origin);

        Ok(ncursesw::mvwadd_wch(self._handle(), origin.try_into()?, wch)?)
    }

    fn mvadd_wchstr(&self, origin: Origin, wchstr: &ComplexString) -> result!(()) {
        assert_origin!("mvadd_wchstr", self.size()?, origin);

        Ok(ncursesw::mvwadd_wchstr(self._handle(), origin.try_into()?, wchstr)?)
    }

    fn mvaddwstr(&self, origin: Origin, wstr: &WideString) -> result!(()) {
        assert_origin!("mvaddwstr", self.size()?, origin);

        Ok(ncursesw::mvwaddwstr(self._handle(), origin.try_into()?, wstr)?)
    }
}
