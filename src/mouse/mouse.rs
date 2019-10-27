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

use std::hash::{Hash, Hasher};

use crate::mouse::mouseevents::MouseEvents;
use crate::mouse::mousemask::MouseMask;
use crate::mouse::mouseorigin::MouseOrigin;
use crate::ncurseswwinerror::NCurseswWinError;
use ncursesw::mouse::{MEVENT, mousemask, getmouse, ungetmouse};

pub struct Mouse {
    handle: MEVENT,
    mask:   MouseMask
}

impl Mouse {
    pub fn new(id: i16, mask: MouseMask) -> result!(Self) {
        mousemask(mask.mask()?, None)?;

        Ok(Self { handle: MEVENT { id, x: 0, y: 0, z: 0, bstate: 0 }, mask })
    }

    pub fn refresh(&mut self) -> result!(bool) {
        let mut handle: [MEVENT; 1] = [self.handle];

        mousemask(self.mask.mask()?, None)?;

        getmouse(handle.as_mut_ptr())?;

        if self.handle.id == handle[0].id {
            self.handle = handle[0];

            Ok(true)
        } else {
            ungetmouse(handle.as_mut_ptr())?;

            Ok(false)
        }
    }

    pub fn push(&mut self) -> result!(()) {
        ungetmouse(&mut self.handle)?;

        Ok(())
    }

    pub fn id(&self) -> i16 {
        self.handle.id
    }

    pub fn origin(&self) -> MouseOrigin {
        MouseOrigin::new(self.handle.y, self.handle.x, self.handle.z)
    }

    pub fn events(&self) -> MouseEvents {
        MouseEvents::new(self.handle.bstate)
    }

    pub fn mask(&self) -> MouseMask {
        self.mask
    }

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
