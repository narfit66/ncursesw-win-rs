/*
    src/form/form.rs

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

use std::{ptr, fmt, mem, convert::TryFrom, hash::{Hash, Hasher}};

use errno::errno;
use strum::IntoEnumIterator;

use ncursesw::{
    SCREEN, normal, form, form::{FormOptions, FORM, FIELD},
    shims::nform, shims::constants::E_OK
};
use crate::{
    Screen, Size, Window, HasHandle, NCurseswWinError, AttributesType,
    form::{Field, PostedForm, callbacks::*}
};

#[deprecated(since = "0.5.0")]
pub use ncursesw::form::Form_Hook;

static MODULE_PATH: &str = "ncurseswwin::form::";

/// Form.
pub struct Form {
    screen:        Option<SCREEN>, // optional pointer to NCurses screen type internal structure.
    handle:        FORM,           // pointer to ncurses form type internal structure.
    field_handles: *mut FIELD,     // pointer to list of field pointers (null terminated).
    free_on_drop:  bool            // call `free_form()` when instance goes out of scope.
}

impl Form {
    pub(in crate::form) fn _from(screen: Option<SCREEN>, handle: FORM, field_handles: *mut FIELD, free_on_drop: bool) -> Self {
        assert!(screen.map_or_else(|| true, |screen| !screen.is_null()), "Form::_from() : screen.is_null()");
        assert!(!handle.is_null(), "Form::_from() : handle.is_null()");
        assert!(!field_handles.is_null(), "Form::_from() : field_handles.is_null()");

        Self { screen, handle, field_handles, free_on_drop }
    }

    pub(in crate::form) fn _screen(&self) -> Option<SCREEN> {
        self.screen
    }

    pub(in crate::form) fn _handle(&self) -> FORM {
        self.handle
    }
}

impl Form {
    /// Creates a new form connected to specified fields.
    pub fn new(fields: &[&Field]) -> result!(Self) {
        // allocate enougth contiguous memory to store all the field handles plus
        // a null and set all pointers initially to null.
        let field_handles = unsafe { libc::calloc(fields.len() + 1, mem::size_of::<FIELD>()) as *mut FIELD };

        // check that we we're able to allocate our memory.
        if !field_handles.is_null() {
            // get all the field handles and write them to memory.
            for (offset, field_handle) in fields.iter().map(|field| field._handle()).enumerate() {
                unsafe { ptr::write(field_handles.offset(isize::try_from(offset)?), field_handle) };
            }

            // don't unallocate field_handles when it goes out of scope, we'll do it
            // ourselves as self.field_handles will point to our contiguous memory.
            mem::forget(field_handles);

            // call the ncursesw shims new_form() function with our allocated memory.
            match unsafe { nform::new_form(field_handles) } {
                Some(form) => Ok(Self::_from(None, form, field_handles, true)),
                None       => Err(NCurseswWinError::FormError { source: form::ncursesw_form_error_from_rc("Form::new", errno().into()) })
            }
        } else {
            Err(NCurseswWinError::OutOfMemory { func: "Form::new".to_string() })
        }
    }

    #[deprecated(since = "0.5.0", note = "Use Form::new() instead")]
    /// Creates a new form connected to specified fields.
    pub fn new_form(fields: &[&Field]) -> result!(Self) {
        Self::new(fields)
    }

    /// Creates a new form on the specified sceen connected to specified fields.
    pub fn new_sp(screen: &Screen, fields: &[&Field]) -> result!(Self) {
        // allocate enougth contiguous memory to store all the field handles plus
        // a null and set all pointers initially to null.
        let field_handles = unsafe { libc::calloc(fields.len() + 1, mem::size_of::<FIELD>()) as *mut FIELD };

        // check that we we're able to allocate our memory.
        if !field_handles.is_null() {
            // get all the field handles and write them to memory.
            for (offset, field_handle) in fields.iter().map(|field| field._handle()).enumerate() {
                unsafe { ptr::write(field_handles.offset(isize::try_from(offset)?), field_handle) };
            }

            // don't unallocate field_handles when it goes out of scope, we'll do it
            // ourselves as self.field_handles will point to our contiguous memory.
            mem::forget(field_handles);

            // call the ncursesw shims new_form() function with our allocated memory.
            match unsafe { nform::new_form_sp(screen._handle(), field_handles) } {
                Some(form) => Ok(Self::_from(Some(screen._handle()), form, field_handles, true)),
                None       => Err(NCurseswWinError::FormError { source: form::ncursesw_form_error_from_rc("Form::new_sp", errno().into()) })
            }
        } else {
            Err(NCurseswWinError::OutOfMemory { func: "Form::new_sp".to_string() })
        }
    }

    #[deprecated(since = "0.5.0", note = "Use Form::new_sp() instead")]
    /// Creates a new form on the specified sceen connected to specified fields.
    pub fn new_form_sp(screen: &Screen, fields: &[&Field]) -> result!(Self) {
        Self::new_sp(screen, fields)
    }

    /// The screen associated with the form.
    pub fn screen(&self) -> Option<Screen> {
        self.screen.map_or_else(|| None, |screen| Some(Screen::_from(screen, false)))
    }

    /// Returns the current field of the given form.
    pub fn current_field(&self) -> result!(Field) {
        Ok(Field::_from(form::current_field(Some(self.handle))?, false))
    }

    /// Tests whether there is off-screen data ahead in the given form.
    pub fn data_ahead(&self) -> bool {
        form::data_ahead(self.handle)
    }

    /// Tests whether there is off-screen data behind in the given form.
    pub fn data_behind(&self) -> bool {
        form::data_behind(self.handle)
    }

    /// Returns the count of fields in form.
    pub fn field_count(&self) -> result!(usize) {
        Ok(usize::try_from(form::field_count(Some(self.handle))?)?)
    }

    #[deprecated(since = "0.5.0")]
    /// Returns the current field init hook.
    pub fn field_init(&self) -> result!(Form_Hook) {
        Ok(form::field_init(Some(self.handle))?)
    }

    #[deprecated(since = "0.5.0")]
    /// Returns the current field term hook.
    pub fn field_term(&self) -> result!(Form_Hook) {
        Ok(form::field_term(Some(self.handle))?)
    }

    /// Returns a vector of the fields of the given form.
    pub fn form_fields(&self) -> result!(Vec<Field>) {
        Ok(form::form_fields(Some(self.handle))?.iter().map(|handle| Field::_from(*handle, false)).collect())
    }

    #[deprecated(since = "0.5.0")]
    /// Returns the current form init hook.
    pub fn form_init(&self) -> result!(Form_Hook) {
        Ok(form::form_init(Some(self.handle))?)
    }

    /// Returns the form's current options.
    pub fn form_opts(&self) -> FormOptions {
        form::form_opts(Some(self.handle))
    }

    /// Turns off the given options, and leaves others alone.
    pub fn form_opts_off(&self, opts: FormOptions) -> result!(()) {
        Ok(form::form_opts_off(Some(self.handle), opts)?)
    }

    /// Turns on the given options, and leaves others alone.
    pub fn form_opts_on(&self, opts: FormOptions) -> result!(()) {
        Ok(form::form_opts_on(Some(self.handle), opts)?)
    }

    /// Returns the form's current page number.
    pub fn form_page(&self) -> result!(usize) {
        Ok(usize::try_from(form::form_page(Some(self.handle))?)?)
    }

    /// Return the forms sub-window.
    pub fn form_sub(&self) -> result!(Window) {
        Ok(Window::_from(self.screen, form::form_sub(Some(self.handle))?, false))
    }

    #[deprecated(since = "0.5.0")]
    /// Returns the current form term hook.
    pub fn form_term(&self) -> result!(Form_Hook) {
        Ok(form::form_term(Some(self.handle))?)
    }

    /// Returns the forms user pointer.
    // TODO: needs testing!
    pub fn form_userptr<T>(&self) -> result!(Option<Box<T>>) {
        Ok(unsafe { form::form_userptr(Some(self.handle))?.as_mut().map(|userptr| Box::from_raw(userptr as *mut libc::c_void as *mut T)) })
    }

    /// Return the forms main window.
    pub fn form_win(&self) -> result!(Window) {
        Ok(Window::_from(self.screen, form::form_win(Some(self.handle))?, false))
    }

    /// Displays a form to its associated sub-window. To trigger physical display
    /// of the sub-window, set `refresh` to `true` or call `refresh()` or some
    /// equivalent NCurses routine (the implicit `doupdate()` triggered by a
    /// NCurses input request will do).
    pub fn post_form(&self, refresh: bool) -> result!(PostedForm) {
        PostedForm::new(self, refresh)
    }

    /// Returns the minimum size required for the sub-window of form.
    pub fn scale_form(&self) -> result!(Size) {
        Ok(Size::try_from(form::scale_form(self.handle)?)?)
    }

    /// Sets the current field of the given form.
    pub fn set_current_field(&self, field: &Field) -> result!(()) {
        Ok(form::set_current_field(self.handle, field._handle())?)
    }

    /// Sets a callback to be called at form-post time and each time
    /// the selected field changes (after the change).
    pub fn set_field_init<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        CALLBACKS
            .lock()
            .unwrap_or_else(|_| panic!("{}set_field_init() : CALLBACKS.lock() failed!!!", MODULE_PATH))
            .insert(CallbackKey::new(Some(self.handle), CallbackType::FieldInit), Some(Box::new(move |menu| func(menu))));

        Ok(form::set_field_init(Some(self.handle), Some(extern_field_init))?)
    }

    /// Sets a callback to be called at form-unpost time and each time
    /// the selected field changes (before the change).
    pub fn set_field_term<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        CALLBACKS
            .lock()
            .unwrap_or_else(|_| panic!("{}set_field_term() : CALLBACKS.lock() failed!!!", MODULE_PATH))
            .insert(CallbackKey::new(Some(self.handle), CallbackType::FieldTerm), Some(Box::new(move |menu| func(menu))));

        Ok(form::set_field_term(Some(self.handle), Some(extern_field_term))?)
    }

    /// Clear the list of current fields and replace them with a new list.
    pub fn set_form_fields(&self, fields: &[&Field]) -> result!(()) {
        // unallocate the field_handles memory.
        unsafe { libc::free(self.field_handles as *mut libc::c_void) };

        // allocate enougth contiguous memory to store all the field handles plus
        // a null and set all pointers initially to null.
        let field_handles = unsafe { libc::calloc(fields.len() + 1, mem::size_of::<FIELD>()) as *mut FIELD };

        // check that we we're able to allocate our memory.
        if !field_handles.is_null() {
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
        } else {
            Err(NCurseswWinError::OutOfMemory { func: "set_form_fields".to_string() })
        }
    }

    /// Sets a callback to be called at form-post time and just after
    /// a page change once it is posted.
    pub fn set_form_init<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        CALLBACKS
            .lock()
            .unwrap_or_else(|_| panic!("{}set_form_init() : CALLBACKS.lock() failed!!!", MODULE_PATH))
            .insert(CallbackKey::new(Some(self.handle), CallbackType::FormInit), Some(Box::new(move |menu| func(menu))));

        Ok(form::set_form_init(Some(self.handle), Some(extern_form_init))?)
    }

    /// Sets all the given form's options.
    pub fn set_form_opts(&self, opts: FormOptions) -> result!(()) {
        Ok(form::set_form_opts(Some(self.handle), opts)?)
    }

    /// Sets the form's page number (goes to page `number` of the form).
    pub fn set_form_page(&self, number: usize) -> result!(()) {
        Ok(form::set_form_page(self.handle, i32::try_from(number)?)?)
    }

    /// Sets the forms sub-window.
    pub fn set_form_sub(&self, window: Option<&Window>) -> result!(()) {
        assert!(self._screen() == window.map_or_else(|| None, |window| window._screen()));

        Ok(form::set_form_sub(Some(self.handle), window.map_or_else(|| None, |window| Some(window._handle())))?)
    }

    /// Sets a callback to be called at form-unpost time and just before
    /// a page change once it is posted.
    pub fn set_form_term<F>(&self, func: F) -> result!(())
        where F: Fn(&Self) + 'static + Send
    {
        CALLBACKS
            .lock()
            .unwrap_or_else(|_| panic!("{}set_form_init() : CALLBACKS.lock() failed!!!", MODULE_PATH))
            .insert(CallbackKey::new(Some(self.handle), CallbackType::FormTerm), Some(Box::new(move |menu| func(menu))));

        Ok(form::set_form_term(Some(self.handle), Some(extern_form_term))?)
    }

    /// Sets the forms user pointer.
    // TODO: needs testing!
    pub fn set_form_userptr<T>(&self, userptr: Option<Box<&T>>) -> result!(()) {
        Ok(form::set_form_userptr(Some(self.handle), userptr.map_or_else(|| None, |userptr| Some(Box::into_raw(userptr) as *mut libc::c_void)))?)
    }

    /// Set the forms main window.
    pub fn set_form_win(&self, window: Option<&Window>) -> result!(()) {
        assert!(self._screen() == window.map_or_else(|| None, |window| window._screen()));

        Ok(form::set_form_win(Some(self.handle), window.map_or_else(|| None, |window| Some(window._handle())))?)
    }

    /// Removes the focus from the current field of the form.
    /// In such state, inquiries via `self.current_field()` will error.
    pub fn unfocus_current_field(&self) -> result!(()) {
        Ok(form::unfocus_current_field(self.handle)?)
    }

    pub fn field_back(&self, field: &Field) -> normal::Attributes {
        self.screenify_attributes(field.field_back())
    }

    pub fn field_fore(&self, field: &Field) -> normal::Attributes {
        self.screenify_attributes(field.field_fore())
    }

    pub fn set_field_back(&self, field: &Field, attrs: normal::Attributes) -> result!(()) {
        assert!(self.screen == attrs.screen());

        field.set_field_back(attrs)
    }

    pub fn set_field_fore(&self, field: &Field, attrs: normal::Attributes) -> result!(()) {
        assert!(self.screen == attrs.screen());

        field.set_field_fore(attrs)
    }

    fn screenify_attributes(&self, attrs: normal::Attributes) -> normal::Attributes {
        self.screen.map_or_else(|| normal::Attributes::new(attrs.into()), |screen| normal::Attributes::new_sp(screen, attrs.into()))
    }
}

impl Drop for Form {
    fn drop(&mut self) {
        if self.free_on_drop {
            // free the ncurses internal structure.
            if let Err(source) = form::free_form(self.handle) {
                panic!("{} @ {:?}", source, self)
            }

            // unallocate the field_handles memory.
            unsafe { libc::free(self.field_handles as *mut libc::c_void) };

            // remove any callbacks created for this instance.
            let mut callbacks = CALLBACKS
                .lock()
                .unwrap_or_else(|_| panic!("Form::drop() : CALLBACKS.lock() failed!!!"));

            // remove any callbacks set against the form and free memory if required.
            let mut shrink_to_fit = false;

            for cb_type in CallbackType::iter() {
                if callbacks.remove(&CallbackKey::new(Some(self.handle), cb_type)).is_some() {
                    shrink_to_fit = true;
                }
            }

            if shrink_to_fit {
                callbacks.shrink_to_fit();
            }
        }
    }
}

unsafe impl Send for Form { } // too make thread safe
unsafe impl Sync for Form { } // too make thread safe

impl PartialEq for Form {
    fn eq(&self, rhs: &Self) -> bool {
        self.screen == rhs._screen() && ptr::eq(self.handle, rhs.handle)
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
        write!(f, "Form {{ screen: {:?}, handle: {:p}, field_handles: {:p}, free_on_drop: {} }}", self.screen, self.handle, self.field_handles, self.free_on_drop)
    }
}
