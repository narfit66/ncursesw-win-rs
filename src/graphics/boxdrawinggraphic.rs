/*
    src/graphics/boxdrawinggraphic.rs

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

use std::convert::{TryFrom, Into};

use crate::ncurseswwinerror::NCurseswWinError;

/// The boxdrawing graphic characters.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BoxDrawingGraphic {
    UpperLeftCorner,
    LowerLeftCorner,
    UpperRightCorner,
    LowerRightCorner,
    RightTee,
    LeftTee,
    LowerTee,
    UpperTee,
    HorizontalLine,
    UpperHorizontalLine,
    LowerHorizontalLine,
    VerticalLine,
    LeftVerticalLine,
    RightVerticalLine,
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

const UPPERLEFTCORNER: u16     = 0b_000_011_010;
const LOWERLEFTCORNER: u16     = 0b_010_011_000;
const UPPERRIGHTCORNER: u16    = 0b_000_110_010;
const LOWERRIGHTCORNER: u16    = 0b_010_110_000;
const RIGHTTEE: u16            = 0b_010_110_010;
const LEFTTEE: u16             = 0b_010_011_010;
const LOWERTEE: u16            = 0b_010_111_000;
const UPPERTEE: u16            = 0b_000_111_010;
const HORIZONTALLINE: u16      = 0b_000_111_000;
const UPPERHORIZONTALLINE: u16 = 0b_111_000_000;
const LOWERHORIZONTALLINE: u16 = 0b_000_000_111;
const VERTICALLINE: u16        = 0b_010_010_010;
const LEFTVERTICALLINE: u16    = 0b_100_100_100;
const RIGHTVERTICALLINE: u16   = 0b_001_001_001;
const PLUS: u16                = 0b_010_111_010;

impl Into<u16> for BoxDrawingGraphic {
    fn into(self) -> u16 {
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

impl TryFrom<u16> for BoxDrawingGraphic {
    type Error = NCurseswWinError;

    fn try_from(raw: u16) -> Result<Self, Self::Error> {
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
    pub fn transform(self, rhs: Self, remap: bool) -> Self {
        let into_value = |raw: Self| -> u16 {
            Self::into(match raw {
                BoxDrawingGraphic::UpperHorizontalLine |
                BoxDrawingGraphic::LowerHorizontalLine => if remap {
                    BoxDrawingGraphic::HorizontalLine
                } else {
                    raw
                },
                BoxDrawingGraphic::LeftVerticalLine |
                BoxDrawingGraphic::RightVerticalLine   => if remap {
                    BoxDrawingGraphic::VerticalLine
                } else {
                    raw
                },
                _ => raw
            })
        };

        match Self::try_from(into_value(self) | into_value(rhs)) {
            Err(_) => self,
            Ok(g)  => g
        }
    }
}

#[test]
fn transform_test() {
    let ul = BoxDrawingGraphic::UpperLeftCorner;
    let hl = BoxDrawingGraphic::HorizontalLine;
    let vl = BoxDrawingGraphic::VerticalLine;
    let ut = BoxDrawingGraphic::UpperTee;
    let pl = BoxDrawingGraphic::Plus;

    assert_eq!(ul.transform(hl, true), ut);
    assert_eq!(hl.transform(ul, true), ut);

    assert_eq!(hl.transform(vl, true), pl);
    assert_eq!(vl.transform(hl, true), pl);

    assert_eq!(pl.transform(hl, true), pl);
    assert_eq!(pl.transform(vl, true), pl);

    assert_eq!(hl.transform(hl, true), hl);
}
