/*
    src/graphics/boxdrawinggraphic.rs

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

#![allow(unused_imports)] // supress warning about un-used HashMap (this is used in the test transform_test()).

use std::{convert::TryFrom, collections::HashMap};
use crate::NCurseswWinError;

// Uniform the size of the mask for BoxDrawingGraphic.
type GraphicMask = u16;

/// The box drawing graphic characters.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BoxDrawingGraphic {
    /// Upper-left corner.
    UpperLeftCorner,
    /// Lower-left corner.
    LowerLeftCorner,
    /// Upper-right corner.
    UpperRightCorner,
    /// Lower-right corner.
    LowerRightCorner,
    /// Right-side tee.
    RightTee,
    /// Left-side tee.
    LeftTee,
    /// Lower-side tee.
    LowerTee,
    /// Upper-side tee.
    UpperTee,
    /// Horizontal line.
    HorizontalLine,
    /// Upper-horizontal line.
    UpperHorizontalLine,
    /// Lower-horizontal line.
    LowerHorizontalLine,
    /// Vertical line.
    VerticalLine,
    /// Left-vertical line.
    LeftVerticalLine,
    /// Right-vertical line.
    RightVerticalLine,
    /// Plus/Cross.
    Plus
}

// Base the raw value of a BoxDrawingGraphic on a 3x3 matrix of a box character so we can OR two
// raw values together (see transform function) to determine the corrected BoxDrawingGraphic to use.
// UpperHorizontalLine, LowerVerticalLine, LeftVerticalLine and RightVerticalLine have
// pseudo values to make them unique for Into/TryFrom purposes.
//
// for example:
//                        000
// UpperLeftCorner : ┌ :  011 : 0b000_011_010
//                        010
//
//                        000
// HorizontalLine  : ─ :  111 : 0b000_111_000
//                        000
//
//
// input 1 : 0b000_011_010
//           0b000_111_010 result : ┬ : UpperTee
// input 2 : 0b000_111_000
//
// This will work with the exception of the origin of the graphic if it is on the border
// of the window we will calculate a Plus with horizontal and vertical line types so we
// will need to take this into account.

const UPPERLEFTCORNER: GraphicMask     = 0b_000_011_010;
const LOWERLEFTCORNER: GraphicMask     = 0b_010_011_000;
const UPPERRIGHTCORNER: GraphicMask    = 0b_000_110_010;
const LOWERRIGHTCORNER: GraphicMask    = 0b_010_110_000;
const RIGHTTEE: GraphicMask            = 0b_010_110_010;
const LEFTTEE: GraphicMask             = 0b_010_011_010;
const LOWERTEE: GraphicMask            = 0b_010_111_000;
const UPPERTEE: GraphicMask            = 0b_000_111_010;
const HORIZONTALLINE: GraphicMask      = 0b_000_111_000;
const UPPERHORIZONTALLINE: GraphicMask = 0b_111_000_000;
const LOWERHORIZONTALLINE: GraphicMask = 0b_000_000_111;
const VERTICALLINE: GraphicMask        = 0b_010_010_010;
const LEFTVERTICALLINE: GraphicMask    = 0b_100_100_100;
const RIGHTVERTICALLINE: GraphicMask   = 0b_001_001_001;
const PLUS: GraphicMask                = 0b_010_111_010;

impl Into<GraphicMask> for BoxDrawingGraphic {
    fn into(self) -> GraphicMask {
        match self {
            BoxDrawingGraphic::UpperLeftCorner     => UPPERLEFTCORNER,
            BoxDrawingGraphic::LowerLeftCorner     => LOWERLEFTCORNER,
            BoxDrawingGraphic::UpperRightCorner    => UPPERRIGHTCORNER,
            BoxDrawingGraphic::LowerRightCorner    => LOWERRIGHTCORNER,
            BoxDrawingGraphic::RightTee            => RIGHTTEE,
            BoxDrawingGraphic::LeftTee             => LEFTTEE,
            BoxDrawingGraphic::LowerTee            => LOWERTEE,
            BoxDrawingGraphic::UpperTee            => UPPERTEE,
            BoxDrawingGraphic::HorizontalLine      => HORIZONTALLINE,
            BoxDrawingGraphic::UpperHorizontalLine => UPPERHORIZONTALLINE,
            BoxDrawingGraphic::LowerHorizontalLine => LOWERHORIZONTALLINE,
            BoxDrawingGraphic::VerticalLine        => VERTICALLINE,
            BoxDrawingGraphic::LeftVerticalLine    => LEFTVERTICALLINE,
            BoxDrawingGraphic::RightVerticalLine   => RIGHTVERTICALLINE,
            BoxDrawingGraphic::Plus                => PLUS
        }
    }
}

impl TryFrom<GraphicMask> for BoxDrawingGraphic {
    type Error = NCurseswWinError;

    fn try_from(raw: GraphicMask) -> Result<Self, Self::Error> {
        match raw {
            UPPERLEFTCORNER     => Ok(BoxDrawingGraphic::UpperLeftCorner),
            LOWERLEFTCORNER     => Ok(BoxDrawingGraphic::LowerLeftCorner),
            UPPERRIGHTCORNER    => Ok(BoxDrawingGraphic::UpperRightCorner),
            LOWERRIGHTCORNER    => Ok(BoxDrawingGraphic::LowerRightCorner),
            RIGHTTEE            => Ok(BoxDrawingGraphic::RightTee),
            LEFTTEE             => Ok(BoxDrawingGraphic::LeftTee),
            LOWERTEE            => Ok(BoxDrawingGraphic::LowerTee),
            UPPERTEE            => Ok(BoxDrawingGraphic::UpperTee),
            HORIZONTALLINE      => Ok(BoxDrawingGraphic::HorizontalLine),
            UPPERHORIZONTALLINE => Ok(BoxDrawingGraphic::UpperHorizontalLine),
            LOWERHORIZONTALLINE => Ok(BoxDrawingGraphic::LowerHorizontalLine),
            VERTICALLINE        => Ok(BoxDrawingGraphic::VerticalLine),
            LEFTVERTICALLINE    => Ok(BoxDrawingGraphic::LeftVerticalLine),
            RIGHTVERTICALLINE   => Ok(BoxDrawingGraphic::RightVerticalLine),
            PLUS                => Ok(BoxDrawingGraphic::Plus),
            _                   => Err(NCurseswWinError::InternalError)
        }
    }
}

impl BoxDrawingGraphic {
    pub(in crate) fn transform(self, rhs: Self, remap: bool) -> Self {
        let into_value = |box_drawing_graphic: Self| -> GraphicMask {
            Self::into(match box_drawing_graphic {
                BoxDrawingGraphic::UpperHorizontalLine |
                BoxDrawingGraphic::LowerHorizontalLine => if remap {
                    BoxDrawingGraphic::HorizontalLine
                } else {
                    box_drawing_graphic
                },
                BoxDrawingGraphic::LeftVerticalLine |
                BoxDrawingGraphic::RightVerticalLine   => if remap {
                    BoxDrawingGraphic::VerticalLine
                } else {
                    box_drawing_graphic
                },
                _                                      => box_drawing_graphic
            })
        };

        match Self::try_from(into_value(self) | into_value(rhs)) {
            Err(_)                  => self,
            Ok(box_drawing_graphic) => box_drawing_graphic
        }
    }
}

#[test]
fn transform_test() {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    struct MatrixKey {
        lhs   : BoxDrawingGraphic,
        rhs   : BoxDrawingGraphic,
        remap : bool
    }

    impl MatrixKey {
        fn new(lhs: BoxDrawingGraphic, rhs: BoxDrawingGraphic, remap: bool) -> Self {
            Self { lhs, rhs, remap }
        }

        fn lhs(self) -> BoxDrawingGraphic {
            self.lhs
        }

        fn rhs(self) -> BoxDrawingGraphic {
            self.rhs
        }

        fn remap(self) -> bool {
            self.remap
        }
    }

    let upper_left_corner     = BoxDrawingGraphic::UpperLeftCorner;
    let lower_left_corner     = BoxDrawingGraphic::LowerLeftCorner;
    let upper_right_corner    = BoxDrawingGraphic::UpperRightCorner;
    let lower_right_corner    = BoxDrawingGraphic::LowerRightCorner;
    let right_tee             = BoxDrawingGraphic::RightTee;
    let left_tee              = BoxDrawingGraphic::LeftTee;
    let lower_tee             = BoxDrawingGraphic::LowerTee;
    let upper_tee             = BoxDrawingGraphic::UpperTee;
    let horizontal_line       = BoxDrawingGraphic::HorizontalLine;
    let upper_horizontal_line = BoxDrawingGraphic::UpperHorizontalLine;
    let lower_horizontal_line = BoxDrawingGraphic::LowerHorizontalLine;
    let vertical_line         = BoxDrawingGraphic::VerticalLine;
    let left_vertical_line    = BoxDrawingGraphic::LeftVerticalLine;
    let right_vertical_line   = BoxDrawingGraphic::RightVerticalLine;
    let plus                  = BoxDrawingGraphic::Plus;

    let transform_results = {
        let mut transform_results: HashMap<MatrixKey, BoxDrawingGraphic> = HashMap::new();

        transform_results.insert(MatrixKey::new(upper_left_corner, upper_left_corner, false),         upper_left_corner);
        transform_results.insert(MatrixKey::new(upper_left_corner, lower_left_corner, false),         left_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, upper_right_corner, false),        upper_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, lower_right_corner, false),        plus);
        transform_results.insert(MatrixKey::new(upper_left_corner, right_tee, false),                 plus);
        transform_results.insert(MatrixKey::new(upper_left_corner, left_tee, false),                  left_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, lower_tee, false),                 plus);
        transform_results.insert(MatrixKey::new(upper_left_corner, upper_tee, false),                 upper_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, horizontal_line, false),           upper_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, upper_horizontal_line, false),     upper_left_corner);
        transform_results.insert(MatrixKey::new(upper_left_corner, lower_horizontal_line, false),     upper_left_corner);
        transform_results.insert(MatrixKey::new(upper_left_corner, vertical_line, false),             left_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, left_vertical_line, false),        upper_left_corner);
        transform_results.insert(MatrixKey::new(upper_left_corner, right_vertical_line, false),       upper_left_corner);
        transform_results.insert(MatrixKey::new(upper_left_corner, plus, false),                      plus);

        transform_results.insert(MatrixKey::new(lower_left_corner, upper_left_corner, false),         left_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, lower_left_corner, false),         lower_left_corner);
        transform_results.insert(MatrixKey::new(lower_left_corner, upper_right_corner, false),        plus);
        transform_results.insert(MatrixKey::new(lower_left_corner, lower_right_corner, false),        lower_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, right_tee, false),                 plus);
        transform_results.insert(MatrixKey::new(lower_left_corner, left_tee, false),                  left_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, lower_tee, false),                 lower_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, upper_tee, false),                 plus);
        transform_results.insert(MatrixKey::new(lower_left_corner, horizontal_line, false),           lower_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, upper_horizontal_line, false),     lower_left_corner);
        transform_results.insert(MatrixKey::new(lower_left_corner, lower_horizontal_line, false),     lower_left_corner);
        transform_results.insert(MatrixKey::new(lower_left_corner, vertical_line, false),             left_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, left_vertical_line, false),        lower_left_corner);
        transform_results.insert(MatrixKey::new(lower_left_corner, right_vertical_line, false),       lower_left_corner);
        transform_results.insert(MatrixKey::new(lower_left_corner, plus, false),                      plus);

        transform_results.insert(MatrixKey::new(upper_right_corner, upper_left_corner, false),        upper_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, lower_left_corner, false),        plus);
        transform_results.insert(MatrixKey::new(upper_right_corner, upper_right_corner, false),       upper_right_corner);
        transform_results.insert(MatrixKey::new(upper_right_corner, lower_right_corner, false),       right_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, right_tee, false),                right_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, left_tee, false),                 plus);
        transform_results.insert(MatrixKey::new(upper_right_corner, lower_tee, false),                plus);
        transform_results.insert(MatrixKey::new(upper_right_corner, upper_tee, false),                upper_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, horizontal_line, false),          upper_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, upper_horizontal_line, false),    upper_right_corner);
        transform_results.insert(MatrixKey::new(upper_right_corner, lower_horizontal_line, false),    upper_right_corner);
        transform_results.insert(MatrixKey::new(upper_right_corner, vertical_line, false),            right_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, left_vertical_line, false),       upper_right_corner);
        transform_results.insert(MatrixKey::new(upper_right_corner, right_vertical_line, false),      upper_right_corner);
        transform_results.insert(MatrixKey::new(upper_right_corner, plus, false),                     plus);

        transform_results.insert(MatrixKey::new(lower_right_corner, upper_left_corner, false),        plus);
        transform_results.insert(MatrixKey::new(lower_right_corner, lower_left_corner, false),        lower_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, upper_right_corner, false),       right_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, lower_right_corner, false),       lower_right_corner);
        transform_results.insert(MatrixKey::new(lower_right_corner, right_tee, false),                right_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, left_tee, false),                 plus);
        transform_results.insert(MatrixKey::new(lower_right_corner, lower_tee, false),                lower_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, upper_tee, false),                plus);
        transform_results.insert(MatrixKey::new(lower_right_corner, horizontal_line, false),          lower_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, upper_horizontal_line, false),    lower_right_corner);
        transform_results.insert(MatrixKey::new(lower_right_corner, lower_horizontal_line, false),    lower_right_corner);
        transform_results.insert(MatrixKey::new(lower_right_corner, vertical_line, false),            right_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, left_vertical_line, false),       lower_right_corner);
        transform_results.insert(MatrixKey::new(lower_right_corner, right_vertical_line, false),      lower_right_corner);
        transform_results.insert(MatrixKey::new(lower_right_corner, plus, false),                     plus);

        transform_results.insert(MatrixKey::new(right_tee, upper_left_corner, false),                 plus);
        transform_results.insert(MatrixKey::new(right_tee, lower_left_corner, false),                 plus);
        transform_results.insert(MatrixKey::new(right_tee, upper_right_corner, false),                right_tee);
        transform_results.insert(MatrixKey::new(right_tee, lower_right_corner, false),                right_tee);
        transform_results.insert(MatrixKey::new(right_tee, right_tee, false),                         right_tee);
        transform_results.insert(MatrixKey::new(right_tee, left_tee, false),                          plus);
        transform_results.insert(MatrixKey::new(right_tee, lower_tee, false),                         plus);
        transform_results.insert(MatrixKey::new(right_tee, upper_tee, false),                         plus);
        transform_results.insert(MatrixKey::new(right_tee, horizontal_line, false),                   plus);
        transform_results.insert(MatrixKey::new(right_tee, upper_horizontal_line, false),             right_tee);
        transform_results.insert(MatrixKey::new(right_tee, lower_horizontal_line, false),             right_tee);
        transform_results.insert(MatrixKey::new(right_tee, vertical_line, false),                     right_tee);
        transform_results.insert(MatrixKey::new(right_tee, left_vertical_line, false),                right_tee);
        transform_results.insert(MatrixKey::new(right_tee, right_vertical_line, false),               right_tee);
        transform_results.insert(MatrixKey::new(right_tee, plus, false),                              plus);

        transform_results.insert(MatrixKey::new(left_tee, upper_left_corner, false),                  left_tee);
        transform_results.insert(MatrixKey::new(left_tee, lower_left_corner, false),                  left_tee);
        transform_results.insert(MatrixKey::new(left_tee, upper_right_corner, false),                 plus);
        transform_results.insert(MatrixKey::new(left_tee, lower_right_corner, false),                 plus);
        transform_results.insert(MatrixKey::new(left_tee, right_tee, false),                          plus);
        transform_results.insert(MatrixKey::new(left_tee, left_tee, false),                           left_tee);
        transform_results.insert(MatrixKey::new(left_tee, lower_tee, false),                          plus);
        transform_results.insert(MatrixKey::new(left_tee, upper_tee, false),                          plus);
        transform_results.insert(MatrixKey::new(left_tee, horizontal_line, false),                    plus);
        transform_results.insert(MatrixKey::new(left_tee, upper_horizontal_line, false),              left_tee);
        transform_results.insert(MatrixKey::new(left_tee, lower_horizontal_line, false),              left_tee);
        transform_results.insert(MatrixKey::new(left_tee, vertical_line, false),                      left_tee);
        transform_results.insert(MatrixKey::new(left_tee, left_vertical_line, false),                 left_tee);
        transform_results.insert(MatrixKey::new(left_tee, right_vertical_line, false),                left_tee);
        transform_results.insert(MatrixKey::new(left_tee, plus, false),                               plus);

        transform_results.insert(MatrixKey::new(lower_tee, upper_left_corner, false),                 plus);
        transform_results.insert(MatrixKey::new(lower_tee, lower_left_corner, false),                 lower_tee);
        transform_results.insert(MatrixKey::new(lower_tee, upper_right_corner, false),                plus);
        transform_results.insert(MatrixKey::new(lower_tee, lower_right_corner, false),                lower_tee);
        transform_results.insert(MatrixKey::new(lower_tee, right_tee, false),                         plus);
        transform_results.insert(MatrixKey::new(lower_tee, left_tee, false),                          plus);
        transform_results.insert(MatrixKey::new(lower_tee, lower_tee, false),                         lower_tee);
        transform_results.insert(MatrixKey::new(lower_tee, upper_tee, false),                         plus);
        transform_results.insert(MatrixKey::new(lower_tee, horizontal_line, false),                   lower_tee);
        transform_results.insert(MatrixKey::new(lower_tee, upper_horizontal_line, false),             lower_tee);
        transform_results.insert(MatrixKey::new(lower_tee, lower_horizontal_line, false),             lower_tee);
        transform_results.insert(MatrixKey::new(lower_tee, vertical_line, false),                     plus);
        transform_results.insert(MatrixKey::new(lower_tee, left_vertical_line, false),                lower_tee);
        transform_results.insert(MatrixKey::new(lower_tee, right_vertical_line, false),               lower_tee);
        transform_results.insert(MatrixKey::new(lower_tee, plus, false),                              plus);

        transform_results.insert(MatrixKey::new(upper_tee, upper_left_corner, false),                 upper_tee);
        transform_results.insert(MatrixKey::new(upper_tee, lower_left_corner, false),                 plus);
        transform_results.insert(MatrixKey::new(upper_tee, upper_right_corner, false),                upper_tee);
        transform_results.insert(MatrixKey::new(upper_tee, lower_right_corner, false),                plus);
        transform_results.insert(MatrixKey::new(upper_tee, right_tee, false),                         plus);
        transform_results.insert(MatrixKey::new(upper_tee, left_tee, false),                          plus);
        transform_results.insert(MatrixKey::new(upper_tee, lower_tee, false),                         plus);
        transform_results.insert(MatrixKey::new(upper_tee, upper_tee, false),                         upper_tee);
        transform_results.insert(MatrixKey::new(upper_tee, horizontal_line, false),                   upper_tee);
        transform_results.insert(MatrixKey::new(upper_tee, upper_horizontal_line, false),             upper_tee);
        transform_results.insert(MatrixKey::new(upper_tee, lower_horizontal_line, false),             upper_tee);
        transform_results.insert(MatrixKey::new(upper_tee, vertical_line, false),                     plus);
        transform_results.insert(MatrixKey::new(upper_tee, left_vertical_line, false),                upper_tee);
        transform_results.insert(MatrixKey::new(upper_tee, right_vertical_line, false),               upper_tee);
        transform_results.insert(MatrixKey::new(upper_tee, plus, false),                              plus);

        transform_results.insert(MatrixKey::new(horizontal_line, upper_left_corner, false),           upper_tee);
        transform_results.insert(MatrixKey::new(horizontal_line, lower_left_corner, false),           lower_tee);
        transform_results.insert(MatrixKey::new(horizontal_line, upper_right_corner, false),          upper_tee);
        transform_results.insert(MatrixKey::new(horizontal_line, lower_right_corner, false),          lower_tee);
        transform_results.insert(MatrixKey::new(horizontal_line, right_tee, false),                   plus);
        transform_results.insert(MatrixKey::new(horizontal_line, left_tee, false),                    plus);
        transform_results.insert(MatrixKey::new(horizontal_line, lower_tee, false),                   lower_tee);
        transform_results.insert(MatrixKey::new(horizontal_line, upper_tee, false),                   upper_tee);
        transform_results.insert(MatrixKey::new(horizontal_line, horizontal_line, false),             horizontal_line);
        transform_results.insert(MatrixKey::new(horizontal_line, upper_horizontal_line, false),       horizontal_line);
        transform_results.insert(MatrixKey::new(horizontal_line, lower_horizontal_line, false),       horizontal_line);
        transform_results.insert(MatrixKey::new(horizontal_line, vertical_line, false),               plus);
        transform_results.insert(MatrixKey::new(horizontal_line, left_vertical_line, false),          horizontal_line);
        transform_results.insert(MatrixKey::new(horizontal_line, right_vertical_line, false),         horizontal_line);
        transform_results.insert(MatrixKey::new(horizontal_line, plus, false),                        plus);

        transform_results.insert(MatrixKey::new(upper_horizontal_line, upper_left_corner, false),     upper_horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, lower_left_corner, false),     upper_horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, upper_right_corner, false),    upper_horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, lower_right_corner, false),    upper_horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, right_tee, false),             upper_horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, left_tee, false),              upper_horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, lower_tee, false),             upper_horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, upper_tee, false),             upper_horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, horizontal_line, false),       upper_horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, upper_horizontal_line, false), upper_horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, lower_horizontal_line, false), upper_horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, vertical_line, false),         upper_horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, left_vertical_line, false),    upper_horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, right_vertical_line, false),   upper_horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, plus, false),                  upper_horizontal_line);

        transform_results.insert(MatrixKey::new(lower_horizontal_line, upper_left_corner, false),     lower_horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, lower_left_corner, false),     lower_horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, upper_right_corner, false),    lower_horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, lower_right_corner, false),    lower_horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, right_tee, false),             lower_horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, left_tee, false),              lower_horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, lower_tee, false),             lower_horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, upper_tee, false),             lower_horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, horizontal_line, false),       lower_horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, upper_horizontal_line, false), lower_horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, lower_horizontal_line, false), lower_horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, vertical_line, false),         lower_horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, left_vertical_line, false),    lower_horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, right_vertical_line, false),   lower_horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, plus, false),                  lower_horizontal_line);

        transform_results.insert(MatrixKey::new(vertical_line, upper_left_corner, false),             left_tee);
        transform_results.insert(MatrixKey::new(vertical_line, lower_left_corner, false),             left_tee);
        transform_results.insert(MatrixKey::new(vertical_line, upper_right_corner, false),            right_tee);
        transform_results.insert(MatrixKey::new(vertical_line, lower_right_corner, false),            right_tee);
        transform_results.insert(MatrixKey::new(vertical_line, right_tee, false),                     right_tee);
        transform_results.insert(MatrixKey::new(vertical_line, left_tee, false),                      left_tee);
        transform_results.insert(MatrixKey::new(vertical_line, lower_tee, false),                     plus);
        transform_results.insert(MatrixKey::new(vertical_line, upper_tee, false),                     plus);
        transform_results.insert(MatrixKey::new(vertical_line, horizontal_line, false),               plus);
        transform_results.insert(MatrixKey::new(vertical_line, upper_horizontal_line, false),         vertical_line);
        transform_results.insert(MatrixKey::new(vertical_line, lower_horizontal_line, false),         vertical_line);
        transform_results.insert(MatrixKey::new(vertical_line, vertical_line, false),                 vertical_line);
        transform_results.insert(MatrixKey::new(vertical_line, left_vertical_line, false),            vertical_line);
        transform_results.insert(MatrixKey::new(vertical_line, right_vertical_line, false),           vertical_line);
        transform_results.insert(MatrixKey::new(vertical_line, plus, false),                          plus);

        transform_results.insert(MatrixKey::new(left_vertical_line, upper_left_corner, false),        left_vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, lower_left_corner, false),        left_vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, upper_right_corner, false),       left_vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, lower_right_corner, false),       left_vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, right_tee, false),                left_vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, left_tee, false),                 left_vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, lower_tee, false),                left_vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, upper_tee, false),                left_vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, horizontal_line, false),          left_vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, upper_horizontal_line, false),    left_vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, lower_horizontal_line, false),    left_vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, vertical_line, false),            left_vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, left_vertical_line, false),       left_vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, right_vertical_line, false),      left_vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, plus, false),                     left_vertical_line);

        transform_results.insert(MatrixKey::new(right_vertical_line, upper_left_corner, false),       right_vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, lower_left_corner, false),       right_vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, upper_right_corner, false),      right_vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, lower_right_corner, false),      right_vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, right_tee, false),               right_vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, left_tee, false),                right_vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, lower_tee, false),               right_vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, upper_tee, false),               right_vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, horizontal_line, false),         right_vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, upper_horizontal_line, false),   right_vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, lower_horizontal_line, false),   right_vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, vertical_line, false),           right_vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, left_vertical_line, false),      right_vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, right_vertical_line, false),     right_vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, plus, false),                    right_vertical_line);

        transform_results.insert(MatrixKey::new(plus, upper_left_corner, false),                      plus);
        transform_results.insert(MatrixKey::new(plus, lower_left_corner, false),                      plus);
        transform_results.insert(MatrixKey::new(plus, upper_right_corner, false),                     plus);
        transform_results.insert(MatrixKey::new(plus, lower_right_corner, false),                     plus);
        transform_results.insert(MatrixKey::new(plus, right_tee, false),                              plus);
        transform_results.insert(MatrixKey::new(plus, left_tee, false),                               plus);
        transform_results.insert(MatrixKey::new(plus, lower_tee, false),                              plus);
        transform_results.insert(MatrixKey::new(plus, upper_tee, false),                              plus);
        transform_results.insert(MatrixKey::new(plus, horizontal_line, false),                        plus);
        transform_results.insert(MatrixKey::new(plus, upper_horizontal_line, false),                  plus);
        transform_results.insert(MatrixKey::new(plus, lower_horizontal_line, false),                  plus);
        transform_results.insert(MatrixKey::new(plus, vertical_line, false),                          plus);
        transform_results.insert(MatrixKey::new(plus, left_vertical_line, false),                     plus);
        transform_results.insert(MatrixKey::new(plus, right_vertical_line, false),                    plus);
        transform_results.insert(MatrixKey::new(plus, plus, false),                                   plus);

        transform_results.insert(MatrixKey::new(upper_left_corner, upper_left_corner, true),          upper_left_corner);
        transform_results.insert(MatrixKey::new(upper_left_corner, lower_left_corner, true),          left_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, upper_right_corner, true),         upper_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, lower_right_corner, true),         plus);
        transform_results.insert(MatrixKey::new(upper_left_corner, right_tee, true),                  plus);
        transform_results.insert(MatrixKey::new(upper_left_corner, left_tee, true),                   left_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, lower_tee, true),                  plus);
        transform_results.insert(MatrixKey::new(upper_left_corner, upper_tee, true),                  upper_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, horizontal_line, true),            upper_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, upper_horizontal_line, true),      upper_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, lower_horizontal_line, true),      upper_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, vertical_line, true),              left_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, left_vertical_line, true),         left_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, right_vertical_line, true),        left_tee);
        transform_results.insert(MatrixKey::new(upper_left_corner, plus, true),                       plus);

        transform_results.insert(MatrixKey::new(lower_left_corner, upper_left_corner, true),          left_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, lower_left_corner, true),          lower_left_corner);
        transform_results.insert(MatrixKey::new(lower_left_corner, upper_right_corner, true),         plus);
        transform_results.insert(MatrixKey::new(lower_left_corner, lower_right_corner, true),         lower_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, right_tee, true),                  plus);
        transform_results.insert(MatrixKey::new(lower_left_corner, left_tee, true),                   left_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, lower_tee, true),                  lower_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, upper_tee, true),                  plus);
        transform_results.insert(MatrixKey::new(lower_left_corner, horizontal_line, true),            lower_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, upper_horizontal_line, true),      lower_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, lower_horizontal_line, true),      lower_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, vertical_line, true),              left_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, left_vertical_line, true),         left_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, right_vertical_line, true),        left_tee);
        transform_results.insert(MatrixKey::new(lower_left_corner, plus, true),                       plus);

        transform_results.insert(MatrixKey::new(upper_right_corner, upper_left_corner, true),         upper_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, lower_left_corner, true),         plus);
        transform_results.insert(MatrixKey::new(upper_right_corner, upper_right_corner, true),        upper_right_corner);
        transform_results.insert(MatrixKey::new(upper_right_corner, lower_right_corner, true),        right_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, right_tee, true),                 right_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, left_tee, true),                  plus);
        transform_results.insert(MatrixKey::new(upper_right_corner, lower_tee, true),                 plus);
        transform_results.insert(MatrixKey::new(upper_right_corner, upper_tee, true),                 upper_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, horizontal_line, true),           upper_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, upper_horizontal_line, true),     upper_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, lower_horizontal_line, true),     upper_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, vertical_line, true),             right_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, left_vertical_line, true),        right_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, right_vertical_line, true),       right_tee);
        transform_results.insert(MatrixKey::new(upper_right_corner, plus, true),                      plus);

        transform_results.insert(MatrixKey::new(lower_right_corner, upper_left_corner, true),         plus);
        transform_results.insert(MatrixKey::new(lower_right_corner, lower_left_corner, true),         lower_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, upper_right_corner, true),        right_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, lower_right_corner, true),        lower_right_corner);
        transform_results.insert(MatrixKey::new(lower_right_corner, right_tee, true),                 right_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, left_tee, true),                  plus);
        transform_results.insert(MatrixKey::new(lower_right_corner, lower_tee, true),                 lower_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, upper_tee, true),                 plus);
        transform_results.insert(MatrixKey::new(lower_right_corner, horizontal_line, true),           lower_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, upper_horizontal_line, true),     lower_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, lower_horizontal_line, true),     lower_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, vertical_line, true),             right_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, left_vertical_line, true),        right_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, right_vertical_line, true),       right_tee);
        transform_results.insert(MatrixKey::new(lower_right_corner, plus, true),                      plus);

        transform_results.insert(MatrixKey::new(right_tee, upper_left_corner, true),                  plus);
        transform_results.insert(MatrixKey::new(right_tee, lower_left_corner, true),                  plus);
        transform_results.insert(MatrixKey::new(right_tee, upper_right_corner, true),                 right_tee);
        transform_results.insert(MatrixKey::new(right_tee, lower_right_corner, true),                 right_tee);
        transform_results.insert(MatrixKey::new(right_tee, right_tee, true),                          right_tee);
        transform_results.insert(MatrixKey::new(right_tee, left_tee, true),                           plus);
        transform_results.insert(MatrixKey::new(right_tee, lower_tee, true),                          plus);
        transform_results.insert(MatrixKey::new(right_tee, upper_tee, true),                          plus);
        transform_results.insert(MatrixKey::new(right_tee, horizontal_line, true),                    plus);
        transform_results.insert(MatrixKey::new(right_tee, upper_horizontal_line, true),              plus);
        transform_results.insert(MatrixKey::new(right_tee, lower_horizontal_line, true),              plus);
        transform_results.insert(MatrixKey::new(right_tee, vertical_line, true),                      right_tee);
        transform_results.insert(MatrixKey::new(right_tee, left_vertical_line, true),                 right_tee);
        transform_results.insert(MatrixKey::new(right_tee, right_vertical_line, true),                right_tee);
        transform_results.insert(MatrixKey::new(right_tee, plus, true),                               plus);

        transform_results.insert(MatrixKey::new(left_tee, upper_left_corner, true),                   left_tee);
        transform_results.insert(MatrixKey::new(left_tee, lower_left_corner, true),                   left_tee);
        transform_results.insert(MatrixKey::new(left_tee, upper_right_corner, true),                  plus);
        transform_results.insert(MatrixKey::new(left_tee, lower_right_corner, true),                  plus);
        transform_results.insert(MatrixKey::new(left_tee, right_tee, true),                           plus);
        transform_results.insert(MatrixKey::new(left_tee, left_tee, true),                            left_tee);
        transform_results.insert(MatrixKey::new(left_tee, lower_tee, true),                           plus);
        transform_results.insert(MatrixKey::new(left_tee, upper_tee, true),                           plus);
        transform_results.insert(MatrixKey::new(left_tee, horizontal_line, true),                     plus);
        transform_results.insert(MatrixKey::new(left_tee, upper_horizontal_line, true),               plus);
        transform_results.insert(MatrixKey::new(left_tee, lower_horizontal_line, true),               plus);
        transform_results.insert(MatrixKey::new(left_tee, vertical_line, true),                       left_tee);
        transform_results.insert(MatrixKey::new(left_tee, left_vertical_line, true),                  left_tee);
        transform_results.insert(MatrixKey::new(left_tee, right_vertical_line, true),                 left_tee);
        transform_results.insert(MatrixKey::new(left_tee, plus, true),                                plus);

        transform_results.insert(MatrixKey::new(lower_tee, upper_left_corner, true),                  plus);
        transform_results.insert(MatrixKey::new(lower_tee, lower_left_corner, true),                  lower_tee);
        transform_results.insert(MatrixKey::new(lower_tee, upper_right_corner, true),                 plus);
        transform_results.insert(MatrixKey::new(lower_tee, lower_right_corner, true),                 lower_tee);
        transform_results.insert(MatrixKey::new(lower_tee, right_tee, true),                          plus);
        transform_results.insert(MatrixKey::new(lower_tee, left_tee, true),                           plus);
        transform_results.insert(MatrixKey::new(lower_tee, lower_tee, true),                          lower_tee);
        transform_results.insert(MatrixKey::new(lower_tee, upper_tee, true),                          plus);
        transform_results.insert(MatrixKey::new(lower_tee, horizontal_line, true),                    lower_tee);
        transform_results.insert(MatrixKey::new(lower_tee, upper_horizontal_line, true),              lower_tee);
        transform_results.insert(MatrixKey::new(lower_tee, lower_horizontal_line, true),              lower_tee);
        transform_results.insert(MatrixKey::new(lower_tee, vertical_line, true),                      plus);
        transform_results.insert(MatrixKey::new(lower_tee, left_vertical_line, true),                 plus);
        transform_results.insert(MatrixKey::new(lower_tee, right_vertical_line, true),                plus);
        transform_results.insert(MatrixKey::new(lower_tee, plus, true),                               plus);

        transform_results.insert(MatrixKey::new(upper_tee, upper_left_corner, true),                  upper_tee);
        transform_results.insert(MatrixKey::new(upper_tee, lower_left_corner, true),                  plus);
        transform_results.insert(MatrixKey::new(upper_tee, upper_right_corner, true),                 upper_tee);
        transform_results.insert(MatrixKey::new(upper_tee, lower_right_corner, true),                 plus);
        transform_results.insert(MatrixKey::new(upper_tee, right_tee, true),                          plus);
        transform_results.insert(MatrixKey::new(upper_tee, left_tee, true),                           plus);
        transform_results.insert(MatrixKey::new(upper_tee, lower_tee, true),                          plus);
        transform_results.insert(MatrixKey::new(upper_tee, upper_tee, true),                          upper_tee);
        transform_results.insert(MatrixKey::new(upper_tee, horizontal_line, true),                    upper_tee);
        transform_results.insert(MatrixKey::new(upper_tee, upper_horizontal_line, true),              upper_tee);
        transform_results.insert(MatrixKey::new(upper_tee, lower_horizontal_line, true),              upper_tee);
        transform_results.insert(MatrixKey::new(upper_tee, vertical_line, true),                      plus);
        transform_results.insert(MatrixKey::new(upper_tee, left_vertical_line, true),                 plus);
        transform_results.insert(MatrixKey::new(upper_tee, right_vertical_line, true),                plus);
        transform_results.insert(MatrixKey::new(upper_tee, plus, true),                               plus);

        transform_results.insert(MatrixKey::new(horizontal_line, upper_left_corner, true),            upper_tee);
        transform_results.insert(MatrixKey::new(horizontal_line, lower_left_corner, true),            lower_tee);
        transform_results.insert(MatrixKey::new(horizontal_line, upper_right_corner, true),           upper_tee);
        transform_results.insert(MatrixKey::new(horizontal_line, lower_right_corner, true),           lower_tee);
        transform_results.insert(MatrixKey::new(horizontal_line, right_tee, true),                    plus);
        transform_results.insert(MatrixKey::new(horizontal_line, left_tee, true),                     plus);
        transform_results.insert(MatrixKey::new(horizontal_line, lower_tee, true),                    lower_tee);
        transform_results.insert(MatrixKey::new(horizontal_line, upper_tee, true),                    upper_tee);
        transform_results.insert(MatrixKey::new(horizontal_line, horizontal_line, true),              horizontal_line);
        transform_results.insert(MatrixKey::new(horizontal_line, upper_horizontal_line, true),        horizontal_line);
        transform_results.insert(MatrixKey::new(horizontal_line, lower_horizontal_line, true),        horizontal_line);
        transform_results.insert(MatrixKey::new(horizontal_line, vertical_line, true),                plus);
        transform_results.insert(MatrixKey::new(horizontal_line, left_vertical_line, true),           plus);
        transform_results.insert(MatrixKey::new(horizontal_line, right_vertical_line, true),          plus);
        transform_results.insert(MatrixKey::new(horizontal_line, plus, true),                         plus);

        transform_results.insert(MatrixKey::new(upper_horizontal_line, upper_left_corner, true),      upper_tee);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, lower_left_corner, true),      lower_tee);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, upper_right_corner, true),     upper_tee);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, lower_right_corner, true),     lower_tee);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, right_tee, true),              plus);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, left_tee, true),               plus);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, lower_tee, true),              lower_tee);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, upper_tee, true),              upper_tee);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, horizontal_line, true),        horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, upper_horizontal_line, true),  horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, lower_horizontal_line, true),  horizontal_line);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, vertical_line, true),          plus);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, left_vertical_line, true),     plus);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, right_vertical_line, true),    plus);
        transform_results.insert(MatrixKey::new(upper_horizontal_line, plus, true),                   plus);

        transform_results.insert(MatrixKey::new(lower_horizontal_line, upper_left_corner, true),      upper_tee);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, lower_left_corner, true),      lower_tee);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, upper_right_corner, true),     upper_tee);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, lower_right_corner, true),     lower_tee);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, right_tee, true),              plus);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, left_tee, true),               plus);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, lower_tee, true),              lower_tee);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, upper_tee, true),              upper_tee);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, horizontal_line, true),        horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, upper_horizontal_line, true),  horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, lower_horizontal_line, true),  horizontal_line);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, vertical_line, true),          plus);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, left_vertical_line, true),     plus);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, right_vertical_line, true),    plus);
        transform_results.insert(MatrixKey::new(lower_horizontal_line, plus, true),                   plus);

        transform_results.insert(MatrixKey::new(vertical_line, upper_left_corner, true),              left_tee);
        transform_results.insert(MatrixKey::new(vertical_line, lower_left_corner, true),              left_tee);
        transform_results.insert(MatrixKey::new(vertical_line, upper_right_corner, true),             right_tee);
        transform_results.insert(MatrixKey::new(vertical_line, lower_right_corner, true),             right_tee);
        transform_results.insert(MatrixKey::new(vertical_line, right_tee, true),                      right_tee);
        transform_results.insert(MatrixKey::new(vertical_line, left_tee, true),                       left_tee);
        transform_results.insert(MatrixKey::new(vertical_line, lower_tee, true),                      plus);
        transform_results.insert(MatrixKey::new(vertical_line, upper_tee, true),                      plus);
        transform_results.insert(MatrixKey::new(vertical_line, horizontal_line, true),                plus);
        transform_results.insert(MatrixKey::new(vertical_line, upper_horizontal_line, true),          plus);
        transform_results.insert(MatrixKey::new(vertical_line, lower_horizontal_line, true),          plus);
        transform_results.insert(MatrixKey::new(vertical_line, vertical_line, true),                  vertical_line);
        transform_results.insert(MatrixKey::new(vertical_line, left_vertical_line, true),             vertical_line);
        transform_results.insert(MatrixKey::new(vertical_line, right_vertical_line, true),            vertical_line);
        transform_results.insert(MatrixKey::new(vertical_line, plus, true),                           plus);

        transform_results.insert(MatrixKey::new(left_vertical_line, upper_left_corner, true),         left_tee);
        transform_results.insert(MatrixKey::new(left_vertical_line, lower_left_corner, true),         left_tee);
        transform_results.insert(MatrixKey::new(left_vertical_line, upper_right_corner, true),        right_tee);
        transform_results.insert(MatrixKey::new(left_vertical_line, lower_right_corner, true),        right_tee);
        transform_results.insert(MatrixKey::new(left_vertical_line, right_tee, true),                 right_tee);
        transform_results.insert(MatrixKey::new(left_vertical_line, left_tee, true),                  left_tee);
        transform_results.insert(MatrixKey::new(left_vertical_line, lower_tee, true),                 plus);
        transform_results.insert(MatrixKey::new(left_vertical_line, upper_tee, true),                 plus);
        transform_results.insert(MatrixKey::new(left_vertical_line, horizontal_line, true),           plus);
        transform_results.insert(MatrixKey::new(left_vertical_line, upper_horizontal_line, true),     plus);
        transform_results.insert(MatrixKey::new(left_vertical_line, lower_horizontal_line, true),     plus);
        transform_results.insert(MatrixKey::new(left_vertical_line, vertical_line, true),             vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, left_vertical_line, true),        vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, right_vertical_line, true),       vertical_line);
        transform_results.insert(MatrixKey::new(left_vertical_line, plus, true),                      plus);

        transform_results.insert(MatrixKey::new(right_vertical_line, upper_left_corner, true),        left_tee);
        transform_results.insert(MatrixKey::new(right_vertical_line, lower_left_corner, true),        left_tee);
        transform_results.insert(MatrixKey::new(right_vertical_line, upper_right_corner, true),       right_tee);
        transform_results.insert(MatrixKey::new(right_vertical_line, lower_right_corner, true),       right_tee);
        transform_results.insert(MatrixKey::new(right_vertical_line, right_tee, true),                right_tee);
        transform_results.insert(MatrixKey::new(right_vertical_line, left_tee, true),                 left_tee);
        transform_results.insert(MatrixKey::new(right_vertical_line, lower_tee, true),                plus);
        transform_results.insert(MatrixKey::new(right_vertical_line, upper_tee, true),                plus);
        transform_results.insert(MatrixKey::new(right_vertical_line, horizontal_line, true),          plus);
        transform_results.insert(MatrixKey::new(right_vertical_line, upper_horizontal_line, true),    plus);
        transform_results.insert(MatrixKey::new(right_vertical_line, lower_horizontal_line, true),    plus);
        transform_results.insert(MatrixKey::new(right_vertical_line, vertical_line, true),            vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, left_vertical_line, true),       vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, right_vertical_line, true),      vertical_line);
        transform_results.insert(MatrixKey::new(right_vertical_line, plus, true),                     plus);

        transform_results.insert(MatrixKey::new(plus, upper_left_corner, true),                       plus);
        transform_results.insert(MatrixKey::new(plus, lower_left_corner, true),                       plus);
        transform_results.insert(MatrixKey::new(plus, upper_right_corner, true),                      plus);
        transform_results.insert(MatrixKey::new(plus, lower_right_corner, true),                      plus);
        transform_results.insert(MatrixKey::new(plus, right_tee, true),                               plus);
        transform_results.insert(MatrixKey::new(plus, left_tee, true),                                plus);
        transform_results.insert(MatrixKey::new(plus, lower_tee, true),                               plus);
        transform_results.insert(MatrixKey::new(plus, upper_tee, true),                               plus);
        transform_results.insert(MatrixKey::new(plus, horizontal_line, true),                         plus);
        transform_results.insert(MatrixKey::new(plus, upper_horizontal_line, true),                   plus);
        transform_results.insert(MatrixKey::new(plus, lower_horizontal_line, true),                   plus);
        transform_results.insert(MatrixKey::new(plus, vertical_line, true),                           plus);
        transform_results.insert(MatrixKey::new(plus, left_vertical_line, true),                      plus);
        transform_results.insert(MatrixKey::new(plus, right_vertical_line, true),                     plus);
        transform_results.insert(MatrixKey::new(plus, plus, true),                                    plus);

        transform_results
    };

    for (key, result) in transform_results {
        eprintln!("{:?} + {:?}, remap: {} = {:?}", key.lhs(), key.rhs(), key.remap(), result);

        assert_eq!(key.lhs().transform(key.rhs(), key.remap()), result);
    }
}
