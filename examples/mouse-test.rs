extern crate ascii;
extern crate ncurseswwin;

use ascii::AsciiChar;
use ncurseswwin::*;

fn main() {
    // We wrap all our use of ncurseswin with this function.
    ncursesw_init(|ncurses| {
        // In here we get an initialized NCurses window(stdscr) and then proceed
        // to use it exactly like we normally would use it.
        panic!(match mouse_test(&ncurses.initial_window()) {
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

fn mouse_test(window: &Window) -> Result<(), NCurseswError> {
    curs_set(CursorType::Visible)?;
    set_echo(false)?;

    window.keypad(true)?;

    let mut origin = Origin { y: 1, x: 1 };

    window.mvaddstr(origin, &format!("Mouse Version : {} ", mouse_version()))?;
    origin.y += 2;
    window.mvaddstr(origin, "Hit <Return> to continue : ")?;
    origin.y += 2;

    window.getch()?;

    curs_set(CursorType::Invisible)?;

    /*if !has_mouse() { // TODO: this doesn't seem to be returning true, $TERM=xterm-256color
        panic!("no mouse detected!!!");
    }*/

    let mouse = &mut Mouse::new(0, MouseMask::AllMouseEvents)?;

    loop {
        match window.getch()? {
            CharacterResult::Key(kb)      => {
                match kb {
                    KeyBinding::MouseEvent => {
                        if let Ok(registered_mouse) = mouse.refresh() {  // get the mouse event
                            if registered_mouse {                        // is the mouse event for our mouse
                                let mouse_events = mouse.events();

                                if mouse_events.button_1_released() {
                                    mouse_button_event(window, origin, 1, "released", mouse.origin())?;
                                } else if mouse_events.button_1_pressed() {
                                    mouse_button_event(window, origin, 1, "pressed", mouse.origin())?;
                                } else if mouse_events.button_1_clicked() {
                                    mouse_button_event(window, origin, 1, "clicked", mouse.origin())?;
                                } else if mouse_events.button_1_double_clicked() {
                                    mouse_button_event(window, origin, 1, "double clicked", mouse.origin())?;
                                } else if mouse_events.button_1_triple_clicked() {
                                    mouse_button_event(window, origin, 1, "triple clicked", mouse.origin())?;
                                } else if mouse_events.button_2_released() {
                                    mouse_button_event(window, origin, 2, "released", mouse.origin())?;
                                } else if mouse_events.button_2_pressed() {
                                    mouse_button_event(window, origin, 2, "pressed", mouse.origin())?;
                                } else if mouse_events.button_2_clicked() {
                                    mouse_button_event(window, origin, 2, "clicked", mouse.origin())?;
                                } else if mouse_events.button_2_double_clicked() {
                                    mouse_button_event(window, origin, 2, "double clicked", mouse.origin())?;
                                } else if mouse_events.button_2_triple_clicked() {
                                    mouse_button_event(window, origin, 2, "triple clicked", mouse.origin())?;
                                } else if mouse_events.button_3_released() {
                                    mouse_button_event(window, origin, 3, "released", mouse.origin())?;
                                } else if mouse_events.button_3_pressed() {
                                    mouse_button_event(window, origin, 3, "pressed", mouse.origin())?;
                                } else if mouse_events.button_3_clicked() {
                                    mouse_button_event(window, origin, 3, "clicked", mouse.origin())?;
                                } else if mouse_events.button_3_double_clicked() {
                                    mouse_button_event(window, origin, 3, "double clicked", mouse.origin())?;
                                } else if mouse_events.button_3_triple_clicked() {
                                    mouse_button_event(window, origin, 3, "triple clicked", mouse.origin())?;
                                } else if mouse_events.button_4_released() {
                                    mouse_button_event(window, origin, 4, "released", mouse.origin())?;
                                } else if mouse_events.button_4_pressed() {
                                    mouse_button_event(window, origin, 4, "pressed", mouse.origin())?;
                                } else if mouse_events.button_4_clicked() {
                                    mouse_button_event(window, origin, 4, "clicked", mouse.origin())?;
                                } else if mouse_events.button_4_double_clicked() {
                                    mouse_button_event(window, origin, 4, "double clicked", mouse.origin())?;
                                } else if mouse_events.button_4_triple_clicked() {
                                    mouse_button_event(window, origin, 4, "triple clicked", mouse.origin())?;
                                } else if mouse_events.button_5_released() {
                                    mouse_button_event(window, origin, 5, "released", mouse.origin())?;
                                } else if mouse_events.button_5_pressed() {
                                    mouse_button_event(window, origin, 5, "pressed", mouse.origin())?;
                                } else if mouse_events.button_5_clicked() {
                                    mouse_button_event(window, origin, 5, "clicked", mouse.origin())?;
                                } else if mouse_events.button_5_double_clicked() {
                                    mouse_button_event(window, origin, 5, "double clicked", mouse.origin())?;
                                } else if mouse_events.button_5_triple_clicked() {
                                    mouse_button_event(window, origin, 5, "triple clicked", mouse.origin())?;
                                }

                                let old_origin = origin;

                                origin.y += 1;

                                if mouse_events.button_ctrl() {
                                    window.mvaddstr(origin, "with <ctrl> pressed")?;
                                } else if mouse_events.button_shift() {
                                    window.mvaddstr(origin, "with <shift> pressed")?;
                                } else if mouse_events.button_alt() {
                                    window.mvaddstr(origin, "with <alt> pressed")?;
                                }

                                origin = old_origin;
                            }
                        }
                    },
                    _                      => window.mvaddstr(origin, &format!("{:?}", kb))?
                }
            },
            CharacterResult::Character(c) => {
                clear_to_eol(window, origin)?;

                window.mvaddstr(origin, &format!("{}", c))?;

                if c == 'q' || c == 'Q' {
                    break;
                }
            }
        }
    }

    Ok(())
}

fn mouse_button_event(window: &Window, origin: Origin, button: u8, str: &str, mouse_origin: MouseOrigin) -> Result<(), NCurseswError> {
    clear_to_eol(window, origin)?;

    window.mvaddstr(origin, &format!("B{} {} @ {}", button, str, mouse_origin))?;

    window.mvaddch(mouse_origin.origin(), ChtypeChar::new(AsciiChar::Asterisk))
}

fn clear_to_eol(window: &Window, origin: Origin) -> Result<(), NCurseswError> {
    window.set_cursor(origin)?;
    window.clrtoeol()
}
