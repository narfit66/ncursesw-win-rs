/*
    src/form/funcs.rs

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

use ncursesw::form::FormRequest;
use crate::{Window, HasHandle, NCurseswWinError};

/// Searches in the name-table for a request with the given name and returns
/// its request code as a `Some`. Otherwise `None` is returned.
pub fn form_request_by_name(name: &str) -> result!(Option<FormRequest>) {
    Ok(ncursesw::form::form_request_by_name(name)?)
}

/// Returns the printable name of a form request code.
pub fn form_request_name(request: FormRequest) -> result!(String) {
    Ok(ncursesw::form::form_request_name(request)?)
}

/// Sets the forms sub-window. if `window` is `None` the `stdscr()` is used.
pub fn set_form_sub(window: Option<&Window>) -> result!(()) {
    Ok(ncursesw::form::set_form_sub(None, window.map_or_else(|| None, |window| Some(window._handle())))?)
}

/// Sets the forms main window. if `window` is `None` the `stdscr()` is used.
pub fn set_form_win(window: Option<&Window>) -> result!(()) {
    Ok(ncursesw::form::set_form_win(None, window.map_or_else(|| None, |window| Some(window._handle())))?)
}
