/*
    src/menu/menu.rs

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

use std::{ptr, fmt, convert::{TryFrom, TryInto}};

use ncursesw::{normal, menu, menu::MENU/*, menu::ITEM*/};
use crate::{
    Window, HasHandle, NCurseswWinError,
    menu::MenuSize, menu::MenuItem, menu::MenuSpacing, menu::PostedMenu
};

pub use ncursesw::menu::{
    Menu_Hook, MenuOptions, MenuRequest
};

/// Menu.
pub struct Menu {
    handle:       MENU, // pointer to ncurses menu item internal structure
    free_on_drop: bool
}

impl Menu {
    // make a new instance from the passed ncurses menu item pointer.
    pub(in crate::menu) fn _from(handle: MENU, free_on_drop: bool) -> Self {
        assert!(!handle.is_null(), "Menu::_from() : handle.is_null()");

        Self { handle, free_on_drop }
    }

    pub(in crate::menu) fn _handle(&self) -> MENU {
        self.handle
    }
}

impl Menu {
    pub fn new(items: &Vec<&MenuItem>) -> result!(Self) {
        /*
        let item_handles = &items.iter().map(|item| item._handle()).collect();

        Ok(Self::_from(menu::new_menu(item_handles)?, true))
        */
        let mut item_handles = vec!();

        for item in items {
            item_handles.push(item._handle())
        }

        Ok(Self::_from(menu::new_menu(&item_handles)?, true))
        /*
        let item_handles: &mut Vec<ITEM> = &mut items.iter().map(|item| item._handle()).collect();

        item_handles.push(ptr::null_mut());

        eprintln!("Menu::new() item_handles: {:?}", item_handles);

        match unsafe { ncursesw::shims::nmenu::new_menu(item_handles.as_mut_ptr()) } {
            Some(menu) => Ok(Self::_from(menu, true)),
            None       => panic!("Menu::new() failed!")
        }
        */
    }

    #[deprecated(since = "0.4.0", note = "Use Menu::new() instead")]
    pub fn new_menu(items: &Vec<&MenuItem>) -> result!(Self) {
        Self::new(items)
    }

    pub fn current_item(&self) -> result!(MenuItem) {
        Ok(MenuItem::_from(menu::current_item(self.handle)?, false))
    }

    pub fn item_count(&self) -> result!(usize) {
        Ok(usize::try_from(menu::item_count(self.handle)?)?)
    }

    pub fn item_init(&self) -> result!(Menu_Hook) {
        Ok(menu::item_init(self.handle)?)
    }

    pub fn item_term(&self) -> result!(Menu_Hook) {
        Ok(menu::item_term(self.handle)?)
    }

    pub fn menu_back(&self) -> normal::Attributes {
        menu::menu_back(self.handle)
    }

    pub fn menu_fore(&self) -> normal::Attributes {
        menu::menu_fore(self.handle)
    }

    pub fn menu_format(&self) -> result!(MenuSize) {
        Ok(MenuSize::try_from(menu::menu_format(self.handle))?)
    }

    pub fn menu_grey(&self) -> normal::Attributes {
        menu::menu_grey(self.handle)
    }

    pub fn menu_init(&self) -> result!(Menu_Hook) {
        Ok(menu::menu_init(self.handle)?)
    }

    pub fn menu_items(&self) -> result!(Vec<MenuItem>) {
        let handles = menu::menu_items(self.handle)?;

        Ok(handles.iter().map(|handle| MenuItem::_from(*handle, false)).collect())
    }

    pub fn menu_mark(&self) -> result!(String) {
        Ok(menu::menu_mark(self.handle)?)
    }

    pub fn menu_opts(&self) -> MenuOptions {
        menu::menu_opts(self.handle)
    }

    pub fn menu_opts_off(&self, opts: MenuOptions) -> result!(()) {
        Ok(menu::menu_opts_off(self.handle, opts)?)
    }

    pub fn menu_opts_on(&self, opts: MenuOptions) -> result!(()) {
        Ok(menu::menu_opts_on(self.handle, opts)?)
    }

    pub fn menu_pad(&self) -> char {
        menu::menu_pad(self.handle)
    }

    pub fn menu_pattern(&self) -> result!(String) {
        Ok(menu::menu_pattern(self.handle)?)
    }

    pub fn menu_spacing(&self) -> result!(MenuSpacing) {
        Ok(MenuSpacing::try_from(menu::menu_spacing(self.handle)?)?)
    }

    pub fn menu_sub(&self) -> result!(Window) {
        Ok(Window::_from(menu::menu_sub(self.handle)?, false))
    }

    pub fn menu_term(&self) -> result!(Menu_Hook) {
        Ok(menu::menu_term(self.handle)?)
    }

    pub fn menu_userptr<T>(&self) -> Option<Box<T>> {
        match menu::menu_userptr(self.handle) {
            Some(ptr) => Some(unsafe { Box::from_raw(ptr as *mut T) }),
            None      => None
        }
    }

    pub fn pos_menu_cursor(&self) -> result!(()) {
        Ok(menu::pos_menu_cursor(self.handle)?)
    }

    pub fn post_menu(&self) -> result!(PostedMenu) {
        PostedMenu::new(self)
    }

    pub fn scale_menu(&self) -> result!(MenuSize) {
        Ok(MenuSize::try_from(menu::scale_menu(self.handle)?)?)
    }

    pub fn set_current_item(&self, item: &MenuItem) -> result!(()) {
        Ok(menu::set_current_item(self.handle, item._handle())?)
    }

    pub fn set_item_init(&self, hook: Menu_Hook) -> result!(()) {
        Ok(menu::set_item_init(self.handle, hook)?)
    }

    pub fn set_item_term(&self, hook: Menu_Hook) -> result!(()) {
        Ok(menu::set_item_term(self.handle, hook)?)
    }

    pub fn set_menu_back(&self, attr: normal::Attributes) -> result!(()) {
        Ok(menu::set_menu_back(self.handle, attr)?)
    }

    pub fn set_menu_fore(&self, attr: normal::Attributes) -> result!(()) {
        Ok(menu::set_menu_fore(self.handle, attr)?)
    }

    pub fn set_menu_format(&self, menu_size: MenuSize) -> result!(()) {
        Ok(menu::set_menu_format(Some(self.handle), menu_size.try_into()?)?)
    }

    pub fn set_menu_grey(&self, attr: normal::Attributes) -> result!(()) {
        Ok(menu::set_menu_grey(self.handle, attr)?)
    }

    pub fn set_menu_init(&self, hook: Menu_Hook) -> result!(()) {
        Ok(menu::set_menu_init(self.handle, hook)?)
    }

    pub fn set_menu_items(&self, items: Vec<&MenuItem>) -> result!(()) {
        let item_handles = items.iter().map(|item| item._handle()).collect();

        Ok(menu::set_menu_items(self.handle, &item_handles)?)
    }

    pub fn set_menu_mark(&self, mark: &str) -> result!(()) {
        Ok(menu::set_menu_mark(self.handle, mark)?)
    }

    pub fn set_menu_opts(&self, opts: MenuOptions) -> result!(()) {
        Ok(menu::set_menu_opts(self.handle, opts)?)
    }

    pub fn set_menu_pad(&self, attr: normal::Attributes) -> result!(()) {
        Ok(menu::set_menu_pad(self.handle, attr)?)
    }

    pub fn set_menu_pattern(&self, pattern: &str) -> result!(()) {
        Ok(menu::set_menu_pattern(self.handle, pattern)?)
    }

    pub fn set_menu_spacing(&self, menu_spacing: MenuSpacing) -> result!(()) {
        Ok(menu::set_menu_spacing(self.handle, menu_spacing.try_into()?)?)
    }

    pub fn set_menu_sub(&self, win: Option<&Window>) -> result!(()) {
        Ok(menu::set_menu_sub(Some(self.handle), match win {
            Some(window) => Some(window._handle()),
            None         => None
        })?)
    }

    pub fn set_menu_term(&self, hook: Menu_Hook) -> result!(()) {
        Ok(menu::set_menu_term(self.handle, hook)?)
    }

    pub fn set_menu_userptr<T>(&self, ptr: Option<Box<&T>>) {
        menu::set_menu_userptr(self.handle, match ptr {
            Some(ptr) => Some(Box::into_raw(ptr) as *mut libc::c_void),
            None      => None
        })
    }

    pub fn set_menu_win(&self, win: Option<&Window>) -> result!(()) {
        Ok(menu::set_menu_win(Some(self.handle), match win {
            Some(window) => Some(window._handle()),
            None         => None
        })?)
    }

    pub fn set_top_row(&self, row: usize) -> result!(()) {
        Ok(menu::set_top_row(self.handle, i32::try_from(row)?)?)
    }

    pub fn top_row(&self) -> result!(usize) {
        Ok(usize::try_from(menu::top_row(self.handle))?)
    }
}

impl Drop for Menu {
    fn drop(&mut self) {
        if self.free_on_drop {
            if let Err(source) = menu::free_menu(self.handle) {
                panic!("{} @ ({:p})", source, self.handle)
            }
        }
    }
}

unsafe impl Send for Menu { } // too make thread safe
unsafe impl Sync for Menu { } // too make thread safe

impl PartialEq for Menu {
    fn eq(&self, rhs: &Self) -> bool {
        ptr::eq(self.handle, rhs.handle)
    }
}

impl Eq for Menu { }

impl fmt::Debug for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Menu {{ handle: {:p}, free_on_drop: {} }}", self.handle, self.free_on_drop)
    }
}
