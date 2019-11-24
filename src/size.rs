/*
    src/size.rs

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

use std::{fmt, convert::{TryFrom, TryInto}};

use crate::NCurseswWinError;

/// Origin using y and x axis
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Size {
    /// The number of lines (y-axis)
    pub lines: u16,
    /// The number of columns (x-axis)
    pub columns: u16
}

/// Default size of 0 lines and 0 columns.
impl Default for Size {
    fn default() -> Self {
        Self { lines: 0, columns: 0 }
    }
}

impl TryInto<ncursesw::Size> for Size {
    type Error = NCurseswWinError;

    fn try_into(self) -> Result<ncursesw::Size, Self::Error> {
        let lines = if let Ok(lines) = i32::try_from(self.lines) {
            lines
        } else {
            return Err(NCurseswWinError::TryIntoSizeError { lines: self.lines, columns: self.columns })
        };

        let columns = if let Ok(columns) = i32::try_from(self.columns) {
            columns
        } else {
            return Err(NCurseswWinError::TryIntoSizeError { lines: self.lines, columns: self.columns })
        };

        Ok(ncursesw::Size { lines, columns })
    }
}

impl TryFrom<ncursesw::Size> for Size {
    type Error = NCurseswWinError;

    fn try_from(size: ncursesw::Size) -> Result<Self, Self::Error> {
        let lines = if let Ok(lines) = u16::try_from(size.lines) {
            lines
        } else {
            return Err(NCurseswWinError::TryFromSizeError { lines: size.lines, columns: size.columns })
        };

        let columns = if let Ok(columns) = u16::try_from(size.columns) {
            columns
        } else {
            return Err(NCurseswWinError::TryFromSizeError { lines: size.lines, columns: size.columns })
        };

        Ok(Self { lines, columns })
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(lines: {}, columns: {})", self.lines, self.columns)
    }
}
