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
use ncursesw::Origin;

use crate::window::Window;
use crate::ncurseswwinerror::NCurseswWinError;

/// A raw pointer that can be user defined.
pub type PanelUserPtr = panels::PANEL_USERPTR;

/// A moveable panel that is a container for a `Window`.
pub struct Panel {
    handle:       PANEL,
    free_on_drop: bool
}

impl Panel {
    /// Create a new Panel instance with it's associated Window.
    pub fn new_panel(window: &Window) -> result!(Self) {
        match panels::new_panel(window.handle()) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Self::from(handle, true))
        }
    }

    // make a new instance from the passed panel structure pointer and specify
    // if the handle is to be free'd when the structure is dropped.
    //
    // free_on_drop is false in call's such as panel_above(&self) where we are
    // 'peeking' the Panel but it would be invalid to free the handle when
    // our instance goes out of scope.
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
    /// Puts panel at the bottom of all panels.
    pub fn bottom_panel(&self) -> result!(()) {
        panels::bottom_panel(self.handle)?;

        Ok(())
    }

    /// Puts the given visible panel on top of all panels in the stack.
    pub fn top_panel(&self) -> result!(()) {
        panels::top_panel(self.handle)?;

        Ok(())
    }

    /// Makes a hidden panel visible by placing it on top of the panels in the panel stack.
    pub fn show_panel(&self) -> result!(()) {
        panels::show_panel(self.handle)?;

        Ok(())
    }

    /// Removes the given panel from the panel stack and thus hides it from view.
    ///
    /// The Panel is not lost, merely removed from the stack.
    pub fn hide_panel(&self) -> result!(()) {
        panels::hide_panel(self.handle)?;

        Ok(())
    }

    /// Returns the window of the given panel.
    pub fn panel_window(&self) -> result!(Window) {
        match panels::panel_window(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Window::from(handle, false))
        }
    }

    /// Replaces the current window of panel with window.
    ///
    /// Useful, for example if you want to resize a panel; if you're using ncurses, you can
    /// call replace_panel on the output of wresize(3x)). It does not change the position of the panel in the stack.
    pub fn replace_panel(&self, window: &Window) -> result!(()) {
        panels::replace_panel(self.handle, window.handle())?;

        Ok(())
    }

    /// Moves the given panel window so that its upper-left corner is at origin.y, origin.x.
    ///
    /// It does not change the position of the panel in the stack. Be sure to use this function, not mvwin(), to move a panel window.
    pub fn move_panel(&self, origin: Origin) -> result!(()) {
        panels::move_panel(self.handle, origin)?;

        Ok(())
    }

    /// Returns true if the panel is in the panel stack, false if it is not.
    pub fn panel_hidden(&self) -> result!(bool) {
        match panels::panel_hidden(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(hidden)  => Ok(hidden)
        }
    }

    /// Returns the panel above panel.
    pub fn panel_above(&self) -> result!(Self) {
        panel_above(Some(self))
    }

    /// Returns the panel just below panel.
    pub fn panel_below(&self) -> result!(Self) {
        panel_below(Some(self))
    }

    /// Sets the panel's user pointer.
    pub fn set_panel_userptr(&self, ptr: Option<PanelUserPtr>) -> result!(()) {
        panels::set_panel_userptr(self.handle, ptr)?;

        Ok(())
    }

    /// Returns the user pointer for the given panel.
    pub fn panel_userptr(&self) -> Option<PanelUserPtr> {
        panels::panel_userptr(self.handle)
    }
}

unsafe impl Send for Panel { } // too make thread safe
unsafe impl Sync for Panel { } // too make thread safe

/// Returns the panel above the specified panel.
///
/// If the specified panel argument is None, it returns the bottom panel in the stack.
pub fn panel_above(panel: Option<&Panel>) -> result!(Panel) {
    match panels::panel_above(match panel {
        None        => None,
        Some(panel) => Some(panel.handle)
    }) {
        Err(source) => Err(NCurseswWinError::NCurseswError { source }),
        Ok(handle)  => Ok(Panel::from(handle, false))
    }
}

/// Returns the panel just below the specified panel.
///
/// If the specified panel argument is None, it returns the top panel in the stack.
pub fn panel_below(panel: Option<&Panel>) -> result!(Panel) {
    match panels::panel_below(match panel {
        None        => None,
        Some(panel) => Some(panel.handle)
    }) {
        Err(source) => Err(NCurseswWinError::NCurseswError { source }),
        Ok(handle)  => Ok(Panel::from(handle, false))
    }
}
