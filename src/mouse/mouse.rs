/*
    src/mouse/mouse.rs

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
    fmt, i16, hash::{Hash, Hasher}, convert::TryFrom,
    sync::Mutex, collections::HashSet
};
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

lazy_static! {
    static ref MOUSEIDS: Mutex<HashSet<i16>> = Mutex::new(HashSet::new());
}

/// A mouse pointer device.
pub struct Mouse {
    screen:  Option<SCREEN>,
    mevent:  MEVENT,
    mask:    MouseMask,
    returns: MouseEvents
}

impl Mouse {
    /// Create a new instance of a mouse pointer.
    pub fn new(mask: MouseMask) -> result!(Self) {
        Ok(Self {
            screen: None,
            mevent: default_mevent()?,
            mask,
            returns: MouseEvents::new(mousemask(mask.mask()?)?)
        })
    }

    pub fn new_sp(screen: &Screen, mask: MouseMask) -> result!(Self) {
        Ok(Self {
            screen: Some(screen._handle()),
            mevent: default_mevent()?,
            mask,
            returns: MouseEvents::new(mousemask_sp(screen._handle(), mask.mask()?)?)
        })
    }

    /// The screen associated with the mouse.
    pub fn screen(&self) -> Option<Screen> {
        self.screen.map(|screen| Screen::_from(screen, false))
    }

    /// Refresh the mouse pointer's events, `self.events()`.
    /// Return `true` if the event on the fifo-queue is for this mouse,
    /// if not then a `false` and the event is pushed back onto the mouse fifo-queue.
    pub fn refresh(&mut self) -> result!(bool) {
        let mut mevent: [MEVENT; 1] = [self.mevent];

        let screen = if let Some(screen) = self.screen {
            mousemask_sp(screen, self.mask.mask()?)?;
            getmouse_sp(screen, mevent.as_mut_ptr())?;

            screen
        } else {
            mousemask(self.mask.mask()?)?;
            getmouse(mevent.as_mut_ptr())?;

            std::ptr::null_mut()
        };

        // check if the event is for this mouse, if not then
        // push the event back onto the mouse fifo-queue.
        let rc = if self.mevent.id == mevent[0].id {
            self.mevent = mevent[0];

            true
        } else {
            if self.screen.is_some() {
                ungetmouse_sp(screen, mevent.as_mut_ptr())?;
            } else {
                ungetmouse(mevent.as_mut_ptr())?;
            }

            false
        };

        Ok(rc)
    }

    /// Push the current mouse event back onto the mouse-fifo queue.
    pub fn push(&mut self) -> result!(()) {
        if let Some(screen) = self.screen {
            ungetmouse_sp(screen, &mut self.mevent)?
        } else {
            ungetmouse(&mut self.mevent)?
        }

        Ok(())
    }

    /// The id of the mouse.
    pub fn id(&self) -> i16 {
        self.mevent.id
    }

    /// The last reported mouse origin for this mouse.
    pub fn origin(&self) -> result!(MouseOrigin) {
        Ok(MouseOrigin::new(u16::try_from(self.mevent.y)?, u16::try_from(self.mevent.x)?, u16::try_from(self.mevent.z)?))
    }

    /// The type of events that the mouse can report on.
    pub fn returns(&self) -> MouseEvents {
        self.returns
    }

    /// The last reported mouse events for this mouse.
    pub fn events(&self) -> MouseEvents {
        MouseEvents::new(self.mevent.bstate)
    }

    /// The mouse mask currently used for this mouse.
    pub fn mask(&self) -> MouseMask {
        self.mask
    }

    /// Set the mouse mask. Returns the previous mouse mask.
    pub fn set_mask(&mut self, mask: MouseMask) -> MouseMask {
        let old_mask = self.mask;

        self.mask = mask;

        old_mask
    }
}

impl Drop for Mouse {
    fn drop(&mut self) {
        if !MOUSEIDS
            .lock()
            .unwrap_or_else(|_| panic!("Mouse::drop() : MOUSEIDS.lock() failed!!!"))
            .remove(&self.id())
        {
            panic!("Mouse::drop() : MOUSEIDS.lock().remove({}) failed!!!", self.id());
        }
    }
}

unsafe impl Send for Mouse { } // too make thread safe
unsafe impl Sync for Mouse { } // too make thread safe

impl PartialEq for Mouse {
    fn eq(&self, rhs: &Self) -> bool {
        self.mevent.id == rhs.mevent.id
    }
}

impl Eq for Mouse { }

impl Hash for Mouse {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mevent.id.hash(state);
    }
}

impl AsRef<Mouse> for Mouse {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Mouse> for Mouse {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl fmt::Debug for Mouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mouse {{ screen: {:?}, mevent: {:?}, mask: {:?} }}", self.screen, self.mevent, self.mask)
    }
}

// Create a default MEVENT.
fn default_mevent() -> result!(MEVENT) {
    Ok(MEVENT { id: obtain_mouse_id()?, x: 0, y: 0, z: 0, bstate: 0 })
}

// Obtain a free mouse id.
fn obtain_mouse_id() -> result!(i16) {
    let mut mouse_ids = MOUSEIDS
        .lock()
        .unwrap_or_else(|_| panic!("obtain_mouse_id() : MOUSEIDS.lock() failed!!!"));

    for id in 0..i16::MAX {
        if mouse_ids.insert(id) {
            return Ok(id);
        }
    }

    Err(NCurseswWinError::MouseId)
}
