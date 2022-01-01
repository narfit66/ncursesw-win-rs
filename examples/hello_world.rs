/*
    examples/hello_world.rs

    Copyright (c) 2020 Stephen Whittle  All rights reserved.

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

use std::{convert::TryFrom, process::exit};
use anyhow::Result;
use ncurseswwin::*;

fn main() {
    if let Err(source) = ncursesw_entry(|stdscr| hello_world(stdscr)) {
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

fn hello_world(stdscr: &Window) -> Result<()> {
    let hello_world = "Hello World!!!";

    // print "Hello World".
    stdscr.mvaddstr(Origin { y: LINES()? / 2, x: (COLS()? / 2) - (u16::try_from(hello_world.len())? / 2) }, hello_world)?;
    // print it on the real screen.
    stdscr.refresh()?;
    // wait for the user input.
    stdscr.getch()?;

    Ok(())
}
