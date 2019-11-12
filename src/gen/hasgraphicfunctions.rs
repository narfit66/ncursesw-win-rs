/*
    src/gen/hasgraphicfunctions.rs

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

#![allow(clippy::too_many_arguments)]

use ncursesw::{
    ChtypeChar, ComplexChar, ComplexString, Origin, Size
};
use crate::{
    BoxDrawingType, BoxDrawingGraphic, HorizontalGraphic, VerticalGraphic,
    NCurseswWinError
};
use crate::gen::*;

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

    fn hline(&self, ch: ChtypeChar, length: i32) -> result!(()) {
        assert_length!("hline", length);

        ncursesw::whline(self._handle(), ch, length)?;

        Ok(())
    }

    fn hline_set(&self, wch: ComplexChar, length: i32) -> result!(()) {
        assert_length!("hline_set", length);

        ncursesw::whline_set(self._handle(), wch, length)?;

        Ok(())
    }

    fn mvhline(&self, origin: Origin, ch: ChtypeChar, length: i32) -> result!(()) {
        assert_origin_hlength!("mvhline", self.size()?, origin, length);

        ncursesw::mvwhline(self._handle(), origin, ch, length)?;

        Ok(())
    }

    fn mvhline_set(&self, origin: Origin, wch: ComplexChar, length: i32) -> result!(()) {
        assert_origin_hlength!("mvhline_set", self.size()?, origin, length);

        ncursesw::mvwhline_set(self._handle(), origin, wch, length)?;

        Ok(())
    }

    fn mvvline(&self, origin: Origin, ch: ChtypeChar, length: i32) -> result!(()) {
        assert_origin_vlength!("mvvline", self.size()?, origin, length);

        ncursesw::mvwvline(self._handle(), origin, ch, length)?;

        Ok(())
    }

    fn mvvline_set(&self, origin: Origin, wch: ComplexChar, length: i32) -> result!(()) {
        assert_origin_vlength!("mvvline_set", self.size()?, origin, length);

        ncursesw::mvwvline_set(self._handle(), origin, wch, length)?;

        Ok(())
    }

    fn vline(&self, ch: ChtypeChar, length: i32) -> result!(()) {
        assert_length!("vline", length);

        ncursesw::wvline(self._handle(), ch, length)?;

        Ok(())
    }

    fn vline_set(&self, wch: ComplexChar, length: i32) -> result!(()) {
        assert_length!("vline_set", length);

        ncursesw::wvline_set(self._handle(), wch, length)?;

        Ok(())
    }

    // Transformative box drawing.

    /// Draw a horizontal line at current cursor of a length using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    fn thline_set(
        &self,
        box_drawing_type: BoxDrawingType,
        graphic:          HorizontalGraphic,
        length:           i32
    ) -> result!(()) {
        self.mvthline_set(self.cursor()?, box_drawing_type, graphic, length)
    }

    /// Draw a horizontal line at origin of a length using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    fn mvthline_set(
        &self,
        origin:           Origin,
        box_drawing_type: BoxDrawingType,
        graphic:          HorizontalGraphic,
        length:           i32
    ) -> result!(()) {
        let window_size = self.size()?;

        assert_origin_hlength!("mvthline_set", window_size, origin, length);

        // build a vector of the complex characters we are going to overwrite
        let complex_chars: Vec<ComplexChar> = ComplexString::into(self.mvin_wchnstr(origin, length)?);
        let mut line_origin = origin;

        // define our default graphic character to use
        let box_drawing_graphic = match graphic {
            HorizontalGraphic::Upper  => BoxDrawingGraphic::UpperHorizontalLine,
            HorizontalGraphic::Center => BoxDrawingGraphic::HorizontalLine,
            HorizontalGraphic::Lower  => BoxDrawingGraphic::LowerHorizontalLine
        };

        // iterate over the vector of complex characters
        for &complex_char in &complex_chars {
            // transform our default graphic character with the existing
            // complex character in the cell of the virtual window.
            let graphic_char = self._transform_graphic(
                complex_char,
                box_drawing_type,
                box_drawing_graphic,
                line_origin,
                Some(_Direction::Horizontal)
            )?;

            // write the character to the virtual window.
            self._put_complex_char(line_origin, complex_char, graphic_char)?;

            // increment our x-axis.
            line_origin.x += 1;

            // check if we've reached the right edge of the window
            if line_origin.x >= window_size.columns {
                break;
            }
        }

        Ok(())
    }

    /// Draw a vertical line at the current cursor of a length using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    fn tvline_set(
        &self,
        box_drawing_type: BoxDrawingType,
        graphic:          VerticalGraphic,
        length:           i32
    ) -> result!(()) {
        self.mvtvline_set(self.cursor()?, box_drawing_type, graphic, length)
    }

    /// Draw a vertical line at origin of a length using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    fn mvtvline_set(
        &self,
        origin:           Origin,
        box_drawing_type: BoxDrawingType,
        graphic:          VerticalGraphic,
        length:           i32
    ) -> result!(()) {
        let window_size = self.size()?;

        assert_origin_vlength!("mvtvline_set", window_size, origin, length);

        let mut complex_chars = vec!();
        let mut line_origin = origin;

        // build a vector of the complex characters we are going to overwrite
        for _ in 0..length {
            complex_chars.push(self.mvin_wch(line_origin)?);
            line_origin.y += 1;
        }

        // reset the origin secified.
        line_origin = origin;

        // define our default graphic character to use
        let box_drawing_graphic = match graphic {
            VerticalGraphic::Left   => BoxDrawingGraphic::LeftVerticalLine,
            VerticalGraphic::Center => BoxDrawingGraphic::VerticalLine,
            VerticalGraphic::Right  => BoxDrawingGraphic::RightVerticalLine
        };

        // iterate over the vector of complex characters
        for &complex_char in &complex_chars {
            // transform our default graphic character with the existing
            // complex character in the cell of the virtual window.
            let graphic_char = self._transform_graphic(
                complex_char,
                box_drawing_type,
                box_drawing_graphic,
                line_origin,
                Some(_Direction::Vertical)
            )?;

            // write the character to the virtual window.
            self._put_complex_char(line_origin, complex_char, graphic_char)?;

            // increment our y-axis.
            line_origin.y += 1;

            // check if we've reached the bottom edge of the window
            if line_origin.y >= window_size.lines {
                break;
            }
        }

        Ok(())
    }

    /// Draw a box at current cursor of a size using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    fn tbox_set(
        &self,
        size:             Size,
        box_drawing_type: BoxDrawingType
    ) -> result!(()) {
        self.mvtbox_set(self.cursor()?, size, box_drawing_type)
    }

    /// Draw a box at origin of a size using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    fn mvtbox_set(
        &self,
        origin:           Origin,
        size:             Size,
        box_drawing_type: BoxDrawingType
    ) -> result!(()) {
        let _window_size = self.size()?; // get the size of the window.

        assert_origin!("mvtbox_set", _window_size, origin);
        assert!(size.lines >= 2 && size.columns >= 2, "mvtbox_set() : size is invalid, size={}", size);
        assert!(
            origin.y + size.lines <= _window_size.lines,
            "mvtbox_set() : attempting to write over window edge, origin.y={} + size.lines={} <= window_size.lines={}", origin.y, size.lines, _window_size.lines
        );
        assert!(
            origin.x + size.columns <= _window_size.columns,
            "mvtbox_set() : attempting to write over window edge, origin.x={} + size.columns={} <= window_size.columns={}", origin.x, size.columns, _window_size.columns
        );

        // write a corner graphic to the virtual window.
        let set_corner_char = |corner_origin: Origin, box_drawing_graphic: BoxDrawingGraphic| -> result!(()) {
            // transform our default graphic character with the existing
            // complex character in the cell of the virtual window.
            let graphic_char = self._transform_graphic(
                self.mvin_wch(corner_origin)?,
                box_drawing_type,
                box_drawing_graphic,
                corner_origin,
                None
            )?;

            // write the character to the virtual window.
            self._put_complex_char(corner_origin, self.mvin_wch(corner_origin)?, graphic_char)
        };

        // update the virtual window with corner characters...
        set_corner_char(origin, BoxDrawingGraphic::UpperLeftCorner)?;
        set_corner_char(Origin { y: origin.y, x: origin.x + (size.columns - 1) }, BoxDrawingGraphic::UpperRightCorner)?;
        set_corner_char(Origin { y: origin.y + (size.lines - 1), x: origin.x }, BoxDrawingGraphic::LowerLeftCorner)?;
        set_corner_char(Origin { y: origin.y + (size.lines - 1), x: origin.x + (size.columns - 1) }, BoxDrawingGraphic::LowerRightCorner)?;

        // ...then do the top, bottom, left and right sides of the box if required.
        if size.columns > 2 {
            self.mvthline_set(Origin { y: origin.y, x: origin.x + 1}, box_drawing_type, HorizontalGraphic::Upper, size.columns - 2)?;
            self.mvthline_set(Origin { y: origin.y + (size.lines - 1), x: origin.x + 1}, box_drawing_type, HorizontalGraphic::Lower, size.columns - 2)?;
        }

        if size.lines > 2 {
            self.mvtvline_set(Origin { y: origin.y + 1, x: origin.x }, box_drawing_type, VerticalGraphic::Left, size.lines - 2)?;
            self.mvtvline_set(Origin { y: origin.y + 1, x: origin.x + (size.columns - 1)}, box_drawing_type, VerticalGraphic::Right, size.lines - 2)?;
        }

        Ok(())
    }
}
