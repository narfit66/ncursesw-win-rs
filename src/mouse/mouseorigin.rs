/*
    src/mouse/mouseorigin.rs

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

use std::fmt::{Display, Formatter, Result};
use crate::Origin;

/// The type of origin as reported by a mouse.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MouseOrigin {
    origin: Origin,
    z:      u16
}

impl MouseOrigin {
    pub(in crate::mouse) fn new(y: u16, x: u16, z: u16) -> Self {
        Self { origin: Origin { y, x }, z }
    }

    /// Mouse Y-X axis.
    pub fn origin(self) -> Origin {
        self.origin
    }

    /// Y-axis.
    pub fn y(self) -> u16 {
        self.origin.y
    }

    /// X-axis.
    pub fn x(self) -> u16 {
        self.origin.x
    }

    /// Z-axis.
    pub fn z(self) -> u16 {
        self.z
    }
}

impl Display for MouseOrigin {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(origin: {}, z: {})", self.origin, self.z)
    }
}
