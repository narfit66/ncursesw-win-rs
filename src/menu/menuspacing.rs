/*
    src/menu/menuspacing.rs

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
use crate::{NCurseswWinError, menu::MenuSize};

/// Menu spacing (layout spacing).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MenuSpacing {
    pub description: u16,
    pub menu_size:   MenuSize
}

impl TryInto<ncursesw::menu::MenuSpacing> for MenuSpacing {
    type Error = NCurseswWinError;

    fn try_into(self) -> Result<ncursesw::menu::MenuSpacing, Self::Error> {
        Ok(ncursesw::menu::MenuSpacing {
            description: u16::try_into(self.description)?,
            menu_size:   MenuSize::try_into(self.menu_size)?
        })
    }
}

impl TryFrom<ncursesw::menu::MenuSpacing> for MenuSpacing {
    type Error = NCurseswWinError;

    fn try_from(menu_spacing: ncursesw::menu::MenuSpacing) -> Result<Self, Self::Error> {
        Ok(Self {
            description: u16::try_from(menu_spacing.description)?,
            menu_size:   MenuSize::try_from(menu_spacing.menu_size)?
        })
    }
}
