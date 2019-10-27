/*
    examples/border_set-test.rs

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
use ncurseswwin::extend::*;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswWinError> } }

fn main() {
    if let Err(e) = main_routine() {
        println!("error: {}", e);
    }
}

fn main_routine() -> result!(()) {
    setlocale(LcCategory::All, "")?;

    ncursesw_init(|ncurses| {
        // initialize ncurses in a safe way.
        if let Err(e) = border_set_test(&ncurses.initial_window()) {
            panic!(e.to_string());
        }
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("A Panic Occurred: {}", errmsg),
        None         => println!("There was an error, but no error message."),
    });

    Ok(())
}

fn border_set_test(initial_window: &Window) -> result!(()) {
    // set the cursor to invisible and switch echoing off.
    curs_set(CursorType::Invisible)?;
    set_echo(false)?;

    // start colors and use the default color pair of white on black.
    start_color()?;
    use_default_colors()?;

    // define color pair 0 and normal attriburs.
    let color_pair0 = ColorPair::default();
    let attrs = Attributes::default();

    // get the size of the initial window (stdscr).
    let initial_size = initial_window.size()?;

    // workout the size of a inner window have a 1 character spacing all the way around.
    let inner_size = Size { lines: initial_size.lines - 2, columns: initial_size.columns - 2 };
    let inner_origin = Origin { y: 1, x: 1 };

    // create our sub window with the inital window.
    let inner_window = initial_window.subwin(inner_size, inner_origin)?;

    // add some default text to the inner window.
    let mut origin = Origin { y: 1, x: 1 };

    inner_window.mvaddstr(origin, "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.")?;
    origin.y += 1;
    inner_window.mvaddstr(origin, "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.")?;

    // define the origin that we want to output our box drawing type.
    origin = Origin { y: origin.y + 2, x: 1 };

    // define all the default box drawing types.
    let box_drawing_types: [BoxDrawingType; 14] = [BoxDrawingType::Ascii,
                                                   BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),
                                                   BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),
                                                   BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),
                                                   BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),
                                                   BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),
                                                   BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash),
                                                   BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),
                                                   BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),
                                                   BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),
                                                   BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),
                                                   BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),
                                                   BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash),
                                                   BoxDrawingType::Double];

    // iterate over the box drawing types.
    for &box_drawing_type in &box_drawing_types {
        // extract the box drawing characters for the box drawing type.
        let ul = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperLeftCorner, &attrs, &color_pair0)?;
        let ll = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerLeftCorner, &attrs, &color_pair0)?;
        let ur = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperRightCorner, &attrs, &color_pair0)?;
        let lr = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerRightCorner, &attrs, &color_pair0)?;
        let hl = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::HorizontalLine, &attrs, &color_pair0)?;
        let vl = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::VerticalLine, &attrs, &color_pair0)?;

        // create a border on the inital window (stdscr).
        initial_window.border_set(vl, vl, hl, hl, ul, ur, ll, lr)?;

        // set our cursor position and clear to the end of line on our sub window.
        inner_window.set_cursor(origin)?;
        inner_window.clrtoeol()?;

        // create a border on the sub window.
        inner_window.border_set(vl, vl, hl, hl, ul, ur, ll, lr)?;

        // add the type of box drawing type on the sub window.
        inner_window.mvaddstr(origin, &format!("box drawing type {:?}", box_drawing_type))?;

        // by refreshing the initial_window we also refresh our inner_window which is a sub window of inital_window
        initial_window.refresh()?;

        // wait for user input (or an event).
        inner_window.getch()?;
    }

    Ok(())
}
