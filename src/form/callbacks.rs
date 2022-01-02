/*
    src/form/callbacks.rs

    Copyright (c) 2020-2022 Stephen Whittle  All rights reserved.

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

#[derive(Clone, Copy, EnumIter, Debug, PartialEq, Eq, Hash)]
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

type Callback = Option<Box<dyn Fn(&Form) + Send>>;

lazy_static! {
    static ref FORMSCREENS: Mutex<HashMap<FormKey, FormValue>> = Mutex::new(HashMap::new());
    static ref CALLBACKS: Mutex<HashMap<CallbackKey, Callback>> = Mutex::new(HashMap::new());
}

macro_rules! extern_form_callback {
    ($func: ident, $cb_t: ident) => {
        pub(in crate::form) extern fn $func(form: FORM) {
            form_callback(form, CallbackType::$cb_t)
        }
    }
}

extern_form_callback!(extern_field_init, FieldInit);
extern_form_callback!(extern_field_term, FieldTerm);
extern_form_callback!(extern_form_init, FormInit);
extern_form_callback!(extern_form_term, FormTerm);

fn form_callback(form: FORM, cb_type: CallbackType) {
    let get_form = || -> Form {
        let screen = FORMSCREENS
            .lock()
            .unwrap_or_else(|_| panic!("{}form_callback({:p}, {:?}) : FORMSCREENS.lock() failed!!!", MODULE_PATH, form, cb_type))
            .get(&FormKey::new(form))
            .unwrap_or_else(|| panic!("{}form_callback({:p}, {:?}) : FORMSCREENS.lock().get() failed!!!", MODULE_PATH, form, cb_type))
            .screen();

        Form::_from(screen, form, unsafe { (*form).field }, false)
    };

    let callbacks = CALLBACKS
        .lock()
        .unwrap_or_else(|_| panic!("{}form_callback({:p}, {:?}) : CALLBACKS.lock() failed!!!", MODULE_PATH, form, cb_type));

    if let Some(ref callback) = callbacks
        .get(&CallbackKey::new(Some(form), cb_type))
        .unwrap_or(&None)
    {
        callback(&get_form())
    } else if let Some(ref callback) = callbacks
        .get(&CallbackKey::new(None, cb_type))
        .unwrap_or(&None)
    {
        callback(&get_form())
    } else {
        panic!("{}form_callback({:p}, {:?}) : callbacks.lock().get() returned None!!!", MODULE_PATH, form, cb_type)
    }
}

pub(in crate::form) fn set_form_screen(form: FORM, screen: Option<SCREEN>) {
    FORMSCREENS
        .lock()
        .unwrap_or_else(|_| panic!("{}set_form_screen({:p}) : FORMSCREENS.lock() failed!!!", MODULE_PATH, form))
        .insert(FormKey::new(form), FormValue::new(screen));
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
    let mut form_screens = FORMSCREENS
        .lock()
        .unwrap_or_else(|_| panic!("{}form_tidyup({:p}) : FORMSCREENS.lock() failed!!!", MODULE_PATH, form));

    form_screens.remove(&FormKey::new(form));
    form_screens.shrink_to_fit();

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
