/*
    src/graphics/boxdrawing.rs

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

use std::collections::HashMap;

use crate::Window;
use crate::graphics::{BoxDrawingType, BoxDrawingTypeDetail, BoxDrawingGraphic};
use ncursesw::gen::{AttributesType, ColorPairType, ColorAttributeTypes, AttributesColorPairType};
use ncursesw::{getcchar, AttributesColorPairSet, ChtypeChar, WideChar, ComplexChar, ComplexString, Origin, Size, NCurseswError};
use ncursesw::shims::ncurses::{wchar_t, NCURSES_ACS};

#[derive(PartialEq, Eq, Hash)]
struct MatrixKey {
    box_drawing_type:    BoxDrawingType,
    box_drawing_graphic: BoxDrawingGraphic
}

impl MatrixKey {
    fn new(box_drawing_type: BoxDrawingType, box_drawing_graphic: BoxDrawingGraphic) -> Self {
        Self { box_drawing_type, box_drawing_graphic }
    }

    fn box_drawing_type(&self) -> BoxDrawingType {
        self.box_drawing_type
    }

    fn box_drawing_graphic(&self) -> BoxDrawingGraphic {
        self.box_drawing_graphic
    }
}

lazy_static! {
    static ref CHTYPEBOXDRAWING: HashMap<BoxDrawingGraphic, char> = {
        let mut graphics = HashMap::new();

        graphics.insert(BoxDrawingGraphic::UpperLeftCorner,     'l');
        graphics.insert(BoxDrawingGraphic::LowerLeftCorner,     'm');
        graphics.insert(BoxDrawingGraphic::UpperRightCorner,    'k');
        graphics.insert(BoxDrawingGraphic::LowerRightCorner,    'j');
        graphics.insert(BoxDrawingGraphic::RightTee,            'u');
        graphics.insert(BoxDrawingGraphic::LeftTee,             't');
        graphics.insert(BoxDrawingGraphic::LowerTee,            'w');
        graphics.insert(BoxDrawingGraphic::UpperTee,            'v');
        graphics.insert(BoxDrawingGraphic::HorizontalLine,      'q');
        graphics.insert(BoxDrawingGraphic::UpperHorizontalLine, 'q');
        graphics.insert(BoxDrawingGraphic::LowerHorizontalLine, 'q');
        graphics.insert(BoxDrawingGraphic::VerticalLine,        'x');
        graphics.insert(BoxDrawingGraphic::LeftVerticalLine,    'x');
        graphics.insert(BoxDrawingGraphic::RightVerticalLine,   'x');
        graphics.insert(BoxDrawingGraphic::Plus,                'g');

        graphics
    };

    static ref WIDEBOXDRAWING: HashMap<MatrixKey, u32> = {
        let mut graphics = HashMap::new();

        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::UpperLeftCorner     ), 0x002b); // #
        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::LowerLeftCorner     ), 0x002b); // #
        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::UpperRightCorner    ), 0x002b); // #
        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::LowerRightCorner    ), 0x002b); // #
        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::RightTee            ), 0x002b); // #
        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::LeftTee             ), 0x002b); // #
        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::LowerTee            ), 0x002b); // #
        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::UpperTee            ), 0x002b); // #
        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::HorizontalLine      ), 0x002d); // -
        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::UpperHorizontalLine ), 0x002d); // -
        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::LowerHorizontalLine ), 0x002d); // -
        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::VerticalLine        ), 0x007c); // |
        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::LeftVerticalLine    ), 0x007c); // |
        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::RightVerticalLine   ), 0x007c); // |
        graphics.insert(MatrixKey::new(BoxDrawingType::Ascii,                                      BoxDrawingGraphic::Plus                ), 0x002b); // +

        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::UpperLeftCorner     ), 0x250c); // ┌
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::LowerLeftCorner     ), 0x2514); // └
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::UpperRightCorner    ), 0x2510); // ┐
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::LowerRightCorner    ), 0x2518); // ┘
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::RightTee            ), 0x2524); // ┤
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::LeftTee             ), 0x251c); // ├
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::LowerTee            ), 0x2534); // ┴
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::UpperTee            ), 0x252c); // ┬
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::HorizontalLine      ), 0x2500); // ─
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::UpperHorizontalLine ), 0x2500); // ─
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::LowerHorizontalLine ), 0x2500); // ─
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::VerticalLine        ), 0x2502); // │
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::LeftVerticalLine    ), 0x2502); // │
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::RightVerticalLine   ), 0x2502); // │
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::Plus                ), 0x253c); // ┼

        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::UpperLeftCorner     ), 0x2577); // ╷
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LowerLeftCorner     ), 0x2576); // ╶
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::UpperRightCorner    ), 0x2574); // ╴
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LowerRightCorner    ), 0x2575); // ╵
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::RightTee            ), 0x2518); // ┘
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LeftTee             ), 0x2514); // └
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LowerTee            ), 0x2514); // └
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::UpperTee            ), 0x2510); // ┐
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::HorizontalLine      ), 0x2574); // ╴
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::UpperHorizontalLine ), 0x2574); // ╴
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LowerHorizontalLine ), 0x2576); // ╶
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::VerticalLine        ), 0x2577); // ╷
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LeftVerticalLine    ), 0x2577); // ╷
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::RightVerticalLine   ), 0x2575); // ╵
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::Plus                ), 0x2518); // ┘

        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::UpperLeftCorner     ), 0x2575); // ╵
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LowerLeftCorner     ), 0x2574); // ╴
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::UpperRightCorner    ), 0x2576); // ╶
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LowerRightCorner    ), 0x2577); // ╷
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::RightTee            ), 0x2510); // ┐
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LeftTee             ), 0x2514); // └
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LowerTee            ), 0x2518); // ┘
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::UpperTee            ), 0x250c); // ┌
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::HorizontalLine      ), 0x2576); // ╶
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::UpperHorizontalLine ), 0x2576); // ╶
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LowerHorizontalLine ), 0x2574); // ╴
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::VerticalLine        ), 0x2575); // ╵
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LeftVerticalLine    ), 0x2575); // ╵
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::RightVerticalLine   ), 0x2577); // ╷
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::Plus                ), 0x2514); // └

        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::UpperLeftCorner     ), 0x250c); // ┌
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::LowerLeftCorner     ), 0x2514); // └
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::UpperRightCorner    ), 0x2510); // ┐
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::LowerRightCorner    ), 0x2518); // ┘
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::RightTee            ), 0x2524); // ┤
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::LeftTee             ), 0x251c); // ├
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::LowerTee            ), 0x2534); // ┴
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::UpperTee            ), 0x252c); // ┬
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::HorizontalLine      ), 0x254c); // ╌
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::UpperHorizontalLine ), 0x254c); // ╌
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::LowerHorizontalLine ), 0x254c); // ╌
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::VerticalLine        ), 0x254e); // ╎
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::LeftVerticalLine    ), 0x254e); // ╎
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::RightVerticalLine   ), 0x254e); // ╎
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::Plus                ), 0x253c); // ┼

        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::UpperLeftCorner     ), 0x250c); // ┌
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::LowerLeftCorner     ), 0x2514); // └
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::UpperRightCorner    ), 0x2510); // ┐
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::LowerRightCorner    ), 0x2518); // ┘
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::RightTee            ), 0x2524); // ┤
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::LeftTee             ), 0x251c); // ├
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::LowerTee            ), 0x2534); // ┴
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::UpperTee            ), 0x252c); // ┬
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::HorizontalLine      ), 0x2504); // ┄
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::UpperHorizontalLine ), 0x2504); // ┄
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::LowerHorizontalLine ), 0x2504); // ┄
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::VerticalLine        ), 0x2506); // ┆
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::LeftVerticalLine    ), 0x2506); // ┆
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::RightVerticalLine   ), 0x2506); // ┆
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::Plus                ), 0x253c); // ┼

        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::UpperLeftCorner     ), 0x250c); // ┌
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::LowerLeftCorner     ), 0x2514); // └
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::UpperRightCorner    ), 0x2510); // ┐
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::LowerRightCorner    ), 0x2518); // ┘
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::RightTee            ), 0x2524); // ┤
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::LeftTee             ), 0x251c); // ├
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::LowerTee            ), 0x2534); // ┴
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::UpperTee            ), 0x252c); // ┬
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::HorizontalLine      ), 0x2508); // ┈
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::UpperHorizontalLine ), 0x2508); // ┈
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::LowerHorizontalLine ), 0x2508); // ┈
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::VerticalLine        ), 0x250a); // ┊
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::LeftVerticalLine    ), 0x250a); // ┊
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::RightVerticalLine   ), 0x250a); // ┊
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::Plus                ), 0x253c); // ┼

        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::UpperLeftCorner     ), 0x250f); // ┏
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::LowerLeftCorner     ), 0x2517); // ┗
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::UpperRightCorner    ), 0x2513); // ┓
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::LowerRightCorner    ), 0x251b); // ┛
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::RightTee            ), 0x252b); // ┫
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::LeftTee             ), 0x2523); // ┣
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::LowerTee            ), 0x253b); // ┻
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::UpperTee            ), 0x2533); // ┳
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::HorizontalLine      ), 0x2501); // ━
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::UpperHorizontalLine ), 0x2501); // ━
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::LowerHorizontalLine ), 0x2501); // ━
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::VerticalLine        ), 0x2503); // ┃
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::LeftVerticalLine    ), 0x2503); // ┃
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::RightVerticalLine   ), 0x2503); // ┃
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),        BoxDrawingGraphic::Plus                ), 0x254b); // ╋

        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::UpperLeftCorner     ), 0x257b); // ╻
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LowerLeftCorner     ), 0x257a); // ╺
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::UpperRightCorner    ), 0x2578); // ╸
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LowerRightCorner    ), 0x2579); // ╹
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::RightTee            ), 0x251b); // ┛
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LeftTee             ), 0x2517); // ┗
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LowerTee            ), 0x2517); // ┗
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::UpperTee            ), 0x2513); // ┓
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::HorizontalLine      ), 0x2578); // ╸
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::UpperHorizontalLine ), 0x2578); // ╸
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LowerHorizontalLine ), 0x257a); // ╺
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::VerticalLine        ), 0x257b); // ╻
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LeftVerticalLine    ), 0x257b); // ╻
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::RightVerticalLine   ), 0x2579); // ╹
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::Plus                ), 0x251b); // ┛

        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::UpperLeftCorner     ), 0x2579); // ╹
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LowerLeftCorner     ), 0x2578); // ╸
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::UpperRightCorner    ), 0x257a); // ╺
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LowerRightCorner    ), 0x257b); // ╻
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::RightTee            ), 0x2513); // ┓
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LeftTee             ), 0x2517); // ┗
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LowerTee            ), 0x251b); // ┛
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::UpperTee            ), 0x250f); // ┏
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::HorizontalLine      ), 0x257a); // ╺
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::UpperHorizontalLine ), 0x257a); // ╺
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LowerHorizontalLine ), 0x2578); // ╸
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::VerticalLine        ), 0x2579); // ╹
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LeftVerticalLine    ), 0x2579); // ╹
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::RightVerticalLine   ), 0x257b); // ╻
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::Plus                ), 0x2517); // ┗

        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::UpperLeftCorner     ), 0x250f); // ┏
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::LowerLeftCorner     ), 0x2517); // ┗
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::UpperRightCorner    ), 0x2513); // ┓
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::LowerRightCorner    ), 0x251b); // ┛
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::RightTee            ), 0x252b); // ┫
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::LeftTee             ), 0x2523); // ┣
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::LowerTee            ), 0x253b); // ┻
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::UpperTee            ), 0x2533); // ┳
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::HorizontalLine      ), 0x254d); // ╍
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::UpperHorizontalLine ), 0x254d); // ╍
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::LowerHorizontalLine ), 0x254d); // ╍
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::VerticalLine        ), 0x254f); // ╏
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::LeftVerticalLine    ), 0x254f); // ╏
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::RightVerticalLine   ), 0x254f); // ╏
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),    BoxDrawingGraphic::Plus                ), 0x254b); // ╋

        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::UpperLeftCorner     ), 0x250f); // ┏
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::LowerLeftCorner     ), 0x2517); // ┗
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::UpperRightCorner    ), 0x2513); // ┓
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::LowerRightCorner    ), 0x251b); // ┛
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::RightTee            ), 0x252b); // ┫
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::LeftTee             ), 0x2523); // ┣
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::LowerTee            ), 0x253b); // ┻
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::UpperTee            ), 0x2533); // ┳
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::HorizontalLine      ), 0x2505); // ┅
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::UpperHorizontalLine ), 0x2505); // ┅
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::LowerHorizontalLine ), 0x2505); // ┅
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::VerticalLine        ), 0x2507); // ┇
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::LeftVerticalLine    ), 0x2507); // ┇
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::RightVerticalLine   ), 0x2507); // ┇
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),    BoxDrawingGraphic::Plus                ), 0x254b); // ╋

        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::UpperLeftCorner     ), 0x250f); // ┏
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::LowerLeftCorner     ), 0x2517); // ┗
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::UpperRightCorner    ), 0x2513); // ┓
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::LowerRightCorner    ), 0x251b); // ┛
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::RightTee            ), 0x252b); // ┫
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::LeftTee             ), 0x2523); // ┣
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::LowerTee            ), 0x253b); // ┻
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::UpperTee            ), 0x2533); // ┳
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::HorizontalLine      ), 0x2509); // ┉
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::UpperHorizontalLine ), 0x2509); // ┉
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::LowerHorizontalLine ), 0x2509); // ┉
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::VerticalLine        ), 0x250b); // ┋
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::LeftVerticalLine    ), 0x250b); // ┋
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::RightVerticalLine   ), 0x250b); // ┋
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash), BoxDrawingGraphic::Plus                ), 0x254b); // ╋

        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::UpperLeftCorner     ), 0x2554); // ╔
        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::LowerLeftCorner     ), 0x255a); // ╚
        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::UpperRightCorner    ), 0x2557); // ╗
        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::LowerRightCorner    ), 0x255d); // ╝
        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::RightTee            ), 0x2563); // ╣
        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::LeftTee             ), 0x2560); // ╠
        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::LowerTee            ), 0x2569); // ╩
        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::UpperTee            ), 0x2566); // ╦
        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::HorizontalLine      ), 0x2550); // ═
        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::UpperHorizontalLine ), 0x2550); // ═
        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::LowerHorizontalLine ), 0x2550); // ═
        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::VerticalLine        ), 0x2551); // ║
        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::LeftVerticalLine    ), 0x2551); // ║
        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::RightVerticalLine   ), 0x2551); // ║
        graphics.insert(MatrixKey::new(BoxDrawingType::Double,                                     BoxDrawingGraphic::Plus                ), 0x256c); // ╬

        graphics
    };
}

pub fn chtype_box_graphic(graphic: BoxDrawingGraphic) -> ChtypeChar {
    ChtypeChar::from(NCURSES_ACS(*CHTYPEBOXDRAWING.get(&graphic).unwrap_or_else(|| panic!("chtype_box_graphic() : unable to retrive {:?}", graphic))))
}

pub fn wide_box_graphic(box_drawing_type: BoxDrawingType, graphic: BoxDrawingGraphic) -> WideChar {
    WideChar::from(*WIDEBOXDRAWING.get(&MatrixKey::new(box_drawing_type, graphic)).unwrap_or_else(|| panic!("wide_box_graphic() : unable to retrive {:?} {:?}", box_drawing_type, graphic)) as wchar_t)
}

pub fn complex_box_graphic<A, P, T>(box_drawing_type: BoxDrawingType, graphic: BoxDrawingGraphic, attrs: &A, color_pair: &P) -> result!(ComplexChar) where A: AttributesType<T>, P: ColorPairType<T>, T: ColorAttributeTypes {
    ComplexChar::from_wide_char(wide_box_graphic(box_drawing_type, graphic), attrs, color_pair)
}

pub fn whline_set(window: &Window, box_drawing_type: BoxDrawingType, number: i32) -> result!(()) {
    mvwhline_set(window, window.get_cursor()?, box_drawing_type, number)
}

pub fn mvwhline_set(window: &Window, origin: Origin, box_drawing_type: BoxDrawingType, number: i32) -> result!(()) {
    assert!(number > 0, "mvwhline_set() : number={} > 0", number);

    // build a vector of the complex characters we are going to overwrite
    let complex_chars: Vec<ComplexChar> = ComplexString::into(window.mvin_wchnstr(origin, number)?);
    let mut line_origin = origin;

    // iterate over the vector of complex characters
    for &complex_char in &complex_chars {
        // get the elements of out complex character i.e. wide character, attributes and color pair
        let char_attr_pair = getcchar(complex_char)?;
        // get the unicode value of our complex characters character
        let wchar: u32 = WideChar::into(char_attr_pair.character());

        // define our default graphic character to use
        let mut box_drawing_graphic = BoxDrawingGraphic::HorizontalLine;

        // iterate and filter wide character graphic characters of the specified box drawing type that have the same unicode value
        for (key, _) in WIDEBOXDRAWING.iter().filter(|(k, v)| k.box_drawing_type() == box_drawing_type && **v == wchar) {
            // transform our selected graphic character with our default graphic character
            box_drawing_graphic = box_drawing_graphic.transform(key.box_drawing_graphic(), true);

            // if we've transformed into a plus or upper/lower tee graphic then...
            if box_drawing_graphic == BoxDrawingGraphic::Plus || box_drawing_graphic == BoxDrawingGraphic::UpperTee || box_drawing_graphic == BoxDrawingGraphic::LowerTee {
                // if we are in the left or right edge of the window then change to the appropriate tee or corner character
                box_drawing_graphic = if line_origin.x == 0 {
                    if line_origin.y == 0 {
                        BoxDrawingGraphic::UpperLeftCorner
                    } else if line_origin.y == window.getmaxx()? {
                        BoxDrawingGraphic::LowerLeftCorner
                    } else {
                        BoxDrawingGraphic::LeftTee
                    }
                } else if line_origin.y == window.getmaxy()? {
                    if line_origin.x == 0 {
                        BoxDrawingGraphic::UpperRightCorner
                    } else if line_origin.x == window.getmaxx()? {
                        BoxDrawingGraphic::LowerRightCorner
                    } else {
                        BoxDrawingGraphic::RightTee
                    }
                } else {
                    box_drawing_graphic
                };
            }

            break;
        }

        // place our complex graphics character onto the window with the existing attributes and color pair
        window.mvadd_wch(line_origin, match char_attr_pair.attributes_and_color_pair() {
            AttributesColorPairSet::Normal(s) => complex_box_graphic(box_drawing_type, box_drawing_graphic, &s.attributes(), &s.color_pair())?,
            AttributesColorPairSet::Extend(s) => complex_box_graphic(box_drawing_type, box_drawing_graphic, &s.attributes(), &s.color_pair())?
        })?;

        line_origin.x += 1;

        // check if we've reached the right edge of the window
        if line_origin.x >= window.getmaxx()? {
            break;
        }
    }

    Ok(())
}

pub fn wvline_set(window: &Window, box_drawing_type: BoxDrawingType, number: i32) -> result!(()) {
    mvwvline_set(window, window.get_cursor()?, box_drawing_type, number)
}

pub fn mvwvline_set(window: &Window, origin: Origin, box_drawing_type: BoxDrawingType, number: i32) -> result!(()) {
    assert!(number > 0, "mvwvline_set() : number={} > 0", number);

    let mut complex_chars = vec!();
    let mut line_origin = origin;

    // build a vector of the complex characters we are going to overwrite
    for _ in 0..number {
        complex_chars.push(window.mvin_wch(line_origin)?);
        line_origin.y += 1;
    }

    line_origin = origin;

    // iterate over the vector of complex characters
    for &complex_char in &complex_chars {
        // get the elements of out complex character i.e. wide character, attributes and color pair
        let char_attr_pair = getcchar(complex_char)?;
        // get the unicode value of our complex characters character
        let wchar: u32 = WideChar::into(char_attr_pair.character());

        // define our default graphic character to use
        let mut box_drawing_graphic = BoxDrawingGraphic::VerticalLine;

        // iterate and filter wide character graphic characters of the specified box drawing type that have the same unicode value
        for (key, _) in WIDEBOXDRAWING.iter().filter(|(k, v)| k.box_drawing_type() == box_drawing_type && **v == wchar) {
            // transform our selected graphic character with our default graphic character
            box_drawing_graphic = box_drawing_graphic.transform(key.box_drawing_graphic(), true);

            // if we've transformed into a plus or left/right tee graphic then...
            if box_drawing_graphic == BoxDrawingGraphic::Plus || box_drawing_graphic == BoxDrawingGraphic::LeftTee || box_drawing_graphic == BoxDrawingGraphic::RightTee {
                // if we are in the left or right edge of the window then change to the appropriate tee or corner character
                box_drawing_graphic = if line_origin.x == 0 {
                    if line_origin.y == 0 {
                        BoxDrawingGraphic::UpperLeftCorner
                    } else if line_origin.y == window.getmaxx()? {
                        BoxDrawingGraphic::LowerLeftCorner
                    } else {
                        BoxDrawingGraphic::UpperTee
                    }
                } else if line_origin.y == window.getmaxy()? {
                    if line_origin.x == 0 {
                        BoxDrawingGraphic::UpperRightCorner
                    } else if line_origin.x == window.getmaxx()? {
                        BoxDrawingGraphic::LowerRightCorner
                    } else {
                        BoxDrawingGraphic::LowerTee
                    }
                } else {
                    box_drawing_graphic
                };
            }

            break;
        }

        // place our complex graphics character onto the window with the existing attributes and color pair
        window.mvadd_wch(line_origin, match char_attr_pair.attributes_and_color_pair() {
            AttributesColorPairSet::Normal(s) => complex_box_graphic(box_drawing_type, box_drawing_graphic, &s.attributes(), &s.color_pair())?,
            AttributesColorPairSet::Extend(s) => complex_box_graphic(box_drawing_type, box_drawing_graphic, &s.attributes(), &s.color_pair())?
        })?;

        line_origin.y += 1;

        // check if we've reached the bottom edge of the window
        if line_origin.y >= window.getmaxy()? {
            break;
        }
    }

    Ok(())
}

pub fn wbox_set(window: &Window, size: Size, box_drawing_type: BoxDrawingType) -> result!(()) {
    mvwbox_set(window, window.get_cursor()?, size, box_drawing_type)
}

pub fn mvwbox_set(window: &Window, origin: Origin, size: Size, box_drawing_type: BoxDrawingType) -> result!(()) {
    let get_corner_char = |corner_origin: Origin, graphic: BoxDrawingGraphic| -> result!(ComplexChar) {
        let char_attr_pair = getcchar(window.mvin_wch(corner_origin)?)?;
        let mut box_drawing_graphic = graphic;

        for (key, _) in WIDEBOXDRAWING.iter().filter(|(k, v)| k.box_drawing_type() == box_drawing_type && **v == WideChar::into(char_attr_pair.character())) {
            box_drawing_graphic = box_drawing_graphic.transform(key.box_drawing_graphic(), true);

            break;
        }

        let cchar = match char_attr_pair.attributes_and_color_pair() {
            AttributesColorPairSet::Normal(s) => complex_box_graphic(box_drawing_type, box_drawing_graphic, &s.attributes(), &s.color_pair())?,
            AttributesColorPairSet::Extend(s) => complex_box_graphic(box_drawing_type, box_drawing_graphic, &s.attributes(), &s.color_pair())?
        };

        Ok(cchar)
    };

    let mut corner_origin = origin;
    window.mvadd_wch(corner_origin, get_corner_char(corner_origin, BoxDrawingGraphic::UpperLeftCorner)?)?;

    corner_origin = Origin { y: origin.y, x: origin.x + size.columns };
    window.mvadd_wch(corner_origin, get_corner_char(corner_origin, BoxDrawingGraphic::UpperRightCorner)?)?;

    corner_origin = Origin { y: origin.y + size.lines, x: origin.x };
    window.mvadd_wch(corner_origin, get_corner_char(corner_origin, BoxDrawingGraphic::LowerLeftCorner)?)?;

    corner_origin = Origin { y: origin.y + size.lines, x: origin.x + size.columns };
    window.mvadd_wch(corner_origin, get_corner_char(corner_origin, BoxDrawingGraphic::LowerRightCorner)?)?;

    mvwhline_set(window, Origin { y: origin.y, x: origin.x + 1}, box_drawing_type, size.columns - 1)?;
    mvwhline_set(window, Origin { y: origin.y + size.lines, x: origin.x + 1}, box_drawing_type, size.columns - 1)?;
    mvwvline_set(window, Origin { y: origin.y + 1, x: origin.x }, box_drawing_type, size.lines - 1)?;
    mvwvline_set(window, Origin { y: origin.y + 1, x: origin.x + size.columns}, box_drawing_type, size.lines - 1)?;

    Ok(())
}
