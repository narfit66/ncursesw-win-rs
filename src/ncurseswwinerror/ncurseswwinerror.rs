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

use std::{num, convert, ffi};

use ncursesw::{
    NCurseswError, panels::NCurseswPanelsError, mouse::NCurseswMouseError,
    menu::NCurseswMenuError, form::NCurseswFormError
};
use crate::{Screen, ripoff::MAX_RIPOFF_LINES};

custom_error::custom_error! {
/// NCurseswWin Errors.
#[derive(PartialEq, Eq)]
pub NCurseswWinError
    InitscrAlreadyCalled = "ncurses has already been initialised",
    InitscrNotCalled = "ncurses has not been initialised",
    StartColorAlreadyCalled = "ncurseswwin::start_color() already called",
    StartColorNotCalled = "ncurseswwin::start_color() not called",
    MaximumRipoffLines { number: usize } = @{ format!("attempt to initialise ripoff {}, maximum ripoff's allowed {}", number, MAX_RIPOFF_LINES) },
    RipoffNotInitialized { number: usize } = "ripoff line {number} has not been initialised",
    InternalError = "an internal error has occured",
    Panic { message: String } = "{message}",
    OutOfMemory { func: String } = "{func}() out of memory!!!",
    MouseId = "unable to obtain a valid mouse id!!!",
    FieldTypeArguments { func: String, args: u8 } = "{func}() too many arguments {args}",
    SoftLabelAlreadyDefined { screen: Option<Screen> } = @{ format!("softlabel already defined with screen {:?}", screen) },

    NCurseswError { source: NCurseswError } = "{source}",
    PanelsError { source: NCurseswPanelsError } = "{source}",
    MouseError { source: NCurseswMouseError } = "{source}",
    MenuError { source: NCurseswMenuError } = "{source}",
    FormError { source: NCurseswFormError } = "{source}",

    TryFromIntError { source: num::TryFromIntError } = "{source}",
    NulError { source: ffi::NulError } = "{source}",
    Infallible { source: convert::Infallible } = "{source}"
}
