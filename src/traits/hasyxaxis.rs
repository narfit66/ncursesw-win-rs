/*
    src/traits/hasyxaxis.rs

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

use ncursesw::{Changed, Origin, Region, Size};
use crate::NCurseswWinError;
use crate::traits::*;

/// Does the window canvas have an x and y axis.
pub trait HasYXAxis: HasHandle + HasYAxis + HasXAxis {
    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use origin() instead")]
    /// get the origin of the window.
    fn getbegyx(&self) -> result!(Origin) {
        self.origin()
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use size() instead")]
    /// get the size of the window.
    fn getmaxyx(&self) -> result!(Size) {
        self.size()
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use cursor() instead")]
    /// get the cursor origin on the window.
    fn getcuryx(&self) -> result!(Origin) {
        self.cursor()
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use set_cursor() instead")]
    /// set the cursor origin on the window.
    fn r#move(&self, origin: Origin) -> result!(()) {
        self.set_cursor(origin)
    }

    fn redrawln(&self, region: Region) -> result!(()) {
        ncursesw::wredrawln(self._handle(), region.top, region.bottom - region.top)?;

        Ok(())
    }

    fn resize(&self, size: Size) -> result!(()) {
        ncursesw::wresize(self._handle(), size)?;

        Ok(())
    }

    fn touchline(&self, region: Region) -> result!(()) {
        ncursesw::touchline(self._handle(), region.top, region.bottom - region.top)?;

        Ok(())
    }

    fn touchln(&self, region: Region, changed: Changed) -> result!(()) {
        ncursesw::wtouchln(self._handle(), region.top, region.bottom - region.top, changed)?;

        Ok(())
    }

    /// get the origin of the window.
    fn origin(&self) -> result!(Origin) {
        match ncursesw::getbegyx(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(origin)  => Ok(origin)
        }
    }

    /// get the size of the window.
    fn size(&self) -> result!(Size) {
        match ncursesw::getmaxyx(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(size)    => Ok(size)
        }
    }

    /// get the cursor origin on the window.
    fn cursor(&self) -> result!(Origin) {
        match ncursesw::getcuryx(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(origin)  => Ok(origin)
        }
    }

    /// set the cursor origin on the window.
    fn set_cursor(&self, origin: Origin) -> result!(()) {
        ncursesw::wmove(self._handle(), origin)?;

        Ok(())
    }
}
