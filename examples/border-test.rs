extern crate ncursesw;
extern crate ncurseswin;

use ncursesw::{curs_set, NCurseswError, CursorType, Origin};
use ncurseswin::*;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswError> } }

fn main() {
    if let Err(e) = main_routine() {
        println!("error: {}", e);
    }
}

fn main_routine() -> result!(()) {
    ncursesw_init(|ncurses| {
        let doit = |initial_window: Window| -> result!(()) {
            curs_set(CursorType::Invisible)?;
            set_echo(false)?;

            let ul = chtype_box_graphic(BoxDrawingGraphic::UpperLeftCorner);
            let ll = chtype_box_graphic(BoxDrawingGraphic::LowerLeftCorner);
            let ur = chtype_box_graphic(BoxDrawingGraphic::UpperRightCorner);
            let lr = chtype_box_graphic(BoxDrawingGraphic::LowerRightCorner);
            let hl = chtype_box_graphic(BoxDrawingGraphic::HorizontalLine);
            let vl = chtype_box_graphic(BoxDrawingGraphic::VerticalLine);

            initial_window.border(vl, vl, hl, hl, ul, ur, ll, lr)?;

            let mut origin = Origin { y: 1, x: 2 };

            initial_window.mvaddstr(origin, "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.")?;
            origin.y += 1;

            initial_window.mvaddstr(origin, "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.")?;

            initial_window.refresh()?;

            initial_window.getch()?;

            Ok(())
        };

        panic!(match doit(ncurses.initial_window()) {
            Err(e) => e.to_string(),
            _      => "this is the end my friend, the only end!!!".to_string()
        })
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("A Panic Occurred: {}", errmsg),
        None         => println!("There was an error, but no error message."),
    });

    Ok(())
}
