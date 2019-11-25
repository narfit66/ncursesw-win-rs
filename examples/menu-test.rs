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

// based on example 17.3 at <http://www.tldp.org/HOWTO/NCURSES-Programming-HOWTO/menus.html>

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
        menu_test(&window)
    })
}

fn menu_test(stdscr: &Window) -> result!(()) {
    // Initialize ncurses instance.
    start_color()?;
    set_input_mode(InputMode::Character)?;
    set_echo(false)?;
    stdscr.keypad(true)?;

    // Initialize all the colors.
    let red = Color::Light(BaseColor::Red);
    let cyan = Color::Light(BaseColor::Cyan);
    let black = Color::Light(BaseColor::Black);

    let color_pairs: [ColorPair; 2] = [ColorPair::new(1, Colors::new(red, black))?,
                                       ColorPair::new(2, Colors::new(cyan, black))?];

    let my_items: [&MenuItem; 11] = [&MenuItem::new(CHOICES[0], CHOICES[0])?,
                                     &MenuItem::new(CHOICES[1], CHOICES[1])?,
                                     &MenuItem::new(CHOICES[2], CHOICES[2])?,
                                     &MenuItem::new(CHOICES[3], CHOICES[3])?,
                                     &MenuItem::new(CHOICES[4], CHOICES[4])?,
                                     &MenuItem::new(CHOICES[5], CHOICES[5])?,
                                     &MenuItem::new(CHOICES[6], CHOICES[6])?,
                                     &MenuItem::new(CHOICES[7], CHOICES[7])?,
                                     &MenuItem::new(CHOICES[8], CHOICES[8])?,
                                     &MenuItem::new(CHOICES[9], CHOICES[9])?,
                                     &MenuItem::new(CHOICES[10], CHOICES[10])?];

    for i in 0..my_items.len() {
        eprintln!("my_items[{}]: {:?}", i, my_items[i]);
        assert!(my_items[i].item_name()? == CHOICES[i]);
        assert!(my_items[i].item_description()? == CHOICES[i]);
    }

    assert!(my_items[0] == my_items[0]);
    assert!(my_items[0] != my_items[1]);

    let my_menu = &Menu::new(my_items.to_vec())?;
    eprintln!("my_menu: {:?}", my_menu);

    assert!(my_menu.item_count()? == my_items.len());

    let current_item = &my_menu.current_item()?;
    eprintln!("current_item: {:?}", current_item);

    eprintln!("my_menu.current_item().item_name(): '{}'", current_item.item_name()?);
    eprintln!("my_menu.current_item().item_description(): '{}'", current_item.item_description()?);

    eprintln!("before my_menu.set_current_item({:?})", my_items[0]);
    my_menu.set_current_item(my_items[0])?;

    eprintln!("before my_menu.menu_items()");
    let test_items = my_menu.menu_items()?;

    for (i, test_item) in test_items.iter().enumerate() {
        eprintln!("test_item: {:?}", test_item);
        assert!(test_item.item_name()? == CHOICES[i]);
        assert!(test_item.item_description()? == CHOICES[i]);
    }

    let my_menu_win = &Window::new(Size { lines: 10, columns: 40 }, Origin { y: 4, x: 4 })?;
    my_menu_win.keypad(true)?;

    eprintln!("before my_menu.set_menu_win()");
    my_menu.set_menu_win(my_menu_win)?;
    let my_menu_win_derwin = &my_menu_win.derwin(Size { lines: 6, columns: 38 }, Origin { y: 3, x: 1 })?;
    eprintln!("before my_menu.set_menu_sub()");
    my_menu.set_menu_sub(my_menu_win_derwin)?;

    eprintln!("(1) my_menu.menu_mark(): '{}'", my_menu.menu_mark()?);

    my_menu.set_menu_mark(" * ")?;

    eprintln!("(2) my_menu.menu_mark(): '{}'", my_menu.menu_mark()?);

    // extract the box drawing characters for the box drawing type.
    let left_side   = chtype_box_graphic(BoxDrawingGraphic::LeftVerticalLine);
    let right_side  = chtype_box_graphic(BoxDrawingGraphic::RightVerticalLine);
    let top_side    = chtype_box_graphic(BoxDrawingGraphic::UpperHorizontalLine);
    let bottom_side = chtype_box_graphic(BoxDrawingGraphic::LowerHorizontalLine);
    let upper_left  = chtype_box_graphic(BoxDrawingGraphic::UpperLeftCorner);
    let upper_right = chtype_box_graphic(BoxDrawingGraphic::UpperRightCorner);
    let lower_left  = chtype_box_graphic(BoxDrawingGraphic::LowerLeftCorner);
    let lower_right = chtype_box_graphic(BoxDrawingGraphic::LowerRightCorner);

    // create a border on the inital window (stdscr).
    my_menu_win.border(left_side, right_side, top_side, bottom_side, upper_left, upper_right, lower_left, lower_right)?;
    my_menu_win.mvaddch(Origin { y: 2, x: 0 }, chtype_box_graphic(BoxDrawingGraphic::LeftTee))?;
    my_menu_win.mvhline(Origin { y: 2, x: 1 }, chtype_box_graphic(BoxDrawingGraphic::HorizontalLine), 38)?;
    my_menu_win.mvaddch(Origin { y: 2, x: 39 }, chtype_box_graphic(BoxDrawingGraphic::RightTee))?;
    print_in_middle(my_menu_win, Origin { y: 1, x: 0 }, 40, "My Menu", color_pairs[0])?;

    eprintln!("before my_menu.post_menu()");
    //my_menu.post_menu()?;
    /* */
    let r = my_menu.post_menu();
    eprintln!("r = {:?}", r);
    /* */

    eprintln!("before my_menu_win.refresh()");
    my_menu_win.refresh()?;

    stdscr.attron(Attributes::default() | color_pairs[1])?;
    stdscr.mvaddstr(Origin { y: LINES()? - 2, x: 0 }, "Use PageUp and PageDown to scoll down or up a page of items")?;
    stdscr.mvaddstr(Origin { y: LINES()? - 1, x: 0 }, "Arrow Keys to navigate (F1 to Exit)")?;
    stdscr.attroff(Attributes::default() | color_pairs[1])?;
    stdscr.refresh()?;

    loop {
        match my_menu_win.getch() {
            Err(source)     => return Err(source),
            Ok(char_result) => if let CharacterResult::Key(key) = char_result {
                if key == KeyBinding::FunctionKey(1) {
                    break;
                } else if key == KeyBinding::DownArrow {
                    my_menu.menu_driver(MenuRequest::DownItem)?;
                } else if key == KeyBinding::UpArrow {
                    my_menu.menu_driver(MenuRequest::UpItem)?;
                }
            }
        }
    }

    eprintln!("before my_menu.unpost_menu()");
    my_menu.unpost_menu()?;

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
    window.refresh()?;

    Ok(())
}