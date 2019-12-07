/*
    examples/menu-test.rs

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

use ncurseswwin::{*, menu::*};

macro_rules! result { ($t: ty) => { Result<$t, NCurseswWinError> } }

const CHOICES: [&str; 5] = ["Choice 1", "Choice 2", "Choice 3", "Choice 4", "Exit"];

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
        set_input_mode(InputMode::Character)?;
        set_echo(false)?;
        cursor_set(CursorType::Invisible)?;

        menu_test(&window)
    })
}

fn menu_test(stdscr: &Window) -> result!(()) {
    stdscr.keypad(true)?;

    let my_item0 = MenuItem::new(CHOICES[0], &format!("{} description", CHOICES[0]))?;
    let my_item1 = MenuItem::new(CHOICES[1], &format!("{} description", CHOICES[1]))?;
    let my_item2 = MenuItem::new(CHOICES[2], &format!("{} description", CHOICES[2]))?;
    let my_item3 = MenuItem::new(CHOICES[3], &format!("{} description", CHOICES[3]))?;
    let my_item4 = MenuItem::new(CHOICES[4], &format!("{} description", CHOICES[4]))?;

    // Create items.
    let mut my_items = vec!();

    my_items.push(&my_item0);
    my_items.push(&my_item1);
    my_items.push(&my_item2);
    my_items.push(&my_item3);
    my_items.push(&my_item4);

    // Crate menu.
    let my_menu = &Menu::new(&my_items)?;

    let mut menu_opts = MenuOptions::default();
    menu_opts.set_show_description(true);

    my_menu.menu_opts_off(menu_opts)?;

    let my_menu_win = &Window::new(Size { lines: 9, columns: 18 }, Origin { y: 4, x: 4 })?;
    my_menu_win.keypad(true)?;

    // Set main window and sub window.
    my_menu.set_menu_win(Some(my_menu_win))?;
    let my_menu_win_der_win = &my_menu_win.derwin(Size { lines: 5, columns: 0 }, Origin { y: 2, x: 2 })?;
    my_menu.set_menu_sub(Some(my_menu_win_der_win))?;

    // Set menu mark to the string " * ".
    my_menu.set_menu_mark(" * ")?;

    // Print a border around the main window.
    my_menu_win.r#box(ChtypeChar::from(0), ChtypeChar::from(0))?;

    let mut origin = Origin { y: LINES()? - 3, x: 0 };
    stdscr.mvaddstr(origin, "Press <Enter> to see the option selected")?;
    origin.y += 1;
    stdscr.mvaddstr(origin, "F1 to exit")?;
    stdscr.refresh()?;

    // Post the menu and refresh the menu's window.
    let posted_menu = &my_menu.post_menu(true)?;

    loop {
        match stdscr.getch()? {
            CharacterResult::Key(key) => {
                if key == KeyBinding::FunctionKey(1) {
                    break;
                } else if key == KeyBinding::DownArrow {
                    if let Err(source) = posted_menu.menu_driver(MenuRequest::DownItem) {
                        if source != request_denied_error() {
                            return Err(source)
                        }
                    }
                } else if key == KeyBinding::UpArrow {
                    if let Err(source) = posted_menu.menu_driver(MenuRequest::UpItem) {
                        if source != request_denied_error() {
                            return Err(source)
                        }
                    }
                }
            },
            CharacterResult::Character(key) => {
                if key == '\n' {
                    origin = Origin { y: 20, x: 0 };

                    stdscr.set_cursor(origin)?;
                    stdscr.clrtoeol()?;
                    stdscr.mvaddstr(origin, &format!("Item selected is : {}", my_menu.current_item()?.item_name()?))?;
                    stdscr.refresh()?;
                    my_menu.pos_menu_cursor()?;
                }
            }
        }

        my_menu_win.refresh()?;
    }

    Ok(())
}

fn request_denied_error() -> NCurseswWinError {
    NCurseswWinError::from(NCurseswMenuError::RequestDenied { func: "menu_driver".to_string() })
}
