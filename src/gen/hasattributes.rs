/*
    src/gen/hasattributes.rs

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
use ncursesw::{
    AttributesColorPairSet, AttributesType, ColorAttributeTypes, ColorPairType,
    WINDOW, normal
};
use crate::{NCurseswWinError, gen::HasHandle};

/// Does the window canvas type have ncursesw attribute type functions.
pub trait HasAttributes: HasHandle<WINDOW> {
    fn attr_get(&self) -> result!(AttributesColorPairSet) {
        Ok(match ncursesw::wattr_get(self._handle())? {
            AttributesColorPairSet::Normal(attrs_colorpair) => {
                attrs_colorpair.attributes().set_screen(self._screen());
                attrs_colorpair.color_pair().set_screen(self._screen());

                AttributesColorPairSet::Normal(attrs_colorpair)
            },
            AttributesColorPairSet::Extend(attrs_colorpair) => {
                attrs_colorpair.attributes().set_screen(self._screen());
                attrs_colorpair.color_pair().set_screen(self._screen());

                AttributesColorPairSet::Extend(attrs_colorpair)
            }
        })
    }

    fn attr_off<A, T>(&self, attrs: A) -> result!(())
        where A: AttributesType<T>,
              T: ColorAttributeTypes
    {
        assert!(self._screen() == attrs.screen());

        Ok(ncursesw::wattr_off(self._handle(), attrs)?)
    }

    fn attr_on<A, T>(&self, attrs: A) -> result!(())
        where A: AttributesType<T>,
              T: ColorAttributeTypes
    {
        assert!(self._screen() == attrs.screen());

        Ok(ncursesw::wattr_on(self._handle(), attrs)?)
    }

    fn attr_set<A, P, T>(&self, attrs: A, color_pair: P) -> result!(())
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        assert!(self._screen() == attrs.screen());
        assert!(self._screen() == color_pair.screen());

        Ok(ncursesw::wattr_set(self._handle(), attrs, color_pair)?)
    }

    fn attroff(&self, attrs: normal::Attributes) -> result!(()) {
        assert!(self._screen() == attrs.screen());

        Ok(ncursesw::wattroff(self._handle(), attrs)?)
    }

    fn attron(&self, attrs: normal::Attributes) -> result!(()) {
        assert!(self._screen() == attrs.screen());

        Ok(ncursesw::wattron(self._handle(), attrs)?)
    }

    fn attrset(&self, attrs: normal::Attributes) -> result!(()) {
        assert!(self._screen() == attrs.screen());

        Ok(ncursesw::wattrset(self._handle(), attrs)?)
    }

    fn color_set<P, T>(&self, color_pair: P) -> result!(())
        where P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        assert!(self._screen() == color_pair.screen());

        Ok(ncursesw::wcolor_set(self._handle(), color_pair)?)
    }

    fn chgat<A, P, T>(&self, length: Option<u16>, attrs: A, color_pair: P) -> result!(())
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        assert!(self._screen() == attrs.screen());
        assert!(self._screen() == color_pair.screen());

        Ok(ncursesw::wchgat(self._handle(), option_length!(length)?, attrs, color_pair)?)
    }

    fn getattrs(&self) -> normal::Attributes {
        let attrs = ncursesw::getattrs(self._handle());

        self._screen().map_or_else(|| normal::Attributes::new(attrs.into()), |screen| normal::Attributes::new_sp(screen, attrs.into()))
    }

    fn standend(&self) -> result!(()) {
        Ok(ncursesw::wstandend(self._handle())?)
    }

    fn standout(&self) -> result!(()) {
        Ok(ncursesw::wstandout(self._handle())?)
    }
}
