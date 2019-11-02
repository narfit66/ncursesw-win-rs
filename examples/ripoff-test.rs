/*
    examples/ripoff_line-test.rs

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

use ncurseswwin::*;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswWinError> } }

fn main() {
    if let Err(e) = main_routine() {
        println!("error: {}", e);
    }
}

fn main_routine() -> result!(()) {
    // ripoff a line from the top of the screen.
    let top_ripoff = RipoffLine::new(Orientation::Top)?;
    // ripoff a line from the bottom of the screen.
    let bottom_ripoff = RipoffLine::new(Orientation::Bottom)?;

    assert!(top_ripoff != bottom_ripoff);

    // initialize ncurses in a safe way.
    ncursesw_init(|window| {
        if let Err(e) = ripoff_line_test(&window, &top_ripoff, &bottom_ripoff) {
            panic!(e.to_string())
        }
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("A Panic Occurred: {}", errmsg),
        None         => println!("There was an error, but no error message."),
    });

    Ok(())
}

fn ripoff_line_test(initial_window: &Window, top_ripoff: &RipoffLine, bottom_ripoff: &RipoffLine) -> result!(()) {
    cursor_set(CursorType::Invisible)?;
    set_echo(false)?;

    let mut origin = Origin { y: 1, x: 0};

    initial_window.mvaddstr(origin, "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.")?;
    origin.y += 1;
    initial_window.mvaddstr(origin, "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.")?;

    initial_window.refresh()?;

    //  update the top ripoff line.
    top_ripoff.update(|ripoff_window, columns| -> result!(()) {
        update_top_ripoff(ripoff_window, columns)
    })?;

    //  update the bottom ripoff line.
    bottom_ripoff.update(|ripoff_window, columns| -> result!(()) {
        update_bottom_ripoff(ripoff_window, columns)
    })?;

    doupdate()?;

    initial_window.getch()?;

    Ok(())
}

fn update_top_ripoff(ripoff_window: &RipoffWindow, columns: i32) -> result!(()) {
    ripoff_window.addstr(&format!("this is the ripoff line at the top of the screen with a maximum of {} columns", columns))?;
    ripoff_window.noutrefresh()?;

    Ok(())
}

fn update_bottom_ripoff(ripoff_window: &RipoffWindow, columns: i32) -> result!(()) {
    ripoff_window.addstr(&format!("this is the ripoff line at the bottom of the screen with a maximum of {} columns", columns))?;
    ripoff_window.noutrefresh()?;

    Ok(())
}
