/*
    src/ncurses.rs

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

#![allow(deprecated)]

use std::panic::{UnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, Ordering};

use ncursesw;
use ncursesw::WINDOW;
use crate::{Window, NCurseswWinError};

lazy_static! {
    pub(crate) static ref INITSCR_CALLED: AtomicBool = AtomicBool::new(false);
    pub(crate) static ref COLOR_STARTED: AtomicBool = AtomicBool::new(false);
}

// NCurses context.
struct NCurses {
    handle: WINDOW
}

// NCurses context, initialise and when out of scope drop ncurses structure.
impl NCurses {
    // Initialise ncurses.
    fn initscr() -> result!(Self) {
        if !INITSCR_CALLED.load(Ordering::SeqCst) {

            let handle = ncursesw::initscr()?;

            COLOR_STARTED.store(false, Ordering::SeqCst);
            INITSCR_CALLED.store(true, Ordering::SeqCst);

            Ok(Self { handle })
        } else {
            Err(NCurseswWinError::InitscrAlreadyCalled)
        }
    }

    // Returns the initial window(stdscr) after initialisation.
    fn initial_window(&self) -> Window {
        Window::from(self.handle, true)
    }
}

impl Drop for NCurses {
    // Unallocate the initialised ncurses instance.
    fn drop(&mut self) {
        match ncursesw::endwin() {
            Err(e) => panic!(e.to_string()),
            _      => {
                COLOR_STARTED.store(false, Ordering::SeqCst);
                INITSCR_CALLED.store(false, Ordering::SeqCst);
            }
        }
    }
}

/// Safely initialise ncurses, panic's will be caught correctly and
/// passed back as `NCurseswWinError::Panic`.
/// ncurses should free (as best it can) memory etc correctly.
pub fn ncursesw_entry<F: FnOnce(&Window) -> result!(T) + UnwindSafe, T>(user_function: F) -> result!(T) {
    // We wrap all our use of ncurseswin with this function.
    match ncursesw_init(|window| {
        // In here we get an initialized Window structure (stdscr) and pass that
        // to our closure, `catch_unwind()` as called in `ncursesw_init()` will
        // return a `Result` of Ok so we will wrap our return of `user_function()`
        // in that.
        match user_function(&window) {
            Err(source) => Ok(Err(source)),
            Ok(value)   => Ok(Ok(value))
        }
    }).unwrap_or_else(|e| Err(match e {
        // This block only runs if there was an error. We might or might not
        // have been able to recover an error message. You technically can pass
        // any value into a panic, but we only get an error message if the panic
        // value was a `String` or `&str`.
        Some(message) => NCurseswWinError::Panic { message },
        None          => NCurseswWinError::Panic { message: "There was a panic, but no message!".to_string() }
    })) {
        // The Err branch matches against the NCurseswWinError::Panic error
        // from the above unwrap_or_else()
        Err(source) => Err(source),
        // The Ok branch unwraps and matches against ncursesw_init_test error
        // or return value
        Ok(result)  => result
    }
}

#[deprecated(since = "0.3.0", note = "Use ncursesw_entry() instead")]
/// Safely initialise ncurses, panic will be caught correctly and ncurses unallocated (as best it can) correctly.
pub fn ncursesw_init<F: FnOnce(&Window) -> R + UnwindSafe, R>(user_function: F) -> Result<R, Option<String>> {
    // use `catch_unwind()` to catch panic's, an error will be a panic
    // so try and convert it into a string.
    catch_unwind(|| {
        // initilise ncurses.
        let ncurses = match NCurses::initscr() {
            Err(source)  => panic!(match source {
                NCurseswWinError::InitscrAlreadyCalled => "NCurses already initialized!",
                _                                      => "ncursesw::initscr() has failed!"
            }),
            Ok(ptr)      => ptr
        };

        user_function(&ncurses.initial_window())
    }).map_err(|e| match e.downcast_ref::<&str>() {
        Some(andstr) => Some(andstr.to_string()),
        None         => match e.downcast_ref::<String>() {
            Some(string) => Some(string.to_string()),
            None         => None
        }
    })
}
