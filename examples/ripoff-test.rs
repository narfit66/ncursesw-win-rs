extern crate ncursesw;
extern crate ncurseswin;

use ncursesw::{curs_set, NCurseswError, CursorType, Orientation};
use ncurseswin::*;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswError> } }

fn main() {
    if let Err(e) = main_routine() {
        println!("error: {}", e);
    }
}

fn main_routine() -> result!(()) {
    let top_ripoff = match ripoffline(Orientation::Top) {
        Err(e) => return Err(e),
        Ok(n)  => n
    };

    let bottom_ripoff = match ripoffline(Orientation::Bottom) {
        Err(e) => return Err(e),
        Ok(n)  => n
    };

    assert!(top_ripoff != bottom_ripoff);

    ncursesw_init(|ncurses| {
        let doit = |initial_window: Window| -> result!(()) {
            curs_set(CursorType::Invisible)?;
            set_echo(false)?;

            initial_window.addstr("If the doors of perception were cleansed every thing would appear to man as it is: Infinite.\nFor man has closed himself up, till he sees all things thro' narrow chinks of his cavern.")?;
            initial_window.refresh()?;

            update_ripoffline(top_ripoff, |ripoff, columns| -> result!(()) {
                ripoff.addstr(&format!("this is the ripoff line at the top of the screen with a maximum of {} columns", columns))?;
                ripoff.noutrefresh()?;

                Ok(())
            })?;

            update_ripoffline(bottom_ripoff, |ripoff, columns| -> result!(()) {
                ripoff.addstr(&format!("this is the ripoff line at the bottom of the screen with a maximum of {} columns", columns))?;
                ripoff.noutrefresh()?;

                Ok(())
            })?;

            ncursesw::doupdate()?;

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
