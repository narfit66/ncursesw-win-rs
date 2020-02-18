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

use ncurseswwin::*;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswWinError> } }

fn main() {
    // initialise ncurses safely, this should trap panic's and
    // pass them as a `NCurseswWinError::Panic`.
    match ncursesw_entry(|window| {
        cursor_set(CursorType::Visible)?;
        set_echo(false)?;

        ncursesw_entry_test_pass(window)?;

        Ok(ncursesw_entry_test_fail()?)
    }) {
        Err(source) => match source {
            NCurseswWinError::Panic { message } => eprintln!("panic: {}", message),
            _                                   => eprintln!("error: {}", source)
        },
        Ok(value)   => {
            assert!(value == -1);

            println!("return: {}", value)
        }
    }
}

fn ncursesw_entry_test_pass(stdscr: &Window) -> result!(()) {
    let mut origin = Origin { y: 0, x: 0};

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
