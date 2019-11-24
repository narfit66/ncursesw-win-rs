/*
    src/gen/hasnonblocking.rs

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

use std::time::Duration;
use std::convert::TryFrom;

use ncursesw::{NCurseswError, shims::ncurses};
use crate::{Timeout, NCurseswWinError, gen::HasHandle};

/// Does the window canvas type support non-blocking functions.
pub trait HasNonBlocking: HasHandle {
    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use get_timeout() instead")]
    fn getdelay(&self) -> result!(Duration) {
        Ok(ncursesw::wgetdelay(self._handle())?)
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use set_timeout() instead")]
    fn timeout(&self, ms: Duration) -> result!(()) {
        Ok(ncursesw::wtimeout(self._handle(), ms)?)
    }

    /// get the non-blocking read timeout in milliseconds.
    fn get_timeout(&self) -> result!(Timeout) {
        match unsafe { ncurses::wgetdelay(self._handle()) } {
            -1 => Ok(None),
            rc => {
                if rc < 0 {
                    Err(NCurseswWinError::from(NCurseswError::LibraryError { func: "wgetdelay".to_string(), rc }))
                } else {
                    Ok(Some(Duration::from_millis(u64::try_from(rc)?)))
                }
            }
        }
    }

    /// set the non-blocking read timeout in milliseconds, use `ms: None` to set blocking read mode.
    fn set_timeout(&self, ms: Timeout) -> result!(()) {
        match ms {
            None     => unsafe { ncurses::wtimeout(self._handle(), -1) },
            Some(ms) => ncursesw::wtimeout(self._handle(), ms)?
        }

        Ok(())
    }
}
