/*
    src/ncurseswwinerror.rs

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

use std::{num, convert};

use ncursesw::{
    Origin, NCurseswError, panels::NCurseswPanelsError, mouse::NCurseswMouseError,
    menu::NCurseswMenuError, shims::constants::ERR
};
use crate::ripoff::MAX_LINES;

custom_error::custom_error! {
/// NCurseswWin Errors.
pub NCurseswWinError
    InitscrAlreadyCalled = "ncurses has already been initialised",
    InitscrNotCalled = "ncurses has not been initialised",
    StartColorAlreadyCalled = "ncurseswwin::start_color() already called",
    StartColorNotCalled = "ncurseswwin::start_color() not called",
    MaximumRipoffLines { number: usize } = @{ format!("attempt to initialise ripoff {}, maximum ripoff's allowed {}", number, MAX_LINES) },
    RipoffNotInitialized { number: usize } = "ripoff line {number} has not been initialised",
    InternalError = "an internal error has occured",
    Panic { message: String } = "{message}",
    TryFromIntError { source: num::TryFromIntError } = "{source}",
    Infallible { source: convert::Infallible } = "{source}",
    NCurseswError { source: NCurseswError } = "{source}",
    PanelsError { source: NCurseswPanelsError } = "{source}",
    MouseError { source: NCurseswMouseError } = "{source}",
    MenuError { source: NCurseswMenuError } = "{source}",

    TryIntoOriginError { y: u16, x: u16 } = "invalid: Origin {{ y: {y}, x: {x} }}",
    TryFromOriginError { y: i32, x: i32 } = "invalid: ncursesw::Origin {{ y: {y}, x: {x} }}",
    TryIntoSizeError { lines: u16, columns: u16 } = "invalid: Size {{ lines: {lines}, columns: {columns} }}",
    TryFromSizeError { lines: i32, columns: i32 } = "invalid: ncursesw::Size {{ lines: {lines}, columns: {columns} }}",
    TryIntoRegionError { top: u16, bottom: u16 } = "invalid: Region {{ top: {top}, bottom: {bottom} }}",
    TryFromRegionError { top: i32, bottom: i32 } = "invalid: ncursesw::Region {{ top: {top}, bottom: {bottom} }}",
    TryFromOriginResultError { origin: Origin, to_screen: bool, result: bool } = "invalid: ncursesw::OriginResult {{ origin: {origin}, to_screen: {to_screen}, result: {result} }}",
    TryIntoMenuSizeError { rows: u16, columns: u16 } = "invalid: MenuSize {{ rows: {rows}, columns: {columns} }}",
    TryFromMenuSizeError { rows: i32, columns: i32 } = "invalid: ncursesw::MenuSize {{ rows: {rows}, columns: {columns} }}"
}

impl PartialEq for NCurseswWinError {
    fn eq(&self, rhs: &Self) -> bool {
        // TODO: must be a better way of doing this!!!
        format!("{}", self) == format!("{}", rhs)
    }
}

impl Eq for NCurseswWinError { }

pub(crate) fn timeout_error(func: &str) -> NCurseswWinError {
    NCurseswWinError::from(NCurseswError::LibraryError { func: func.to_string(), rc: ERR })
}
