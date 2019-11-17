/*
    src/gen/derivable.rs

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

use ncursesw::{Origin, Size};
use crate::NCurseswWinError;
use crate::gen::*;
use crate::window::*;

/// Is the window canvas type capable of creating a derived window.
pub trait Derivable: HasHandle + NCurseswWindow + HasYXAxis + IsWindow {
    fn derwin(&self, size: Size, origin: Origin) -> result!(Window) {
        match ncursesw::derwin(self._handle(), size, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Window::_from(handle, true))
        }
    }

    fn mvderwin(&self, origin: Origin) -> result!(()) {
        assert_origin!("mvderwin", self.size()?, origin);

        ncursesw::mvderwin(self._handle(), origin)?;

        Ok(())
    }
}
