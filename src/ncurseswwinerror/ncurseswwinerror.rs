/*
    src/ncurseswwinerror.rs

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

use std::{num, convert, ffi};

use thiserror::Error;

use ncursesw::{
    NCurseswError, panels::NCurseswPanelsError, mouse::NCurseswMouseError,
    menu::NCurseswMenuError, form::NCurseswFormError
};
use crate::ripoff::MAX_RIPOFF_LINES;

/// NCurseswWin Errors.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum NCurseswWinError {
    #[error("ncurses has already been initialised")]
    InitscrAlreadyCalled,
    #[error("ncurses has not been initialised")]
    InitscrNotCalled,
    #[error("ncurseswwin::start_color() already called")]
    StartColorAlreadyCalled,
    #[error("ncurseswwin::start_color() not called")]
    StartColorNotCalled,
    #[error("attempt to initialise ripoff {number}, maximum ripoff's allowed {}", MAX_RIPOFF_LINES)]
    MaximumRipoffLines { number: usize },
    #[error("an internal error has occured")]
    InternalError,
    #[error("{message}")]
    Panic { message: String },
    #[error("{func}() out of memory!!!")]
    OutOfMemory { func: String },
    #[error("unable to obtain a valid mouse id!!!")]
    MouseId,
    #[error("{func}() too many arguments {args}")]
    FieldTypeArguments { func: String, args: u8 },
    #[error("softlabel already defined.")]
    SoftLabelAlreadyDefined,

    #[error("{source}")]
    NCurseswError { #[from] source: NCurseswError },
    #[error("{source}")]
    PanelsError { #[from] source: NCurseswPanelsError },
    #[error("{source}")]
    MouseError { #[from] source: NCurseswMouseError },
    #[error("{source}")]
    MenuError { #[from] source: NCurseswMenuError },
    #[error("{source}")]
    FormError { #[from] source: NCurseswFormError },

    #[error("{source}")]
    TryFromIntError { #[from] source: num::TryFromIntError },
    #[error("{source}")]
    NulError { #[from] source: ffi::NulError },
    #[error("{source}")]
    Infallible { #[from] source: convert::Infallible }
}
