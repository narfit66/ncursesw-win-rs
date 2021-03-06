/*
    src/menu/funcs.rs

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

use std::convert::TryInto;

use ncursesw::menu::MenuRequest;
use crate::{Window, HasHandle, NCurseswWinError, menu::MenuSize};

pub fn menu_request_by_name(name: &str) -> result!(Option<MenuRequest>) {
    Ok(ncursesw::menu::menu_request_by_name(name)?)
}

pub fn menu_request_name(request: MenuRequest) -> result!(String) {
    Ok(ncursesw::menu::menu_request_name(request)?)
}

pub fn set_menu_format(menu_size: MenuSize) -> result!(()) {
    Ok(ncursesw::menu::set_menu_format(None, menu_size.try_into()?)?)
}

pub fn set_menu_sub(window: Option<&Window>) -> result!(()) {
    Ok(ncursesw::menu::set_menu_sub(None, window.map_or_else(|| None, |window| Some(window._handle())))?)
}

pub fn set_menu_win(window: Option<&Window>) -> result!(()) {
    Ok(ncursesw::menu::set_menu_win(None, window.map_or_else(|| None, |window| Some(window._handle())))?)
}
