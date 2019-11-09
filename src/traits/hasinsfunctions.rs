/*
    src/traits/hasinsfunctions.rs

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

use ncursesw::{ChtypeChar, ComplexChar, WideString};
use crate::NCurseswWinError;
use crate::traits::*;

/// Does the window canvas type have ncursesw insert functions.
pub trait HasInsFunctions: HasHandle {
    fn insch(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::winsch(self._handle(), ch)?;

        Ok(())
    }

    fn insnstr(&self, str: &str, number: i32) -> result!(()) {
        ncursesw::winsnstr(self._handle(), str, number)?;

        Ok(())
    }

    fn ins_nwstr(&self, wstr: &WideString, number: i32) -> result!(()) {
        ncursesw::wins_nwstr(self._handle(), wstr, number)?;

        Ok(())
    }

    fn insstr(&self, str: &str) -> result!(()) {
        ncursesw::winsstr(self._handle(), str)?;

        Ok(())
    }

    fn ins_wch(&self, wch: ComplexChar) -> result!(()) {
        ncursesw::wins_wch(self._handle(), wch)?;

        Ok(())
    }

    fn ins_wstr(&self, wstr: &WideString) -> result!(()) {
        ncursesw::wins_wstr(self._handle(), wstr)?;

        Ok(())
    }
}
