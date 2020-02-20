/*
    src/form/callbacks.rs

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

use ncursesw::{SCREEN, form::FORM};
use crate::form::Form;

static MODULE_PATH: &str = "ncurseswwin::form::callbacks::";

#[derive(PartialEq, Eq, Hash)]
struct FormKey {
    form: FORM
}

impl FormKey {
    fn new(form: FORM) -> Self {
        Self { form }
    }
}

unsafe impl Send for FormKey { }
unsafe impl Sync for FormKey { }

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct FormValue {
    screen: Option<SCREEN>
}

impl FormValue {
    fn new(screen: Option<SCREEN>) -> Self {
        Self { screen }
    }

    fn screen(&self) -> Option<SCREEN> {
        self.screen
    }
}

unsafe impl Send for FormValue { }
unsafe impl Sync for FormValue { }

#[derive(EnumIter, PartialEq, Eq, Hash)]
pub(in crate::form) enum CallbackType {
    FieldInit,
    FieldTerm,
    FormInit,
    FormTerm
}

#[derive(PartialEq, Eq, Hash)]
struct CallbackKey {
    form:          Option<FORM>,
    callback_type: CallbackType
}

impl CallbackKey {
    fn new(form: Option<FORM>, callback_type: CallbackType) -> Self {
        Self { form, callback_type }
    }
}

unsafe impl Send for CallbackKey { }
unsafe impl Sync for CallbackKey { }

type CALLBACK = Option<Box<dyn Fn(&Form) + Send>>;

lazy_static! {
    static ref FORMSCREENS: Mutex<HashMap<FormKey, FormValue>> = Mutex::new(HashMap::new());
    static ref CALLBACKS: Mutex<HashMap<CallbackKey, CALLBACK>> = Mutex::new(HashMap::new());
}

macro_rules! form_callback {
    ($func: ident, $cb_t: ident) => {
        pub(in crate::form) extern fn $func(form: FORM) {
            let callback_form = || -> Form { Form::_from(get_form_screen(form), form, unsafe { (*form).field }, false) };

            let callbacks = CALLBACKS
                .lock()
                .unwrap_or_else(|_| panic!("{}{}({:p}) : CALLBACKS.lock() failed!!!", MODULE_PATH, stringify!($func), form));

            if let Some(ref callback) = callbacks
                .get(&CallbackKey::new(Some(form), CallbackType::$cb_t))
                .unwrap_or_else(|| &None)
            {
                callback(&callback_form())
            } else if let Some(ref callback) = callbacks
                .get(&CallbackKey::new(None, CallbackType::$cb_t))
                .unwrap_or_else(|| &None)
            {
                callback(&callback_form())
            } else {
                panic!("{}{}({:p}) : callbacks.lock().get() returned None!!!", MODULE_PATH, stringify!($func), form)
            }
        }
    }
}

form_callback!(extern_field_init, FieldInit);
form_callback!(extern_field_term, FieldTerm);
form_callback!(extern_form_init, FormInit);
form_callback!(extern_form_term, FormTerm);

pub(in crate::form) fn set_form_screen(form: FORM, screen: Option<SCREEN>) {
    FORMSCREENS
        .lock()
        .unwrap_or_else(|_| panic!("{}get_form_screen({:p}) : FORMSCREENS.lock() failed!!!", MODULE_PATH, form))
        .insert(FormKey::new(form), FormValue::new(screen));
}

pub(in crate::form) fn get_form_screen(form: FORM) -> Option<SCREEN> {
    FORMSCREENS
        .lock()
        .unwrap_or_else(|_| panic!("{}get_form_screen({:p}) : FORMSCREENS.lock() failed!!!", MODULE_PATH, form))
        .get(&FormKey::new(form))
        .unwrap_or_else(|| panic!("{}get_form_screen({:p}) : FORMSCREENS.lock().get() failed!!!", MODULE_PATH, form))
        .screen()
}

pub(in crate::form) fn set_form_callback<F>(form: Option<FORM>, cb_type: CallbackType, func: F)
    where F: Fn(&Form) + 'static + Send
{
    CALLBACKS
        .lock()
        .unwrap_or_else(|_| panic!("{}set_form_callback() : CALLBACKS.lock() failed!!!", MODULE_PATH))
        .insert(CallbackKey::new(form, cb_type), Some(Box::new(move |form| func(form))));
}

pub(in crate::form) fn form_tidyup(form: FORM) {
    FORMSCREENS
        .lock()
        .unwrap_or_else(|_| panic!("{}form_tidyup({:p}) : FORMSCREENS.lock() failed!!!", MODULE_PATH, form))
        .remove(&FormKey::new(form));

    let mut callbacks = CALLBACKS
        .lock()
        .unwrap_or_else(|_| panic!("{}form_tidyup({:p}) : CALLBACKS.lock() failed!!!", MODULE_PATH, form));

    let mut shrink_to_fit = false;

    for cb_type in CallbackType::iter() {
        if callbacks.remove(&CallbackKey::new(Some(form), cb_type)).is_some() {
            shrink_to_fit = true;
        }
    }

    if shrink_to_fit {
        callbacks.shrink_to_fit();
    }
}
