/*
    src/panels/funcs.rs

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

use ncursesw::panels;
use crate::{NCurseswWinError, HasHandle, panels::Panel};

pub use panels::update_panels;

/// Returns the panel above the specified panel.
///
/// If the specified panel argument is None, it returns the bottom panel in the stack.
pub fn panel_above(panel: Option<&Panel>) -> result!(Panel) {
    match panels::panel_above(panel.map(|panel| panel._handle())) {
        Err(source) => Err(NCurseswWinError::PanelsError { source }),
        Ok(handle)  => Ok(Panel::_from(None, handle, false))
    }
}

/// Returns the panel just below the specified panel.
///
/// If the specified panel argument is None, it returns the top panel in the stack.
pub fn panel_below(panel: Option<&Panel>) -> result!(Panel) {
    match panels::panel_below(panel.map(|panel| panel._handle())) {
        Err(source) => Err(NCurseswWinError::PanelsError { source }),
        Ok(handle)  => Ok(Panel::_from(None, handle, false))
    }
}
