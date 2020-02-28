/*
    examples/border_set-test.rs

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

extern crate gettextrs;
extern crate ncurseswwin;

use std::time;

use gettextrs::*;
use ncurseswwin::{*, extend::*};

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
    setlocale(LocaleCategory::LcAll, "");

    // initialize ncurses in a safe way.
    ncursesw_entry(|stdscr| {
        set_input_mode(InputMode::Character)?;
        set_echo(false)?;
        set_newline(false)?;
        intrflush(false)?;

        // set the cursor to invisible and switch echoing off.
        cursor_set(CursorType::Invisible)?;

        // start colors and use the default color pair of white on black.
        start_color()?;
        use_default_colors()?;

        border_set_test(stdscr)
    })
}

fn border_set_test(stdscr: &Window) -> result!(()) {
    // define color pair 0 and normal attriburs.
    let color_pair = ColorPair::default();
    let attrs = Attributes::default();

    // get the size of the initial window (stdscr).
    let initial_size = stdscr.size()?;

    // workout the size of a inner window have a 1 character spacing all the way around.
    let inner_size = Size { lines: initial_size.lines - 2, columns: initial_size.columns - 2 };
    let inner_origin = Origin { y: 1, x: 1 };

    // create our sub window with the inital window.
    let inner_window = stdscr.subwin(inner_size, inner_origin)?;

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
        let left_side   = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LeftVerticalLine, &attrs, &color_pair)?;
        let right_side  = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::RightVerticalLine, &attrs, &color_pair)?;
        let top_side    = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperHorizontalLine, &attrs, &color_pair)?;
        let bottom_side = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerHorizontalLine, &attrs, &color_pair)?;
        let upper_left  = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperLeftCorner, &attrs, &color_pair)?;
        let upper_right = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperRightCorner, &attrs, &color_pair)?;
        let lower_left  = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerLeftCorner, &attrs, &color_pair)?;
        let lower_right = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerRightCorner, &attrs, &color_pair)?;

        // create a border on the inital window (stdscr).
        stdscr.border_set(left_side, right_side, top_side, bottom_side, upper_left, upper_right, lower_left, lower_right)?;

        // set our cursor position and clear to the end of line on our sub window.
        inner_window.set_cursor(origin)?;
        inner_window.clrtoeol()?;

        // create a border on the sub window.
        inner_window.border_set(left_side, right_side, top_side, bottom_side, upper_left, upper_right, lower_left, lower_right)?;

        // add the type of box drawing type on the sub window.
        inner_window.mvaddstr(origin, &format!("box drawing type {:?}", box_drawing_type))?;

        // by refreshing the stdscr we also refresh our inner_window which is a sub window of inital_window
        stdscr.refresh()?;

        // press 'q' or 'Q' to quit, any other key to continue or wait for 5 seconds.
        match inner_window.getch_nonblocking(Some(time::Duration::new(5, 0)))? {
            Some(char_result) => match char_result {
                CharacterResult::Key(key_binding)    => if key_binding == KeyBinding::ResizeEvent {
                    return Err(NCurseswWinError::NCurseswError { source: NCurseswError::KeyResize });
                },
                CharacterResult::Character(character) => if character.to_ascii_lowercase() == 'q' {
                    break;
                }
            },
            None              => () // Timeout.
        }
    }

    Ok(())
}
