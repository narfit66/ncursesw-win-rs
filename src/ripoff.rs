/*
    src/ripoff.rs

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
use ncursesw::{WINDOW, Orientation, NCurseswError};
use crate::window::Window;
use crate::ncurses::{INITSCR_CALLED, INITSCR_ALREADY_CALLED, INITSCR_NOT_CALLED};

const MAX_LINES: usize = 5; // The maximum number of ripoff lines ncurses allows.

lazy_static! {
    static ref RIPOFFCOUNT: AtomicUsize = AtomicUsize::new(0);
    static ref RIPOFFLINES: Mutex<Vec<(Window, i32)>> = Mutex::new(Vec::with_capacity(MAX_LINES));
}

// the macro for defining the ripoff call backs.
macro_rules! ripoff_init_fn {
    ($f: ident, $n: expr) => {
        #[no_mangle]
        extern fn $f(win: WINDOW, cols: i32) -> i32 {
            RIPOFFLINES.lock().unwrap().insert($n, (Window::from(win, false), cols));

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

/// Ripoff a line from either the top or the bottom of the screen.
///
/// Returns the ripoff number, a maximum of 5 lines can be ripped.
pub fn ripoffline(orientation: Orientation) -> result!(usize) {
    // check that initscr() has not been called.
    if INITSCR_CALLED.load(Ordering::SeqCst) {
        panic!(INITSCR_ALREADY_CALLED.to_string());
    };

    // get the ripoff call-back number and validate it (ncurses allows for a maximum of 5 ripoff lines).
    let number = RIPOFFCOUNT.fetch_add(1, Ordering::SeqCst);
    assert!(number < MAX_LINES, "define_ripoff_line() : number={} > {}", number, MAX_LINES);

    // call the ncurses ripoff function with one of our pre-defined call-back function.
    match ncursesw::ripoffline(orientation, match number {
        0 => ripoff_init0,
        1 => ripoff_init1,
        2 => ripoff_init2,
        3 => ripoff_init3,
        4 => ripoff_init4,
        _ => unreachable!()
    }) {
        Err(e) => Err(e),
        _      => Ok(number)
    }
}

/// Update the specified ripped off line using the specified closure.
pub fn update_ripoffline<F>(number: usize, func: F) -> result!(()) where F: Fn(&Window, i32) -> result!(()) {
    // check that initscr() has been called.
    if !INITSCR_CALLED.load(Ordering::SeqCst) {
        panic!(INITSCR_NOT_CALLED.to_string());
    };

    // assert the passed ripoff number is valid.
    assert!(number < MAX_LINES, "update_ripoffline() : number={} > {}", number, MAX_LINES);

    // get the ripoff details and assert that we have a valid ripoff!
    let ripoff = &RIPOFFLINES.lock().unwrap()[number];
    assert!(!ripoff.0.handle().is_null(), "update_ripoffline() : ripoff.0.get_handle().is_null()");

    // call the passed closure to process against the ripoff.
    func(&ripoff.0, ripoff.1)
}
