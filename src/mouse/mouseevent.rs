/*
    src/mouse/mouseevent.rs

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

use std::convert::TryInto;

use ncursesw::mouse::{
    BUTTON1_RELEASED, BUTTON1_PRESSED, BUTTON1_CLICKED, BUTTON1_DOUBLE_CLICKED, BUTTON1_TRIPLE_CLICKED,
    BUTTON2_RELEASED, BUTTON2_PRESSED, BUTTON2_CLICKED, BUTTON2_DOUBLE_CLICKED, BUTTON2_TRIPLE_CLICKED, 
    BUTTON3_RELEASED, BUTTON3_PRESSED, BUTTON3_CLICKED, BUTTON3_DOUBLE_CLICKED, BUTTON3_TRIPLE_CLICKED, 
    BUTTON4_RELEASED, BUTTON4_PRESSED, BUTTON4_CLICKED, BUTTON4_DOUBLE_CLICKED, BUTTON4_TRIPLE_CLICKED, 
    BUTTON5_RELEASED, BUTTON5_PRESSED, BUTTON5_CLICKED, BUTTON5_DOUBLE_CLICKED, BUTTON5_TRIPLE_CLICKED, 
    BUTTON_CTRL, BUTTON_SHIFT, BUTTON_ALT, mmask_t
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(in crate::mouse) enum MouseEvent {
    Button1Released,
    Button1Pressed,
    Button1Clicked,
    Button1DoubleClicked,
    Button1TripleClicked,
    Button2Released,
    Button2Pressed,
    Button2Clicked,
    Button2DoubleClicked,
    Button2TripleClicked,
    Button3Released,
    Button3Pressed,
    Button3Clicked,
    Button3DoubleClicked,
    Button3TripleClicked,
    Button4Released,
    Button4Pressed,
    Button4Clicked,
    Button4DoubleClicked,
    Button4TripleClicked,
    Button5Released,
    Button5Pressed,
    Button5Clicked,
    Button5DoubleClicked,
    Button5TripleClicked,
    ButtonCtrl,
    ButtonShift,
    ButtonAlt
}

impl Into<mmask_t> for MouseEvent {
    fn into(self) -> mmask_t {
        match self {
            MouseEvent::Button1Released      => BUTTON1_RELEASED,
            MouseEvent::Button1Pressed       => BUTTON1_PRESSED,
            MouseEvent::Button1Clicked       => BUTTON1_CLICKED,
            MouseEvent::Button1DoubleClicked => BUTTON1_DOUBLE_CLICKED,
            MouseEvent::Button1TripleClicked => BUTTON1_TRIPLE_CLICKED,
            MouseEvent::Button2Released      => BUTTON2_RELEASED,
            MouseEvent::Button2Pressed       => BUTTON2_PRESSED,
            MouseEvent::Button2Clicked       => BUTTON2_CLICKED,
            MouseEvent::Button2DoubleClicked => BUTTON2_DOUBLE_CLICKED,
            MouseEvent::Button2TripleClicked => BUTTON2_TRIPLE_CLICKED, 
            MouseEvent::Button3Released      => BUTTON3_RELEASED,
            MouseEvent::Button3Pressed       => BUTTON3_PRESSED,
            MouseEvent::Button3Clicked       => BUTTON3_CLICKED,
            MouseEvent::Button3DoubleClicked => BUTTON3_DOUBLE_CLICKED,
            MouseEvent::Button3TripleClicked => BUTTON3_TRIPLE_CLICKED, 
            MouseEvent::Button4Released      => BUTTON4_RELEASED,
            MouseEvent::Button4Pressed       => BUTTON4_PRESSED,
            MouseEvent::Button4Clicked       => BUTTON4_CLICKED,
            MouseEvent::Button4DoubleClicked => BUTTON4_DOUBLE_CLICKED,
            MouseEvent::Button4TripleClicked => BUTTON4_TRIPLE_CLICKED, 
            MouseEvent::Button5Released      => BUTTON5_RELEASED,
            MouseEvent::Button5Pressed       => BUTTON5_PRESSED,
            MouseEvent::Button5Clicked       => BUTTON5_CLICKED,
            MouseEvent::Button5DoubleClicked => BUTTON5_DOUBLE_CLICKED,
            MouseEvent::Button5TripleClicked => BUTTON5_TRIPLE_CLICKED, 
            MouseEvent::ButtonCtrl           => BUTTON_CTRL,
            MouseEvent::ButtonShift          => BUTTON_SHIFT,
            MouseEvent::ButtonAlt            => BUTTON_ALT
        }.try_into().unwrap()
    }
}
