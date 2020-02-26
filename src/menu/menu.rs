/*
    src/menu/menu.rs

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

#![allow(clippy::forget_copy)]

use std::{ptr, fmt, mem, convert::{TryFrom, TryInto}, hash::{Hash, Hasher}};

use errno::errno;

use ncursesw::{
    SCREEN, menu, menu::{MENU, ITEM},
    shims::nmenu, shims::constants::E_OK
};
use crate::{
    normal, Screen, Window, HasHandle, NCurseswWinError, AttributesType,
    menu::{
        MenuSize, MenuItem, MenuSpacing, PostedMenu,
        callbacks::{
            CallbackType, set_menu_screen, set_menu_callback, extern_item_init,
            extern_item_term, extern_menu_init, extern_menu_term, menu_tidyup
        }
    }
};

#[deprecated(since = "0.4.1")]
pub use ncursesw::menu::Menu_Hook;
pub use ncursesw::menu::{MenuOptions, MenuRequest};

/// Menu.
pub struct Menu {
    screen:       Option<SCREEN>, // pointer to optional NCurses screen internal structure.
    handle:       MENU,           // pointer to NCurses menu item internal structure.
    item_handles: *mut ITEM,      // double-pointer to allocated memory that the ncurses menu module uses for menu items.
    free_on_drop: bool
}

impl Menu {
    // make a new instance from the passed ncurses menu item pointer.
    pub(in crate::menu) fn _from(screen: Option<SCREEN>, handle: MENU, item_handles: *mut ITEM, free_on_drop: bool) -> Self {
        assert!(screen.map_or_else(|| true, |screen| !screen.is_null()), "Menu::_from() : screen.is_null()");
        assert!(!handle.is_null(), "Menu::_from() : handle.is_null()");
        assert!(!item_handles.is_null(), "Menu::_from() : item_handles.is_null()");

        if free_on_drop {
            set_menu_screen(handle, screen);
        }

        Self { screen, handle, item_handles, free_on_drop }
    }

    pub(in crate::menu) fn _new<F>(screen: Option<&Screen>, items: &[&MenuItem], new_func: F) -> result!(Self)
        where F: Fn(*mut ITEM) -> result!(Self)
    {
        // allocate enougth contiguous memory to store all the menu item handles plus
        // a null and set all pointers initially to null.
        let item_handles = unsafe { libc::calloc(items.len() + 1, mem::size_of::<ITEM>()) as *mut ITEM };

        // check that we we're able to allocate our memory.
        if !item_handles.is_null() {
            // get all the menu item handles and write them to memory.
            for (offset, item_handle) in items.iter().map(|item| item._handle()).enumerate() {
                unsafe { ptr::write(item_handles.offset(isize::try_from(offset)?), item_handle) };
            }

            // don't unallocate item_handles when it goes out of scope, we'll do it
            // ourselves as self.item_handles will point to our contiguous memory.
            mem::forget(item_handles);

            new_func(item_handles)
        } else {
            Err(NCurseswWinError::OutOfMemory { func: format!("Menu::{}", screen.map_or_else(|| "new", |_| "new_sp")) })
        }
    }

    pub(in crate::menu) fn _screen(&self) -> Option<SCREEN> {
        self.screen
    }

    pub(in crate::menu) fn _handle(&self) -> MENU {
        self.handle
    }
}

impl Menu {
    pub fn new(items: &[&MenuItem]) -> result!(Self) {
        Self::_new(None, items, |item_handles| {
            // call the ncursesw shims new_menu() function with our allocated memory.
            match unsafe { nmenu::new_menu(item_handles) } {
                Some(menu) => Ok(Self::_from(None, menu, item_handles, true)),
                None       => Err(NCurseswWinError::MenuError { source: menu::ncursesw_menu_error_from_rc("Menu::new", errno().into()) })
            }
        })
    }

    #[deprecated(since = "0.4.0", note = "Use Menu::new() instead")]
    pub fn new_menu(items: &[&MenuItem]) -> result!(Self) {
        Self::new(items)
    }

    pub fn new_sp(screen: &Screen, items: &[&MenuItem]) -> result!(Self) {
        Self::_new(Some(screen), items, |item_handles| {
            // call the ncursesw shims new_menu_sp() function with our allocated memory.
            match unsafe { nmenu::new_menu_sp(screen._handle(), item_handles) } {
                Some(menu) => Ok(Self::_from(Some(screen._handle()), menu, item_handles, true)),
                None       => Err(NCurseswWinError::MenuError { source: menu::ncursesw_menu_error_from_rc("Menu::new_sp", errno().into()) })
            }
        })
    }

    #[deprecated(since = "0.5.0", note = "Use Menu::new_sp() instead")]
    pub fn new_menu_sp(screen: &Screen, items: &[&MenuItem]) -> result!(Self) {
        Self::new_sp(screen, items)
    }

    /// Return the screen associated with the menu.
    pub fn screen(&self) -> Option<Screen> {
        self.screen.and_then(|screen| Some(Screen::_from(screen, false)))
    }

    pub fn current_item(&self) -> result!(MenuItem) {
        Ok(MenuItem::_from(menu::current_item(self.handle)?, false))
    }

    pub fn item_count(&self) -> result!(usize) {
        Ok(usize::try_from(menu::item_count(self.handle)?)?)
    }

    #[deprecated(since = "0.5.0")]
    pub fn item_init(&self) -> result!(Menu_Hook) {
        Ok(menu::item_init(Some(self.handle))?)
    }

    #[deprecated(since = "0.5.0")]
    pub fn item_term(&self) -> result!(Menu_Hook) {
        Ok(menu::item_term(Some(self.handle))?)
    }

    pub fn menu_back(&self) -> normal::Attributes {
        self.screenify_attributes(menu::menu_back(Some(self.handle)))
    }

    pub fn menu_fore(&self) -> normal::Attributes {
        self.screenify_attributes(menu::menu_fore(Some(self.handle)))
    }

    pub fn menu_format(&self) -> result!(MenuSize) {
        Ok(MenuSize::try_from(menu::menu_format(Some(self.handle)))?)
    }

    pub fn menu_grey(&self) -> normal::Attributes {
        self.screenify_attributes(menu::menu_grey(Some(self.handle)))
    }

    #[deprecated(since = "0.5.0")]
    pub fn menu_init(&self) -> result!(Menu_Hook) {
        Ok(menu::menu_init(Some(self.handle))?)
    }

    pub fn menu_items(&self) -> result!(Vec<MenuItem>) {
        Ok(menu::menu_items(self.handle)?.iter().map(|handle| MenuItem::_from(*handle, false)).collect())
    }

    pub fn menu_mark(&self) -> result!(String) {
        Ok(menu::menu_mark(Some(self.handle))?)
    }

    pub fn menu_opts(&self) -> MenuOptions {
        menu::menu_opts(Some(self.handle))
    }

    pub fn menu_opts_off(&self, opts: MenuOptions) -> result!(()) {
        Ok(menu::menu_opts_off(Some(self.handle), opts)?)
    }

    pub fn menu_opts_on(&self, opts: MenuOptions) -> result!(()) {
        Ok(menu::menu_opts_on(Some(self.handle), opts)?)
    }

    pub fn menu_pad(&self) -> result!(char) {
        Ok(menu::menu_pad(Some(self.handle))?)
    }

    pub fn menu_pattern(&self) -> result!(String) {
        Ok(menu::menu_pattern(self.handle)?)
    }

    pub fn menu_spacing(&self) -> result!(MenuSpacing) {
        Ok(MenuSpacing::try_from(menu::menu_spacing(Some(self.handle))?)?)
    }

    pub fn menu_sub(&self) -> result!(Window) {
        Ok(Window::_from(self.screen, menu::menu_sub(Some(self.handle))?, false))
    }

    #[deprecated(since = "0.5.0")]
    pub fn menu_term(&self) -> result!(Menu_Hook) {
        Ok(menu::menu_term(Some(self.handle))?)
    }

    // TODO: needs testing!
    pub fn menu_userptr<T>(&self) -> Option<Box<T>> {
        menu::menu_userptr(Some(self.handle)).as_mut().map(|userptr| unsafe { Box::from_raw(*userptr as *mut T) })
    }

    pub fn menu_win(&self) -> result!(Window) {
        Ok(Window::_from(self.screen, menu::menu_win(Some(self.handle))?, false))
    }

    pub fn post_menu(&self, refresh: bool) -> result!(PostedMenu) {
        PostedMenu::new(self, refresh)
    }

    pub fn scale_menu(&self) -> result!(MenuSize) {
        Ok(MenuSize::try_from(menu::scale_menu(self.handle)?)?)
    }

    pub fn set_current_item(&self, item: &MenuItem) -> result!(()) {
        Ok(menu::set_current_item(self.handle, item._handle())?)
    }

    pub fn set_item_init<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        set_menu_callback(Some(self.handle), CallbackType::ItemInit, func);

        Ok(menu::set_item_init(Some(self.handle), Some(extern_item_init))?)
    }

    pub fn set_item_term<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        set_menu_callback(Some(self.handle), CallbackType::ItemTerm, func);

        Ok(menu::set_item_term(Some(self.handle), Some(extern_item_term))?)
    }

    pub fn set_menu_back(&self, attrs: normal::Attributes) -> result!(()) {
        assert!(self.screen == attrs.screen());

        Ok(menu::set_menu_back(Some(self.handle), attrs)?)
    }

    pub fn set_menu_fore(&self, attrs: normal::Attributes) -> result!(()) {
        assert!(self.screen == attrs.screen());

        Ok(menu::set_menu_fore(Some(self.handle), attrs)?)
    }

    pub fn set_menu_format(&self, menu_size: MenuSize) -> result!(()) {
        Ok(menu::set_menu_format(Some(self.handle), menu_size.try_into()?)?)
    }

    pub fn set_menu_grey(&self, attrs: normal::Attributes) -> result!(()) {
        assert!(self.screen == attrs.screen());

        Ok(menu::set_menu_grey(Some(self.handle), attrs)?)
    }

    pub fn set_menu_init<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        set_menu_callback(Some(self.handle), CallbackType::MenuInit, func);

        Ok(menu::set_menu_init(Some(self.handle), Some(extern_menu_init))?)
    }

    pub fn set_menu_items(&mut self, items: &[&MenuItem]) -> result!(()) {
        // unallocate the previous item_handles memory.
        unsafe { libc::free(self.item_handles as *mut libc::c_void) };

        // allocate enougth memory to store all the menu item handles plus
        // a null and set all pointers initially to null.
        let item_handles = unsafe { libc::calloc(items.len() + 1, mem::size_of::<ITEM>()) as *mut ITEM };

        if !item_handles.is_null() {
            // get all the menu item handles and write them to memory.
            for (offset, item_handle) in items.iter().map(|item| item._handle()).enumerate() {
                unsafe { ptr::write(item_handles.offset(isize::try_from(offset)?), item_handle) };
            }

            // don't unallocate item_handles so it can be assigned to self.item_handles
            mem::forget(item_handles);

            self.item_handles = item_handles;

            // call the ncursesw shims set_menu_items() function with our allocated memory.
            match unsafe { nmenu::set_menu_items(self.handle, item_handles as *mut ITEM) } {
                E_OK => Ok(()),
                rc   => Err(NCurseswWinError::MenuError { source: menu::ncursesw_menu_error_from_rc("nmenu::set_menu_items", rc) })
            }
        } else {
            Err(NCurseswWinError::OutOfMemory { func: "Menu::set_menu_items".to_string() })
        }
    }

    pub fn set_menu_mark(&self, mark: &str) -> result!(()) {
        Ok(menu::set_menu_mark(Some(self.handle), mark)?)
    }

    pub fn set_menu_opts(&self, opts: MenuOptions) -> result!(()) {
        Ok(menu::set_menu_opts(Some(self.handle), opts)?)
    }

    pub fn set_menu_pad(&self, pad: char) -> result!(()) {
        Ok(menu::set_menu_pad(Some(self.handle), pad)?)
    }

    pub fn set_menu_pattern(&self, pattern: &str) -> result!(()) {
        Ok(menu::set_menu_pattern(self.handle, pattern)?)
    }

    pub fn set_menu_spacing(&self, menu_spacing: MenuSpacing) -> result!(()) {
        Ok(menu::set_menu_spacing(Some(self.handle), menu_spacing.try_into()?)?)
    }

    pub fn set_menu_sub(&self, window: Option<&Window>) -> result!(()) {
        assert!(self.screen == window.and_then(|window| window._screen()));

        Ok(menu::set_menu_sub(Some(self.handle), window.and_then(|window| Some(window._handle())))?)
    }

    pub fn set_menu_term<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        set_menu_callback(Some(self.handle), CallbackType::MenuTerm, func);

        Ok(menu::set_menu_term(Some(self.handle), Some(extern_menu_term))?)
    }

    // TODO: needs testing!
    pub fn set_menu_userptr<T>(&self, userptr: Option<Box<&T>>) {
        menu::set_menu_userptr(Some(self.handle), userptr.and_then(|userptr| Some(Box::into_raw(userptr) as *mut libc::c_void)))
    }

    pub fn set_menu_win(&self, window: Option<&Window>) -> result!(()) {
        assert!(self.screen == window.and_then(|window| window._screen()));

        Ok(menu::set_menu_win(Some(self.handle), window.and_then(|window| Some(window._handle())))?)
    }

    pub fn set_top_row(&self, row: u16) -> result!(()) {
        Ok(menu::set_top_row(self.handle, i32::try_from(row)?)?)
    }

    pub fn top_row(&self) -> result!(u16) {
        Ok(u16::try_from(menu::top_row(self.handle))?)
    }

    fn screenify_attributes(&self, attrs: normal::Attributes) -> normal::Attributes {
        self.screen.map_or_else(|| attrs, |screen| normal::Attributes::new_sp(screen, attrs.into()))
    }
}

impl Drop for Menu {
    fn drop(&mut self) {
        if self.free_on_drop {
            // free the menu.
            if let Err(source) = menu::free_menu(self.handle) {
                panic!("{} @ {:?}", source, self)
            }

            // unallocate the item_handles memory.
            unsafe { libc::free(self.item_handles as *mut libc::c_void) };

            menu_tidyup(self.handle);
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

impl Hash for Menu {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl fmt::Debug for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Menu {{ handle: {:p}, item_handles: {:p}, free_on_drop: {} }}", self.handle, self.item_handles, self.free_on_drop)
    }
}
