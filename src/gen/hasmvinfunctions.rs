/*
    src/gen/hasmvinfunctions.rs

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

#![allow(deprecated)]

use std::convert::{TryInto, TryFrom};
use ncursesw::{ChtypeChar, ChtypeString, ComplexChar, ComplexString, WideString, WINDOW};
use crate::{Origin, NCurseswWinError, gen::{HasHandle, HasYXAxis}};

/// Does the window canvas type have ncursesw in origin functions.
pub trait HasMvInFunctions: HasHandle<WINDOW> + HasYXAxis {
    fn mvinchnstr(&self, origin: Origin, length: u16) -> result!(ChtypeString) {
        assert_origin_hlength!("mvinchnstr", self.size()?, origin, length);

        Ok(ncursesw::mvwinchnstr(self._handle(), origin.try_into()?, i32::try_from(length)?)?)
    }

    fn mvinch(&self, origin: Origin) -> result!(ChtypeChar) {
        assert_origin!("mvinch", self.size()?, origin);

        Ok(ncursesw::mvwinch(self._handle(), origin.try_into()?))
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvinchnstr() instead")]
    fn mvinchstr(&self, origin: Origin) -> result!(ChtypeString) {
        assert_origin!("mvinchstr", self.size()?, origin);

        Ok(ncursesw::mvwinchstr(self._handle(), origin.try_into()?)?)
    }

    fn mvinnstr(&self, origin: Origin, length: u16) -> result!(String) {
        assert_origin_hlength!("mvinnstr", self.size()?, origin, length);

        Ok(ncursesw::mvwinnstr(self._handle(), origin.try_into()?, i32::try_from(length)?)?)
    }

    fn mvinnwstr(&self, origin: Origin, length: u16) -> result!(WideString) {
        assert_origin_hlength!("mvinnwstr", self.size()?, origin, length);

        Ok(ncursesw::mvwinnwstr(self._handle(), origin.try_into()?, i32::try_from(length)?)?)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvinnstr() instead")]
    fn mvinstr(&self, origin: Origin) -> result!(String) {
        assert_origin!("mvinstr", self.size()?, origin);

        Ok(ncursesw::mvwinstr(self._handle(), origin.try_into()?)?)
    }

    fn mvin_wchnstr(&self, origin: Origin, length: u16) -> result!(ComplexString) {
        assert_origin_hlength!("mvin_wchnstr", self.size()?, origin, length);

        Ok(ncursesw::mvwin_wchnstr(self._handle(), origin.try_into()?, i32::try_from(length)?)?)
    }

    fn mvin_wch(&self, origin: Origin) -> result!(ComplexChar) {
        assert_origin!("mvin_wch", self.size()?, origin);

        Ok(ncursesw::mvwin_wch(self._handle(), origin.try_into()?)?)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvin_wchnstr() instead")]
    fn mvin_wchstr(&self, origin: Origin) -> result!(ComplexString) {
        assert_origin!("mvin_wchstr", self.size()?, origin);

        Ok(ncursesw::mvwin_wchstr(self._handle(), origin.try_into()?)?)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvinnwstr() instead")]
    fn mvinwstr(&self, origin: Origin) -> result!(WideString) {
        assert_origin!("mvinwstr", self.size()?, origin);

        Ok(ncursesw::mvwinwstr(self._handle(), origin.try_into()?)?)
    }
}
