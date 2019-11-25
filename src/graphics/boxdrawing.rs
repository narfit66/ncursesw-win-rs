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

use ncursesw::WideChar;

/// Custom box drawing graphics.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BoxDrawing {
    pub upper_left_corner:     WideChar,
    pub lower_left_corner:     WideChar,
    pub upper_right_corner:    WideChar,
    pub lower_right_corner:    WideChar,
    pub right_tee:             WideChar,
    pub left_tee:              WideChar,
    pub lower_tee:             WideChar,
    pub upper_tee:             WideChar,
    pub horizontal_line:       WideChar,
    pub upper_horizontal_line: WideChar,
    pub lower_horizontal_line: WideChar,
    pub vertical_line:         WideChar,
    pub left_vertical_line:    WideChar,
    pub right_vertical_line:   WideChar,
    pub plus:                  WideChar
}

impl BoxDrawing {
    /// Create a new instance of `BoxDrawing`.
    pub fn new(
        upper_left_corner:     WideChar,
        lower_left_corner:     WideChar,
        upper_right_corner:    WideChar,
        lower_right_corner:    WideChar,
        right_tee:             WideChar,
        left_tee:              WideChar,
        lower_tee:             WideChar,
        upper_tee:             WideChar,
        horizontal_line:       WideChar,
        upper_horizontal_line: WideChar,
        lower_horizontal_line: WideChar,
        vertical_line:         WideChar,
        left_vertical_line:    WideChar,
        right_vertical_line:   WideChar,
        plus:                  WideChar
    ) -> Self {
        Self {
            upper_left_corner,
            lower_left_corner,
            upper_right_corner,
            lower_right_corner,
            right_tee,
            left_tee,
            lower_tee,
            upper_tee,
            horizontal_line,
            upper_horizontal_line,
            lower_horizontal_line,
            vertical_line,
            left_vertical_line,
            right_vertical_line,
            plus
        }
    }
}
