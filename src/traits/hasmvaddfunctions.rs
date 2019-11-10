/*
    src/traits/hasmvaddfunctions.rs

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

use ncursesw::{ChtypeChar, ChtypeString, ComplexChar, ComplexString, Origin, WideString};
use crate::NCurseswWinError;
use crate::traits::*;

/// Does the window canvas type have ncursesw origin add functions.
pub trait HasMvAddFunctions: HasHandle + HasYXAxis {
    fn mvaddchnstr(&self, origin: Origin, chstr: &ChtypeString, length: i32) -> result!(()) {
        assert_origin_hlength!("mvaddchnstr", self.size()?, origin, length);

        ncursesw::mvwaddchnstr(self._handle(), origin, chstr, length)?;

        Ok(())
    }

    fn mvaddch(&self, origin: Origin, ch: ChtypeChar) -> result!(()) {
        assert_origin!("mvaddch", self.size()?, origin);

        ncursesw::mvwaddch(self._handle(), origin, ch)?;

        Ok(())
    }

    fn mvaddchstr(&self, origin: Origin, chstr: &ChtypeString) -> result!(()) {
        assert_origin!("mvaddchstr", self.size()?, origin);

        ncursesw::mvwaddchstr(self._handle(), origin, chstr)?;

        Ok(())
    }

    fn mvaddnstr(&self, origin: Origin, str: &str, length: i32) -> result!(()) {
        assert_origin_hlength!("mvaddnstr", self.size()?, origin, length);

        ncursesw::mvwaddnstr(self._handle(), origin, str, length)?;

        Ok(())
    }

    fn mvaddnwstr(&self, origin: Origin, wstr: &WideString, length: i32) -> result!(()) {
        assert_origin_hlength!("mvaddnwstr", self.size()?, origin, length);

        ncursesw::mvwaddnwstr(self._handle(), origin, wstr, length)?;

        Ok(())
    }

    fn mvaddstr(&self, origin: Origin, str: &str) -> result!(()) {
        assert_origin!("mvaddstr", self.size()?, origin);

        ncursesw::mvwaddstr(self._handle(), origin, str)?;

        Ok(())
    }

    fn mvadd_wchnstr(&self, origin: Origin, wchstr: &ComplexString, length: i32) -> result!(()) {
        assert_origin_hlength!("mvadd_wchnstr", self.size()?, origin, length);

        ncursesw::mvwadd_wchnstr(self._handle(), origin, wchstr, length)?;

        Ok(())
    }

    fn mvadd_wch(&self, origin: Origin, wch: ComplexChar) -> result!(()) {
        assert_origin!("mvadd_wch", self.size()?, origin);

        ncursesw::mvwadd_wch(self._handle(), origin, wch)?;

        Ok(())
    }

    fn mvadd_wchstr(&self, origin: Origin, wchstr: &ComplexString) -> result!(()) {
        assert_origin!("mvadd_wchstr", self.size()?, origin);

        ncursesw::mvwadd_wchstr(self._handle(), origin, wchstr)?;

        Ok(())
    }

    fn mvaddwstr(&self, origin: Origin, wstr: &WideString) -> result!(()) {
        assert_origin!("mvaddwstr", self.size()?, origin);

        ncursesw::mvwaddwstr(self._handle(), origin, wstr)?;

        Ok(())
    }
}
