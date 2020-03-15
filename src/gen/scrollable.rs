/*
    src/gen/scrollable.rs

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
use ncursesw::WINDOW;
use crate::{Region, NCurseswWinError, gen::{HasHandle, HasYXAxis}};

/// Is the window canvas type scrollable.
pub trait Scrollable: HasHandle<WINDOW> + HasYXAxis {
    fn scroll(&self) -> result!(()) {
        Ok(ncursesw::scroll(self._handle())?)
    }

    fn scrl(&self, lines: u16) -> result!(()) {
        Ok(ncursesw::wscrl(self._handle(), i32::try_from(lines)?)?)
    }

    fn scrollok(&self, bf: bool) -> result!(()) {
        Ok(ncursesw::scrollok(self._handle(), bf)?)
    }

    fn setscrreg(&self, region: Region) -> result!(()) {
        Ok(ncursesw::wsetscrreg(self._handle(), region.try_into()?)?)
    }

    fn getscrreg(&self) -> result!(Region) {
        Ok(Region::try_from(ncursesw::wgetscrreg(self._handle())?)?)
    }
}
