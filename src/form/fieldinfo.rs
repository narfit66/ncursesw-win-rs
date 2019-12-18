/*
    src/form/fieldinfo.rs

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

use crate::{Size, NCurseswWinError};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FieldInfo {
    size: Size,
    max:  u16
}

impl FieldInfo {
    pub fn new(size: Size, max: u16) -> Self {
        Self { size, max }
    }

    pub fn size(self) -> Size {
        self.size
    }

    pub fn max(self) -> u16 {
        self.max
    }
}

impl TryInto<ncursesw::form::FieldInfo> for FieldInfo {
    type Error = NCurseswWinError;

    fn try_into(self) -> Result<ncursesw::form::FieldInfo, Self::Error> {
        Ok(ncursesw::form::FieldInfo::new(self.size().try_into()?, u16::try_into(self.max)?))
    }
}

impl TryFrom<ncursesw::form::FieldInfo> for FieldInfo {
    type Error = NCurseswWinError;

    fn try_from(info: ncursesw::form::FieldInfo) -> Result<Self, Self::Error> {
        Ok(Self { size: info.size().try_into()?, max: u16::try_from(info.max())? })
    }
}

impl fmt::Display for FieldInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(size: {}, max: {})", self.size, self.max)
    }
}
