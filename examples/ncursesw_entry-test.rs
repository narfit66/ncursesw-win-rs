/*
    examples/ncursesw_entry-test.rs

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
use ncurseswwin::*;

macro_rules! result { ($type: ty) => { Result<$type, NCurseswWinError> } }

fn main() {
    // initialise ncurses safely, this should trap panic's and
    // pass them as a `NCurseswWinError::Panic`.
    match ncursesw_entry(|stdscr| {
        set_input_mode(InputMode::Character)?;
        set_echo(false)?;
        set_newline(false)?;
        intrflush(false)?;

        cursor_set(CursorType::Visible)?;

        ncursesw_entry_test_pass(stdscr)?;

        Ok(ncursesw_entry_test_fail()?)
    }) {
        Err(source) => {
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
        },
        Ok(value)   => {
            assert!(value == -1);

            println!("return: {}", value);

            exit(0);
        }
    }
}

fn ncursesw_entry_test_pass(stdscr: &Window) -> result!(()) {
    let mut origin = Origin::default();

    stdscr.mvaddstr(origin, "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.")?;
    origin.y += 1;
    stdscr.mvaddstr(origin, "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.")?;
    origin.y += 2;
    stdscr.mvaddstr(origin, "Press <Return> to continue: ")?;

    stdscr.refresh()?;

    stdscr.getch()?;

    Ok(())
}

// leave un-commented what needs testing
fn ncursesw_entry_test_fail() -> result!(i32) {
    //panic!("there was a panic!");        // cause a panic
    Err(NCurseswWinError::InternalError) // return an error
    //Ok(-1)                               // return a value
}
