/*
    src/mouse/mouse.rs

    Copyright (c) 2019, 2020 Stephen Whittle  All rights reserved.

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

use std::{fmt, hash::{Hash, Hasher}, convert::TryFrom};

use ncursesw::{
    SCREEN,
    mouse::{
        MEVENT, mousemask, getmouse, ungetmouse,
        mousemask_sp, getmouse_sp, ungetmouse_sp
    }
};
use crate::{
    Screen, MouseMask, NCurseswWinError,
    mouse::MouseOrigin, mouse::MouseEvents
};

/// A mouse pointer device.
pub struct Mouse {
    screen:    Option<SCREEN>,
    handle:    MEVENT,
    mask:      MouseMask,
    reporting: MouseEvents
}

impl Mouse {
    /// Create a new instance of a mouse pointer.
    pub fn new(id: i16, mask: MouseMask) -> result!(Self) {
        let reporting = MouseEvents::new(mousemask(mask.mask()?)?);

        Ok(Self { screen: None, handle: default_mevent(id), mask, reporting })
    }

    pub fn new_sp(screen: Screen, id: i16, mask: MouseMask) -> result!(Self) {
        let reporting = MouseEvents::new(mousemask_sp(screen._handle(), mask.mask()?)?);

        Ok(Self { screen: Some(screen._handle()), handle: default_mevent(id), mask, reporting })
    }

    pub fn screen(&self) -> Option<Screen> {
        if let Some(screen) = self.screen {
            Some(Screen::_from(screen, false))
        } else {
            None
        }
    }

    /// Refresh the mouse pointer's events.
    /// Return `true` if the event on the fifo-queue is for this mouse,
    /// if not then a `false` and the event is pushed back onto the mouse fifo-queue.
    pub fn refresh(&mut self) -> result!(bool) {
        let mut handle: [MEVENT; 1] = [self.handle];

        let screen = if let Some(screen) = self.screen {
            mousemask_sp(screen, self.mask.mask()?)?;
            getmouse_sp(screen, handle.as_mut_ptr())?;

            screen
        } else {
            mousemask(self.mask.mask()?)?;
            getmouse(handle.as_mut_ptr())?;

            std::ptr::null_mut()
        };

        // check if the event is for this mouse, if not then
        // push the event back onto the mouse fifo-queue.
        let rc = if self.handle.id == handle[0].id {
            self.handle = handle[0];

            true
        } else {
            if self.screen.is_some() {
                ungetmouse_sp(screen, handle.as_mut_ptr())?;
            } else {
                ungetmouse(handle.as_mut_ptr())?;
            }

            false
        };

        Ok(rc)
    }

    /// Push the current mouse event back onto the mouse-fifo queue.
    pub fn push(&mut self) -> result!(()) {
        if let Some(screen) = self.screen {
            ungetmouse_sp(screen, &mut self.handle)?
        } else {
            ungetmouse(&mut self.handle)?
        }

        Ok(())
    }

    /// The id of the mouse.
    pub fn id(&self) -> i16 {
        self.handle.id
    }

    /// The last reported mouse origin for this mouse.
    pub fn origin(&self) -> result!(MouseOrigin) {
        Ok(MouseOrigin::new(u16::try_from(self.handle.y)?, u16::try_from(self.handle.x)?, u16::try_from(self.handle.z)?))
    }

    /// The type of events that the mouse can report on.
    pub fn reporting(&self) -> MouseEvents {
        self.reporting
    }

    /// The last reported mouse events for this mouse.
    pub fn events(&self) -> MouseEvents {
        MouseEvents::new(self.handle.bstate)
    }

    /// The mouse mask currently used for this mouse.
    pub fn mask(&self) -> MouseMask {
        self.mask
    }

    /// Set the mouse mask. Returns the previous mpuse mask.
    pub fn set_mask(&mut self, mask: MouseMask) -> MouseMask {
        let old_mask = self.mask;

        self.mask = mask;

        old_mask
    }
}

unsafe impl Send for Mouse { } // too make thread safe
unsafe impl Sync for Mouse { } // too make thread safe

impl PartialEq for Mouse {
    fn eq(&self, rhs: &Self) -> bool {
        self.handle.id == rhs.handle.id
    }
}

impl Eq for Mouse { }

impl Hash for Mouse {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.id.hash(state);
    }
}

impl fmt::Debug for Mouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mouse {{ screen: {:?}, handle: {:?}, mask: {:?} }}", self.screen, self.handle, self.mask)
    }
}

fn default_mevent(id: i16) -> MEVENT {
    MEVENT { id, x: 0, y: 0, z: 0, bstate: 0 }
}
