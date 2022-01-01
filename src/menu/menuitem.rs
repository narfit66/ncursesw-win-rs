/*
    src/menu/menuitem.rs

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

use std::{ptr, fmt, convert::TryFrom, hash::{Hash, Hasher}};
use ncursesw::{menu, menu::ITEM, menu::ItemOptions};
use crate::NCurseswWinError;

/// Menu item.
pub struct MenuItem {
    handle:       ITEM, // pointer to ncurses menu item internal structure
    free_on_drop: bool
}

impl MenuItem {
    pub(in crate::menu) fn _from(handle: ITEM, free_on_drop: bool) -> Self {
        assert!(!handle.is_null(), "MenuItem::_from() : handle.is_null()");

        Self { handle, free_on_drop }
    }

    pub(in crate::menu) fn _handle(&self) -> ITEM {
        self.handle
    }
}

impl MenuItem {
    /// Create a new menu item instance.
    pub fn new(name: &str, description: &str) -> result!(Self) {
        Ok(Self::_from(menu::new_item(name, description)?, true))
    }

    #[deprecated(since = "0.4.0", note = "Use MenuItem::new() instead")]
    /// Create a new menu item instance.
    pub fn new_item(name: &str, description: &str) -> result!(Self) {
        Self::new(name, description)
    }

    /// The menu items description.
    pub fn item_description(&self) -> result!(String) {
        Ok(menu::item_description(self.handle)?)
    }

    /// The menu items index within it's attached menu.
    pub fn item_index(&self) -> result!(usize) {
        Ok(usize::try_from(menu::item_index(self.handle)?)?)
    }

    /// The menu items name.
    pub fn item_name(&self) -> result!(String) {
        Ok(menu::item_name(self.handle)?)
    }

    /// The menu items options.
    pub fn item_opts(&self) -> ItemOptions {
        menu::item_opts(Some(self.handle))
    }

    /// Set the menu items passed options off.
    pub fn item_opts_off(&self, opts: ItemOptions) -> result!(()) {
        Ok(menu::item_opts_off(Some(self.handle), opts)?)
    }

    /// Set the menu items passed options on.
    pub fn item_opts_on(&self, opts: ItemOptions) -> result!(()) {
        Ok(menu::item_opts_on(Some(self.handle), opts)?)
    }

    /// The menu items user client code defined value.
    // TODO: needs testing!
    pub fn item_userptr<T>(&self) -> Option<Box<T>> {
        menu::item_userptr(Some(self.handle)).as_mut().map(|userptr| unsafe { Box::from_raw(*userptr as *mut T) })
    }

    /// In a multi-valued menu indicated that the menu item has been selected.
    pub fn item_value(&self) -> bool {
        menu::item_value(self.handle)
    }

    /// Is the menu item visible with the menu.
    pub fn item_visible(&self) -> bool {
        menu::item_visible(self.handle)
    }

    /// Set the menu items passed options.
    pub fn set_item_opts(&self, opts: ItemOptions) -> result!(()) {
        Ok(menu::set_item_opts(Some(self.handle), opts)?)
    }

    /// Set the menu items user client code defined value.
    // TODO: needs testing!
    pub fn set_item_userptr<T>(&self, userptr: Option<Box<&T>>) {
        menu::set_item_userptr(Some(self.handle), userptr.map(|userptr| Box::into_raw(userptr) as *mut libc::c_void))
    }

    /// In a multi-valued menu sets the menu item too selected.
    pub fn set_item_value(&self, value: bool) -> result!(()) {
        Ok(menu::set_item_value(self.handle, value)?)
    }
}

impl Drop for MenuItem {
    fn drop(&mut self) {
        if self.free_on_drop {
            if let Err(source) = menu::free_item(self.handle) {
                panic!("{} @ {:?}", source, self)
            }
        }
    }
}

unsafe impl Send for MenuItem { } // too make thread safe
unsafe impl Sync for MenuItem { } // too make thread safe

impl PartialEq for MenuItem {
    fn eq(&self, rhs: &Self) -> bool {
        ptr::eq(self.handle, rhs.handle)
    }
}

impl Eq for MenuItem { }

impl Hash for MenuItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl fmt::Debug for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MenuItem {{ handle: {:p}, free_on_drop: {} }}", self.handle, self.free_on_drop)
    }
}
