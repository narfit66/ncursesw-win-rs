/*
    src/traits/mod.rs

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

mod hasadd;
mod hasmvadd;
mod hasdel;
mod hasmvdel;
mod graphicstransform;
mod hasgraphics;
mod hashandle;
mod hasin;
mod hasmvin;
mod hasins;
mod hasmvins;
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
mod hasget;
mod hasmvget;

mod mouseable;

mod basecanvas;
mod ncurseswwindow;
mod cansubwindow;

mod ispad;
mod iswindow;

pub use crate::traits::hasadd::*;
pub use crate::traits::hasmvadd::*;
pub use crate::traits::hasdel::*;
pub use crate::traits::hasmvdel::*;
pub(in crate) use crate::traits::graphicstransform::*;
pub use crate::traits::hasgraphics::*;
pub(in crate) use crate::traits::hashandle::*;
pub use crate::traits::hasin::*;
pub use crate::traits::hasmvin::*;
pub use crate::traits::hasins::*;
pub use crate::traits::hasmvins::*;
pub use crate::traits::hasyaxis::*;
pub use crate::traits::hasyxaxis::*;
pub use crate::traits::hasxaxis::*;
pub use crate::traits::moveable::*;
pub use crate::traits::derivable::*;
pub use crate::traits::scrollable::*;

pub use crate::traits::hasbackground::*;

pub use crate::traits::hasattributes::*;
pub use crate::traits::hasmvattributes::*;

pub use crate::traits::hasnonblocking::*;
pub use crate::traits::hasget::*;
pub use crate::traits::hasmvget::*;

pub use crate::traits::mouseable::*;

pub use crate::traits::basecanvas::*;
pub(in crate) use crate::traits::ncurseswwindow::*;
pub use crate::traits::cansubwindow::*;

pub use crate::traits::ispad::*;
pub use crate::traits::iswindow::*;
