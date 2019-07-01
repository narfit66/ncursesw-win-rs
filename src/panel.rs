/*
    src/panel.rs

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
use ncursesw::panels;
use ncursesw::panels::PANEL;
use ncursesw::{NCurseswError, Origin};

use crate::window::Window;

pub type PanelUserPtr = panels::PANEL_USERPTR;

pub struct Panel {
    handle:       PANEL,
    free_on_drop: bool
}

unsafe impl Send for Panel { }
unsafe impl Sync for Panel { }

impl Panel {
    pub fn new_panel(window: &Window) -> result!(Self) {
        match panels::new_panel(window.get_handle()) {
            Err(e)     => Err(e),
            Ok(handle) => Ok(Self::from(handle, true))
        }
    }

    #[inline]
    pub(crate) fn from(handle: PANEL, free_on_drop: bool) -> Self {
        Self { handle, free_on_drop }
    }
}

impl Drop for Panel {
    fn drop(&mut self) {
        if self.free_on_drop {
            if let Err(e) = panels::del_panel(self.handle) {
                panic!(e.to_string())
            }
        }
    }
}

impl Panel {
    /// puts panel at the bottom of all panels.
    pub fn bottom_panel(&self) -> result!(()) {
        panels::bottom_panel(self.handle)
    }

    /// puts the given visible panel on top of all panels in the stack.
    pub fn top_panel(&self) -> result!(()) {
        panels::top_panel(self.handle)
    }

    /// makes a hidden panel visible by placing it on top of the panels in the panel stack.
    pub fn show_panel(&self) -> result!(()) {
        panels::show_panel(self.handle)
    }

    /// removes the given panel from the panel stack and thus hides it from view. The Panel is not lost, merely removed from the stack.
    pub fn hide_panel(&self) -> result!(()) {
        panels::hide_panel(self.handle)
    }

    /// returns the window of the given panel.
    pub fn panel_window(&self) -> result!(Window) {
        match panels::panel_window(self.handle) {
            Err(e)     => Err(e),
            Ok(handle) => Ok(Window::from(handle, false))
        }
    }

    /// replaces the current window of panel with window (useful, for example if you want to resize a panel; if you're using ncurses, you can call replace_panel on the output of wresize(3x)). It does not change the position of the panel in the stack.
    pub fn replace_panel(&self, window: &Window) -> result!(()) {
        panels::replace_panel(self.handle, window.get_handle())
    }

    /// moves the given panel window so that its upper-left corner is at origin.y, origin.x. It does not change the position of the panel in the stack. Be sure to use this function, not mvwin(), to move a panel window.
    pub fn move_panel(&self, origin: Origin) -> result!(()) {
        panels::move_panel(self.handle, origin)
    }

    /// returns true if the panel is in the panel stack, false if it is not.
    pub fn panel_hidden(&self) -> result!(bool) {
        panels::panel_hidden(self.handle)
    }

    /// returns the panel above panel.
    pub fn panel_above(&self) -> result!(Self) {
        match panels::panel_above(Some(self.handle)) {
            Err(e)     => Err(e),
            Ok(handle) => Ok(Panel::from(handle, false))
        }
    }

    /// returns the panel just below panel.
    pub fn panel_below(&self) -> result!(Self) {
        match panels::panel_below(Some(self.handle)) {
            Err(e)     => Err(e),
            Ok(handle) => Ok(Panel::from(handle, false))
        }
    }

    /// sets the panel's user pointer.
    pub fn set_panel_userptr(&self, ptr: Option<PanelUserPtr>) -> result!(()) {
        panels::set_panel_userptr(self.handle, ptr)
    }

    /// returns the user pointer for the given panel.
    pub fn panel_userptr(&self) -> Option<PanelUserPtr> {
        panels::panel_userptr(self.handle)
    }
}

/// returns the panel above the specified panel. If the specified panel argument is None, it returns the bottom panel in the stack.
pub fn panel_above(panel: Option<&Panel>) -> result!(Panel) {
    match panels::panel_above(match panel {
        None        => None,
        Some(panel) => Some(panel.handle)
    }) {
        Err(e)     => Err(e),
        Ok(handle) => Ok(Panel::from(handle, false))
    }
}

/// returns the panel just below the specified panel. If the specified panel argument is None, it returns the top panel in the stack.
pub fn panel_below(panel: Option<&Panel>) -> result!(Panel) {
    match panels::panel_below(match panel {
        None        => None,
        Some(panel) => Some(panel.handle)
    }) {
        Err(e)     => Err(e),
        Ok(handle) => Ok(Panel::from(handle, false))
    }
}
