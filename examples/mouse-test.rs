extern crate ascii;
extern crate ncurseswwin;

use ascii::AsciiChar;
use ncurseswwin::*;
use strum::IntoEnumIterator;

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

    let next_origin = Origin { y: origin.y + 1, x: origin.x };

    window.getch()?;

    curs_set(CursorType::Invisible)?;

    if !has_mouse_interface() {  // check if ncursesw supports a mouse pointer
        panic!("no mouse interface detected!!!");
    }

    let mouse = &mut Mouse::new(0, MouseMask::AllMouseEvents)?;

    if !has_mouse() {            // has a mouse pointer been defined.
        panic!("no mouse detected!!!");
    }

    loop {
        match window.getch()? {
            CharacterResult::Key(kb)      => {
                match kb {
                    KeyBinding::MouseEvent => {
                        if let Ok(registered_mouse) = mouse.refresh() {  // get the mouse event
                            if registered_mouse {                        // is the mouse event for our mouse
                                let mouse_events = mouse.events();

                                for button in MouseButton::iter() {
                                    for event in MouseButtonEvent::iter() {
                                        if mouse_events.button_state(button, event) {
                                            mouse_button_event(window, origin, button.into(), &format!("{}", event), mouse.origin())?;
                                        }
                                    }
                                }

                                if mouse_events.ctrl_button() {
                                    other_event(window, next_origin, "with <ctrl> pressed")?;
                                } else if mouse_events.shift_button() {
                                    other_event(window, next_origin, "with <shift> pressed")?;
                                } else if mouse_events.alt_button() {
                                    other_event(window, next_origin, "with <alt> pressed")?;
                                }
                            }
                        }
                    },
                    _                      => other_event(window, origin, &format!("{:?}", kb))?
                }
            },
            CharacterResult::Character(c) => {
                other_event(window, origin, &format!("{}", c))?;

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

fn other_event(window: &Window, origin: Origin, str: &str) -> Result<(), NCurseswError> {
    clear_to_eol(window, origin)?;

    window.mvaddstr(origin, str)
}

fn clear_to_eol(window: &Window, origin: Origin) -> Result<(), NCurseswError> {
    window.set_cursor(origin)?;
    window.clrtoeol()
}
