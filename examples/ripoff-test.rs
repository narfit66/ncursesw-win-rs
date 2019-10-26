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

    ncursesw_init(|ncurses| {
        if let Err(e) = test_ripofflines(&ncurses.initial_window(), &top_ripoff, &bottom_ripoff) {
            panic!(e.to_string())
        }
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("A Panic Occurred: {}", errmsg),
        None         => println!("There was an error, but no error message."),
    });

    Ok(())
}

fn test_ripofflines(initial_window: &Window, top_ripoff: &RipoffLine, bottom_ripoff: &RipoffLine) -> result!(()) {
    curs_set(CursorType::Invisible)?;
    set_echo(false)?;

    let mut origin = Origin { y: 1, x: 0};

    initial_window.mvaddstr(origin, "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.")?;
    origin.y += 1;
    initial_window.mvaddstr(origin, "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.")?;

    initial_window.refresh()?;

    //  update the top ripoff line.
    top_ripoff.update(|ripoff_window, columns| -> result!(()) {
        ripoff_window.addstr(&format!("this is the ripoff line at the top of the screen with a maximum of {} columns", columns))?;
        ripoff_window.noutrefresh()?;

        Ok(())
    })?;

    //  update the bottom ripoff line.
    bottom_ripoff.update(|ripoff_window, columns| -> result!(()) {
        ripoff_window.addstr(&format!("this is the ripoff line at the bottom of the screen with a maximum of {} columns", columns))?;
        ripoff_window.noutrefresh()?;

        Ok(())
    })?;

    ncursesw::doupdate()?;

    initial_window.getch()?;

    Ok(())
}
