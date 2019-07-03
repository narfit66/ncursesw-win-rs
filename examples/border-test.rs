extern crate ncursesw;
extern crate ncurseswwin;

use ncursesw::{curs_set, NCurseswError, CursorType, Origin};
use ncurseswwin::*;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswError> } }

fn main() {
    if let Err(e) = main_routine() {
        println!("error: {}", e);
    }
}

fn main_routine() -> result!(()) {
    ncursesw_init(|ncurses| {
        let doit = |initial_window: &Window| -> result!(()) {
            // set the cursor to invisible and switch echoing off.
            curs_set(CursorType::Invisible)?;
            set_echo(false)?;

            // extract the box drawing characters for the box drawing type.
            let ul = chtype_box_graphic(BoxDrawingGraphic::UpperLeftCorner);
            let ll = chtype_box_graphic(BoxDrawingGraphic::LowerLeftCorner);
            let ur = chtype_box_graphic(BoxDrawingGraphic::UpperRightCorner);
            let lr = chtype_box_graphic(BoxDrawingGraphic::LowerRightCorner);
            let hl = chtype_box_graphic(BoxDrawingGraphic::HorizontalLine);
            let vl = chtype_box_graphic(BoxDrawingGraphic::VerticalLine);

            // create a border on the inital window (stdscr).
            initial_window.border(vl, vl, hl, hl, ul, ur, ll, lr)?;

            // add some default text to the inner window.
            let mut origin = Origin { y: 1, x: 2 };

            initial_window.mvaddstr(origin, "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.")?;
            origin.y += 1;
            initial_window.mvaddstr(origin, "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.")?;

            // refresh our window.
            initial_window.refresh()?;

            // wait for user input (or an event).
            initial_window.getch()?;

            Ok(())
        };

        // initialize ncurses in a safe way.
        if let Err(e) = doit(&ncurses.initial_window()) {
            panic!(e.to_string())
        }
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("A Panic Occurred: {}", errmsg),
        None         => println!("There was an error, but no error message."),
    });

    Ok(())
}
