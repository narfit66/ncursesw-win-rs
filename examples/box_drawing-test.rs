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

use std::time;

use ncurseswwin::*;
use ncurseswwin::normal::*;

use rand::prelude::*;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswWinError> } }

fn main() {
    if let Err(e) = main_routine() {
        println!("error: {}", e);
    }
}

fn main_routine() -> result!(()) {
    setlocale(LcCategory::All, "")?;

    ncursesw_init(|ncurses| {
        panic!(match box_drawing_test(&ncurses.initial_window()) {
            Err(e) => e.to_string(),
            _      => "this is the end my friend, the only end!!!".to_string()
        })
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("A Panic Occurred: {}", errmsg),
        None         => println!("There was an error, but no error message."),
    });

    Ok(())
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

    // make a handle to the thread-local generator.
    let mut rng = thread_rng();

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
        let ls = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LeftVerticalLine, &attrs, &border_color_pair)?;
        let rs = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::RightVerticalLine, &attrs, &border_color_pair)?;
        let ts = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperHorizontalLine, &attrs, &border_color_pair)?;
        let bs = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerHorizontalLine, &attrs, &border_color_pair)?;
        let ul = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperLeftCorner, &attrs, &border_color_pair)?;
        let ur = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperRightCorner, &attrs, &border_color_pair)?;
        let ll = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerLeftCorner, &attrs, &border_color_pair)?;
        let lr = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerRightCorner, &attrs, &border_color_pair)?;

        // create a border on the inital window (stdscr).
        window.border_set(ls, rs, ts, bs, ul, ur, ll, lr)?;

        // top-left corner box
        window.mvtbox_set(Origin { y: 0, x: 0 }, corner_box_size, box_drawing_type)?;
        // top-right corner box
        window.mvtbox_set(Origin { y: 0, x: window_size.columns - corner_box_size.columns }, corner_box_size, box_drawing_type)?;
        // bottom-left corner box
        window.mvtbox_set(Origin { y: window_size.lines - corner_box_size.lines, x: 0 }, corner_box_size, box_drawing_type)?;
        // bottom-right corner box
        window.mvtbox_set(Origin { y: window_size.lines - corner_box_size.lines, x: window_size.columns - corner_box_size.columns }, corner_box_size, box_drawing_type)?;

        // generate 20 random sized box's and add them with a random origin.
        for _ in 0..20 {
            let lines = rng.gen_range(10, window_size.lines - 10);
            let columns = rng.gen_range(10, window_size.columns - 10);

            let box_size = Size { lines, columns };

            let y = rng.gen_range(0, window_size.lines - box_size.lines);
            let x = rng.gen_range(0, window_size.columns - box_size.columns);

            window.mvtbox_set(Origin { y, x }, box_size, box_drawing_type)?;
        }

        // add the type of box drawing type on the window.
        let display_str = &format!("box drawing type {:?}", box_drawing_type);
        window.mvadd_wchstr(display_origin, &ComplexString::from_str(display_str, &attrs, &display_color_pair)?)?;

        // press a key or wait for 5 seconds
        window.getch_nonblocking(Some(time::Duration::new(5, 0)))?;

        // clear the window
        window.clear()?;
    }

    Ok(())
}
