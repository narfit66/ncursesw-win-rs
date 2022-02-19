/*
    src/ripoff/ripoffline.rs

    Copyright (c) 2019-2022 Stephen Whittle  All rights reserved.

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

use std::{
    fmt, convert::TryFrom, sync::{Mutex, atomic::{AtomicUsize, Ordering}},
    hash::{Hash, Hasher}
};
use ncursesw::{SCREEN, WINDOW, Orientation, shims::constants};
use crate::{
    Screen, RipoffWindow, NCurseswWinError, HasHandle,
    ncurses::INITSCR_CALLED
};

pub(in crate) const MAX_RIPOFF_LINES: usize = 5; // The maximum number of ripoff lines ncurses allows.

lazy_static! {
    static ref RIPOFFCOUNT:       AtomicUsize = AtomicUsize::new(0);
    static ref RIPOFFINITCOUNT:   AtomicUsize = AtomicUsize::new(0);
    static ref RIPOFFLINESCREENS: Mutex<Vec<Option<Screen>>> = Mutex::new(Vec::with_capacity(MAX_RIPOFF_LINES));
    static ref RIPOFFLINES:       Mutex<Vec<(RipoffWindow, i32)>> = Mutex::new(Vec::with_capacity(MAX_RIPOFF_LINES));
}

#[no_mangle]
// Our ripoff call-back function that will be called via NCurses `initscr()`, a maximum of 5
// rippoff line's can be defined and call by `initscr()`, they are called in the order they
// are defined by `RipoffLine::new()` and/or `RipoffLine::new_sp()`. With each call that
// `initscr()` makes to this function a new `WINDOW` is created and passed along with the
// number of columns that the terminal/screen supports.
extern fn ripoff_init(window: WINDOW, columns: i32) -> i32 {
    // Get the number of times `initscr()` has called (base 0).
    let number = RIPOFFINITCOUNT.fetch_add(1, Ordering::SeqCst);

    // Get the screen associated with this ripoff line.
    let screen = &RIPOFFLINESCREENS
        .lock()
        .unwrap_or_else(|_| panic!("ripoff_init() : &RIPOFFLINESCREENS.lock()[{}] failed!!!", number))[number];

    // Save the window and columns assigned by `initscr()`.
    RIPOFFLINES
        .lock()
        .unwrap_or_else(|_| panic!("ripoff_init() : RIPOFFLINES.lock() failed!!!"))
        .insert(number, (RipoffWindow::_from(screen.as_ref().map(|screen| screen._handle()), window, false), columns));

    constants::OK
}

/// A ripoff line.
pub struct RipoffLine {
    screen:      Option<SCREEN>,
    orientation: Orientation,
    number:      usize
}

impl RipoffLine {
    fn _from(screen: Option<SCREEN>, orientation: Orientation, number: usize) -> Self {
        assert!(screen.map_or_else(|| true, |screen| !screen.is_null()), "RipoffLine::_from() : screen.is_null()");

        Self { screen, orientation, number }
    }
}

impl RipoffLine {
    /// Create a new instance of a RipoffLine (NCurses allows for a maximum of 5 ripoff lines).
    pub fn new(orientation: Orientation) -> result!(Self) {
        let number = create_ripoff_line(None, orientation)?;

        Ok(Self::_from(None, orientation, number))
    }

    /// Create a new instance of a RipoffLine for a Screen (NCurses allows for a maximum of 5 ripoff lines).
    pub fn new_sp(screen: &Screen, orientation: Orientation) -> result!(Self) {
        let number = create_ripoff_line(Some(screen._handle()), orientation)?;

        Ok(Self::_from(Some(screen._handle()), orientation, number))
    }

    /// Returns the screen associated with the ripoff line.
    pub fn screen(&self) -> Option<Screen> {
        self.screen.map(|screen| Screen::_from(screen, false))
    }

    /// Returns the orientation of the ripoff line.
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Returns the number of the ripoff line.
    pub fn number(&self) -> usize {
        self.number
    }

    /// Update the ripoff line.
    pub fn update<F: Fn(&RipoffWindow, u16) -> result!(T), T>(&self, func: F) -> result!(T) {
        // Check that `initscr()` has been called.
        if !INITSCR_CALLED.load(Ordering::SeqCst) {
            return Err(NCurseswWinError::InitscrNotCalled)
        }

        // Get the ripoff details and assert that we have a valid ripoff!
        let (ref ripoff_window, ripoff_columns) = &RIPOFFLINES
            .lock()
            .unwrap_or_else(|_| panic!("RipoffLine::update() : RIPOFFLINES.lock()[{}] failed!!!", self.number))[self.number];

        // Call the passed closure to process against the ripoff.
        func(ripoff_window, u16::try_from(*ripoff_columns)?)
    }
}

unsafe impl Send for RipoffLine { } // too make thread safe
unsafe impl Sync for RipoffLine { } // too make thread safe

impl PartialEq for RipoffLine {
    fn eq(&self, rhs: &Self) -> bool {
        self.number == rhs.number
    }
}

impl Eq for RipoffLine { }

impl Hash for RipoffLine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

impl AsRef<RipoffLine> for RipoffLine {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Clone for RipoffLine {
    fn clone(&self) -> Self {
        Self::_from(self.screen.clone(), self.orientation.clone(), self.number.clone())
    }
}

impl fmt::Debug for RipoffLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RipoffLine {{ screen: {:?}, number: {} }}", self.screen, self.number)
    }
}

fn create_ripoff_line(screen: Option<SCREEN>, orientation: Orientation) -> result!(usize) {
    // Return the ripoff callback number (base 0).
    if INITSCR_CALLED.load(Ordering::SeqCst) {
        return Err(NCurseswWinError::InitscrAlreadyCalled)
    }

    let number = RIPOFFCOUNT.fetch_add(1, Ordering::SeqCst);

    if number >= MAX_RIPOFF_LINES {
        return Err(NCurseswWinError::MaximumRipoffLines { number })
    }

    // Call the NCurses ripoff function with one of our pre-defined call-back function.
    // NCurses allows for a maximum of 5 ripoff lines.
    if let Err(source) = screen
        .map_or_else(|| ncursesw::ripoffline(orientation, ripoff_init), |screen| ncursesw::ripoffline_sp(screen, orientation, ripoff_init))
    {
        return Err(NCurseswWinError::NCurseswError { source })
    }

    // Save the screen of the ripoff window so the `ripoff_init` function can initialise
    // it's `RipoffWindow` correctly.
    RIPOFFLINESCREENS
        .lock()
        .unwrap_or_else(|_| panic!("RipoffLine::new_sp() : RIPOFFLINESCREENS.lock() failed!!!"))
        .insert(number, screen.map(|screen| Screen::_from(screen, false)));

    Ok(number)
}
