/*
    src/origin.rs

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

use std::{fmt, convert::{TryFrom, TryInto}};
use crate::NCurseswWinError;

/// Origin using y and x axis
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Origin {
    /// y-axis (row)
    pub y: u16,
    /// x-axis (column)
    pub x: u16
}

/// Default origin of 0,0.
impl Default for Origin {
    fn default() -> Self {
        Self { y: 0, x: 0 }
    }
}

impl TryInto<ncursesw::Origin> for Origin {
    type Error = NCurseswWinError;

    fn try_into(self) -> Result<ncursesw::Origin, Self::Error> {
        Ok(ncursesw::Origin { y: u16::try_into(self.y)? , x: u16::try_into(self.x)? })
    }
}

impl TryFrom<ncursesw::Origin> for Origin {
    type Error = NCurseswWinError;

    fn try_from(origin: ncursesw::Origin) -> Result<Self, Self::Error> {
        Ok(Self { y: u16::try_from(origin.y)?, x: u16::try_from(origin.x)? })
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(y: {}, x: {})", self.y, self.x)
    }
}
