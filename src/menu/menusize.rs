/*
    src/menu/menusize.rs

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

use ncursesw;
use crate::NCurseswWinError;

/// Menu size (rows and columns).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MenuSize {
    /// Number of rows.
    pub rows:    u16,
    /// Number of columns.
    pub columns: u16
}

/// The default NCurses menu size (16 rows and 1 column).
impl Default for MenuSize {
    fn default() -> Self {
        let default = ncursesw::menu::MenuSize::default();

        let rows = if let Ok(rows) = u16::try_from(default.rows) {
            rows
        } else {
            16
        };

        let columns = if let Ok(columns) = u16::try_from(default.columns) {
            columns
        } else {
            1
        };

        Self { rows, columns }
    }
}

impl TryInto<ncursesw::menu::MenuSize> for MenuSize {
    type Error = NCurseswWinError;

    fn try_into(self) -> Result<ncursesw::menu::MenuSize, Self::Error> {
        Ok(ncursesw::menu::MenuSize { rows: u16::try_into(self.rows)?, columns: u16::try_into(self.columns)? })
    }
}

impl TryFrom<ncursesw::menu::MenuSize> for MenuSize {
    type Error = NCurseswWinError;

    fn try_from(menu_size: ncursesw::menu::MenuSize) -> Result<Self, Self::Error> {
        Ok(Self { rows: u16::try_from(menu_size.rows)?, columns: u16::try_from(menu_size.columns)? })
    }
}

impl fmt::Display for MenuSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(rows: {}, columns: {})", self.rows, self.columns)
    }
}
