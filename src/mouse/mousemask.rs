/*
    src/mouse/mousemask.rs

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

use ncursesw::mouse::{mmask_t, REPORT_MOUSE_POSITION, ALL_MOUSE_EVENTS};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MouseMask {
    ReportMousePosition,
    AllMouseEvents
}

impl Into<mmask_t> for MouseMask {
    fn into(self) -> mmask_t {
        let mask = match self {
            MouseMask::ReportMousePosition => REPORT_MOUSE_POSITION,
            MouseMask::AllMouseEvents      => ALL_MOUSE_EVENTS
        };

        mask.try_into().unwrap()
    }
}