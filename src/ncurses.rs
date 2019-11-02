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

use std::panic::{UnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, Ordering};

use ncursesw;
use ncursesw::WINDOW;
use crate::window::Window;
use crate::ncurseswwinerror::NCurseswWinError;

lazy_static! {
    pub(crate) static ref INITSCR_CALLED: AtomicBool = AtomicBool::new(false);
    pub(crate) static ref COLOR_STARTED: AtomicBool = AtomicBool::new(false);
}

/// NCurses context.
struct NCurses {
    handle: WINDOW
}

/// NCurses context, initialise and when out of scope drop ncurses structure.
impl NCurses {
    /// Initialise ncurses.
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

    /// Returns the initial window(stdscr) after initialisation.
    fn initial_window(&self) -> Window {
        Window::from(self.handle, true)
    }
}

impl Drop for NCurses {
    /// Unallocate the initialised ncurses instance.
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

/// Safely initialise ncurses, panic will be caught correctly and ncurses unallocated (as best it can) correctly.
pub fn ncursesw_init<F: FnOnce(&Window) -> R + UnwindSafe, R>(user_function: F) -> Result<R, Option<String>> {
    catch_unwind(|| {
        let ncurses = match NCurses::initscr() {
            Err(e)  => {
                panic!(match e {
                    NCurseswWinError::InitscrAlreadyCalled => "NCurses already initialized!",
                    _                                      => "ncursesw::initscr() has failed!."
                })
            },
            Ok(ptr) => ptr
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
