/*
    src/gen/hasattributes.rs

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

use std::convert::TryFrom;

use ncursesw::{
    AttributesColorPairSet, AttributesType, ColorAttributeTypes, ColorPairType, normal
};
use crate::{NCurseswWinError, gen::HasHandle};

/// Does the window canvas type have ncursesw attribute type functions.
pub trait HasAttributes: HasHandle {
    fn attr_get(&self) -> result!(AttributesColorPairSet) {
        Ok(ncursesw::wattr_get(self._handle())?)
    }

    fn attr_off<A, T>(&self, attrs: A) -> result!(())
        where A: AttributesType<T>,
              T: ColorAttributeTypes
    {
        Ok(ncursesw::wattr_off(self._handle(), attrs)?)
    }

    fn attr_on<A, T>(&self, attrs: A) -> result!(())
        where A: AttributesType<T>,
              T: ColorAttributeTypes
    {
        Ok(ncursesw::wattr_on(self._handle(), attrs)?)
    }

    fn attr_set<A, P, T>(&self, attrs: A, color_pair: P) -> result!(())
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        Ok(ncursesw::wattr_set(self._handle(), attrs, color_pair)?)
    }

    fn attroff(&self, attrs: normal::Attributes) -> result!(()) {
        Ok(ncursesw::wattroff(self._handle(), attrs)?)
    }

    fn attron(&self, attrs: normal::Attributes) -> result!(()) {
        Ok(ncursesw::wattron(self._handle(), attrs)?)
    }

    fn attrset(&self, attrs: normal::Attributes) -> result!(()) {
        Ok(ncursesw::wattrset(self._handle(), attrs)?)
    }

    fn color_set<P, T>(&self, color_pair: P) -> result!(())
        where P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        Ok(ncursesw::wcolor_set(self._handle(), color_pair)?)
    }

    fn chgat<A, P, T>(&self, length: u16, attrs: A, color_pair: P) -> result!(())
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        Ok(ncursesw::wchgat(self._handle(), i32::try_from(length)?, attrs, color_pair)?)
    }

    fn getattrs(&self) -> normal::Attributes {
        ncursesw::getattrs(self._handle())
    }

    fn standend(&self) -> result!(()) {
        Ok(ncursesw::wstandend(self._handle())?)
    }

    fn standout(&self) -> result!(()) {
        Ok(ncursesw::wstandout(self._handle())?)
    }
}
