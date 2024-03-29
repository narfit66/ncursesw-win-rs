/*
    src/mouse/originresult.rs

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
use crate::{Origin, NCurseswWinError};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OriginResult {
    origin:    Origin,
    to_screen: bool,
    result:    bool
}

impl OriginResult {
    pub(in crate::mouse) fn new(y: u16, x: u16, to_screen: bool, result: bool) -> Self {
        Self { origin: Origin { y, x }, to_screen, result }
    }

    pub fn origin(self) -> Origin {
        self.origin
    }

    pub fn to_screen(self) -> bool {
        self.to_screen
    }

    pub fn result(self) -> bool {
        self.result
    }
}

impl TryFrom<ncursesw::mouse::OriginResult> for OriginResult {
    type Error = NCurseswWinError;

    fn try_from(origin_result: ncursesw::mouse::OriginResult) -> Result<Self, Self::Error> {
        let origin = Origin::try_from(origin_result.origin())?;

        Ok(Self::new(origin.y, origin.x, origin_result.to_screen(), origin_result.result()))
    }
}
