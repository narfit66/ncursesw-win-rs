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

#![allow(clippy::forget_copy)]

use std::{
    ptr, fmt, mem, convert::{TryFrom, TryInto}, hash::{Hash, Hasher},
    collections::HashMap, sync::Mutex
};

use errno::errno;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use ncursesw::{normal, menu, menu::{MENU, ITEM, E_OK}, shims::nmenu};
use crate::{
    Window, HasHandle, NCurseswWinError,
    menu::{MenuSize, MenuItem, MenuSpacing, PostedMenu}
};

#[deprecated(since = "0.4.1")]
pub use ncursesw::menu::Menu_Hook;
pub use ncursesw::menu::{MenuOptions, MenuRequest};

static MODULE_PATH: &str = "ncurseswwin::menu::";

#[derive(EnumIter, PartialEq, Eq, Hash)]
enum CallbackType {
    ItemInit,
    ItemTerm,
    MenuInit,
    MenuTerm
}

#[derive(PartialEq, Eq, Hash)]
struct CallbackKey {
    menu:          MENU,
    callback_type: CallbackType
}

impl CallbackKey {
    fn new(menu: MENU, callback_type: CallbackType) -> Self {
        Self { menu, callback_type }
    }
}

unsafe impl Send for CallbackKey { }
unsafe impl Sync for CallbackKey { }

type CALLBACK = Option<Box<dyn Fn(&Menu) + Send>>;

lazy_static! {
    static ref CALLBACKS: Mutex<HashMap<CallbackKey, CALLBACK>> = Mutex::new(HashMap::new());
}

macro_rules! menu_callback {
    ($func: ident, $cb_t: ident) => {
        extern fn $func(menu: MENU) {
            if let Some(ref callback) = *CALLBACKS
                .lock()
                .unwrap_or_else(|_| panic!("{}{}({:p}) : *CALLBACKS.lock() failed!!!", MODULE_PATH, stringify!($func), menu))
                .get(&CallbackKey::new(menu, CallbackType::$cb_t))
                .unwrap_or_else(|| panic!("{}{}({:p}) : *CALLBACKS.lock().get() failed!!!", MODULE_PATH, stringify!($func), menu))
            {
                callback(&Menu::_from(menu, unsafe { (*menu).items }, false))
            } else {
                panic!("{}{}({:p}) : *CALLBACKS.lock().get() returned None!!!", MODULE_PATH, stringify!($func), menu)
            }
        }
    }
}

menu_callback!(extern_item_init, ItemInit);
menu_callback!(extern_item_term, ItemTerm);
menu_callback!(extern_menu_init, MenuInit);
menu_callback!(extern_menu_term, MenuTerm);

/// Menu.
pub struct Menu {
    handle:       MENU,       // pointer to ncurses menu item internal structure
    item_handles: *mut ITEM,  // double-pointer to allocated memory that the ncurses menu module uses for menu items.
    free_on_drop: bool
}

impl Menu {
    // make a new instance from the passed ncurses menu item pointer.
    fn _from(handle: MENU, item_handles: *mut ITEM, free_on_drop: bool) -> Self {
        assert!(!handle.is_null(), "Menu::_from() : handle.is_null()");
        assert!(!item_handles.is_null(), "Menu::_from() : item_handles.is_null()");

        Self { handle, item_handles, free_on_drop }
    }

    pub(in crate::menu) fn _handle(&self) -> MENU {
        self.handle
    }
}

impl Menu {
    pub fn new(items: &[&MenuItem]) -> result!(Self) {
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

            // call the ncursesw shims new_menu() function with our allocated memory.
            match unsafe { nmenu::new_menu(item_handles as *mut ITEM) } {
                Some(menu) => Ok(Self::_from(menu, item_handles, true)),
                None       => Err(NCurseswWinError::MenuError { source: menu::ncursesw_menu_error_from_rc("Menu::new", errno().into()) })
            }
        } else {
            Err(NCurseswWinError::OutOfMemory { func: "Menu::new".to_string() })
        }
    }

    #[deprecated(since = "0.4.0", note = "Use Menu::new() instead")]
    pub fn new_menu(items: &[&MenuItem]) -> result!(Self) {
        Self::new(items)
    }

    pub fn current_item(&self) -> result!(MenuItem) {
        Ok(MenuItem::_from(menu::current_item(self.handle)?, false))
    }

    pub fn item_count(&self) -> result!(usize) {
        Ok(usize::try_from(menu::item_count(self.handle)?)?)
    }

    #[deprecated(since = "0.4.1")]
    pub fn item_init(&self) -> result!(Menu_Hook) {
        Ok(menu::item_init(self.handle)?)
    }

    #[deprecated(since = "0.4.1")]
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

    #[deprecated(since = "0.4.1")]
    pub fn menu_init(&self) -> result!(Menu_Hook) {
        Ok(menu::menu_init(self.handle)?)
    }

    pub fn menu_items(&self) -> result!(Vec<MenuItem>) {
        Ok(menu::menu_items(self.handle)?.iter().map(|handle| MenuItem::_from(*handle, false)).collect())
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

    #[deprecated(since = "0.4.1")]
    pub fn menu_term(&self) -> result!(Menu_Hook) {
        Ok(menu::menu_term(self.handle)?)
    }

    // TODO: needs testing!
    pub fn menu_userptr<T>(&self) -> Option<Box<T>> {
        menu::menu_userptr(self.handle).as_mut().map(|ptr| unsafe { Box::from_raw(*ptr as *mut T) })
    }

    pub fn menu_win(&self) -> result!(Window) {
        Ok(Window::_from(menu::menu_win(self.handle)?, false))
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
        CALLBACKS
            .lock()
            .unwrap_or_else(|_| panic!("{}set_item_init() : CALLBACKS.lock() failed!!!", MODULE_PATH))
            .insert(CallbackKey::new(self.handle, CallbackType::ItemInit), Some(Box::new(move |menu| func(menu))));

        Ok(menu::set_item_init(self.handle, Some(extern_item_init))?)
    }

    pub fn set_item_term<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        CALLBACKS
            .lock()
            .unwrap_or_else(|_| panic!("{}set_item_term() : CALLBACKS.lock() failed!!!", MODULE_PATH))
            .insert(CallbackKey::new(self.handle, CallbackType::ItemTerm), Some(Box::new(move |menu| func(menu))));

        Ok(menu::set_item_term(self.handle, Some(extern_item_term))?)
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

    pub fn set_menu_init<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        CALLBACKS
            .lock()
            .unwrap_or_else(|_| panic!("{}set_menu_init() : CALLBACKS.lock() failed!!!", MODULE_PATH))
            .insert(CallbackKey::new(self.handle, CallbackType::MenuInit), Some(Box::new(move |menu| func(menu))));

        Ok(menu::set_menu_init(self.handle, Some(extern_menu_init))?)
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
        Ok(menu::set_menu_mark(self.handle, mark)?)
    }

    pub fn set_menu_opts(&self, opts: MenuOptions) -> result!(()) {
        Ok(menu::set_menu_opts(self.handle, opts)?)
    }

    pub fn set_menu_pad(&self, pad: char) -> result!(()) {
        Ok(menu::set_menu_pad(self.handle, pad)?)
    }

    pub fn set_menu_pattern(&self, pattern: &str) -> result!(()) {
        Ok(menu::set_menu_pattern(self.handle, pattern)?)
    }

    pub fn set_menu_spacing(&self, menu_spacing: MenuSpacing) -> result!(()) {
        Ok(menu::set_menu_spacing(self.handle, menu_spacing.try_into()?)?)
    }

    pub fn set_menu_sub(&self, window: Option<&Window>) -> result!(()) {
        Ok(menu::set_menu_sub(Some(self.handle), match window {
            Some(window) => Some(window._handle()),
            None         => None
        })?)
    }

    pub fn set_menu_term<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        CALLBACKS
            .lock()
            .unwrap_or_else(|_| panic!("{}set_menu_term() : CALLBACKS.lock() failed!!!", MODULE_PATH))
            .insert(CallbackKey::new(self.handle, CallbackType::MenuTerm), Some(Box::new(move |menu| func(menu))));

        Ok(menu::set_menu_term(self.handle, Some(extern_menu_term))?)
    }

    // TODO: needs testing!
    pub fn set_menu_userptr<T>(&self, ptr: Option<Box<&T>>) {
        menu::set_menu_userptr(self.handle, match ptr {
            Some(ptr) => Some(Box::into_raw(ptr) as *mut libc::c_void),
            None      => None
        })
    }

    pub fn set_menu_win(&self, window: Option<&Window>) -> result!(()) {
        Ok(menu::set_menu_win(Some(self.handle), match window {
            Some(window) => Some(window._handle()),
            None         => None
        })?)
    }

    pub fn set_top_row(&self, row: u16) -> result!(()) {
        Ok(menu::set_top_row(self.handle, i32::try_from(row)?)?)
    }

    pub fn top_row(&self) -> result!(u16) {
        Ok(u16::try_from(menu::top_row(self.handle))?)
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

            // remove any callbacks created for this instance.
            let mut callbacks = CALLBACKS
                .lock()
                .unwrap_or_else(|_| panic!("Menu::drop() : CALLBACKS.lock() failed!!!"));

            let mut shrink_to_fit = false;

            for cb_type in CallbackType::iter() {
                if callbacks.remove(&CallbackKey::new(self.handle, cb_type)).is_some() {
                    shrink_to_fit = true;
                }
            }

            if shrink_to_fit {
                callbacks.shrink_to_fit();
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
