/*
    src/traits/hasgraphicfunctions.rs

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
    AttributesColorPairType, AttributesColorPairSet, ChtypeChar, ComplexChar,
    ComplexString, Origin, Size, WideChar, getcchar
};
use crate::graphics::{
    WIDEBOXDRAWING, complex_box_graphic, BoxDrawingType, BoxDrawingGraphic,
    HorizontalGraphic, VerticalGraphic
};
use crate::ncurseswwinerror::NCurseswWinError;
use crate::traits::*;

// constant to control remaping during BoxDrawingGraphic.transform()
const BOX_DRAWING_GRAPHIC_REMAP: bool = true;

/// Does the window canvas type have ncursesw graphics functions.
pub trait HasGraphicFunctions: HasYXAxis + HasMvAddFunctions + HasMvInFunctions + HasMvInsFunctions + GraphicsTransform {
    fn border(
        &self,
        ls: ChtypeChar,
        rs: ChtypeChar,
        ts: ChtypeChar,
        bs: ChtypeChar,
        tl: ChtypeChar,
        tr: ChtypeChar,
        bl: ChtypeChar,
        br: ChtypeChar) -> result!(())
    {
        ncursesw::wborder(self._handle(), ls, rs, ts, bs, tl, tr, bl, br)?;

        Ok(())
    }

    fn border_set(
        &self,
        ls: ComplexChar,
        rs: ComplexChar,
        ts: ComplexChar,
        bs: ComplexChar,
        tl: ComplexChar,
        tr: ComplexChar,
        bl: ComplexChar,
        br: ComplexChar) -> result!(())
    {
        ncursesw::wborder_set(self._handle(), ls, rs, ts, bs, tl, tr, bl, br)?;

        Ok(())
    }

    fn r#box(&self, verch: ChtypeChar, horch: ChtypeChar) -> result!(()) {
        ncursesw::r#box(self._handle(), verch, horch)?;

        Ok(())
    }

    fn box_set(&self, verch: ComplexChar, horch: ComplexChar) -> result!(()) {
        ncursesw::box_set(self._handle(), verch, horch)?;

        Ok(())
    }

    fn hline(&self, ch: ChtypeChar, number: i32) -> result!(()) {
        ncursesw::whline(self._handle(), ch, number)?;

        Ok(())
    }

    fn hline_set(&self, wch: ComplexChar, number: i32) -> result!(()) {
        ncursesw::whline_set(self._handle(), wch, number)?;

        Ok(())
    }

    fn mvhline(&self, origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
        ncursesw::mvwhline(self._handle(), origin, ch, number)?;

        Ok(())
    }

    fn mvhline_set(&self, origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
        ncursesw::mvwhline_set(self._handle(), origin, wch, number)?;

        Ok(())
    }

    fn mvvline(&self, origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
        ncursesw::mvwvline(self._handle(), origin, ch, number)?;

        Ok(())
    }

    fn mvvline_set(&self, origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
        ncursesw::mvwvline_set(self._handle(), origin, wch, number)?;

        Ok(())
    }

    fn vline(&self, ch: ChtypeChar, number: i32) -> result!(()) {
        ncursesw::wvline(self._handle(), ch, number)?;

        Ok(())
    }

    fn vline_set(&self, wch: ComplexChar, number: i32) -> result!(()) {
        ncursesw::wvline_set(self._handle(), wch, number)?;

        Ok(())
    }

    /// Transformative box drawing.

    /// Draw a horizontal line at current cursor of a length using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    fn thline_set(&self, box_drawing_type: BoxDrawingType, graphic: HorizontalGraphic, length: i32) -> result!(()) {
        assert!(length > 0, "thline_set() : length={} > 0", length);

        self.mvthline_set(self.cursor()?, box_drawing_type, graphic, length)
    }

    /// Draw a horizontal line at origin of a length using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    fn mvthline_set(&self, origin: Origin, box_drawing_type: BoxDrawingType, graphic: HorizontalGraphic, length: i32) -> result!(()) {
        assert!(length > 0, "mvthline_set() : length={} > 0", length);

        // build a vector of the complex characters we are going to overwrite
        let complex_chars: Vec<ComplexChar> = ComplexString::into(self.mvin_wchnstr(origin, length)?);
        let mut line_origin = origin;

        // iterate over the vector of complex characters
        for &complex_char in &complex_chars {
            // get the elements of our complex character i.e. wide character, attributes and color pair
            let char_attr_pair = getcchar(complex_char)?;
            // get the unicode value of our complex characters character
            let wchar: u32 = WideChar::into(char_attr_pair.character());

            // define our default graphic character to use
            let mut box_drawing_graphic = match graphic {
                HorizontalGraphic::Upper  => BoxDrawingGraphic::UpperHorizontalLine,
                HorizontalGraphic::Center => BoxDrawingGraphic::HorizontalLine,
                HorizontalGraphic::Lower  => BoxDrawingGraphic::LowerHorizontalLine
            };

            // iterate and filter wide character graphic characters of the specified box drawing type that have the same unicode value
            for (key, _) in WIDEBOXDRAWING.iter().filter(|(k, v)| k.box_drawing_type() == box_drawing_type && **v == wchar) {
                // transform our selected graphic character with our default graphic character then...
                // if we've transformed into a plus or left/right tee graphic then...
                // if we are in the upper or lower edge of the window then change to the appropriate tee or corner character
                box_drawing_graphic = self._transform_by_position(
                    box_drawing_graphic.transform(key.box_drawing_graphic(), BOX_DRAWING_GRAPHIC_REMAP),
                    line_origin,
                    _Direction::Horizontal
                )?;

                break;
            }

            // place our complex graphics character onto the window with the existing attributes and color pair
            self.mvadd_wch(line_origin, match char_attr_pair.attributes_and_color_pair() {
                AttributesColorPairSet::Normal(set)   => complex_box_graphic(box_drawing_type, box_drawing_graphic, &set.attributes(), &set.color_pair())?,
                AttributesColorPairSet::Extended(set) => complex_box_graphic(box_drawing_type, box_drawing_graphic, &set.attributes(), &set.color_pair())?
            })?;

            line_origin.x += 1;

            // check if we've reached the right edge of the window
            if line_origin.x >= self.getmaxx()? {
                break;
            }
        }

        Ok(())
    }

    /// Draw a vertical line at the current cursor of a length using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    fn tvline_set(&self, box_drawing_type: BoxDrawingType, graphic: VerticalGraphic, length: i32) -> result!(()) {
        assert!(length > 0, "tvline_set() : length={} > 0", length);

        self.mvtvline_set(self.cursor()?, box_drawing_type, graphic, length)
    }

    /// Draw a vertical line at origin of a length using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    fn mvtvline_set(&self, origin: Origin, box_drawing_type: BoxDrawingType, graphic: VerticalGraphic, length: i32) -> result!(()) {
        assert!(length > 0, "mvtvline_set() : length={} > 0", length);

        let mut complex_chars = vec!();
        let mut line_origin = origin;

        // build a vector of the complex characters we are going to overwrite
        for _ in 0..length {
            complex_chars.push(self.mvin_wch(line_origin)?);
            line_origin.y += 1;
        }

        line_origin = origin;

        // iterate over the vector of complex characters
        for &complex_char in &complex_chars {
            // get the elements of our complex character i.e. wide character, attributes and color pair
            let char_attr_pair = getcchar(complex_char)?;
            // get the unicode value of our complex characters character
            let wchar: u32 = WideChar::into(char_attr_pair.character());

            // define our default graphic character to use
            let mut box_drawing_graphic = match graphic {
                VerticalGraphic::Left   => BoxDrawingGraphic::LeftVerticalLine,
                VerticalGraphic::Center => BoxDrawingGraphic::VerticalLine,
                VerticalGraphic::Right  => BoxDrawingGraphic::RightVerticalLine
            };

            // iterate and filter wide character graphic characters of the specified box drawing type that have the same unicode value
            for (key, _) in WIDEBOXDRAWING.iter().filter(|(k, v)| k.box_drawing_type() == box_drawing_type && **v == wchar) {
                // transform our selected graphic character with our default graphic character then...
                // if we've transformed into a plus or left/right tee graphic then...
                // if we are in the left or right edge of the window then change to the appropriate tee or corner character
                box_drawing_graphic = self._transform_by_position(
                    box_drawing_graphic.transform(key.box_drawing_graphic(), BOX_DRAWING_GRAPHIC_REMAP),
                    line_origin,
                    _Direction::Vertical
                )?;

                break;
            }

            // place our complex graphics character onto the window with the existing attributes and color pair
            self.mvadd_wch(line_origin, match char_attr_pair.attributes_and_color_pair() {
                AttributesColorPairSet::Normal(set)   => complex_box_graphic(box_drawing_type, box_drawing_graphic, &set.attributes(), &set.color_pair())?,
                AttributesColorPairSet::Extended(set) => complex_box_graphic(box_drawing_type, box_drawing_graphic, &set.attributes(), &set.color_pair())?
            })?;

            line_origin.y += 1;

            // check if we've reached the bottom edge of the window
            if line_origin.y >= self.getmaxy()? {
                break;
            }
        }

        Ok(())
    }

    /// Draw a box at current cursor of a size using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    fn tbox_set(&self, size: Size, box_drawing_type: BoxDrawingType) -> result!(()) {
        self.mvtbox_set(self.cursor()?, size, box_drawing_type)
    }

    /// Draw a box at origin of a size using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    fn mvtbox_set(&self, origin: Origin, size: Size, box_drawing_type: BoxDrawingType) -> result!(()) {
        let get_corner_char = |corner_origin: Origin, graphic: BoxDrawingGraphic| -> result!(ComplexChar) {
            let char_attr_pair = getcchar(self.mvin_wch(corner_origin)?)?;
            let mut box_drawing_graphic = graphic;

            for (key, _) in WIDEBOXDRAWING.iter().filter(|(k, v)| k.box_drawing_type() == box_drawing_type && **v == WideChar::into(char_attr_pair.character())) {
                box_drawing_graphic = box_drawing_graphic.transform(key.box_drawing_graphic(), BOX_DRAWING_GRAPHIC_REMAP);

                break;
            }

            let cchar = match char_attr_pair.attributes_and_color_pair() {
                AttributesColorPairSet::Normal(set)   => complex_box_graphic(box_drawing_type, box_drawing_graphic, &set.attributes(), &set.color_pair())?,
                AttributesColorPairSet::Extended(set) => complex_box_graphic(box_drawing_type, box_drawing_graphic, &set.attributes(), &set.color_pair())?
            };

            Ok(cchar)
        };

        let mut corner_origin = origin;
        self.mvadd_wch(corner_origin, get_corner_char(corner_origin, BoxDrawingGraphic::UpperLeftCorner)?)?;

        corner_origin = Origin { y: origin.y, x: origin.x + (size.columns - 1) };
        self.mvadd_wch(corner_origin, get_corner_char(corner_origin, BoxDrawingGraphic::UpperRightCorner)?)?;

        corner_origin = Origin { y: origin.y + (size.lines - 1), x: origin.x };
        self.mvadd_wch(corner_origin, get_corner_char(corner_origin, BoxDrawingGraphic::LowerLeftCorner)?)?;

        corner_origin = Origin { y: origin.y + (size.lines - 1), x: origin.x + (size.columns - 1) };
        let screen_size = Origin { y: ncursesw::LINES() - 1, x: ncursesw::COLS() - 1 };
        if corner_origin == screen_size {
            self.mvins_wch(corner_origin, get_corner_char(corner_origin, BoxDrawingGraphic::LowerRightCorner)?)?;
        } else {
            self.mvadd_wch(corner_origin, get_corner_char(corner_origin, BoxDrawingGraphic::LowerRightCorner)?)?;
        }

        self.mvthline_set(Origin { y: origin.y, x: origin.x + 1}, box_drawing_type, HorizontalGraphic::Upper, size.columns - 2)?;
        self.mvthline_set(Origin { y: origin.y + (size.lines - 1), x: origin.x + 1}, box_drawing_type, HorizontalGraphic::Lower, size.columns - 2)?;
        self.mvtvline_set(Origin { y: origin.y + 1, x: origin.x }, box_drawing_type, VerticalGraphic::Left, size.lines - 2)?;
        self.mvtvline_set(Origin { y: origin.y + 1, x: origin.x + (size.columns - 1)}, box_drawing_type, VerticalGraphic::Right, size.lines - 2)?;

        Ok(())
    }

    /*
    // if we are in the left or right edge of the window then change to the appropriate tee or corner character
    fn _transform_by_position(
        &self,
        box_drawing_graphic: BoxDrawingGraphic,
        origin: Origin,
        direction: Direction
    ) -> result!(BoxDrawingGraphic) {
        // can we transform our box_drawing_graphic
        if if box_drawing_graphic == BoxDrawingGraphic::Plus {
            true
        } else {
            match direction {
                Direction::Vertical   => box_drawing_graphic == BoxDrawingGraphic::LeftTee || box_drawing_graphic == BoxDrawingGraphic::RightTee,
                Direction::Horizontal => box_drawing_graphic == BoxDrawingGraphic::UpperTee || box_drawing_graphic == BoxDrawingGraphic::LowerTee
            }
        } {
            let max_y = self.getmaxy()?;
            let max_x = self.getmaxx()?;

            Ok(if origin.x == 0 && origin.y != max_y {
                if origin.y == 0 {
                    BoxDrawingGraphic::UpperLeftCorner
                } else if origin.y == max_y {
                    BoxDrawingGraphic::LowerLeftCorner
                } else {
                    match direction {
                        Direction::Vertical   => BoxDrawingGraphic::UpperTee,
                        Direction::Horizontal => BoxDrawingGraphic::LeftTee
                    }
                }
            } else if origin.y == max_y {
                if origin.x == 0 {
                    BoxDrawingGraphic::UpperRightCorner
                } else if origin.x == max_x {
                    BoxDrawingGraphic::LowerRightCorner
                } else {
                    match direction {
                        Direction::Vertical   => BoxDrawingGraphic::LowerTee,
                        Direction::Horizontal => BoxDrawingGraphic::RightTee
                    }
                }
            } else {
                box_drawing_graphic
            })
        } else {
            Ok(box_drawing_graphic)
        }
    }*/
}
