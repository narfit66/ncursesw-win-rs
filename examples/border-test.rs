/*
    examples/border-test.rs

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

extern crate ncurseswwin;

use std::process::exit;
use anyhow::Result;
use ncurseswwin::*;

fn main() {
    // initialize ncurses in a safe way.
    if let Err(source) = ncursesw_entry(|stdscr| {
        set_input_mode(InputMode::Character)?;
        set_echo(false)?;
        set_newline(false)?;
        intrflush(false)?;

        // set the cursor to invisible and switch echoing off.
        cursor_set(CursorType::Invisible)?;

        border_test(stdscr)
    }) {
        if let Some(err) = source.downcast_ref::<NCurseswWinError>() {
            match err {
                NCurseswWinError::Panic { message } => eprintln!("panic: {}", message),
                _                                   => eprintln!("error: {}", err)
            }
        } else {
            eprintln!("error: {}", source);
        }

        source.chain().skip(1).for_each(|cause| eprintln!("cause: {}", cause));

        exit(1);
    }

    exit(0);
}

fn border_test(stdscr: &Window) -> Result<()> {
    // extract the box drawing characters for the box drawing type.
    let left_side   = chtype_box_graphic(BoxDrawingGraphic::LeftVerticalLine);
    let right_side  = chtype_box_graphic(BoxDrawingGraphic::RightVerticalLine);
    let top_side    = chtype_box_graphic(BoxDrawingGraphic::UpperHorizontalLine);
    let bottom_side = chtype_box_graphic(BoxDrawingGraphic::LowerHorizontalLine);
    let upper_left  = chtype_box_graphic(BoxDrawingGraphic::UpperLeftCorner);
    let upper_right = chtype_box_graphic(BoxDrawingGraphic::UpperRightCorner);
    let lower_left  = chtype_box_graphic(BoxDrawingGraphic::LowerLeftCorner);
    let lower_right = chtype_box_graphic(BoxDrawingGraphic::LowerRightCorner);

    // create a border on the inital window (stdscr).
    stdscr.border(left_side, right_side, top_side, bottom_side, upper_left, upper_right, lower_left, lower_right)?;

    // add some default text to the inner window.
    let mut origin = Origin { y: 1, x: 2 };

    stdscr.mvaddstr(origin, "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.")?;
    origin.y += 1;
    stdscr.mvaddstr(origin, "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.")?;

    // refresh our window.
    stdscr.refresh()?;

    // wait for user input (or an event).
    stdscr.getch()?;

    Ok(())
}
