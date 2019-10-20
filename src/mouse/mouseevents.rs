/*
    src/mouse/mouseevents.rs

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

use crate::mouse::mouseevent::MouseEvent;
use ncursesw::mouse::mmask_t;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct MouseEvents {
    raw: mmask_t
}

macro_rules! getter {
    ($name: ident, $attr: ident) => {
        pub fn $name(&self) -> bool {
            let e: mmask_t = MouseEvent::$attr.into();

            (self.raw & e) > 0
        }
    };
}

impl MouseEvents {
    pub (in crate::mouse) fn new(mask: mmask_t) -> Self {
        Self { raw: mask }
    }

    getter!(button_1_released, Button1Released);
    getter!(button_1_pressed, Button1Pressed);
    getter!(button_1_clicked, Button1Clicked);
    getter!(button_1_double_clicked, Button1DoubleClicked);
    getter!(button_1_triple_clicked, Button1TripleClicked);
    getter!(button_2_released, Button2Released);
    getter!(button_2_pressed, Button2Pressed);
    getter!(button_2_clicked, Button2Clicked);
    getter!(button_2_double_clicked, Button2DoubleClicked);
    getter!(button_2_triple_clicked, Button2TripleClicked);
    getter!(button_3_released, Button3Released);
    getter!(button_3_pressed, Button3Pressed);
    getter!(button_3_clicked, Button3Clicked);
    getter!(button_3_double_clicked, Button3DoubleClicked);
    getter!(button_3_triple_clicked, Button3TripleClicked);
    getter!(button_4_released, Button4Released);
    getter!(button_4_pressed, Button4Pressed);
    getter!(button_4_clicked, Button4Clicked);
    getter!(button_4_double_clicked, Button4DoubleClicked);
    getter!(button_4_triple_clicked, Button4TripleClicked);
    getter!(button_5_released, Button5Released);
    getter!(button_5_pressed, Button5Pressed);
    getter!(button_5_clicked, Button5Clicked);
    getter!(button_5_double_clicked, Button5DoubleClicked);
    getter!(button_5_triple_clicked, Button5TripleClicked);
    getter!(button_ctrl, ButtonCtrl);
    getter!(button_shift, ButtonShift);
    getter!(button_alt, ButtonAlt);
}
