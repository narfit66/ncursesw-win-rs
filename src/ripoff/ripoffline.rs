/*
    src/ripoff/ripoffline.rs

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

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use ncursesw;
use ncursesw::{WINDOW, Orientation};
use crate::ripoff::RipoffWindow;
use crate::ncurses::INITSCR_CALLED;
use crate::ncurseswwinerror::NCurseswWinError;

pub(crate) const MAX_LINES: usize = 5; // The maximum number of ripoff lines ncurses allows.

lazy_static! {
    static ref RIPOFFCOUNT: AtomicUsize = AtomicUsize::new(0);
    static ref RIPOFFLINES: Mutex<Vec<(RipoffWindow, i32)>> = Mutex::new(Vec::with_capacity(MAX_LINES));
}

// the macro for defining the ripoff call backs.
macro_rules! ripoff_init_fn {
    ($f: ident, $n: expr) => {
        #[no_mangle]
        extern fn $f(win: WINDOW, cols: i32) -> i32 {
            RIPOFFLINES
                .lock()
                .unwrap_or_else(|_| panic!("ripoff_init{}() : RIPOFFLINES.lock() failed!", $n))
                .insert($n, (RipoffWindow::from(win), cols));

            ncursesw::shims::constants::OK
        }
    }
}

// our ripoff call-back functions that will be called via ncurses initscr(), each one with store
// the window structure that initscr() will assign to it in the static RIPOFFLINES.
ripoff_init_fn!(ripoff_init0, 0);
ripoff_init_fn!(ripoff_init1, 1);
ripoff_init_fn!(ripoff_init2, 2);
ripoff_init_fn!(ripoff_init3, 3);
ripoff_init_fn!(ripoff_init4, 4);

/// A type of ripoff line.
#[derive(PartialEq, Eq, Hash)]
pub struct RipoffLine {
    number: usize
}

impl RipoffLine {
    /// Create a new instance of a RipoffLine (ncurses allows for a maximum of 5 ripoff lines.
    pub fn new(orientation: Orientation) -> result!(Self) {
        // check that initscr() has not been called.
        if INITSCR_CALLED.load(Ordering::SeqCst) {
            Err(NCurseswWinError::InitscrAlreadyCalled)
        } else {
            // get the ripoff call-back number.
            let number = RIPOFFCOUNT.fetch_add(1, Ordering::SeqCst);

            // call the ncurses ripoff function with one of our pre-defined call-back function.
            // ncurses allows for a maximum of 5 ripoff lines.
            match ncursesw::ripoffline(orientation, match number {
                0 => ripoff_init0,
                1 => ripoff_init1,
                2 => ripoff_init2,
                3 => ripoff_init3,
                4 => ripoff_init4,
                _ => return Err(NCurseswWinError::MaximumRipoffLines { number })
            }) {
                Err(source) => Err(NCurseswWinError::NCurseswError { source }),
                _           => Ok(Self { number })
            }
        }
    }

    /// The number of the ripoff.
    pub fn number(&self) -> usize {
        self.number
    }

    /// Update the ripoff line.
    pub fn update<F: Fn(&RipoffWindow, i32) -> result!(T), T>(&self, user_function: F) -> result!(T) {
        // check that initscr() has been called.
        if !INITSCR_CALLED.load(Ordering::SeqCst) {
            Err(NCurseswWinError::InitscrNotCalled)
        } else {
            // get the ripoff details and assert that we have a valid ripoff!
            let (ripoff_window, ripoff_columns) = &RIPOFFLINES
                .lock()
                .unwrap_or_else(|_| panic!("RipoffLine.update() : RIPOFFLINES.lock() failed!"))[self.number];

            if ripoff_window.handle().is_null() {
                Err(NCurseswWinError::RipoffNotInitialized { number: self.number })
            } else {
                // call the passed closure to process against the ripoff.
                user_function(&ripoff_window, *ripoff_columns)
            }
        }
    }
}

unsafe impl Send for RipoffLine { } // too make thread safe
unsafe impl Sync for RipoffLine { } // too make thread safe
