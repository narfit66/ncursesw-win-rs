/*
    examples/get_wch_nonblocking-test.rs

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

fn getch_nonblocking_test(window: &Window) -> result!(()) {
    window.keypad(true)?;

    let timeout = time::Duration::new(5, 0);

    let display_origin = Origin { y: 2, x: 2 };
    let display_str = &format!("Press 'q' or 'Q' to quit, any other key to continue or wait for {:?} :", timeout);
    let getch_origin = Origin { y: display_origin.y, x: display_origin.x + display_str.len() as u16 + 1 };
    let getch_result_origin = Origin { y: getch_origin.y, x: getch_origin.x + 3 };

    window.mvaddstr(display_origin, display_str)?;

    let lower_q = WideChar::new('q');
    let upper_q = WideChar::new('Q');

    loop {
        // press 'q' or 'Q' to quit, any other key to continue or wait until we timeout.
        let getch_result = match window.mvget_wch_nonblocking(getch_origin, Some(timeout))? {
            Some(char_result) => match char_result {
                CharacterResult::Key(key_binding)     => format!("key binding: {:?}", key_binding),
                CharacterResult::Character(character) => if character == lower_q || character == upper_q {
                    break;
                } else {
                    format!("character: '{:?}'", character)
                }
            },
            None              => "timeout reached!!!".to_string()
        };

        window.clrtoeol()?;
        window.mvaddstr(getch_result_origin, &getch_result)?;
    }

    Ok(())
}
