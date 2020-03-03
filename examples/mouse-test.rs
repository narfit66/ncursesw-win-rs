/*
    examples/mouse-test.rs

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

extern crate ascii;
extern crate ncurseswwin;

use std::process::exit;
use ascii::AsciiChar;
use anyhow::Result;
use ncurseswwin::*;

macro_rules! result { ($type: ty) => { Result<$type, NCurseswWinError> } }

fn main() {
    // initialize ncurses in a safe way.
    if let Err(source) = ncursesw_entry(|stdscr| {
        set_input_mode(InputMode::Character)?;
        set_echo(false)?;
        set_newline(false)?;
        intrflush(false)?;

        cursor_set(CursorType::Visible)?;

        mouse_test(stdscr)
    }) {
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

fn mouse_test(stdscr: &Window) -> Result<()> {
    stdscr.keypad(true)?;

    let mut origin = Origin { y: 1, x: 1 };

    stdscr.mvaddstr(origin, &format!("Mouse Version : {} ", mouse_version()))?;
    origin.y += 2;
    stdscr.mvaddstr(origin, "Hit <Return> to continue : ")?;
    origin.y += 2;

    let next_origin = Origin { y: origin.y + 1, x: origin.x };

    stdscr.getch()?;

    cursor_set(CursorType::Invisible)?;

    if !has_mouse_interface() {  // check if ncursesw supports a mouse pointer
        panic!("no mouse interface detected!!!");
    }

    let mouse = &mut Mouse::new(MouseMask::AllMouseEvents)?;

    if !has_mouse() {            // has a mouse pointer been defined.
        panic!("no mouse detected!!!");
    }

    loop {
        match stdscr.getch()? {
            CharacterResult::Key(key_binding)     => {
                match key_binding {
                    KeyBinding::MouseEvent => {
                        if let Ok(registered_mouse) = mouse.refresh() {  // get the mouse event
                            if registered_mouse {                        // is the mouse event for our mouse
                                let mouse_events = mouse.events();

                                if let Some(button_event) = mouse_events.button_state() {
                                    mouse_button_event(
                                        stdscr,
                                        origin,
                                        button_event.button().number(),
                                        &format!("{}", button_event.event()),
                                        mouse.origin()?
                                    )?;
                                } else {
                                    other_event(stdscr, origin, "no mouse button event!!!!")?;
                                }

                                if mouse_events.ctrl_button() {
                                    other_event(stdscr, next_origin, "with <ctrl> pressed")?;
                                } else if mouse_events.shift_button() {
                                    other_event(stdscr, next_origin, "with <shift> pressed")?;
                                } else if mouse_events.alt_button() {
                                    other_event(stdscr, next_origin, "with <alt> pressed")?;
                                }
                            }
                        }
                    },
                    _                      => other_event(stdscr, origin, &format!("{:?}", key_binding))?
                }
            },
            CharacterResult::Character(character) => {
                other_event(stdscr, origin, &format!("{}", character))?;

                if character.to_ascii_lowercase() == 'q' {
                    break;
                }
            }
        }
    }

    Ok(())
}

fn mouse_button_event(window: &Window, origin: Origin, button: u8, str: &str, mouse_origin: MouseOrigin) -> result!(()) {
    clear_to_eol(window, origin)?;

    window.mvaddstr(origin, &format!("B{} {} @ {}", button, str, mouse_origin))?;

    let asterisk = ChtypeChar::new(AsciiChar::Asterisk);

    if mouse_origin.origin() == terminal_bottom_right_origin()? {
        window.mvinsch(mouse_origin.origin(), asterisk)?;
    } else {
        window.mvaddch(mouse_origin.origin(), asterisk)?;
    }

    Ok(())
}

fn other_event(window: &Window, origin: Origin, str: &str) -> result!(()) {
    clear_to_eol(window, origin)?;

    window.mvaddstr(origin, str)?;

    Ok(())
}

fn clear_to_eol(window: &Window, origin: Origin) -> result!(()) {
    window.set_cursor(origin)?;
    window.clrtoeol()?;

    Ok(())
}
