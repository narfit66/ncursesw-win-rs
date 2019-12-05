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

// based on example 17.5 at <http://www.tldp.org/HOWTO/NCURSES-Programming-HOWTO/menus.html>

extern crate ncurseswwin;

use ncurseswwin::{*, normal::*, menu::*};

macro_rules! result { ($t: ty) => { Result<$t, NCurseswWinError> } }

const CHOICES: [&str; 11] = ["Choice 1", "Choice 2", "Choice 3",
                             "Choice 4", "Choice 5", "Choice 6",
                             "Choice 7", "Choice 8", "Choice 9",
                             "Choice 10", "Exit"];

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
        menu_test(window)
    })
}

fn menu_test(stdscr: &Window) -> result!(()) {
    // Initialize ncurses instance.
    start_color()?;
    set_input_mode(InputMode::Character)?;
    set_echo(false)?;
    stdscr.keypad(true)?;

    // Initialize all the colors.
    let red = Color::Dark(BaseColor::Red);
    let cyan = Color::Dark(BaseColor::Cyan);
    let black = Color::Dark(BaseColor::Black);

    let color_pairs: [ColorPair; 2] = [ColorPair::new(1, Colors::new(red, black))?,
                                       ColorPair::new(2, Colors::new(cyan, black))?];

    let my_item0 = MenuItem::new(CHOICES[0], CHOICES[0])?;
    let my_item1 = MenuItem::new(CHOICES[1], CHOICES[1])?;
    let my_item2 = MenuItem::new(CHOICES[2], CHOICES[2])?;
    let my_item3 = MenuItem::new(CHOICES[3], CHOICES[3])?;
    let my_item4 = MenuItem::new(CHOICES[4], CHOICES[4])?;
    let my_item5 = MenuItem::new(CHOICES[5], CHOICES[5])?;
    let my_item6 = MenuItem::new(CHOICES[6], CHOICES[6])?;
    let my_item7 = MenuItem::new(CHOICES[7], CHOICES[7])?;
    let my_item8 = MenuItem::new(CHOICES[8], CHOICES[8])?;
    let my_item9 = MenuItem::new(CHOICES[9], CHOICES[9])?;
    let my_item10 = MenuItem::new(CHOICES[10], CHOICES[10])?;

    let mut my_items = vec!();

    my_items.push(&my_item0);
    my_items.push(&my_item1);
    my_items.push(&my_item2);
    my_items.push(&my_item3);
    my_items.push(&my_item4);
    my_items.push(&my_item5);
    my_items.push(&my_item6);
    my_items.push(&my_item7);
    my_items.push(&my_item8);
    my_items.push(&my_item9);
    my_items.push(&my_item10);

    let my_menu = &Menu::new(&mut my_items)?;

    let my_menu_win = &Window::new(Size { lines: 10, columns: 40 }, Origin { y: 4, x: 4 })?;
    my_menu_win.keypad(true)?;

    my_menu.set_menu_win(Some(my_menu_win))?;
    let my_menu_win_derwin = &my_menu_win.derwin(Size { lines: 6, columns: 38 }, Origin { y: 3, x: 1 })?;
    my_menu.set_menu_sub(Some(my_menu_win_derwin))?;
    my_menu.set_menu_format(MenuSize { rows: 5, columns: 1 })?;

    my_menu.set_menu_mark(" * ")?;

    my_menu_win.r#box(ChtypeChar::from(0), ChtypeChar::from(0))?;

    print_in_middle(my_menu_win, Origin { y: 1, x: 0 }, 40, "My Menu", color_pairs[0])?;
    my_menu_win.mvaddch(Origin { y: 2, x: 0 }, chtype_box_graphic(BoxDrawingGraphic::LeftTee))?;
    my_menu_win.mvhline(Origin { y: 2, x: 1 }, chtype_box_graphic(BoxDrawingGraphic::HorizontalLine), 38)?;
    my_menu_win.mvaddch(Origin { y: 2, x: 39 }, chtype_box_graphic(BoxDrawingGraphic::RightTee))?;

    let posted_menu = &my_menu.post_menu(true)?;
    //my_menu_win.refresh()?;

    stdscr.attron(Attributes::default() | color_pairs[1])?;
    let mut origin = Origin { y: LINES()? - 2, x: 0 };
    stdscr.mvaddstr(origin, "Use PageUp and PageDown to scoll down or up a page of items")?;
    origin.y += 1;
    stdscr.mvaddstr(origin, "Arrow Keys to navigate (F1 to Exit)")?;
    stdscr.attroff(Attributes::default() | color_pairs[1])?;

    stdscr.refresh()?;

    loop {
        match my_menu_win.getch() {
            Err(source)     => return Err(source),
            Ok(char_result) => match char_result {
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
                    } else if key == KeyBinding::NextPage {
                        if let Err(source) = posted_menu.menu_driver(MenuRequest::ScrollDownPage) {
                            if source != request_denied_error() {
                                return Err(source)
                            }
                        }
                    } else if key == KeyBinding::PreviousPage {
                        if let Err(source) = posted_menu.menu_driver(MenuRequest::ScrollUpPage) {
                            if source != request_denied_error() {
                                return Err(source)
                            }
                        }
                    }
                },
                CharacterResult::Character(key) => {
                    if key == 's' {
                        origin = Origin { y: 20, x: 0 };

                        stdscr.set_cursor(origin)?;
                        stdscr.clrtoeol()?;
                        stdscr.mvaddstr(origin, &format!("Item selected is : {}", my_menu.current_item()?.item_name()?))?;
                        stdscr.refresh()?;
                        my_menu.pos_menu_cursor()?;
                    }
                }
            }
        }

        my_menu_win.refresh()?;
    }

    Ok(())
}

fn print_in_middle(
    window:       &Window,
    start_origin: Origin,
    width:        u16,
    string:       &str,
    color_pair:   ColorPair
) -> result!(()) {
    let mut origin = window.cursor()?;
    let mut width = width;

    if start_origin.y != 0 {
        origin.y = start_origin.y;
    }
    if start_origin.x != 0 {
        origin.x = start_origin.x;
    }
    if width == 0 {
        width = 80;
    }
    origin.x = start_origin.x + ((width / string.len() as u16) / 2);

    window.attron(Attributes::default() | color_pair)?;
    window.mvaddstr(origin, string)?;
    window.attroff(Attributes::default() | color_pair)?;
    window.refresh()
}

fn request_denied_error() -> NCurseswWinError {
    NCurseswWinError::from(NCurseswMenuError::RequestDenied { func: "menu_driver".to_string() })
}
