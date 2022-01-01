/*
    src/ncurses/ncurses.rs

    Copyright (c) 2019-2021 Stephen Whittle  All rights reserved.

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

use std::sync::atomic::Ordering;
use anyhow::Result;
use crate::{Window, gen::HasHandle, ncurses::{INITSCR_CALLED, COLOR_STARTED}};

// NCurses context.
pub(in crate::ncurses) struct NCurses {
    handle: ncursesw::WINDOW
}

// NCurses context, initialise and when out of scope drop ncurses structure.
impl NCurses {
    // Initialise ncurses.
    pub fn new() -> Result<Self> {
        if !INITSCR_CALLED.load(Ordering::SeqCst) {
            let handle = ncursesw::initscr()?;

            assert!(!handle.is_null(), "NCurses::new() : handle.is_null()");

            COLOR_STARTED.store(false, Ordering::SeqCst);
            INITSCR_CALLED.store(true, Ordering::SeqCst);

            Ok(Self { handle })
        } else {
            panic!("NCurses already initialised!!!")
        }
    }

    // Returns the initial window(stdscr) after initialisation.
    pub fn stdscr(&self) -> Window {
        Window::_from(None, self.handle, true)
    }
}

impl Drop for NCurses {
    // Unallocate the initialised ncurses instance.
    fn drop(&mut self) {
        match ncursesw::endwin() {
            Err(source) => panic!("{} @ ({:p})", source, self.handle),
            _           => {
                COLOR_STARTED.store(false, Ordering::SeqCst);
                INITSCR_CALLED.store(false, Ordering::SeqCst);
            }
        }
    }
}
