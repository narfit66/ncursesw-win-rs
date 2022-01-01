/*
    src/gen/graphicstransform.rs

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

#![allow(clippy::never_loop)]

use std::convert::TryInto;
use ncursesw::{AttributesColorPairSet, ComplexChar, WideChar, getcchar};
use crate::{
    graphics::WIDEBOXDRAWING, Origin, Size,
    BoxDrawingType, BoxDrawingGraphic, NCurseswWinError,
    complex_box_graphic, gen::*
};

// Direction of travel that graphics drawing maybe going
// i.e. `_Direction::Horizontal` when drawing a horizontal line
//      on the virtual window.
pub enum _Direction {
    Horizontal,
    Vertical
}

// constant to control remaping during BoxDrawingGraphic.transform()
const REMAP_BOX_DRAWING_GRAPHIC: bool = true;

pub trait GraphicsTransform: HasYXAxis + HasMvAddFunctions + HasMvInFunctions + HasMvInsFunctions {
    fn _transform_graphic(
        &self,
        current_complex_char: ComplexChar,
        box_drawing_type:     BoxDrawingType,
        box_drawing_graphic:  BoxDrawingGraphic,
        origin:               Origin,
        direction:            Option<_Direction>
    ) -> result!(ComplexChar) {
        let window_size = self.size()?;

        assert_origin!("_transform_graphic", window_size, origin);

        let mut box_drawing_graphic = box_drawing_graphic;

        // extract our complex character into it's character, attributes and color pair.
        let char_attr_pair = getcchar(current_complex_char)?;

        // if we have a custom box drawing type then extract the `BoxDrawingGraphic`
        // from the contained `BoxDrawing` struct otherwise extract the
        // `BoxDrawingType` from the crate defined values.
        if let BoxDrawingType::Custom(box_drawing) = box_drawing_type {
            let wide_char = char_attr_pair.character();

            box_drawing_graphic = match if wide_char == box_drawing.upper_left_corner {
                Some(BoxDrawingGraphic::UpperLeftCorner)
            } else if wide_char == box_drawing.lower_left_corner {
                Some(BoxDrawingGraphic::LowerLeftCorner)
            } else if wide_char == box_drawing.upper_right_corner {
                Some(BoxDrawingGraphic::UpperRightCorner)
            } else if wide_char == box_drawing.lower_right_corner {
                Some(BoxDrawingGraphic::LowerRightCorner)
            } else if wide_char == box_drawing.right_tee {
                Some(BoxDrawingGraphic::RightTee)
            } else if wide_char == box_drawing.left_tee {
                Some(BoxDrawingGraphic::LeftTee)
            } else if wide_char == box_drawing.lower_tee {
                Some(BoxDrawingGraphic::LowerTee)
            } else if wide_char == box_drawing.upper_tee {
                Some(BoxDrawingGraphic::UpperTee)
            } else if wide_char == box_drawing.horizontal_line {
                Some(BoxDrawingGraphic::HorizontalLine)
            } else if wide_char == box_drawing.upper_horizontal_line {
                Some(BoxDrawingGraphic::UpperHorizontalLine)
            } else if wide_char == box_drawing.lower_horizontal_line {
                Some(BoxDrawingGraphic::LowerHorizontalLine)
            } else if wide_char == box_drawing.vertical_line {
                Some(BoxDrawingGraphic::VerticalLine)
            } else if wide_char == box_drawing.left_vertical_line {
                Some(BoxDrawingGraphic::LeftVerticalLine)
            } else if wide_char == box_drawing.right_vertical_line {
                Some(BoxDrawingGraphic::RightVerticalLine)
            } else if wide_char == box_drawing.plus {
                Some(BoxDrawingGraphic::Plus)
            } else {
                None
            } {
                Some(custom_graphic) => transform_by_position(
                    origin,
                    window_size,
                    box_drawing_graphic.transform(custom_graphic, REMAP_BOX_DRAWING_GRAPHIC),
                    direction
                ),
                None          => box_drawing_graphic
            }
        } else {
            let wchar: u32 = WideChar::into(char_attr_pair.character());

            for (key, _) in WIDEBOXDRAWING.iter().filter(|(k, v)| k.box_drawing_type() == box_drawing_type && **v == wchar) {
                box_drawing_graphic = transform_by_position(
                    origin,
                    window_size,
                    box_drawing_graphic.transform(key.box_drawing_graphic(), REMAP_BOX_DRAWING_GRAPHIC),
                    direction
                );

                break;
            }
        }

        // return the transformed (or not!) complex character.
        match char_attr_pair.attributes_and_color_pair() {
            AttributesColorPairSet::Normal(attrs_colorpair) =>
                complex_box_graphic(box_drawing_type, box_drawing_graphic, &attrs_colorpair.attributes(), &attrs_colorpair.color_pair()),
            AttributesColorPairSet::Extend(attrs_colorpair) =>
                complex_box_graphic(box_drawing_type, box_drawing_graphic, &attrs_colorpair.attributes(), &attrs_colorpair.color_pair())
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
            if origin == crate::terminal_bottom_right_origin()? {
                self.mvins_wch(Origin::try_into(origin)?, new_complex_char)?;
            } else {
                self.mvadd_wch(Origin::try_into(origin)?, new_complex_char)?;
            }
        }

        Ok(())
    }
}

fn transform_by_position(
    origin:              Origin,
    window_size:         Size,
    box_drawing_graphic: BoxDrawingGraphic,
    direction:           Option<_Direction>
) -> BoxDrawingGraphic {
    match direction {
        // if we are doing a vertical or horizontal line then don't
        // transform by position on the virtual window.
        Some(_) => box_drawing_graphic,
        // as we've just transformed our box drawing graphic then let's
        // just make sure if the graphic we are dealing with should be
        // changed dependent it's position on the virtual window.
        None    => if origin.x == 0 && origin.y == 0 {
            BoxDrawingGraphic::UpperLeftCorner
        } else if origin.x == 0 && origin.y == window_size.lines {
            BoxDrawingGraphic::LowerLeftCorner
        } else if origin.x == window_size.columns && origin.y == 0 {
            BoxDrawingGraphic::UpperRightCorner
        } else if origin.x == window_size.columns && origin.y == window_size.lines {
            BoxDrawingGraphic::LowerRightCorner
        } else {
            box_drawing_graphic
        }
    }
}

#[test]
fn transform_by_position_test() {
    let max_lines = 24;
    let max_columns = 80;
    let window_size = Size { lines: max_lines, columns: max_columns };

    let upper_left_corner = Origin { y: 0, x: 0 };
    let lower_left_corner = Origin { y: max_lines, x: 0 };
    let upper_right_corner = Origin { y: 0, x: max_columns };
    let lower_right_corner = Origin { y: max_lines, x: max_columns };

    let center_origin = Origin { y: max_lines / 2, x: max_columns / 2};

    // simulate Window.mvtbox_set()
    assert_eq!(
        transform_by_position(upper_left_corner, window_size, BoxDrawingGraphic::Plus, None),
        BoxDrawingGraphic::UpperLeftCorner
    );
    assert_eq!(
        transform_by_position(lower_left_corner, window_size, BoxDrawingGraphic::Plus, None),
        BoxDrawingGraphic::LowerLeftCorner
    );
    assert_eq!(
        transform_by_position(upper_right_corner, window_size, BoxDrawingGraphic::Plus, None),
        BoxDrawingGraphic::UpperRightCorner
    );
    assert_eq!(
        transform_by_position(lower_right_corner, window_size, BoxDrawingGraphic::Plus, None),
        BoxDrawingGraphic::LowerRightCorner
    );

    assert_eq!(
        transform_by_position(center_origin, window_size, BoxDrawingGraphic::Plus, None),
        BoxDrawingGraphic::Plus
    );

    // simulate Window.mvhline_set()
    assert_eq!(
        transform_by_position(upper_left_corner, window_size, BoxDrawingGraphic::HorizontalLine, Some(_Direction::Horizontal)),
        BoxDrawingGraphic::HorizontalLine
    );
    assert_eq!(
        transform_by_position(lower_left_corner, window_size, BoxDrawingGraphic::HorizontalLine, Some(_Direction::Horizontal)),
        BoxDrawingGraphic::HorizontalLine
    );
    assert_eq!(
        transform_by_position(upper_right_corner, window_size, BoxDrawingGraphic::HorizontalLine, Some(_Direction::Horizontal)),
        BoxDrawingGraphic::HorizontalLine
    );
    assert_eq!(
        transform_by_position(lower_right_corner, window_size, BoxDrawingGraphic::HorizontalLine, Some(_Direction::Horizontal)),
        BoxDrawingGraphic::HorizontalLine
    );

    assert_eq!(
        transform_by_position(center_origin, window_size, BoxDrawingGraphic::Plus, Some(_Direction::Horizontal)),
        BoxDrawingGraphic::Plus
    );

    // simulate Window.mvvline_set()
    assert_eq!(
        transform_by_position(upper_left_corner, window_size, BoxDrawingGraphic::VerticalLine, Some(_Direction::Vertical)),
        BoxDrawingGraphic::VerticalLine
    );
    assert_eq!(
        transform_by_position(lower_left_corner, window_size, BoxDrawingGraphic::VerticalLine, Some(_Direction::Vertical)),
        BoxDrawingGraphic::VerticalLine
    );
    assert_eq!(
        transform_by_position(upper_right_corner, window_size, BoxDrawingGraphic::VerticalLine, Some(_Direction::Vertical)),
        BoxDrawingGraphic::VerticalLine
    );
    assert_eq!(
        transform_by_position(lower_right_corner, window_size, BoxDrawingGraphic::VerticalLine, Some(_Direction::Vertical)),
        BoxDrawingGraphic::VerticalLine
    );

    assert_eq!(
        transform_by_position(center_origin, window_size, BoxDrawingGraphic::Plus, Some(_Direction::Vertical)),
        BoxDrawingGraphic::Plus
    );
}
