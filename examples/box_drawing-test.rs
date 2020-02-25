/*
    examples/box_drawing-test.rs

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
extern crate rand;
extern crate strum;
extern crate strum_macros;

use std::{time, collections::HashMap};

use gettextrs::*;
use ncurseswwin::{*, normal::*};

use rand::prelude::*;
use strum_macros::{Display, EnumIter};
use strum::IntoEnumIterator;

macro_rules! result { ($type: ty) => { Result<$type, NCurseswWinError> } }

#[derive(Copy, Clone, Display, EnumIter, PartialEq, Eq, Hash)]
enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

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
    ncursesw_entry(|window| {
        cursor_set(CursorType::Invisible)?;
        set_echo(false)?;

        box_drawing_test(window)
    })
}

fn box_drawing_test(stdscr: &Window) -> result!(()) {
    start_color()?;
    use_default_colors()?;

    let light_yellow = Color::new(ColorPalette::LightYellow);
    let dark_blue = Color::new(ColorPalette::Blue);
    let dark_red = Color::new(ColorPalette::Red);
    let dark_green = Color::new(ColorPalette::Green);

    let border_color_pair = ColorPair::new(1, Colors::new(light_yellow, dark_blue))?;
    let display_color_pair = ColorPair::new(2, Colors::new(dark_red, dark_green))?;
    let attrs = Attributes::default();

    let stdscr_size = stdscr.size()?;
    let display_origin = Origin { y: 1, x: 1 };
    let corner_box_size = Size { lines: 10, columns: 10 };

    let custom_box_drawing = BoxDrawing::new(
        WideChar::from(0x25e4),
        WideChar::from(0x25e3),
        WideChar::from(0x25e5),
        WideChar::from(0x25e2),
        wide_box_graphic(BoxDrawingType::default(), BoxDrawingGraphic::RightTee),
        wide_box_graphic(BoxDrawingType::default(), BoxDrawingGraphic::LeftTee),
        wide_box_graphic(BoxDrawingType::default(), BoxDrawingGraphic::LowerTee),
        wide_box_graphic(BoxDrawingType::default(), BoxDrawingGraphic::UpperTee),
        wide_box_graphic(BoxDrawingType::default(), BoxDrawingGraphic::HorizontalLine),
        wide_box_graphic(BoxDrawingType::default(), BoxDrawingGraphic::UpperHorizontalLine),
        wide_box_graphic(BoxDrawingType::default(), BoxDrawingGraphic::LowerHorizontalLine),
        wide_box_graphic(BoxDrawingType::default(), BoxDrawingGraphic::VerticalLine),
        wide_box_graphic(BoxDrawingType::default(), BoxDrawingGraphic::LeftVerticalLine),
        wide_box_graphic(BoxDrawingType::default(), BoxDrawingGraphic::RightVerticalLine),
        wide_box_graphic(BoxDrawingType::default(), BoxDrawingGraphic::Plus)
    );

    // define our corner box origins.
    let corner_origins = {
        let mut corner_origins: HashMap<Corner, Origin> = HashMap::new();

        corner_origins.insert(Corner::TopLeft, Origin { y: 0, x: 0 });
        corner_origins.insert(Corner::TopRight, Origin { y: 0, x: stdscr_size.columns - corner_box_size.columns });
        corner_origins.insert(Corner::BottomLeft, Origin { y: stdscr_size.lines - corner_box_size.lines, x: 0 });
        corner_origins.insert(Corner::BottomRight, Origin { y: stdscr_size.lines - corner_box_size.lines, x: stdscr_size.columns - corner_box_size.columns });

        corner_origins
    };

    // define all the default box drawing types.
    let box_drawing_types: [BoxDrawingType; 15] = [BoxDrawingType::Ascii,
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
                                                   BoxDrawingType::Double,
                                                   BoxDrawingType::Custom(custom_box_drawing)];

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

        // create a border on the stdscr.
        stdscr.border_set(left_side, right_side, top_side, bottom_side, upper_left, upper_right, lower_left, lower_right)?;

        // iterate over the box corners and draw a box at the origin.
        for corner in Corner::iter() {
            stdscr.mvtbox_set(
                *(corner_origins.get(&corner).unwrap_or_else(|| panic!("unable to retrive corner {} graphic!", corner))),
                corner_box_size,
                box_drawing_type
            )?;
        }

        // generate 20 random sized box's and add them with a random origin.
        for _ in 0..20 {
            let box_size = Size {
                lines:   rng.gen_range(2, stdscr_size.lines - 2),
                columns: rng.gen_range(2, stdscr_size.columns - 2)
            };

            let box_origin = Origin {
                y: rng.gen_range(0, stdscr_size.lines - box_size.lines),
                x: rng.gen_range(0, stdscr_size.columns - box_size.columns)
            };

            stdscr.mvtbox_set(box_origin, box_size, box_drawing_type)?;
        }

        // add the type of box drawing type on the stdscr.
        let display_str = if let BoxDrawingType::Custom(_) = box_drawing_type {
            "box drawing type Custom()".to_string()
        } else {
            format!("box drawing type {:?}", box_drawing_type)
        };

        stdscr.mvadd_wchstr(display_origin, &ComplexString::from_str(&display_str, &attrs, &display_color_pair)?)?;

        // press 'q' or 'Q' to quit, any other key to continue or wait for 5 seconds,
        // if a resize event happens then error this back up the call chain.
        // (to achive the same thing automatically without having to code
        //  for KeyBinding::ResizeEvent have the ncursesw.key_resize_as_error
        //  feature enabled and this will bubble up through the Err on the
        //  initial match).
        match stdscr.getch_nonblocking(Some(time::Duration::new(5, 0)))? {
            #[cfg(feature = "key_resize_as_error")]
            Some(char_result) => if let CharacterResult::Character(character) = char_result {
                if character.to_ascii_lowercase() == 'q' {
                    break;
                }
            },
            #[cfg(not(feature = "key_resize_as_error"))]
            Some(char_result) => match char_result {
                CharacterResult::Key(key_binding)     => if key_binding == KeyBinding::ResizeEvent {
                    return Err(NCurseswWinError::NCurseswError { source: NCurseswError::KeyResize });
                },
                CharacterResult::Character(character) => if character.to_ascii_lowercase() == 'q' {
                    break;
                }
            },
            None              => () // Timeout
        }

        // clear the stdscr
        stdscr.clear()?;
    }

    Ok(())
}
