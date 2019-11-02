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

use ncurseswwin::*;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswWinError> } }

fn main() {
    match main_routine() {
        Err(source) => match source {
            NCurseswWinError::Panic { message } => println!("panic: {}", message),
            _                                   => println!("error: {}", source)
        },
        Ok(value)   => {
            assert!(value == -1);

            println!("return: {}", value)
        }
    }
}

fn main_routine() -> result!(i32) {
    // We wrap all our use of ncurseswin with this function.
    match ncursesw_init(|window| {
        // In here we get an initialized Window structure (stdscr) and then proceed
        // to use it exactly like we normally would use it.
        match ncursesw_init_test(&window) {
            Err(source) => Ok(Err(source)),
            Ok(value)   => Ok(Ok(value))
        }
    }).unwrap_or_else(|e| Err(match e {
        // This block only runs if there was an error. We might or might not
        // have been able to recover an error message. You technically can pass
        // any value into a panic, but we only get an error message if the panic
        // value was a `String` or `&str`.
        Some(message) => NCurseswWinError::Panic { message },
        None          => NCurseswWinError::Panic { message: "There was a panic, but no message!".to_string() }
    })) {
        // The Err branch matches against the NCurseswWinError::Panic error
        // from the above unwrap_or_else()
        Err(source) => Err(source),
        // The Ok branch unwraps and matches against ncursesw_init_test error
        // or return value
        Ok(result)  => {
            match result {
                Err(source) => Err(source),
                Ok(value)   => Ok(value)
            }
        }
    }
}

fn ncursesw_init_test(initial_window: &Window) -> result!(i32) {
    cursor_set(CursorType::Invisible)?;
    set_echo(false)?;

    ncursesw_init_test_pass(initial_window)?;

    let rc = ncursesw_init_test_fail(initial_window)?;

    Ok(rc)
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

// leave un-commented what needs testing
fn ncursesw_init_test_fail(_window: &Window) -> result!(i32) {
    //panic!("there was a panic!");        // cause a panic
    Err(NCurseswWinError::InternalError) // return an error
    //Ok(-1)                               // return a value
}
