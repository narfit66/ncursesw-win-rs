extern crate ncurseswwin;

use ncurseswwin::*;

fn main() {
    // We wrap all our use of ncurseswin with this function.
    ncursesw_init(|ncurses| {
        fn doit(initial_window: &Window) -> Result<(), NCurseswError> {
            curs_set(CursorType::Visible)?;
            set_echo(false)?;
            initial_window.keypad(true)?;

            let mut origin = Origin { y: 1, x: 1 };

            initial_window.mvaddstr(origin, &format!("Mouse Version : {} ", mouse_version()))?;
            origin.y += 2;
            initial_window.mvaddstr(origin, "Hit <Return> to continue : ")?;

            initial_window.getch()?;

            curs_set(CursorType::Invisible)?;

            if !has_mouse() {
                panic!("no mouse detected!!!");
            }

            let mouse = &mut Mouse::new(0, MouseMask::AllMouseEvents);

            loop {
                match initial_window.getch()? {
                    CharacterResult::Key(kb)      => {
                        match kb {
                            KeyBinding::MouseEvent => {
                                initial_window.clear()?;

                                if mouse.refresh()? {
                                    if mouse.events().button_1_released() {
                                        initial_window.mvaddstr(origin, "B1 released")?;
                                    } else if mouse.events().button_1_pressed() {
                                        initial_window.mvaddstr(origin, "B1 pressed")?;
                                    } else if mouse.events().button_1_clicked() {
                                        initial_window.mvaddstr(origin, "B1 clicked")?;
                                    } else if mouse.events().button_1_double_clicked() {
                                        initial_window.mvaddstr(origin, "B1 double clicked")?;
                                    } else if mouse.events().button_1_triple_clicked() {
                                        initial_window.mvaddstr(origin, "B1 triple clicked")?;
                                    } else if mouse.events().button_2_released() {
                                        initial_window.mvaddstr(origin, "B2 released")?;
                                    } else if mouse.events().button_2_pressed() {
                                        initial_window.mvaddstr(origin, "B2 pressed")?;
                                    } else if mouse.events().button_2_clicked() {
                                        initial_window.mvaddstr(origin, "B2 clicked")?;
                                    } else if mouse.events().button_2_double_clicked() {
                                        initial_window.mvaddstr(origin, "B2 double clicked")?;
                                    } else if mouse.events().button_2_triple_clicked() {
                                        initial_window.mvaddstr(origin, "B2 triple clicked")?;
                                    } else if mouse.events().button_3_released() {
                                        initial_window.mvaddstr(origin, "B3 released")?;
                                    } else if mouse.events().button_3_pressed() {
                                        initial_window.mvaddstr(origin, "B3 pressed")?;
                                    } else if mouse.events().button_3_clicked() {
                                        initial_window.mvaddstr(origin, "B3 clicked")?;
                                    } else if mouse.events().button_3_double_clicked() {
                                        initial_window.mvaddstr(origin, "B3 double clicked")?;
                                    } else if mouse.events().button_3_triple_clicked() {
                                        initial_window.mvaddstr(origin, "B3 triple clicked")?;
                                    }
                                }
                            }
                            _                      => { }
                        }
                    },
                    CharacterResult::Character(c) => {
                        if c == 'q' || c == 'Q' {
                            return Ok(())
                        }
                    }
                }
            }
        }

        // In here we get an initialized NCurses window(stdscr) and then proceed
        // to use it exactly like we normally would use it.
        panic!(match doit(&ncurses.initial_window()) {
            Err(e) => e.to_string(),
            Ok(_)  => "this is the end my friend, the only end!!!".to_string()
        })
    }).unwrap_or_else(|e| match e {
        // This block only runs if there was an error. We might or might not
        // have been able to recover an error message. You technically can pass
        // any value into a panic, but we only get an error message if the panic
        // value was a `String` or `&str`.
        Some(errmsg) => println!("A Panic Occurred: {}", errmsg),
        None         => println!("There was an error, but no error message."),
    });
}
