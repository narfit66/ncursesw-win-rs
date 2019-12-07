/*
    examples/getch_nonblocking-test.rs

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

use std::time;

use ncurseswwin::*;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswWinError> } }

fn main() {
    if let Err(source) = main_routine() { match source {
        NCurseswWinError::Panic { message } => println!("panic: {}", message),
        _                                   => println!("error: {}", source)
    }}
}

fn main_routine() -> result!(()) {
    setlocale(LcCategory::All, "")?;

    // initialize ncurses in a safe way.
    ncursesw_entry(|window| {
        cursor_set(CursorType::Visible)?;
        set_echo(true)?;

        getch_nonblocking_test(&window)
    })
}

fn getch_nonblocking_test(stdscr: &Window) -> result!(()) {
    stdscr.keypad(true)?;

    let display_origin = Origin { y: 2, x: 2 };
    let display_str = "Press 'q' or 'Q' to quit, any other key to continue or wait for 5 seconds:";
    let getch_origin = Origin { y: display_origin.y, x: display_origin.x + display_str.len() as u16 + 1 };
    let getch_result_origin = Origin { y: getch_origin.y, x: getch_origin.x + 3 };

    stdscr.mvaddstr(display_origin, display_str)?;

    loop {
        // press 'q' or 'Q' to quit, any other key to continue or wait for 5 seconds,
        let getch_result = match stdscr.mvgetch_nonblocking(getch_origin, Some(time::Duration::new(5, 0)))? {
            Some(char_result) => match char_result {
                CharacterResult::Key(key_binding)     => format!("key binding: {:?}", key_binding),
                CharacterResult::Character(character) => if character == 'q' || character == 'Q' {
                    break;
                } else {
                    format!("character: '{}'", character)
                }
            },
            None              => "timeout!!!".to_string()
        };

        stdscr.clrtoeol()?;
        stdscr.mvaddstr(getch_result_origin, &getch_result)?;
    }

    Ok(())
}
