/*
    src/mouse/mouse.rs

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

use std::{fmt, hash::{Hash, Hasher}, convert::TryFrom};

use ncursesw::mouse::{MEVENT, mousemask, getmouse, ungetmouse};
use crate::{MouseMask, NCurseswWinError, mouse::MouseOrigin, mouse::MouseEvents};

/// A mouse pointer device.
pub struct Mouse {
    handle: MEVENT,
    mask:   MouseMask
}

impl Mouse {
    /// Create a new instance of a mouse pointer.
    pub fn new(id: i16, mask: MouseMask) -> result!(Self) {
        mousemask(mask.mask()?, None)?;

        Ok(Self { handle: MEVENT { id, x: 0, y: 0, z: 0, bstate: 0 }, mask })
    }

    /// Refresh the mouse pointer's events.
    /// Return `true` if the event on the fifo-queue is for this mouse,
    /// if not then a `false` and the event is pushed back onto the mouse fifo-queue.
    pub fn refresh(&mut self) -> result!(bool) {
        let mut handle: [MEVENT; 1] = [self.handle];

        // set the mouse mask for the mouse.
        mousemask(self.mask.mask()?, None)?;

        // get an event.
        getmouse(handle.as_mut_ptr())?;

        // check if the event is for this mouse, if not then
        // push the event back onto the mouse fifo-queue.
        Ok(if self.handle.id == handle[0].id {
            self.handle = handle[0];

            true
        } else {
            ungetmouse(handle.as_mut_ptr())?;

            false
        })
    }

    /// Push the current mouse event back onto the mouse-fifo queue.
    pub fn push(&mut self) -> result!(()) {
        Ok(ungetmouse(&mut self.handle)?)
    }

    /// The id of the mouse.
    pub fn id(&self) -> i16 {
        self.handle.id
    }

    /// The last reported mouse origin for this mouse.
    pub fn origin(&self) -> result!(MouseOrigin) {
        Ok(MouseOrigin::new(u16::try_from(self.handle.y)?, u16::try_from(self.handle.x)?, u16::try_from(self.handle.z)?))
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
        write!(f, "Mouse {{ handle: {:?}, mask: {:?} }}", self.handle, self.mask)
    }
}
