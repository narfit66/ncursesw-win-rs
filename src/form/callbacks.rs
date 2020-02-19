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

use strum_macros::EnumIter;

use ncursesw::form::FORM;
use crate::form::Form;

static MODULE_PATH: &str = "ncurseswwin::form::callbacks::";

#[derive(EnumIter, PartialEq, Eq, Hash)]
pub(in crate::form) enum CallbackType {
    FieldInit,
    FieldTerm,
    FormInit,
    FormTerm
}

#[derive(PartialEq, Eq, Hash)]
pub(in crate::form) struct CallbackKey {
    form:          Option<FORM>,
    callback_type: CallbackType
}

impl CallbackKey {
    pub(in crate::form) fn new(form: Option<FORM>, callback_type: CallbackType) -> Self {
        Self { form, callback_type }
    }
}

unsafe impl Send for CallbackKey { }
unsafe impl Sync for CallbackKey { }

type CALLBACK = Option<Box<dyn Fn(&Form) + Send>>;

lazy_static! {
    pub(in crate::form) static ref CALLBACKS: Mutex<HashMap<CallbackKey, CALLBACK>> = Mutex::new(HashMap::new());
}

macro_rules! form_callback {
    ($func: ident, $cb_t: ident) => {
        pub(in crate::form) extern fn $func(form: FORM) {
            let callbacks = CALLBACKS
                .lock()
                .unwrap_or_else(|_| panic!("{}{}({:p}) : CALLBACKS.lock() failed!!!", MODULE_PATH, stringify!($func), form));

            if let Some(ref callback) = callbacks
                .get(&CallbackKey::new(Some(form), CallbackType::$cb_t))
                .unwrap_or_else(|| &None)
            {
                callback(&Form::_from(None, form, unsafe { (*form).field }, false))
            } else if let Some(ref callback) = callbacks                             // default does not work yet see src/form/funcs.rs!!!
                .get(&CallbackKey::new(None, CallbackType::$cb_t))
                .unwrap_or_else(|| &None)
            {
                callback(&Form::_from(None, form, unsafe { (*form).field }, false))
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
