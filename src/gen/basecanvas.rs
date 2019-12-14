/*
    src/gen/basecanvas.rs

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

use std::path;

use ncursesw::WINDOW;
use crate::{NCurseswWinError, gen::HasHandle};

/// Does the window canvas type have ncursesw add functions.
pub trait BaseCanvas: HasHandle<WINDOW> {
    fn clearok(&self, bf: bool) -> result!(()) {
        Ok(ncursesw::clearok(self._handle(), bf)?)
    }

    fn clear(&self) -> result!(()) {
        Ok(ncursesw::wclear(self._handle())?)
    }

    fn clrtobot(&self) -> result!(()) {
        Ok(ncursesw::wclrtobot(self._handle())?)
    }

    fn clrtoeol(&self) -> result!(()) {
        Ok(ncursesw::wclrtoeol(self._handle())?)
    }

    fn cursyncup(&self) {
        ncursesw::wcursyncup(self._handle());
    }

    fn erase(&self) -> result!(()) {
        Ok(ncursesw::werase(self._handle())?)
    }

    fn idcok(&self, bf: bool) {
        ncursesw::idcok(self._handle(), bf)
    }

    fn idlok(&self, bf: bool) -> result!(()) {
        Ok(ncursesw::idlok(self._handle(), bf)?)
    }

    fn immedok(&self, bf: bool) {
        ncursesw::immedok(self._handle(), bf)
    }

    fn insertln(&self) -> result!(()) {
        Ok(ncursesw::winsertln(self._handle())?)
    }

    fn intrflush(&self, bf: bool) -> result!(()) {
        Ok(ncursesw::intrflush(self._handle(), bf)?)
    }

    fn is_cleared(&self) -> bool {
        ncursesw::is_cleared(self._handle())
    }

    fn is_idcok(&self) -> bool {
        ncursesw::is_idcok(self._handle())
    }

    fn is_idlok(&self) -> bool {
        ncursesw::is_idlok(self._handle())
    }

    fn is_immedok(&self) -> bool {
        ncursesw::is_immedok(self._handle())
    }

    fn is_keypad(&self) -> bool {
        ncursesw::is_keypad(self._handle())
    }

    fn is_leaveok(&self) -> bool {
        ncursesw::is_leaveok(self._handle())
    }

    fn is_nodelay(&self) -> bool {
        ncursesw::is_nodelay(self._handle())
    }

    fn is_notimeout(&self) -> bool {
        ncursesw::is_notimeout(self._handle())
    }

    fn is_pad(&self) -> bool {
        ncursesw::is_pad(self._handle())
    }

    fn is_syncok(&self) -> bool {
        ncursesw::is_syncok(self._handle())
    }

    fn is_wintouched(&self) -> bool {
        ncursesw::is_wintouched(self._handle())
    }

    fn keypad(&self, bf: bool) -> result!(()) {
        Ok(ncursesw::keypad(self._handle(), bf)?)
    }

    fn leaveok(&self, bf: bool) -> result!(()) {
        Ok(ncursesw::leaveok(self._handle(), bf)?)
    }

    fn meta(&self, bf: bool) -> result!(()) {
        Ok(ncursesw::meta(self._handle(), bf)?)
    }

    fn nodelay(&self, bf: bool) -> result!(()) {
        Ok(ncursesw::nodelay(self._handle(), bf)?)
    }

    fn notimeout(&self, bf: bool) -> result!(()) {
        Ok(ncursesw::notimeout(self._handle(), bf)?)
    }

    fn putwin(&self, path: &path::Path) -> result!(()) {
        Ok(ncursesw::putwin(self._handle(), path)?)
    }

    fn redrawwin(&self) -> result!(()) {
        Ok(ncursesw::redrawwin(self._handle())?)
    }

    fn syncdown(&self) {
        ncursesw::wsyncdown(self._handle());
    }

    fn syncok(&self, bf: bool) -> result!(()) {
        Ok(ncursesw::syncok(self._handle(), bf)?)
    }

    fn syncup(&self) {
        ncursesw::wsyncup(self._handle());
    }

    fn touchwin(&self) -> result!(()) {
        Ok(ncursesw::touchwin(self._handle())?)
    }

    fn untouchwin(&self) -> result!(()) {
        Ok(ncursesw::untouchwin(self._handle())?)
    }
}
