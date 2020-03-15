/*
    src/gen/hasyaxis.rs

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

use std::convert::TryFrom;
use ncursesw::WINDOW;
use crate::{Position, NCurseswWinError, gen::HasHandle};

/// Does the window canvas have an y-axis.
pub trait HasYAxis: HasHandle<WINDOW> {
    fn getbegy(&self) -> result!(u16) {
        Ok(u16::try_from(ncursesw::getbegy(self._handle())?)?)
    }

    fn getmaxy(&self) -> result!(u16) {
        Ok(u16::try_from(ncursesw::getmaxy(self._handle())?)?)
    }

    fn getcury(&self) -> result!(u16) {
        Ok(u16::try_from(ncursesw::getcury(self._handle())?)?)
    }

    fn insdelln(&self, position: Position, lines: u16) -> result!(()) {
        let lines = match position {
            Position::InsertAbove => i32::try_from(lines)?,
            Position::DeleteBelow => {
                let neg = i32::try_from(lines)?.overflowing_neg();

                neg.0
            }
        };

        Ok(ncursesw::winsdelln(self._handle(), lines)?)
    }

    fn is_linetouched(&self, line: u16) -> result!(bool) {
        Ok(ncursesw::is_linetouched(self._handle(), i32::try_from(line)?))
    }
}
