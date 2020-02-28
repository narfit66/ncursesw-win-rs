/*
    examples/window-test.rs

    Copyright (c) 2020 Stephen Whittle  All rights reserved.

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

extern crate ncurseswwin;

use std::convert::TryFrom;

use ncurseswwin::*;

macro_rules! result { ($type: ty) => { Result<$type, NCurseswWinError> } }

fn main() {
    if let Err(source) = main_routine() {
        match source {
            NCurseswWinError::Panic { message } => eprintln!("panic: {}", message),
            _                                   => eprintln!("error: {}", source)
        }
    }
}

fn main_routine() -> result!(()) {
    // We wrap all our use of ncurseswin with this function.
    ncursesw_entry(|_| {
        set_input_mode(InputMode::Character)?;
        set_echo(false)?;
        set_newline(false)?;
        intrflush(false)?;

        cursor_set(CursorType::Invisible)?;

        // call our test routine.
        window_test()
    })
}

fn window_test() -> result!(()) {
    // create a window (size default and origin default provided us with
    // the maximum size of the terminal/display).
    let window = &Window::new(Size::default(), Origin::default())?;

    window.keypad(true)?;

    // extract the box drawing characters for the box drawing type.
    let left_side   = chtype_box_graphic(BoxDrawingGraphic::LeftVerticalLine);
    let right_side  = chtype_box_graphic(BoxDrawingGraphic::RightVerticalLine);
    let top_side    = chtype_box_graphic(BoxDrawingGraphic::UpperHorizontalLine);
    let bottom_side = chtype_box_graphic(BoxDrawingGraphic::LowerHorizontalLine);
    let upper_left  = chtype_box_graphic(BoxDrawingGraphic::UpperLeftCorner);
    let upper_right = chtype_box_graphic(BoxDrawingGraphic::UpperRightCorner);
    let lower_left  = chtype_box_graphic(BoxDrawingGraphic::LowerLeftCorner);
    let lower_right = chtype_box_graphic(BoxDrawingGraphic::LowerRightCorner);

    // create a border on the window.
    window.border(left_side, right_side, top_side, bottom_side, upper_left, upper_right, lower_left, lower_right)?;

    // the text we are going to output.
    let line1 = "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.";
    let line2 = "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.";
    let line3 = "Press any key to exit";

    let window_size = window.size()?;

    // calculate the initial origin for line 1.
    let mut origin = Origin { y: (window_size.lines / 2) - 2, x: calc_x_axis(line1, window_size.columns)? };

    // output our lines centered on the x-axis.
    window.mvaddstr(origin, line1)?;
    origin.y += 1;
    origin.x = calc_x_axis(line2, window_size.columns)?;
    window.mvaddstr(origin, line2)?;
    origin.y += 2;
    origin.x = calc_x_axis(line3, window_size.columns)?;
    window.mvaddstr(origin, line3)?;

    // wait for the user to press a key.
    window.getch()?;

    Ok(())
}

// calculate a centered x-axis based on the length of the string we are outputing.
fn calc_x_axis(line: &str, columns: u16) -> result!(u16) {
    Ok((columns / 2) - (u16::try_from(line.len())? / 2))
}
