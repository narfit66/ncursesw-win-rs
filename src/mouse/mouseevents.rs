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

use strum::IntoEnumIterator;

use crate::mouse::mousebuttonevent::MouseButtonEvent;
use crate::mouse::mousebuttonstate::MouseButtonState;
use crate::mouse::mousebutton::MouseButton;
use crate::mouse::mouseevent::MouseEvent;
use ncursesw::mouse::mmask_t;

macro_rules! pub_getter {
    ($name: ident, $attr: ident) => {
        pub fn $name(&self) -> bool {
            let event_mask: mmask_t = MouseEvent::$attr.into();

            (self.mask & event_mask) > 0
        }
    };
}

macro_rules! private_method {
    ($name: ident, $attr: ident) => {
        fn $name(&self) -> bool {
            let event_mask: mmask_t = MouseEvent::$attr.into();

            (self.mask & event_mask) > 0
        }
    };
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct MouseEvents {
    mask: mmask_t
}

impl MouseEvents {
    pub(in crate::mouse) fn new(mask: mmask_t) -> Self {
        Self { mask }
    }

    pub fn button_state(&self) -> Option<MouseButtonState> {
        let _button_state = | button: MouseButton, event: MouseButtonEvent | -> bool {
            match event {
                MouseButtonEvent::Released      => self.released(button),
                MouseButtonEvent::Pressed       => self.pressed(button),
                MouseButtonEvent::Clicked       => self.clicked(button),
                MouseButtonEvent::DoubleClicked => self.double_clicked(button),
                MouseButtonEvent::TripleClicked => self.triple_clicked(button)
            }
        };

        for button in MouseButton::iter() {
            for event in MouseButtonEvent::iter() {
                if _button_state(button, event) {
                    return Some(MouseButtonState::new(button, event))
                }
            }
        }

        None
    }

    pub_getter!(ctrl_button, ButtonCtrl);
    pub_getter!(shift_button, ButtonShift);
    pub_getter!(alt_button, ButtonAlt);

    fn released(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::One   => self.button_1_released(),
            MouseButton::Two   => self.button_2_released(),
            MouseButton::Three => self.button_3_released(),
            MouseButton::Four  => self.button_4_released(),
            MouseButton::Five  => self.button_5_released()
        }
    }

    fn pressed(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::One   => self.button_1_pressed(),
            MouseButton::Two   => self.button_2_pressed(),
            MouseButton::Three => self.button_3_pressed(),
            MouseButton::Four  => self.button_4_pressed(),
            MouseButton::Five  => self.button_5_pressed()
        }
    }

    fn clicked(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::One   => self.button_1_clicked(),
            MouseButton::Two   => self.button_2_clicked(),
            MouseButton::Three => self.button_3_clicked(),
            MouseButton::Four  => self.button_4_clicked(),
            MouseButton::Five  => self.button_5_clicked()
        }
    }

    fn double_clicked(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::One   => self.button_1_double_clicked(),
            MouseButton::Two   => self.button_2_double_clicked(),
            MouseButton::Three => self.button_3_double_clicked(),
            MouseButton::Four  => self.button_4_double_clicked(),
            MouseButton::Five  => self.button_5_double_clicked()
        }
    }

    fn triple_clicked(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::One   => self.button_1_triple_clicked(),
            MouseButton::Two   => self.button_2_triple_clicked(),
            MouseButton::Three => self.button_3_triple_clicked(),
            MouseButton::Four  => self.button_4_triple_clicked(),
            MouseButton::Five  => self.button_5_triple_clicked()
        }
    }

    private_method!(button_1_released, Button1Released);
    private_method!(button_1_pressed, Button1Pressed);
    private_method!(button_1_clicked, Button1Clicked);
    private_method!(button_1_double_clicked, Button1DoubleClicked);
    private_method!(button_1_triple_clicked, Button1TripleClicked);
    private_method!(button_2_released, Button2Released);
    private_method!(button_2_pressed, Button2Pressed);
    private_method!(button_2_clicked, Button2Clicked);
    private_method!(button_2_double_clicked, Button2DoubleClicked);
    private_method!(button_2_triple_clicked, Button2TripleClicked);
    private_method!(button_3_released, Button3Released);
    private_method!(button_3_pressed, Button3Pressed);
    private_method!(button_3_clicked, Button3Clicked);
    private_method!(button_3_double_clicked, Button3DoubleClicked);
    private_method!(button_3_triple_clicked, Button3TripleClicked);
    private_method!(button_4_released, Button4Released);
    private_method!(button_4_pressed, Button4Pressed);
    private_method!(button_4_clicked, Button4Clicked);
    private_method!(button_4_double_clicked, Button4DoubleClicked);
    private_method!(button_4_triple_clicked, Button4TripleClicked);
    private_method!(button_5_released, Button5Released);
    private_method!(button_5_pressed, Button5Pressed);
    private_method!(button_5_clicked, Button5Clicked);
    private_method!(button_5_double_clicked, Button5DoubleClicked);
    private_method!(button_5_triple_clicked, Button5TripleClicked);
}
