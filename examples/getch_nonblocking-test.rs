/*
    examples/getch_nonblocking-test.rs

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

use std::{convert::TryFrom, time, process::exit};
use anyhow::Result;
use gettextrs::*;
use ncurseswwin::*;

fn main() {
    if let Err(source) = main_routine() {
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
    }

    exit(0);
}

fn main_routine() -> Result<()> {
    setlocale(LocaleCategory::LcAll, "");

    // initialize ncurses in a safe way.
    ncursesw_entry(|stdscr| {
        set_input_mode(InputMode::Character)?;
        set_echo(false)?;
        set_newline(false)?;
        intrflush(false)?;

        cursor_set(CursorType::Visible)?;

        getch_nonblocking_test(stdscr)
    })
}

fn getch_nonblocking_test(stdscr: &Window) -> Result<()> {
    stdscr.keypad(true)?;

    let timeout = time::Duration::new(5, 0);

    let display_origin = Origin { y: 2, x: 2 };
    let display_str = &format!("Press 'q' or 'Q' to quit, any other key to continue or wait for {:?} :", timeout);
    let getch_origin = Origin { y: display_origin.y, x: display_origin.x + u16::try_from(display_str.len())? + 1 };
    let getch_result_origin = Origin { y: getch_origin.y, x: getch_origin.x + 3 };

    stdscr.mvaddstr(display_origin, display_str)?;

    loop {
        // press 'q' or 'Q' to quit, any other key to continue or wait until we timeout.
        let getch_result = match stdscr.mvgetch_nonblocking(getch_origin, Some(timeout))? {
            Some(char_result) => match char_result {
                CharacterResult::Key(key_binding)     => format!("key binding: {:?}", key_binding),
                CharacterResult::Character(character) => if character.to_ascii_lowercase() == 'q' {
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
