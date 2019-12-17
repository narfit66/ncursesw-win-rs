/*
    src/form/form.rs

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

use std::{
    ptr, fmt, mem, convert::TryFrom, hash::{Hash, Hasher},
    collections::HashMap, sync::Mutex
};

use errno::errno;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use ncursesw::{form, form::{FormOptions, E_OK, FORM, FIELD}, shims::nform};
use crate::{Size, Window, HasHandle, NCurseswWinError, form::{Field, PostedForm}};

#[deprecated(since = "0.4.1")]
pub use ncursesw::form::Form_Hook;

static MODULE_PATH: &str = "ncurseswwin::form::";

#[derive(EnumIter, PartialEq, Eq, Hash)]
enum CallbackType {
    FieldInit,
    FieldTerm,
    FormInit,
    FormTerm
}

#[derive(PartialEq, Eq, Hash)]
struct FormKey {
    form:          FORM,
    callback_type: CallbackType
}

impl FormKey {
    fn new(form: FORM, callback_type: CallbackType) -> Self {
        Self { form, callback_type }
    }
}

unsafe impl Send for FormKey { }
unsafe impl Sync for FormKey { }

lazy_static! {
    static ref CALLBACKS: Mutex<HashMap<FormKey, Option<Box<dyn Fn(&Form) + Send>>>> = Mutex::new(HashMap::new());
}

macro_rules! menu_callback {
    ($f: ident, $cb_t: ident) => {
        extern fn $f(form: FORM) {
            if let Some(ref internal_fn) = *CALLBACKS
                .lock()
                .unwrap_or_else(|_| panic!("{}{}({:p}) : *CALLBACKS.lock() failed!!!", MODULE_PATH, stringify!($f), form))
                .get(&FormKey::new(form, CallbackType::$cb_t))
                .unwrap_or_else(|| panic!("{}{}({:p}) : *CALLBACKS().get() failed!!!", MODULE_PATH, stringify!($f), form))
            {
                internal_fn(&Form::_from(form, unsafe { (*form).field }, false))
            } else {
                panic!("{}{}() : *CALLBACKS.lock().get() returned None!!!", MODULE_PATH, stringify!($f))
            }
        }
    }
}

menu_callback!(extern_field_init, FieldInit);
menu_callback!(extern_field_term, FieldTerm);
menu_callback!(extern_form_init, FormInit);
menu_callback!(extern_form_term, FormTerm);

/// Form.
pub struct Form {
    handle:        FORM,        // pointer to ncurses field type internal structure
    field_handles: *mut FIELD,
    free_on_drop:  bool
}

impl Form {
    // make a new instance from the passed ncurses pointers.
    pub(in crate::form) fn _from(handle: FORM, field_handles: *mut FIELD, free_on_drop: bool) -> Self {
        assert!(!handle.is_null(), "Form::_from() : handle.is_null()");
        assert!(!field_handles.is_null(), "Form::_from() : field_handles.is_null()");

        Self { handle, field_handles, free_on_drop }
    }

    pub(in crate::form) fn _handle(&self) -> FORM {
        self.handle
    }
}

impl Form {
    pub fn new(fields: &[&Field]) -> result!(Self) {
        // allocate enougth contiguous memory to store all the field handles plus
        // a null and set all pointers initially to null.
        let field_handles = unsafe { libc::calloc(fields.len() + 1, mem::size_of::<FIELD>()) as *mut FIELD };

        // check that we we're able to allocate our memory.
        if field_handles.is_null() {
            Err(NCurseswWinError::OutOfMemory { func: "Form::new".to_string() })
        } else {
            // get all the field handles and write them to memory.
            for (offset, field_handle) in fields.iter().map(|field| field._handle()).enumerate() {
                unsafe { ptr::write(field_handles.offset(isize::try_from(offset)?), field_handle) };
            }

            // don't unallocate field_handles when it goes out of scope, we'll do it
            // ourselves as self.field_handles will point to our contiguous memory.
            mem::forget(field_handles);

            // call the ncursesw shims new_form() function with our allocated memory.
            match unsafe { nform::new_form(field_handles as *mut FIELD) } {
                Some(form) => Ok(Self::_from(form, field_handles, true)),
                None       => Err(NCurseswWinError::FormError { source: form::ncursesw_form_error_from_rc("Form::new", errno().into()) })
            }
        }
    }

    #[deprecated(since = "0.4.1", note = "Use Form::new() instead")]
    pub fn new_form(fields: &[&Field]) -> result!(Self) {
        Self::new(fields)
    }

    pub fn current_field(&self) -> result!(Field) {
        Ok(Field::_from(form::current_field(self.handle)?, false))
    }

    pub fn data_ahead(&self) -> bool {
        form::data_ahead(self.handle)
    }

    pub fn data_behind(&self) -> bool {
        form::data_behind(self.handle)
    }

    pub fn field_count(&self) -> result!(usize) {
        Ok(usize::try_from(form::field_count(self.handle)?)?)
    }

    #[deprecated(since = "0.4.1")]
    pub fn field_init(&self) -> result!(Form_Hook) {
        Ok(form::field_init(self.handle)?)
    }

    #[deprecated(since = "0.4.1")]
    pub fn field_term(&self) -> result!(Form_Hook) {
        Ok(form::field_term(self.handle)?)
    }

    pub fn form_fields(&self) -> result!(Vec<Field>) {
        let field_handles = form::form_fields(self.handle)?;

        Ok(field_handles.iter().map(|handle| Field::_from(*handle, false)).collect())
    }

    #[deprecated(since = "0.4.1")]
    pub fn form_init(&self) -> result!(Form_Hook) {
        Ok(form::form_init(self.handle)?)
    }

    pub fn form_opts(&self) -> FormOptions {
        form::form_opts(self.handle)
    }

    pub fn form_opts_off(&self, opts: FormOptions) -> result!(()) {
        Ok(form::form_opts_off(self.handle, opts)?)
    }

    pub fn form_opts_on(&self, opts: FormOptions) -> result!(()) {
        Ok(form::form_opts_on(self.handle, opts)?)
    }

    pub fn form_page(&self) -> result!(usize) {
        Ok(usize::try_from(form::form_page(self.handle)?)?)
    }

    pub fn form_sub(&self) -> result!(Window) {
        Ok(Window::_from(form::form_sub(self.handle)?, false))
    }

    #[deprecated(since = "0.4.1")]
    pub fn form_term(&self) -> result!(Form_Hook) {
        Ok(form::form_term(self.handle)?)
    }

    // TODO: needs testing!
    pub fn form_userptr<T>(&self) -> result!(Option<Box<T>>) {
        let ptr = form::form_userptr(self.handle)?;

        Ok(unsafe { ptr.as_mut().map(|ptr| Box::from_raw(ptr as *mut libc::c_void as *mut T))})
    }

    pub fn form_win(&self) -> result!(Window) {
        Ok(Window::_from(form::form_win(self.handle)?, false))
    }

    pub fn post_form(&self, refresh: bool) -> result!(PostedForm) {
        PostedForm::new(self, refresh)
    }

    pub fn scale_form(&self) -> result!(Size) {
        Ok(Size::try_from(form::scale_form(self.handle)?)?)
    }

    pub fn set_current_field(&self, field: Field) -> result!(()) {
        Ok(form::set_current_field(self.handle, field._handle())?)
    }

    pub fn set_field_init<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        CALLBACKS
            .lock()
            .unwrap_or_else(|_| panic!("{}set_field_init() : CALLBACKS.lock() failed!!!", MODULE_PATH))
            .insert(FormKey::new(self.handle, CallbackType::FieldInit), Some(Box::new(move |menu| func(menu))));

        Ok(form::set_field_init(self.handle, Some(extern_field_init))?)
    }

    pub fn set_field_term<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        CALLBACKS
            .lock()
            .unwrap_or_else(|_| panic!("{}set_field_term() : CALLBACKS.lock() failed!!!", MODULE_PATH))
            .insert(FormKey::new(self.handle, CallbackType::FieldTerm), Some(Box::new(move |menu| func(menu))));

        Ok(form::set_field_term(self.handle, Some(extern_field_term))?)
    }

    pub fn set_form_fields(&self, fields: &[&Field]) -> result!(()) {
        // unallocate the field_handles memory.
        unsafe { libc::free(self.field_handles as *mut libc::c_void) };

        // allocate enougth contiguous memory to store all the field handles plus
        // a null and set all pointers initially to null.
        let field_handles = unsafe { libc::calloc(fields.len() + 1, mem::size_of::<FIELD>()) as *mut FIELD };

        // check that we we're able to allocate our memory.
        if field_handles.is_null() {
            Err(NCurseswWinError::OutOfMemory { func: "set_form_fields".to_string() })
        } else {
            // get all the field handles and write them to memory.
            for (offset, field_handle) in fields.iter().map(|field| field._handle()).enumerate() {
                unsafe { ptr::write(field_handles.offset(isize::try_from(offset)?), field_handle) };
            }

            // don't unallocate field_handles when it goes out of scope, we'll do it
            // ourselves as self.field_handles will point to our contiguous memory.
            mem::forget(field_handles);

            // call the ncursesw shims set_form_fields() function with our allocated memory.
            match unsafe { nform::set_form_fields(self.handle, field_handles as *mut FIELD) } {
                E_OK => Ok(()),
                rc   => Err(NCurseswWinError::FormError { source: form::ncursesw_form_error_from_rc("set_form_fields", rc) })
            }
        }
    }

    pub fn set_form_init<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        CALLBACKS
            .lock()
            .unwrap_or_else(|_| panic!("{}set_form_init() : CALLBACKS.lock() failed!!!", MODULE_PATH))
            .insert(FormKey::new(self.handle, CallbackType::FormInit), Some(Box::new(move |menu| func(menu))));

        Ok(form::set_form_init(self.handle, Some(extern_form_init))?)
    }

    pub fn set_form_opts(&self, opts: FormOptions) -> result!(()) {
        Ok(form::set_form_opts(self.handle, opts)?)
    }

    pub fn set_form_page(&self, n: usize) -> result!(()) {
        Ok(form::set_form_page(self.handle, i32::try_from(n)?)?)
    }

    pub fn set_form_sub(&self, sub: Option<Window>) -> result!(()) {
        Ok(form::set_form_sub(Some(self.handle), match sub {
            Some(window) => Some(window._handle()),
            None         => None
        })?)
    }

    pub fn set_form_term<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        CALLBACKS
            .lock()
            .unwrap_or_else(|_| panic!("{}set_form_init() : CALLBACKS.lock() failed!!!", MODULE_PATH))
            .insert(FormKey::new(self.handle, CallbackType::FormTerm), Some(Box::new(move |menu| func(menu))));

        Ok(form::set_form_term(self.handle, Some(extern_form_term))?)
    }

    // TODO: needs testing!
    pub fn set_form_userptr<T>(&self, userptr: Option<Box<&T>>) -> result!(()) {
        Ok(form::set_form_userptr(self.handle, match userptr {
            Some(ptr) => Some(Box::into_raw(ptr) as *mut libc::c_void),
            None      => None
        })?)
    }

    pub fn set_form_win(&self, win: Option<Window>) -> result!(()) {
        Ok(form::set_form_win(Some(self.handle), match win {
            Some(window) => Some(window._handle()),
            None         => None
        })?)
    }

    pub fn unfocus_current_field(&self) -> result!(()) {
        Ok(form::unfocus_current_field(self.handle)?)
    }
}

impl Drop for Form {
    fn drop(&mut self) {
        if self.free_on_drop {
            if let Err(source) = form::free_form(self.handle) {
                panic!("{} @ {:?}", source, self)
            }

            // unallocate the field_handles memory.
            unsafe { libc::free(self.field_handles as *mut libc::c_void) };

            // remove any callbacks created for this instance.
            let mut callbacks = CALLBACKS
                .lock()
                .unwrap_or_else(|_| panic!("Form::drop() : CALLBACKS.lock() failed!!!"));

            for cb_type in CallbackType::iter() {
                callbacks.remove(&FormKey::new(self.handle, cb_type));
            }
        }
    }
}

unsafe impl Send for Form { } // too make thread safe
unsafe impl Sync for Form { } // too make thread safe

impl PartialEq for Form {
    fn eq(&self, rhs: &Self) -> bool {
        ptr::eq(self.handle, rhs.handle)
    }
}

impl Eq for Form { }

impl Hash for Form {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl fmt::Debug for Form {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Form {{ handle: {:p}, field_handles: {:p} }}", self.handle, self.field_handles)
    }
}
