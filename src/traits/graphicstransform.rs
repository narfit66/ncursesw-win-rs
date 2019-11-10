/*
    src/traits/graphicstransform.rs

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

#![allow(clippy::never_loop)]

use ncursesw::{
    AttributesColorPairType, AttributesColorPairSet, ComplexChar,
    Origin, WideChar, getcchar
};
use crate::graphics::{
    WIDEBOXDRAWING, complex_box_graphic, BoxDrawingType, BoxDrawingGraphic
};
use crate::NCurseswWinError;
use crate::traits::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum _Direction {
    Horizontal,
    Vertical
}

// constant to control remaping during BoxDrawingGraphic.transform()
const BOX_DRAWING_GRAPHIC_REMAP: bool = true;

pub trait GraphicsTransform: HasYXAxis + HasMvAddFunctions + HasMvInFunctions + HasMvInsFunctions {
    fn _transform_graphic(
        &self,
        current_complex_char: ComplexChar,
        box_drawing_type:     BoxDrawingType,
        box_drawing_graphic:  BoxDrawingGraphic,
        origin:               Origin,
        direction:            Option<_Direction>
    ) -> result!(ComplexChar) {
        assert_origin!("_transform_graphic", self.size()?, origin);

        let mut box_drawing_graphic = box_drawing_graphic;

        let char_attr_pair = getcchar(current_complex_char)?;
        let wchar: u32 = WideChar::into(char_attr_pair.character());

        for (key, _) in WIDEBOXDRAWING.iter().filter(|(k, v)| k.box_drawing_type() == box_drawing_type && **v == wchar) {
            let graphic = box_drawing_graphic.transform(key.box_drawing_graphic(), BOX_DRAWING_GRAPHIC_REMAP);

            box_drawing_graphic = match direction {
                None            => graphic,
                Some(direction) => self.__transform_by_position(graphic, origin, direction)?
            };

            break;
        }

        match char_attr_pair.attributes_and_color_pair() {
            AttributesColorPairSet::Normal(set)   => complex_box_graphic(box_drawing_type, box_drawing_graphic, &set.attributes(), &set.color_pair()),
            AttributesColorPairSet::Extended(set) => complex_box_graphic(box_drawing_type, box_drawing_graphic, &set.attributes(), &set.color_pair())
        }
    }

    // if we are in the left or right edge of the window then change to the appropriate tee or corner character
    fn __transform_by_position(
        &self,
        box_drawing_graphic: BoxDrawingGraphic,
        origin:              Origin,
        direction:           _Direction
    ) -> result!(BoxDrawingGraphic) {
        // can we transform our box_drawing_graphic
        if if box_drawing_graphic == BoxDrawingGraphic::Plus {
            true
        } else {
            match direction {
                _Direction::Vertical   => box_drawing_graphic == BoxDrawingGraphic::LeftTee || box_drawing_graphic == BoxDrawingGraphic::RightTee,
                _Direction::Horizontal => box_drawing_graphic == BoxDrawingGraphic::UpperTee || box_drawing_graphic == BoxDrawingGraphic::LowerTee
            }
        } {
            // as we have already transformed our box drawing graphic before calling
            // this routine let's just make sure that the graphic we are dealing with
            // should be changed dependent it's position on the virtual window.

            let max_y_axis = self.getmaxy()?;
            let max_x_axis = self.getmaxx()?;

            Ok(if origin.x == 0 {
                if origin.y == 0 {
                    BoxDrawingGraphic::UpperLeftCorner
                } else if origin.y == max_y_axis {
                    BoxDrawingGraphic::LowerLeftCorner
                } else {
                    box_drawing_graphic
                }
            } else if origin.y == max_y_axis {
                if origin.x == 0 {
                    BoxDrawingGraphic::UpperRightCorner
                } else if origin.x == max_x_axis {
                    BoxDrawingGraphic::LowerRightCorner
                } else {
                    box_drawing_graphic
                }
            } else {
                box_drawing_graphic
            })
        } else {
            Ok(box_drawing_graphic)
        }
    }

    // put a complex chr on a virtual window (self) at a given origin.
    fn _put_complex_char(
        &self,
        origin:                     Origin,
        current_complex_char:       ComplexChar,
        new_complex_char:           ComplexChar
    ) -> result!(()) {
        assert_origin!("_put_complex_char", self.size()?, origin);

        // only update our virtual screen if required.
        if current_complex_char != new_complex_char {
            // if we are at the bottom right origin of the screen then
            // insert our new graphic otherwise add the character
            // (mvadd_wch() will error otherwise!).
            if origin == crate::terminal_bottom_right_origin() {
                self.mvins_wch(origin, new_complex_char)?;
            } else {
                self.mvadd_wch(origin, new_complex_char)?;
            }
        }

        Ok(())
    }
}
