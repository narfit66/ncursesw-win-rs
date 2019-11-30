/*
    src/menu/postedmenu.rs

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

use std::fmt;

use ncursesw::menu::{post_menu, unpost_menu, menu_driver};
use crate::{NCurseswWinError, menu::{Menu, MenuRequest}};

/// A Posted (Visible) Menu.
pub struct PostedMenu<'a> {
    menu:   &'a Menu,
    posted: bool
}

impl<'a> PostedMenu<'a> {
    pub(in crate::menu) fn new(menu: &'a Menu) -> result!(Self) {
        post_menu(menu._handle())?;

        Ok(Self { menu, posted: true })
    }

    pub fn menu_driver(&self, request: MenuRequest) -> result!(Option<i32>) {
        Ok(menu_driver(self.menu._handle(), request)?)
    }

    pub fn menu(&self) -> Menu {
        Menu::_from(self.menu._handle(), false)
    }

    pub fn repost(&mut self) -> result!(()) {
        post_menu(self.menu._handle())?;

        self.posted = true;

        Ok(())
    }

    pub fn unpost(&mut self) -> result!(()) {
        unpost_menu(self.menu._handle())?;

        self.posted = false;

        Ok(())
    }
}

impl<'a> Drop for PostedMenu<'a> {
    fn drop(&mut self) {
        if self.posted {
            if let Err(source) = unpost_menu(self.menu._handle()) {
                panic!("{} @ ({:?})", source, self.menu)
            }
        }
    }
}

unsafe impl<'a> Send for PostedMenu<'a> { } // too make thread safe
unsafe impl<'a> Sync for PostedMenu<'a> { } // too make thread safe

impl<'a> fmt::Debug for PostedMenu<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PostedMenu {{ menu: {:?} }}", self.menu)
    }
}
