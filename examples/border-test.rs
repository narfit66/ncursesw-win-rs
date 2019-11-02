/*
    examples/border-test.rs

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

extern crate ncurseswwin;

use ncurseswwin::*;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswWinError> } }

fn main() {
    if let Err(e) = main_routine() {
        println!("error: {}", e);
    }
}

fn main_routine() -> result!(()) {
    ncursesw_init(|ncurses| {
        // initialize ncurses in a safe way.
        if let Err(e) = border_test(&ncurses.initial_window()) {
            panic!(e.to_string())
        }
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("A Panic Occurred: {}", errmsg),
        None         => println!("There was an error, but no error message."),
    });

    Ok(())
}

fn border_test(initial_window: &Window) -> result!(()) {
    // set the cursor to invisible and switch echoing off.
    curs_set(CursorType::Invisible)?;
    set_echo(false)?;

    // extract the box drawing characters for the box drawing type.
    let ul = chtype_box_graphic(BoxDrawingGraphic::UpperLeftCorner);
    let ll = chtype_box_graphic(BoxDrawingGraphic::LowerLeftCorner);
    let ur = chtype_box_graphic(BoxDrawingGraphic::UpperRightCorner);
    let lr = chtype_box_graphic(BoxDrawingGraphic::LowerRightCorner);
    let hl = chtype_box_graphic(BoxDrawingGraphic::HorizontalLine);
    let vl = chtype_box_graphic(BoxDrawingGraphic::VerticalLine);

    // create a border on the inital window (stdscr).
    initial_window.border(vl, vl, hl, hl, ul, ur, ll, lr)?;

    // add some default text to the inner window.
    let mut origin = Origin { y: 1, x: 2 };

    initial_window.mvaddstr(origin, "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.")?;
    origin.y += 1;
    initial_window.mvaddstr(origin, "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.")?;

    // refresh our window.
    initial_window.refresh()?;

    // wait for user input (or an event).
    initial_window.getch()?;

    Ok(())
}
