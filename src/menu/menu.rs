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

use ncursesw::menu;
use ncursesw::normal;
use ncursesw::menu::MENU;

use crate::{Window, NCurseswWinError, menu::MenuItem};
use crate::gen::HasHandle;

pub use ncursesw::menu::{
    MenuHook, MenuOptions, MenuRequest, MenuSpacing, MenuSize
};

/// Menu.
pub struct Menu {
    handle:       MENU, // pointer to ncurses menu item internal structure
    free_on_drop: bool
}

impl Menu {
    // make a new instance from the passed ncurses menu item pointer.
    fn _from(handle: MENU, free_on_drop: bool) -> Self {
        Self { handle, free_on_drop }
    }

    fn _handle(&self) -> MENU {
        self.handle
    }
}

impl Menu {
    pub fn new(items: Vec<&MenuItem>) -> result!(Self) {
        let mut item_handles = vec!();

        for item in items {
            item_handles.push(item._handle())
        }

        let handle = menu::new_menu(item_handles)?;

        Ok(Self::_from(handle, true))
    }

    #[deprecated(since = "0.3.2", note = "Use MenuItem::new() instead")]
    pub fn new_item(items: Vec<&MenuItem>) -> result!(Self) {
        Self::new(items)
    }

    pub fn current_item(&self) -> result!(MenuItem) {
        let handle = menu::current_item(self.handle)?;

        Ok(MenuItem::_from(handle, false))
    }

    pub fn item_count(&self) -> result!(i32) {
        let count = menu::item_count(self.handle)?;

        Ok(count)
    }

    pub fn item_init(&self) -> result!(MenuHook) {
        let func =  menu::item_init(self.handle)?;

        Ok(func)
    }

    pub fn item_term(&self) -> result!(MenuHook) {
        let func = menu::item_term(self.handle)?;

        Ok(func)
    }

    pub fn menu_back(&self) -> normal::Attributes {
        menu::menu_back(self.handle)
    }

    pub fn menu_driver(&self, request: MenuRequest) -> result!(Option<i32>) {
        let item_count = menu::menu_driver(self.handle, request)?;

        Ok(item_count)
    }

    pub fn menu_fore(&self) -> normal::Attributes {
        menu::menu_fore(self.handle)
    }

    pub fn menu_format(&self) -> MenuSize {
        menu::menu_format(self.handle)
    }

    pub fn menu_grey(&self) -> normal::Attributes {
        menu::menu_grey(self.handle)
    }

    pub fn menu_init(&self) -> result!(MenuHook) {
        let func = menu::menu_init(self.handle)?;

        Ok(func)
    }

    pub fn menu_items(&self) -> Option<Vec<MenuItem>> {
        match menu::menu_items(self.handle) {
            Some(handles) => {
                let mut menu_items = vec!();

                for handle in handles {
                    menu_items.push(MenuItem::_from(handle, false))
                }

                Some(menu_items)
            },
            None          => None
        }
    }

    pub fn menu_mark(&self) -> Option<String> {
        menu::menu_mark(self.handle)
    }

    pub fn menu_opts(&self) -> MenuOptions {
        menu::menu_opts(self.handle)
    }

    pub fn menu_opts_off(&self, opts: MenuOptions) -> result!(()) {
        menu::menu_opts_off(self.handle, opts)?;

        Ok(())
    }

    pub fn menu_opts_on(&self, opts: MenuOptions) -> result!(()) {
        menu::menu_opts_on(self.handle, opts)?;

        Ok(())
    }

    pub fn menu_pad(&self) -> char {
        menu::menu_pad(self.handle)
    }

    pub fn menu_pattern(&self) -> Option<String> {
        menu::menu_pattern(self.handle)
    }

    pub fn menu_spacing(&self) -> result!(MenuSpacing) {
        let menu_spacing = menu::menu_spacing(self.handle)?;

        Ok(menu_spacing)
    }

    pub fn menu_sub(&self) -> result!(Window) {
        let handle = menu::menu_sub(self.handle)?;

        Ok(Window::_from(handle, false))
    }

    pub fn menu_term(&self) -> result!(MenuHook) {
        let func = menu::menu_term(self.handle)?;

        Ok(func)
    }

    pub fn menu_userptr<T>(&self) -> Option<Box<T>> {
        match menu::menu_userptr(self.handle) {
            Some(ptr) => Some(unsafe { Box::from_raw(ptr as *mut T) }),
            None      => None
        }
    }

    pub fn pos_menu_cursor(&self) -> result!(()) {
        menu::pos_menu_cursor(self.handle)?;

        Ok(())
    }

    pub fn post_menu(&self) -> result!(()) {
        menu::post_menu(self.handle)?;

        Ok(())
    }

    pub fn scale_menu(&self) -> result!(MenuSize) {
        let menu_size = menu::scale_menu(self.handle)?;

        Ok(menu_size)
    }

    pub fn set_current_item(&self, item: &MenuItem) -> result!(()) {
        menu::set_current_item(self.handle, item._handle())?;

        Ok(())
    }

    pub fn set_item_init(&self, hook: MenuHook) -> result!(()) {
        menu::set_item_init(self.handle, hook)?;

        Ok(())
    }

    pub fn set_item_term(&self, hook: MenuHook) -> result!(()) {
        menu::set_item_term(self.handle, hook)?;

        Ok(())
    }

    pub fn set_menu_back(&self, attr: normal::Attributes) -> result!(()) {
        menu::set_menu_back(self.handle, attr)?;

        Ok(())
    }

    pub fn set_menu_fore(&self, attr: normal::Attributes) -> result!(()) {
        menu::set_menu_fore(self.handle, attr)?;

        Ok(())
    }

    pub fn set_menu_format(&self, menu_size: MenuSize) -> result!(()) {
        menu::set_menu_format(Some(self.handle), menu_size)?;

        Ok(())
    }

    pub fn set_menu_grey(&self, attr: normal::Attributes) -> result!(()) {
        menu::set_menu_grey(self.handle, attr)?;

        Ok(())
    }

    pub fn set_menu_init(&self, hook: MenuHook) -> result!(()) {
        menu::set_menu_init(self.handle, hook)?;

        Ok(())
    }

    pub fn set_menu_items(&self, items: Vec<&MenuItem>) -> result!(()) {
        let mut handles = vec!();

        for item in items {
            handles.push(item._handle())
        }

        menu::set_menu_items(self.handle, handles)?;

        Ok(())
    }

    pub fn set_menu_mark(&self, mark: &str) -> result!(()) {
        menu::set_menu_mark(self.handle, mark)?;

        Ok(())
    }

    pub fn set_menu_opts(&self, opts: MenuOptions) -> result!(()) {
        menu::set_menu_opts(self.handle, opts)?;

        Ok(())
    }

    pub fn set_menu_pad(&self, attr: normal::Attributes) -> result!(()) {
        menu::set_menu_pad(self.handle, attr)?;

        Ok(())
    }

    pub fn set_menu_pattern(&self, pattern: &str) -> result!(()) {
        menu::set_menu_pattern(self.handle, pattern)?;

        Ok(())
    }

    pub fn set_menu_spacing(&self, menu_spacing: MenuSpacing) -> result!(()) {
        menu::set_menu_spacing(self.handle, menu_spacing)?;

        Ok(())
    }

    pub fn set_menu_sub(&self, win: &Window) -> result!(()) {
        menu::set_menu_sub(self.handle, Some(win._handle()))?;

        Ok(())
    }

    pub fn set_menu_term(&self, hook: MenuHook) -> result!(()) {
        menu::set_menu_term(self.handle, hook)?;

        Ok(())
    }

    pub fn set_menu_userptr<T>(&self, ptr: Option<Box<&T>>) {
        menu::set_menu_userptr(self.handle, match ptr {
            Some(ptr) => Some(Box::into_raw(ptr) as *mut libc::c_void),
            None      => None
        })
    }

    pub fn set_menu_win(&self, win: &Window) -> result!(()) {
        menu::set_menu_win(self.handle, Some(win._handle()))?;

        Ok(())
    }

    pub fn set_top_row(&self, row: i32) -> result!(()) {
        menu::set_top_row(self.handle, row)?;

        Ok(())
    }

    pub fn top_row(&self) -> i32 {
        menu::top_row(self.handle)
    }

    pub fn unpost_menu(&self) -> result!(()) {
        menu::unpost_menu(self.handle)?;

        Ok(())
    }
}

impl Drop for Menu {
    fn drop(&mut self) {
        if self.free_on_drop {
            if let Err(e) = menu::free_menu(self.handle) {
                panic!(e.to_string())
            }
        }
    }
}

unsafe impl Send for Menu { } // too make thread safe
unsafe impl Sync for Menu { } // too make thread safe
