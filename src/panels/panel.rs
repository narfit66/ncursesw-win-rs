/*
    src/panels/panel.rs

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

use std::{ptr, fmt, convert::TryInto, hash::{Hash, Hasher}};
use ncursesw::{panels, SCREEN, panels::PANEL};
use crate::{Screen, Origin, Window, NCurseswWinError, gen::HasHandle, panels::funcs};

/// A moveable panel that is a container for a `Window`.
pub struct Panel {
    screen:       Option<SCREEN>,
    handle:       PANEL,
    free_on_drop: bool
}

impl HasHandle<PANEL> for Panel {
    // make a new instance from the passed panel structure pointer and specify
    // if the handle is to be free'd when the structure is dropped.
    //
    // free_on_drop is false in call's such as panel_above(&self) where we are
    // 'peeking' the Panel but it would be invalid to free the handle when
    // our instance goes out of scope.
    fn _from(screen: Option<SCREEN>, handle: PANEL, free_on_drop: bool) -> Self {
        assert!(screen.map_or_else(|| true, |screen| !screen.is_null()), "Panel::_from() : screen.is_null()");
        assert!(!handle.is_null(), "Panel::_from() : handle.is_null()");

        Self { screen, handle, free_on_drop }
    }

    fn _screen(&self) -> Option<SCREEN> {
        self.screen
    }

    fn _handle(&self) -> PANEL {
        self.handle
    }
}

impl Panel {
    /// Create a new Panel instance with it's associated Window.
    pub fn new(window: &Window) -> result!(Self) {
        Ok(Self::_from(window._screen(), panels::new_panel(window._handle())?, true))
    }

    #[deprecated(since = "0.3.1", note = "Use Panel::new() instead")]
    /// Create a new Panel instance with it's associated Window.
    pub fn new_panel(window: &Window) -> result!(Self) {
        Self::new(window)
    }

    /// The screen associated with this `Panel`.
    pub fn screen(&self) -> Option<Screen> {
        self.screen.map(|screen| Screen::_from(screen, false))
    }

    /// Puts this `Panel` at the bottom of all panels.
    pub fn bottom_panel(&self) -> result!(()) {
        Ok(panels::bottom_panel(self.handle)?)
    }

    /// Puts the given visible `Panel` on top of all panels in the stack.
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
        Ok(Window::_from(self.screen, panels::panel_window(self.handle)?, false))
    }

    /// Replaces the current window of panel with window.
    ///
    /// Useful, for example if you want to resize a panel; if you're using ncurses, you can
    /// call replace_panel on the output of wresize(3x)). It does not change the position of the panel in the stack.
    pub fn replace_panel(&self, window: &Window) -> result!(()) {
        assert!(self.screen == window._screen());

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

    /// Returns the panel above this `Panel`.
    pub fn panel_above(&self) -> result!(Self) {
        funcs::panel_above(Some(self))
    }

    /// Returns the panel just below this `Panel`.
    pub fn panel_below(&self) -> result!(Self) {
        funcs::panel_below(Some(self))
    }

    /// Sets the panel's user pointer to the passed `Panel`.
    pub fn set_panel_userptr<T>(&self, ptr: Option<Box<&T>>) -> result!(()) {
        Ok(panels::set_panel_userptr(self.handle, ptr.map(|ptr| Box::into_raw(ptr) as *const libc::c_void))?)
    }

    /// Returns the user pointers `Panel` for the given panel.
    pub fn panel_userptr<T>(&self) -> Option<Box<T>> {
        panels::panel_userptr(self.handle).map(|ptr| unsafe { Box::from_raw(ptr as *mut T) })
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
        self.screen == rhs.screen && ptr::eq(self.handle, rhs.handle)
    }
}

impl Eq for Panel { }

impl Hash for Panel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl AsRef<Panel> for Panel {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Panel> for Panel {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Clone for Panel {
    fn clone(&self) -> Self {
        Self::_from(self.screen, self.handle, false)
    }
}

impl fmt::Debug for Panel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Panel {{ screen: {:?}, handle: {:p}, free_on_drop: {} }}", self.screen, self.handle, self.free_on_drop)
    }
}
