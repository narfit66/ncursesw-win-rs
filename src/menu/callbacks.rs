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

use strum_macros::EnumIter;

use ncursesw::menu::MENU;
use crate::menu::Menu;

static MODULE_PATH: &str = "ncurseswwin::menu::callbacks::";

#[derive(EnumIter, PartialEq, Eq, Hash)]
pub(in crate::menu) enum CallbackType {
    ItemInit,
    ItemTerm,
    MenuInit,
    MenuTerm
}

#[derive(PartialEq, Eq, Hash)]
pub(in crate::menu) struct CallbackKey {
    menu:          Option<MENU>,
    callback_type: CallbackType
}

impl CallbackKey {
    pub(in crate::menu) fn new(menu: Option<MENU>, callback_type: CallbackType) -> Self {
        Self { menu, callback_type }
    }
}

unsafe impl Send for CallbackKey { }
unsafe impl Sync for CallbackKey { }

type CALLBACK = Option<Box<dyn Fn(&Menu) + Send>>;

lazy_static! {
    pub(in crate::menu) static ref CALLBACKS: Mutex<HashMap<CallbackKey, CALLBACK>> = Mutex::new(HashMap::new());
}

macro_rules! menu_callback {
    ($func: ident, $cb_t: ident) => {
        pub(in crate::menu) extern fn $func(menu: MENU) {
            let callbacks = CALLBACKS
                .lock()
                .unwrap_or_else(|_| panic!("{}{}({:p}) : CALLBACKS.lock() failed!!!", MODULE_PATH, stringify!($func), menu));

            if let Some(ref callback) = callbacks
                .get(&CallbackKey::new(Some(menu), CallbackType::$cb_t))
                .unwrap_or_else(|| &None)
            {
                callback(&Menu::_from(None, menu, unsafe { (*menu).items }, false))
            } else if let Some(ref callback) = callbacks                             // default does not work yet see src/menu/funcs.rs!!!
                .get(&CallbackKey::new(None, CallbackType::$cb_t))
                .unwrap_or_else(|| &None)
            {
                callback(&Menu::_from(None, menu, unsafe { (*menu).items }, false))
            } else {
                panic!("{}{}({:p}) : *callbacks.lock().get() returned None!!!", MODULE_PATH, stringify!($func), menu)
            }
        }
    }
}

menu_callback!(extern_item_init, ItemInit);
menu_callback!(extern_item_term, ItemTerm);
menu_callback!(extern_menu_init, MenuInit);
menu_callback!(extern_menu_term, MenuTerm);
