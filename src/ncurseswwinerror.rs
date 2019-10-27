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

custom_error::custom_error! {
/// NCurseswWin Errors.
pub NCurseswWinError
    InitscrAlreadyCalled = "ncurses has already been initialised",
    InitscrNotCalled = "ncurses has not been initialised",
    StartColorAlreadyCalled = "ncurseswwin::start_color() already called",
    StartColorNotCalled = "ncurseswwin::start_color() not called",
    MaximumRipoffLines { number: usize } = @{ format!("attempt to initialise ripoff {}, maximum ripoff's allowed {}", number, crate::ripoff::MAX_LINES) },
    RipoffNotInitialized { number: usize } = "ripoff line {number} has not been initialised",
    InternalError = "an internal error has occured",
    IntError { source: std::num::TryFromIntError } = "{source}",
    NCurseswError { source: ncursesw::NCurseswError } = "{source}"
}

impl PartialEq for NCurseswWinError {
    fn eq(&self, rhs: &Self) -> bool {
        // TODO: must be a better way of doing this!!!
        if format!("{}", self) == format!("{}", rhs) {
            true
        } else {
            false
        }
    }
}

impl Eq for NCurseswWinError { }
