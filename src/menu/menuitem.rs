/*
    src/menu/menuitem.rs

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

use crate::NCurseswWinError;
use ncursesw::{menu, menu::ITEM};

pub use ncursesw::menu::ItemOptions;

/// Menu item.
pub struct MenuItem {
    handle:       ITEM, // pointer to ncurses menu item internal structure
    free_on_drop: bool
}

impl MenuItem {
    // make a new instance from the passed ncurses menu item pointer.
    pub(in crate::menu) fn _from(handle: ITEM, free_on_drop: bool) -> Self {
        Self { handle, free_on_drop }
    }

    // get the ncurses menu item pointer for this Window structure.
    pub(in crate::menu) fn _handle(&self) -> ITEM {
        self.handle
    }
}

impl MenuItem {
    pub fn new(name: &str, description: &str) -> result!(Self) {
        let handle = menu::new_item(name, description)?;

        Ok(Self::_from(handle, true))
    }

    #[deprecated(since = "0.3.2", note = "Use MenuItem::new() instead")]
    pub fn new_item(name: &str, description: &str) -> result!(Self) {
        Self::new(name, description)
    }

    pub fn item_description(&self) -> Option<String> {
        menu::item_description(self.handle)
    }

    pub fn item_index(&self) -> result!(i32) {
        let index = menu::item_index(self.handle)?;

        Ok(index)
    }

    pub fn item_name(&self) -> Option<String> {
        menu::item_name(self.handle)
    }

    pub fn item_opts(&self) -> ItemOptions {
        menu::item_opts(self.handle)
    }

    pub fn item_opts_off(&self, opts: ItemOptions) -> result!(()) {
        menu::item_opts_off(self.handle, opts)?;

        Ok(())
    }

    pub fn item_opts_on(&self, opts: ItemOptions) -> result!(()) {
        menu::item_opts_on(self.handle, opts)?;

        Ok(())
    }

    pub fn item_userptr<T>(&self) -> Option<Box<T>> {
        match menu::item_userptr(self.handle) {
            Some(ptr) => Some(unsafe { Box::from_raw(ptr as *mut T) }),
            None      => None
        }
    }

    pub fn item_value(&self) -> bool {
        menu::item_value(self.handle)
    }

    pub fn item_visible(&self) -> bool {
        menu::item_visible(self.handle)
    }

    pub fn set_item_opts(&self, opts: ItemOptions) -> result!(()) {
        menu::set_item_opts(self.handle, opts)?;

        Ok(())
    }

    pub fn set_item_userptr<T>(&self, ptr: Option<Box<&T>>) {
        menu::set_item_userptr(self.handle, match ptr {
            Some(ptr) => Some(Box::into_raw(ptr) as *mut libc::c_void),
            None      => None
        })
    }

    pub fn set_item_value(&self, value: bool) -> result!(()) {
        menu::set_item_value(self.handle, value)?;

        Ok(())
    }
}

impl Drop for MenuItem {
    fn drop(&mut self) {
        // TODO: remove comments once we've sorted out why Menu::post_menu()
        //       is failing with no menu items connected.
        /*
        if self.free_on_drop {
            if let Err(source) = menu::free_item(self.handle) {
                panic!(source.to_string())
            }
        }
        */
    }
}

unsafe impl Send for MenuItem { } // too make thread safe
unsafe impl Sync for MenuItem { } // too make thread safe
