/*
    src/screen/funcs.rs

    Copyright (c) 2020 Stephen Whittle  All rights reserved.

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

use std::{os::unix::io::AsRawFd, io::{Write, Read}};
use crate::{NCurseswWinError, screen::Screen};

pub fn new_prescr() -> result!(Screen) {
    Ok(Screen::_from(ncursesw::new_prescr()?, true))
}

pub fn newterm<O, I>(screen: &Screen, term: Option<&str>, output: &O, input: &I) -> result!(Screen)
    where O: AsRawFd + Write,
          I: AsRawFd + Read
{
    Ok(Screen::_from(ncursesw::newterm_sp(screen._handle(), term, output, input)?, true))
}

pub fn set_term(screen: &Screen) -> result!(Screen) {
    Ok(Screen::_from(ncursesw::set_term(screen._handle())?, false))
}
