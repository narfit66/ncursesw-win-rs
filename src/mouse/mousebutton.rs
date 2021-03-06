/*
    src/mouse/mousebutton.rs

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

use strum_macros::{Display, EnumIter};

/// A mouse button.
#[derive(Copy, Clone, Debug, Display, EnumIter, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// Mouse button number 1.
    One,
    /// Mouse button number 2.
    Two,
    /// Mouse button number 3.
    Three,
    /// Mouse button number 4.
    Four,
    /// Mouse button number 5.
    Five
}

impl MouseButton {
    /// A mouse button number as a `u8`.
    pub fn number(self) -> u8 {
        match self {
            MouseButton::One   => 1,
            MouseButton::Two   => 2,
            MouseButton::Three => 3,
            MouseButton::Four  => 4,
            MouseButton::Five  => 5
        }
    }
}
