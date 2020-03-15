/*
    src/graphics/funcs.rs

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

#![allow(clippy::too_many_arguments)]

use std::collections::HashMap;
use crate::{
    BoxDrawingType, BoxDrawingTypeDetail, BoxDrawingGraphic, NCurseswWinError,
    graphics::MatrixKey
};
use ncursesw::{
    ChtypeChar, WideChar, ComplexChar,
    AttributesType, ColorPairType, ColorAttributeTypes,
    shims::ncurses::{wchar_t, NCURSES_ACS}
};

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

    pub(in crate) static ref WIDEBOXDRAWING: HashMap<MatrixKey, u32> = {
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

        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::UpperLeftCorner     ), 0x250c); // ┌
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LowerLeftCorner     ), 0x2514); // └
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::UpperRightCorner    ), 0x2510); // ┐
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LowerRightCorner    ), 0x2518); // ┘
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
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::Plus                ), 0x253c); // ┼

        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::UpperLeftCorner     ), 0x250c); // ┌
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LowerLeftCorner     ), 0x2514); // └
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::UpperRightCorner    ), 0x2510); // ┐
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LowerRightCorner    ), 0x2518); // ┘
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
        graphics.insert(MatrixKey::new(BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::Plus                ), 0x253c); // ┼
        //

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

        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::UpperLeftCorner     ), 0x250f); // ┏
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LowerLeftCorner     ), 0x2517); // ┗
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::UpperRightCorner    ), 0x2513); // ┓
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::LowerRightCorner    ), 0x251b); // ┛
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
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),      BoxDrawingGraphic::Plus                ), 0x254b); // ╋

        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::UpperLeftCorner     ), 0x250f); // ┏
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LowerLeftCorner     ), 0x2517); // ┗
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::UpperRightCorner    ), 0x2513); // ┓
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::LowerRightCorner    ), 0x251b); // ┛
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
        graphics.insert(MatrixKey::new(BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),     BoxDrawingGraphic::Plus                ), 0x254b); // ╋

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

/// Obtain the box drawing graphic of ChtypeChar type.
pub fn chtype_box_graphic(box_drawing_graphic: BoxDrawingGraphic) -> ChtypeChar {
    // extract the chtype character value represnted by the box drawing
    // graphic from the crate defined values.
    ChtypeChar::from(
        NCURSES_ACS(
            *CHTYPEBOXDRAWING
            .get(&box_drawing_graphic)
            .unwrap_or_else(|| panic!("chtype_box_graphic() : unable to retrive {:?}", box_drawing_graphic))
        )
    )
}

/// Obtain the box drawing graphic of WideChar type.
pub fn wide_box_graphic(
    box_drawing_type:    BoxDrawingType,
    box_drawing_graphic: BoxDrawingGraphic
) -> WideChar {
    // if we have a custom box drawing type then extract the wide character
    // value from the contained `BoxDrawing` struct otherwise extract the
    // wide character value from the crate defined values.
    if let BoxDrawingType::Custom(box_drawing) = box_drawing_type {
        match box_drawing_graphic {
            BoxDrawingGraphic::UpperLeftCorner     => box_drawing.upper_left_corner,
            BoxDrawingGraphic::LowerLeftCorner     => box_drawing.lower_left_corner,
            BoxDrawingGraphic::UpperRightCorner    => box_drawing.upper_right_corner,
            BoxDrawingGraphic::LowerRightCorner    => box_drawing.lower_right_corner,
            BoxDrawingGraphic::RightTee            => box_drawing.right_tee,
            BoxDrawingGraphic::LeftTee             => box_drawing.left_tee,
            BoxDrawingGraphic::LowerTee            => box_drawing.lower_tee,
            BoxDrawingGraphic::UpperTee            => box_drawing.upper_tee,
            BoxDrawingGraphic::HorizontalLine      => box_drawing.horizontal_line,
            BoxDrawingGraphic::UpperHorizontalLine => box_drawing.upper_horizontal_line,
            BoxDrawingGraphic::LowerHorizontalLine => box_drawing.lower_horizontal_line,
            BoxDrawingGraphic::VerticalLine        => box_drawing.vertical_line,
            BoxDrawingGraphic::LeftVerticalLine    => box_drawing.left_vertical_line,
            BoxDrawingGraphic::RightVerticalLine   => box_drawing.right_vertical_line,
            BoxDrawingGraphic::Plus                => box_drawing.plus
        }
    } else {
        WideChar::from(
            *WIDEBOXDRAWING
            .get(&MatrixKey::new(box_drawing_type, box_drawing_graphic))
            .unwrap_or_else(|| panic!("wide_box_graphic() : unable to retrive {:?}, {:?}", box_drawing_type, box_drawing_graphic)) as wchar_t
        )
    }
}

/// Obtain the box drawing graphic of ComplexChar type.
pub fn complex_box_graphic<A, P, T>(
    box_drawing_type:    BoxDrawingType,
    box_drawing_graphic: BoxDrawingGraphic,
    attrs:               &A,
    color_pair:          &P
) -> result!(ComplexChar)
    where A: AttributesType<T>,
          P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    Ok(ComplexChar::from_wide_char(wide_box_graphic(box_drawing_type, box_drawing_graphic), attrs, color_pair)?)
}
