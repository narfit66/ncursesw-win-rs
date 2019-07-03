extern crate ncursesw;
extern crate ncurseswwin;

use ncursesw::{curs_set, NCurseswError, CursorType, Origin, Orientation};
use ncurseswwin::*;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswError> } }

fn main() {
    if let Err(e) = main_routine() {
        println!("error: {}", e);
    }
}

fn main_routine() -> result!(()) {
    // ripoff a line from the top of the screen.
    let top_ripoff = match ripoffline(Orientation::Top) {
        Err(e) => return Err(e),
        Ok(n)  => n
    };

    // ripoff a line from the bottom of the screen.
    let bottom_ripoff = match ripoffline(Orientation::Bottom) {
        Err(e) => return Err(e),
        Ok(n)  => n
    };

    assert!(top_ripoff != bottom_ripoff);

    ncursesw_init(|ncurses| {
        let doit = |initial_window: &Window| -> result!(()) {
            curs_set(CursorType::Invisible)?;
            set_echo(false)?;

            let mut origin = Origin { y: 1, x: 0};

            initial_window.mvaddstr(origin, "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.")?;
            origin.y += 1;
            initial_window.mvaddstr(origin, "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.")?;

            initial_window.refresh()?;

            //  update the top ripoff line.
            update_ripoffline(top_ripoff, |ripoff, columns| -> result!(()) {
                ripoff.addstr(&format!("this is the ripoff line at the top of the screen with a maximum of {} columns", columns))?;
                ripoff.noutrefresh()?;

                Ok(())
            })?;

            //  update the bottom ripoff line.
            update_ripoffline(bottom_ripoff, |ripoff, columns| -> result!(()) {
                ripoff.addstr(&format!("this is the ripoff line at the bottom of the screen with a maximum of {} columns", columns))?;
                ripoff.noutrefresh()?;

                Ok(())
            })?;

            ncursesw::doupdate()?;

            initial_window.getch()?;

            Ok(())
        };

        if let Err(e) = doit(&ncurses.initial_window()) {
            panic!(e.to_string())
        }
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("A Panic Occurred: {}", errmsg),
        None         => println!("There was an error, but no error message."),
    });

    Ok(())
}
