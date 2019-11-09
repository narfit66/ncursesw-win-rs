/*
    src/traits/basecanvas.rs

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

use crate::NCurseswWinError;
use crate::traits::*;

/// Does the window canvas type have ncursesw add functions.
pub trait BaseCanvas: HasHandle {
    fn clearok(&self, bf: bool) -> result!(()) {
        ncursesw::clearok(self._handle(), bf)?;

        Ok(())
    }

    fn clear(&self) -> result!(()) {
        ncursesw::wclear(self._handle())?;

        Ok(())
    }

    fn clrtobot(&self) -> result!(()) {
        ncursesw::wclrtobot(self._handle())?;

        Ok(())
    }

    fn clrtoeol(&self) -> result!(()) {
        ncursesw::wclrtoeol(self._handle())?;

        Ok(())
    }

    fn cursyncup(&self) {
        ncursesw::wcursyncup(self._handle());
    }

    /*pub fn echochar(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::wechochar(self.handle, ch)?;

        Ok(())
    }

    pub fn echo_wchar(&self, wch: ComplexChar) -> result!(()) {
        ncursesw::wecho_wchar(self.handle, wch)?;

        Ok(())
    }*/

    fn erase(&self) -> result!(()) {
        ncursesw::werase(self._handle())?;

        Ok(())
    }

    fn idcok(&self, bf: bool) {
        ncursesw::idcok(self._handle(), bf)
    }

    fn idlok(&self, bf: bool) -> result!(()) {
        ncursesw::idlok(self._handle(), bf)?;

        Ok(())
    }

    fn immedok(&self, bf: bool) {
        ncursesw::immedok(self._handle(), bf)
    }

    fn insertln(&self) -> result!(()) {
        ncursesw::winsertln(self._handle())?;

        Ok(())
    }

    fn intrflush(&self, bf: bool) -> result!(()) {
        ncursesw::intrflush(self._handle(), bf)?;

        Ok(())
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
        ncursesw::keypad(self._handle(), bf)?;

        Ok(())
    }

    fn leaveok(&self, bf: bool) -> result!(()) {
        ncursesw::leaveok(self._handle(), bf)?;

        Ok(())
    }

    fn meta(&self, bf: bool) -> result!(()) {
        ncursesw::meta(self._handle(), bf)?;

        Ok(())
    }

    fn nodelay(&self, bf: bool) -> result!(()) {
        ncursesw::nodelay(self._handle(), bf)?;

        Ok(())
    }

    fn notimeout(&self, bf: bool) -> result!(()) {
        ncursesw::notimeout(self._handle(), bf)?;

        Ok(())
    }

    /*pub fn noutrefresh(&self) -> result!(()) {
        ncursesw::wnoutrefresh(self.handle)?;

        Ok(())
    }*/

    fn putwin(&self, path: &path::Path) -> result!(()) {
        ncursesw::putwin(self._handle(), path)?;

        Ok(())
    }

    fn redrawwin(&self) -> result!(()) {
        ncursesw::redrawwin(self._handle())?;

        Ok(())
    }

    /*pub fn refresh(&self) -> result!(()) {
        ncursesw::wrefresh(self.handle)?;

        Ok(())
    }

    pub fn resize(&self, size: Size) -> result!(()) {
        ncursesw::wresize(self.handle, size)?;

        Ok(())
    }*/

    fn syncdown(&self) {
        ncursesw::wsyncdown(self._handle());
    }

    fn syncok(&self, bf: bool) -> result!(()) {
        ncursesw::syncok(self._handle(), bf)?;

        Ok(())
    }

    fn syncup(&self) {
        ncursesw::wsyncup(self._handle());
    }

    fn touchwin(&self) -> result!(()) {
        ncursesw::touchwin(self._handle())?;

        Ok(())
    }

    fn untouchwin(&self) -> result!(()) {
        ncursesw::untouchwin(self._handle())?;

        Ok(())
    }
}
