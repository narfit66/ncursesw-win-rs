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

use std::{convert::TryFrom, env, io, process::exit};
use anyhow::{Result, Context};
use ncurseswwin::*;

fn main() {
    if let Err(source) = main_routine() {
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

fn main_routine() -> Result<()> {
    // We wrap our use of ncurseswin with this function.
    safe_entry(|| {
        let term = &env::var("TERM").with_context(|| "$TERM is invalid!!!")?;

        // create a screen using stdout and stdin for output and input.
        let screen = &Screen::new(Some(term), &io::stdout().lock(), &io::stdin().lock())?;

        assert!(&screen.termname()? == term);

        screen.set_input_mode(InputMode::Character)?;
        screen.set_echo(false)?;
        screen.set_newline(false)?;
        screen.intrflush(false)?;

        // make the screens cursor invisible.
        screen.cursor_set(CursorType::Invisible)?;

        screen_test(screen)
    })
}

fn screen_test(screen: &Screen) -> Result<()> {
    // create a window on our screen.
    let window = &Window::new_sp(screen, Size::default(), Origin::default())?;

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

    // the text we are going to output.
    let line1 = "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.";
    let line2 = "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.";
    let line3 = "Press any key to exit";

    // get the window's size.
    let window_size = window.size()?;

    // calculate the initial origin for line 1.
    let mut origin = Origin { y: (window_size.lines / 2) - 2, x: calc_x_axis(line1, window_size)? };

    // output our lines centered on the x-axis.
    window.mvaddstr(origin, line1)?;
    origin.y += 1;
    origin.x = calc_x_axis(line2, window_size)?;
    window.mvaddstr(origin, line2)?;
    origin.y += 2;
    origin.x = calc_x_axis(line3, window_size)?;
    window.mvaddstr(origin, line3)?;

    // wait for the user to press a key.
    window.getch()?;

    Ok(())
}

fn calc_x_axis(line: &str, window_size: Size) -> Result<u16> {
    Ok((window_size.columns / 2) - (u16::try_from(line.len())? / 2))
}
