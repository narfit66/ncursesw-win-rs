/*
    src/gen/mod.rs

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

mod hasaddfunctions;
mod hasmvaddfunctions;
mod hasdelfunctions;
mod hasmvdelfunctions;
mod graphicstransform;
mod hasgraphicfunctions;
mod hashandle;
mod hasinfunctions;
mod hasmvinfunctions;
mod hasinsfunctions;
mod hasmvinsfunctions;
mod hasyaxis;
mod hasyxaxis;
mod hasxaxis;
mod moveable;
mod derivable;
mod scrollable;

mod hasbackground;

mod hasattributes;
mod hasmvattributes;

mod hasnonblocking;
mod hasgetfunctions;
mod hasmvgetfunctions;

mod mouseable;

mod basecanvas;
mod ncurseswwindow;
mod cansubwindow;

mod ispad;
mod iswindow;

pub use crate::gen::hasaddfunctions::*;
pub use crate::gen::hasmvaddfunctions::*;
pub use crate::gen::hasdelfunctions::*;
pub use crate::gen::hasmvdelfunctions::*;
pub(in crate) use crate::gen::graphicstransform::*;
pub use crate::gen::hasgraphicfunctions::*;
pub(in crate) use crate::gen::hashandle::*;
pub use crate::gen::hasinfunctions::*;
pub use crate::gen::hasmvinfunctions::*;
pub use crate::gen::hasinsfunctions::*;
pub use crate::gen::hasmvinsfunctions::*;
pub use crate::gen::hasyaxis::*;
pub use crate::gen::hasyxaxis::*;
pub use crate::gen::hasxaxis::*;
pub use crate::gen::moveable::*;
pub use crate::gen::derivable::*;
pub use crate::gen::scrollable::*;

pub use crate::gen::hasbackground::*;

pub use crate::gen::hasattributes::*;
pub use crate::gen::hasmvattributes::*;

pub use crate::gen::hasnonblocking::*;
pub use crate::gen::hasgetfunctions::*;
pub use crate::gen::hasmvgetfunctions::*;

pub use crate::gen::mouseable::*;

pub use crate::gen::basecanvas::*;
pub use crate::gen::ncurseswwindow::*;
pub use crate::gen::cansubwindow::*;

pub use crate::gen::ispad::*;
pub use crate::gen::iswindow::*;
