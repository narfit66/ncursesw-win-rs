/*
    src/traits/hasmvattributes.rs

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

use ncursesw::{AttributesType, ColorPairType, ColorAttributeTypes, Origin};
use crate::NCurseswWinError;
use crate::traits::*;

/// Does the window canvas type have ncursesw attribute origin functions.
pub trait HasMvAttributes: HasHandle + HasYXAxis {
    fn mvchgat<A, P, T>(&self, origin: Origin, number: i32, attrs: A, color_pair: P) -> result!(())
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        ncursesw::mvwchgat(self._handle(), origin, number, attrs, color_pair)?;

        Ok(())
    }
}
