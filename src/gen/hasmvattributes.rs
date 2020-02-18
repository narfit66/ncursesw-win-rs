/*
    src/gen/hasmvattributes.rs

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

use ncursesw::{AttributesType, ColorPairType, ColorAttributeTypes, WINDOW};
use crate::{Origin, NCurseswWinError, gen::{HasHandle, HasYXAxis}};

/// Does the window canvas type have ncursesw attribute origin functions.
pub trait HasMvAttributes: HasHandle<WINDOW> + HasYXAxis {
    fn mvchgat<A, P, T>(&self, origin: Origin, length: Option<u16>, attrs: A, color_pair: P) -> result!(())
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        assert_origin!("mvchgat", self.size()?, origin);
        assert!(self._screen() == attrs.screen());
        assert!(self._screen() == color_pair.screen());

        Ok(ncursesw::mvwchgat(self._handle(), origin.try_into()?, option_length!(length)?, attrs, color_pair)?)
    }
}
