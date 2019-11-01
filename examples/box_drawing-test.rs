/*
    examples/box_drawing-test.rs

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
extern crate rand;
extern crate strum;
extern crate strum_macros;

use std::time;
use std::collections::HashMap;

use ncurseswwin::*;
use ncurseswwin::normal::*;

use rand::prelude::*;
use strum_macros::{Display, EnumIter};
use strum::IntoEnumIterator;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswWinError> } }

#[derive(Copy, Clone, Display, EnumIter, PartialEq, Eq, Hash)]
enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

fn main() {
    match main_routine() {
        Err(e) => {
            match e {
                NCurseswWinError::Panic { message } => eprintln!("panic: {}", message),
                error                               => eprintln!("error: {}", error)
            }
        },
        _      => ()
    }
}

fn main_routine() -> result!(()) {
    setlocale(LcCategory::All, "")?;

    ncursesw_init(|window| { box_drawing_test(&window) })?
}

fn box_drawing_test(window: &Window) -> result!(()) {
    curs_set(CursorType::Invisible)?;
    set_echo(false)?;

    start_color()?;
    use_default_colors()?;

    let light_yellow = Color::Light(BaseColor::Yellow);
    let dark_blue = Color::Dark(BaseColor::Blue);
    let dark_red = Color::Dark(BaseColor::Red);
    let dark_green = Color::Dark(BaseColor::Green);

    let border_color_pair = ColorPair::new(1, Colors::new(light_yellow, dark_blue))?;
    let display_color_pair = ColorPair::new(2, Colors::new(dark_red, dark_green))?;
    let attrs = Attributes::default();

    let window_size = window.size()?;
    let display_origin = Origin { y: 3, x: 40 };
    let corner_box_size = Size { lines: 10, columns: 10 };

    // define our corner box origins.
    let corner_origins = {
        let mut corner_origins: HashMap<Corner, Origin> = HashMap::new();

        corner_origins.insert(Corner::TopLeft, Origin { y: 0, x: 0 });
        corner_origins.insert(Corner::TopRight, Origin { y: 0, x: window_size.columns - corner_box_size.columns });
        corner_origins.insert(Corner::BottomLeft, Origin { y: window_size.lines - corner_box_size.lines, x: 0 });
        corner_origins.insert(Corner::BottomRight, Origin { y: window_size.lines - corner_box_size.lines, x: window_size.columns - corner_box_size.columns });

        corner_origins
    };

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

    // make a handle to the thread-local generator.
    let mut rng = thread_rng();

    // iterate over the box drawing types.
    for &box_drawing_type in &box_drawing_types {
        let left_side   = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LeftVerticalLine, &attrs, &border_color_pair)?;
        let right_side  = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::RightVerticalLine, &attrs, &border_color_pair)?;
        let top_side    = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperHorizontalLine, &attrs, &border_color_pair)?;
        let bottom_side = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerHorizontalLine, &attrs, &border_color_pair)?;
        let upper_left  = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperLeftCorner, &attrs, &border_color_pair)?;
        let upper_right = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperRightCorner, &attrs, &border_color_pair)?;
        let lower_left  = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerLeftCorner, &attrs, &border_color_pair)?;
        let lower_right = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerRightCorner, &attrs, &border_color_pair)?;

        // create a border on the window.
        window.border_set(left_side, right_side, top_side, bottom_side, upper_left, upper_right, lower_left, lower_right)?;

        // iterate over the box corners and draw a box at the origin.
        for corner in Corner::iter() {
            window.mvtbox_set(
                *(corner_origins.get(&corner).unwrap_or_else(|| panic!("unable to retrive corner {} graphic!", corner))),
                corner_box_size,
                box_drawing_type
            )?;
        }

        // generate 20 random sized box's and add them with a random origin.
        for _ in 0..20 {
            let box_size = Size {
                lines:   rng.gen_range(10, window_size.lines - 10),
                columns: rng.gen_range(10, window_size.columns - 10)
            };

            let box_origin = Origin {
                y: rng.gen_range(0, window_size.lines - box_size.lines),
                x: rng.gen_range(0, window_size.columns - box_size.columns)
            };

            window.mvtbox_set(box_origin, box_size, box_drawing_type)?;
        }

        // add the type of box drawing type on the window.
        let display_str = &format!("box drawing type {:?}", box_drawing_type);
        window.mvadd_wchstr(display_origin, &ComplexString::from_str(display_str, &attrs, &display_color_pair)?)?;

        // press 'q' or 'Q' to quit, any other key to continue or wait for 5 seconds,
        // if a resize event happens then error this back up the call chain.
        // (to achive the same thing automatically without having to code
        //  for KeyBinding::ResizeEvent have the ncursesw.key_resize_as_error
        //  feature enabled and this will bubble up through the Err on the
        //  initial match).
        match window.getch_nonblocking(Some(time::Duration::new(5, 0))) {
            Err(err)  => return Err(err),
            #[cfg(feature = "key_resize_as_error")]
            Ok(value) => {
                if let Some(CharacterResult::Character(ch)) = value {
                    if ch == 'q' || ch == 'Q' {
                        break;
                    }
                }
            }
            #[cfg(not(feature = "key_resize_as_error"))]
            Ok(value) => {
                if let Some(char_result) = value {
                    match char_result {
                        CharacterResult::Key(key)      => if key == KeyBinding::ResizeEvent {
                            return Err(NCurseswWinError::NCurseswError { source: NCurseswError::KeyResize });
                        },
                        CharacterResult::Character(ch) => if ch == 'q' || ch == 'Q' {
                            break;
                        }
                    }
                }
            }
        }

        // clear the window
        window.clear()?;
    }

    Ok(())
}
