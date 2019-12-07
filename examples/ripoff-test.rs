/*
    examples/ripoff_line-test.rs

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
    if let Err(source) = main_routine() { match source {
        NCurseswWinError::Panic { message } => println!("panic: {}", message),
        _                                   => println!("error: {}", source)
    }}
}

fn main_routine() -> result!(()) {
    // ripoff a line from the top of the screen.
    let top_ripoff = RipoffLine::new(Orientation::Top)?;
    // ripoff a line from the bottom of the screen.
    let bottom_ripoff = RipoffLine::new(Orientation::Bottom)?;

    assert!(top_ripoff != bottom_ripoff);

    // We wrap all our use of ncurseswin with this function.
    ncursesw_entry(|window| {
        cursor_set(CursorType::Invisible)?;
        set_echo(false)?;

        ripoff_line_test(&window, &top_ripoff, &bottom_ripoff)
    })
}

fn ripoff_line_test(stdscr: &Window, top_ripoff: &RipoffLine, bottom_ripoff: &RipoffLine) -> result!(()) {
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

    let stdscr_size = stdscr.size()?;

    let line1 = "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.";
    let line2 = "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.";

    let mut origin = Origin { y: (stdscr_size.lines / 2) - 1, x: (stdscr_size.columns / 2) - (line1.len() as u16 / 2) + 1};

    stdscr.mvaddstr(origin, line1)?;
    origin.y += 1;
    origin.x = (stdscr_size.columns / 2) - (line2.len() as u16 / 2) + 1;
    stdscr.mvaddstr(origin, line2)?;

    //  update the top ripoff line.
    top_ripoff.update(|ripoff_window, columns| -> result!(()) {
        update_top_ripoff(ripoff_window, columns)
    })?;

    //  update the bottom ripoff line.
    bottom_ripoff.update(|ripoff_window, columns| -> result!(()) {
        update_bottom_ripoff(ripoff_window, columns)
    })?;

    doupdate()?;

    stdscr.getch()?;

    Ok(())
}

fn update_top_ripoff(ripoff_window: &RipoffWindow, columns: u16) -> result!(()) {
    let ripoff_message = &format!("this is the ripoff line at the top of the screen with a maximum of {} columns", columns);

    ripoff_window.set_column((columns / 2) - (ripoff_message.len() as u16 / 2))?;

    ripoff_window.addstr(ripoff_message)?;
    ripoff_window.noutrefresh()?;

    Ok(())
}

fn update_bottom_ripoff(ripoff_window: &RipoffWindow, columns: u16) -> result!(()) {
    let ripoff_message = &format!("this is the ripoff line at the bottom of the screen with a maximum of {} columns", columns);

    ripoff_window.set_column((columns / 2) - (ripoff_message.len() as u16 / 2))?;

    ripoff_window.addstr(ripoff_message)?;
    ripoff_window.noutrefresh()?;

    Ok(())
}
