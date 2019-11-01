/*
    examples/ncursesw_init-test.rs

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

extern crate ascii;
extern crate ncurseswwin;

use ascii::AsciiChar;
use ncurseswwin::*;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswWinError> } }

// run ncursesw_init_test(), the error returned by ncursesw_init_test_fail()
// will be caught as an error.
fn main() {
    // We wrap all our use of ncurseswin with this function.
    match ncursesw_init(|window| {
        ncursesw_init_test(&window)
    }) {
        Err(e) => {
            match e {
                NCurseswWinError::Panic { message } => eprintln!("panic: {}", message),
                error                               => eprintln!("error: {}", error)
            }
        },
        _      => ()
    }
}

fn ncursesw_init_test(initial_window: &Window) -> result!(()) {
    cursor_set(CursorType::Invisible)?;
    set_echo(false)?;

    ncursesw_init_test_pass(initial_window)?;
    ncursesw_init_test_fail(initial_window)?;

    Ok(())
}

fn ncursesw_init_test_pass(window: &Window) -> result!(()) {
    let mut origin = Origin { y: 0, x: 0};

    window.mvaddstr(origin, "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.")?;
    origin.y += 1;
    window.mvaddstr(origin, "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.")?;
    origin.y += 2;
    window.mvaddstr(origin, "Press <Return> to continue: ")?;

    window.refresh()?;

    window.getch()?;

    Ok(())
}

// this will cause an NCurseswError to be returned!!!
fn ncursesw_init_test_fail(window: &Window) -> result!(()) {
    window.mvaddch(Origin { y: LINES() + 1, x: COLS() + 1 }, ChtypeChar::new(AsciiChar::Asterisk))?;

    Ok(())
}
