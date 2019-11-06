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

use ncursesw::{Origin};
use crate::graphics::BoxDrawingGraphic;
use crate::ncurseswwinerror::NCurseswWinError;
use crate::traits::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum _Direction {
    Horizontal,
    Vertical
}

pub trait GraphicsTransform: HasYXAxis + HasMvAdd + HasMvIn + HasMvIns {
    // if we are in the left or right edge of the window then change to the appropriate tee or corner character
    fn _transform_by_position(
        &self,
        box_drawing_graphic: BoxDrawingGraphic,
        origin: Origin,
        direction: _Direction
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
            let max_y = self.getmaxy()?;
            let max_x = self.getmaxx()?;

            Ok(if origin.x == 0 && origin.y != max_y {
                if origin.y == 0 {
                    BoxDrawingGraphic::UpperLeftCorner
                } else if origin.y == max_y {
                    BoxDrawingGraphic::LowerLeftCorner
                } else {
                    match direction {
                        _Direction::Vertical   => BoxDrawingGraphic::UpperTee,
                        _Direction::Horizontal => BoxDrawingGraphic::LeftTee
                    }
                }
            } else if origin.y == max_y {
                if origin.x == 0 {
                    BoxDrawingGraphic::UpperRightCorner
                } else if origin.x == max_x {
                    BoxDrawingGraphic::LowerRightCorner
                } else {
                    match direction {
                        _Direction::Vertical   => BoxDrawingGraphic::LowerTee,
                        _Direction::Horizontal => BoxDrawingGraphic::RightTee
                    }
                }
            } else {
                box_drawing_graphic
            })
        } else {
            Ok(box_drawing_graphic)
        }
    }
}
