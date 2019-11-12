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

use ncursesw::{
    AttributesColorPairSet, AttributesType, ColorAttributeTypes, ColorPairType, normal
};
use crate::NCurseswWinError;
use crate::gen::*;

/// Does the window canvas type have ncursesw attribute type functions.
pub trait HasAttributes: HasHandle {
    fn attr_get(&self) -> result!(AttributesColorPairSet) {
        match ncursesw::wattr_get(self._handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(set)     => Ok(set)
        }
    }

    fn attr_off<A, T>(&self, attrs: A) -> result!(())
        where A: AttributesType<T>,
              T: ColorAttributeTypes
    {
        ncursesw::wattr_off(self._handle(), attrs)?;

        Ok(())
    }

    fn attr_on<A, T>(&self, attrs: A) -> result!(())
        where A: AttributesType<T>,
              T: ColorAttributeTypes
    {
        ncursesw::wattr_on(self._handle(), attrs)?;

        Ok(())
    }

    fn attr_set<A, P, T>(&self, attrs: A, color_pair: P) -> result!(())
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        ncursesw::wattr_set(self._handle(), attrs, color_pair)?;

        Ok(())
    }

    fn attroff(&self, attrs: normal::Attributes) -> result!(()) {
        ncursesw::wattroff(self._handle(), attrs)?;

        Ok(())
    }

    fn attron(&self, attrs: normal::Attributes) -> result!(()) {
        ncursesw::wattron(self._handle(), attrs)?;

        Ok(())
    }

    fn attrset(&self, attrs: normal::Attributes) -> result!(()) {
        ncursesw::wattrset(self._handle(), attrs)?;

        Ok(())
    }

    fn color_set<P, T>(&self, color_pair: P) -> result!(())
        where P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        ncursesw::wcolor_set(self._handle(), color_pair)?;

        Ok(())
    }

    fn chgat<A, P, T>(&self, length: i32, attrs: A, color_pair: P) -> result!(())
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        assert_length!("chgat", length);

        ncursesw::wchgat(self._handle(), length, attrs, color_pair)?;

        Ok(())
    }

    fn getattrs(&self) -> normal::Attributes {
        ncursesw::getattrs(self._handle())
    }

    fn standend(&self) -> result!(()) {
        ncursesw::wstandend(self._handle())?;

        Ok(())
    }

    fn standout(&self) -> result!(()) {
        ncursesw::wstandout(self._handle())?;

        Ok(())
    }
}
