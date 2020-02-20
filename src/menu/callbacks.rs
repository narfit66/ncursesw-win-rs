/*
    src/menu/callbacks.rs

    Copyright (c) 2020 Stephen Whittle  All rights reserved.

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

use std::{collections::HashMap, sync::Mutex};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use ncursesw::{SCREEN, menu::MENU};
use crate::menu::Menu;

static MODULE_PATH: &str = "ncurseswwin::menu::callbacks::";

#[derive(PartialEq, Eq, Hash)]
struct MenuKey {
    menu: MENU
}

impl MenuKey {
    fn new(menu: MENU) -> Self {
        Self { menu }
    }
}

unsafe impl Send for MenuKey { }
unsafe impl Sync for MenuKey { }

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct MenuValue {
    screen: Option<SCREEN>
}

impl MenuValue {
    fn new(screen: Option<SCREEN>) -> Self {
        Self { screen }
    }

    fn screen(&self) -> Option<SCREEN> {
        self.screen
    }
}

unsafe impl Send for MenuValue { }
unsafe impl Sync for MenuValue { }

#[derive(EnumIter, PartialEq, Eq, Hash)]
pub(in crate::menu) enum CallbackType {
    ItemInit,
    ItemTerm,
    MenuInit,
    MenuTerm
}

#[derive(PartialEq, Eq, Hash)]
struct CallbackKey {
    menu:          Option<MENU>,
    callback_type: CallbackType
}

impl CallbackKey {
    fn new(menu: Option<MENU>, callback_type: CallbackType) -> Self {
        Self { menu, callback_type }
    }
}

unsafe impl Send for CallbackKey { }
unsafe impl Sync for CallbackKey { }

type CALLBACK = Option<Box<dyn Fn(&Menu) + Send>>;

lazy_static! {
    static ref MENUSCREENS: Mutex<HashMap<MenuKey, MenuValue>> = Mutex::new(HashMap::new());
    static ref CALLBACKS: Mutex<HashMap<CallbackKey, CALLBACK>> = Mutex::new(HashMap::new());
}

macro_rules! menu_callback {
    ($func: ident, $cb_t: ident) => {
        pub(in crate::menu) extern fn $func(menu: MENU) {
            let callback_menu = || -> Menu { Menu::_from(get_menu_screen(menu), menu, unsafe { (*menu).items }, false) };

            let callbacks = CALLBACKS
                .lock()
                .unwrap_or_else(|_| panic!("{}{}({:p}) : CALLBACKS.lock() failed!!!", MODULE_PATH, stringify!($func), menu));

            if let Some(ref callback) = callbacks
                .get(&CallbackKey::new(Some(menu), CallbackType::$cb_t))
                .unwrap_or_else(|| &None)
            {
                callback(&callback_menu())
            } else if let Some(ref callback) = callbacks
                .get(&CallbackKey::new(None, CallbackType::$cb_t))
                .unwrap_or_else(|| &None)
            {
                callback(&callback_menu())
            } else {
                panic!("{}{}({:p}) : callbacks.lock().get() returned None!!!", MODULE_PATH, stringify!($func), menu)
            }
        }
    }
}

menu_callback!(extern_item_init, ItemInit);
menu_callback!(extern_item_term, ItemTerm);
menu_callback!(extern_menu_init, MenuInit);
menu_callback!(extern_menu_term, MenuTerm);

pub(in crate::menu) fn set_menu_screen(menu: MENU, screen: Option<SCREEN>) {
    MENUSCREENS
        .lock()
        .unwrap_or_else(|_| panic!("{}get_menu_screen({:p}) : MENUSCREENS.lock() failed!!!", MODULE_PATH, menu))
        .insert(MenuKey::new(menu), MenuValue::new(screen));
}

pub(in crate::menu) fn get_menu_screen(menu: MENU) -> Option<SCREEN> {
    MENUSCREENS
        .lock()
        .unwrap_or_else(|_| panic!("{}get_menu_screen({:p}) : MENUSCREENS.lock() failed!!!", MODULE_PATH, menu))
        .get(&MenuKey::new(menu))
        .unwrap_or_else(|| panic!("{}get_menu_screen({:p}) : MENUSCREENS.lock().get() failed!!!", MODULE_PATH, menu))
        .screen()
}

pub(in crate::menu) fn set_menu_callback<F>(menu: Option<MENU>, cb_type: CallbackType, func: F)
    where F: Fn(&Menu) + 'static + Send
{
    CALLBACKS
        .lock()
        .unwrap_or_else(|_| panic!("{}set_menu_callback() : CALLBACKS.lock() failed!!!", MODULE_PATH))
        .insert(CallbackKey::new(menu, cb_type), Some(Box::new(move |menu| func(menu))));
}

pub(in crate::menu) fn menu_tidyup(menu: MENU) {
    MENUSCREENS
        .lock()
        .unwrap_or_else(|_| panic!("{}menu_tidyup({:p}) : MENUSCREENS.lock() failed!!!", MODULE_PATH, menu))
        .remove(&MenuKey::new(menu));

    let mut callbacks = CALLBACKS
        .lock()
        .unwrap_or_else(|_| panic!("{}menu_tidyup({:p}) : CALLBACKS.lock() failed!!!", MODULE_PATH, menu));

    let mut shrink_to_fit = false;

    for cb_type in CallbackType::iter() {
        if callbacks.remove(&CallbackKey::new(Some(menu), cb_type)).is_some() {
            shrink_to_fit = true;
        }
    }

    if shrink_to_fit {
        callbacks.shrink_to_fit();
    }
}
