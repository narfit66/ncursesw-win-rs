/*
    src/panels/panel.rs

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

use std::{ptr, convert::TryInto};

use ncursesw::{panels, panels::PANEL};
use crate::{Origin, Window, NCurseswWinError, gen::HasHandle, panels::funcs};

/// A moveable panel that is a container for a `Window`.
pub struct Panel {
    handle:       PANEL,
    free_on_drop: bool
}

impl Panel {
    // make a new instance from the passed panel structure pointer and specify
    // if the handle is to be free'd when the structure is dropped.
    //
    // free_on_drop is false in call's such as panel_above(&self) where we are
    // 'peeking' the Panel but it would be invalid to free the handle when
    // our instance goes out of scope.
    pub(in crate::panels) fn _from(handle: PANEL, free_on_drop: bool) -> Self {
        assert!(!handle.is_null(), "Panel::_from() : handle.is_null()");

        Self { handle, free_on_drop }
    }

    pub(in crate::panels) fn _handle(&self) -> PANEL {
        self.handle
    }
}

impl Panel {
    /// Create a new Panel instance with it's associated Window.
    pub fn new(window: &Window) -> result!(Self) {
        Ok(Self::_from(panels::new_panel(window._handle())?, true))
    }

    #[deprecated(since = "0.3.1", note = "Use Panel::new() instead")]
    /// Create a new Panel instance with it's associated Window.
    pub fn new_panel(window: &Window) -> result!(Self) {
        Panel::new(window)
    }

    /// Puts panel at the bottom of all panels.
    pub fn bottom_panel(&self) -> result!(()) {
        Ok(panels::bottom_panel(self.handle)?)
    }

    /// Puts the given visible panel on top of all panels in the stack.
    pub fn top_panel(&self) -> result!(()) {
        Ok(panels::top_panel(self.handle)?)
    }

    /// Makes a hidden panel visible by placing it on top of the panels in the panel stack.
    pub fn show_panel(&self) -> result!(()) {
        Ok(panels::show_panel(self.handle)?)
    }

    /// Removes the given panel from the panel stack and thus hides it from view.
    ///
    /// The Panel is not lost, merely removed from the stack.
    pub fn hide_panel(&self) -> result!(()) {
        Ok(panels::hide_panel(self.handle)?)
    }

    /// Returns the window of the given panel.
    pub fn panel_window(&self) -> result!(Window) {
        Ok(Window::_from(panels::panel_window(self.handle)?, false))
    }

    /// Replaces the current window of panel with window.
    ///
    /// Useful, for example if you want to resize a panel; if you're using ncurses, you can
    /// call replace_panel on the output of wresize(3x)). It does not change the position of the panel in the stack.
    pub fn replace_panel(&self, window: &Window) -> result!(()) {
        Ok(panels::replace_panel(self.handle, window._handle())?)
    }

    /// Moves the given panel window so that its upper-left corner is at origin.y, origin.x.
    ///
    /// It does not change the position of the panel in the stack. Be sure to use this function, not mvwin(), to move a panel window.
    pub fn move_panel(&self, origin: Origin) -> result!(()) {
        Ok(panels::move_panel(self.handle, origin.try_into()?)?)
    }

    /// Returns true if the panel is in the panel stack, false if it is not.
    pub fn panel_hidden(&self) -> result!(bool) {
        Ok(panels::panel_hidden(self.handle)?)
    }

    /// Returns the panel above panel.
    pub fn panel_above(&self) -> result!(Self) {
        funcs::panel_above(Some(self))
    }

    /// Returns the panel just below panel.
    pub fn panel_below(&self) -> result!(Self) {
        funcs::panel_below(Some(self))
    }

    /// Sets the panel's user pointer to the passed `Panel`.
    pub fn set_panel_userptr<T>(&self, ptr: Option<Box<&T>>) -> result!(()) {
        Ok(panels::set_panel_userptr(self.handle, match ptr {
            Some(ptr) => Some(Box::into_raw(ptr) as *const libc::c_void),
            None      => None
        })?)
    }

    /// Returns the user pointers `Panel` for the given panel.
    pub fn panel_userptr<T>(&self) -> Option<Box<T>> {
        match panels::panel_userptr(self.handle) {
            Some(ptr) => Some(unsafe { Box::from_raw(ptr as *mut T) }),
            None      => None
        }
    }
}

impl Drop for Panel {
    fn drop(&mut self) {
        if self.free_on_drop {
            if let Err(source) = panels::del_panel(self.handle) {
                panic!("{} @ ({:p})", source, self.handle)
            }
        }
    }
}

unsafe impl Send for Panel { } // too make thread safe
unsafe impl Sync for Panel { } // too make thread safe

impl PartialEq for Panel {
    fn eq(&self, rhs: &Self) -> bool {
        ptr::eq(self.handle, rhs.handle)
    }
}

impl Eq for Panel { }
