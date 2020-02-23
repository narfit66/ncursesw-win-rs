/*
    examples/screen-test.rs

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

use std::{env,io};

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
    // We wrap our use of ncurseswin with this function.
    safe_entry(|| {
        // create a screen using stdout and stdin for output and input.
        let screen = &Screen::new(Some(&env::var("TERM")?), io::stdout(), io::stdin())?;

        // make the screens cursor invisible.
        screen.cursor_set(CursorType::Invisible)?;
        // switch echoing off.
        screen.set_echo(false)?;

        screen_test(screen)
    })
}

fn screen_test(screen: &Screen) -> result!(()) {
    // create a window on our screen.
    let window = &Window::new_sp(screen, Size { columns: 0, lines: 0 }, Origin { y: 0, x: 0 })?;

    // extract the box drawing characters for the box drawing type.
    let left_side   = chtype_box_graphic(BoxDrawingGraphic::LeftVerticalLine);
    let right_side  = chtype_box_graphic(BoxDrawingGraphic::RightVerticalLine);
    let top_side    = chtype_box_graphic(BoxDrawingGraphic::UpperHorizontalLine);
    let bottom_side = chtype_box_graphic(BoxDrawingGraphic::LowerHorizontalLine);
    let upper_left  = chtype_box_graphic(BoxDrawingGraphic::UpperLeftCorner);
    let upper_right = chtype_box_graphic(BoxDrawingGraphic::UpperRightCorner);
    let lower_left  = chtype_box_graphic(BoxDrawingGraphic::LowerLeftCorner);
    let lower_right = chtype_box_graphic(BoxDrawingGraphic::LowerRightCorner);

    // create a border on the inital window.
    window.border(left_side, right_side, top_side, bottom_side, upper_left, upper_right, lower_left, lower_right)?;

    let window_size = window.size()?;

    let line1 = "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.";
    let line2 = "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.";

    let mut origin = Origin { y: (window_size.lines / 2) - 1, x: (window_size.columns / 2) - (line1.len() as u16 / 2) + 1};

    window.mvaddstr(origin, line1)?;
    origin.y += 1;
    origin.x = (window_size.columns / 2) - (line2.len() as u16 / 2) + 1;
    window.mvaddstr(origin, line2)?;

    window.getch()?;

    Ok(())
}