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
use crate::ncurseswwinerror::NCurseswWinError;
use crate::traits::*;

/// Does the window canvas type have ncursesw origin add functions.
pub trait HasMvAddFunctions: HasHandle + HasYXAxis {
    fn mvaddchnstr(&self, origin: Origin, chstr: &ChtypeString, number: i32) -> result!(()) {
        ncursesw::mvwaddchnstr(self._handle(), origin, chstr, number)?;

        Ok(())
    }

    fn mvaddch(&self, origin: Origin, ch: ChtypeChar) -> result!(()) {
        ncursesw::mvwaddch(self._handle(), origin, ch)?;

        Ok(())
    }

    fn mvaddchstr(&self, origin: Origin, chstr: &ChtypeString) -> result!(()) {
        ncursesw::mvwaddchstr(self._handle(), origin, chstr)?;

        Ok(())
    }

    fn mvaddnstr(&self, origin: Origin, str: &str, number: i32) -> result!(()) {
        ncursesw::mvwaddnstr(self._handle(), origin, str, number)?;

        Ok(())
    }

    fn mvaddnwstr(&self, origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
        ncursesw::mvwaddnwstr(self._handle(), origin, wstr, number)?;

        Ok(())
    }

    fn mvaddstr(&self, origin: Origin, str: &str) -> result!(()) {
        ncursesw::mvwaddstr(self._handle(), origin, str)?;

        Ok(())
    }

    fn mvadd_wchnstr(&self, origin: Origin, wchstr: &ComplexString, number: i32) -> result!(()) {
        ncursesw::mvwadd_wchnstr(self._handle(), origin, wchstr, number)?;

        Ok(())
    }

    fn mvadd_wch(&self, origin: Origin, wch: ComplexChar) -> result!(()) {
        ncursesw::mvwadd_wch(self._handle(), origin, wch)?;

        Ok(())
    }

    fn mvadd_wchstr(&self, origin: Origin, wchstr: &ComplexString) -> result!(()) {
        ncursesw::mvwadd_wchstr(self._handle(), origin, wchstr)?;

        Ok(())
    }

    fn mvaddwstr(&self, origin: Origin, wstr: &WideString) -> result!(()) {
        ncursesw::mvwaddwstr(self._handle(), origin, wstr)?;

        Ok(())
    }
}
