/*
    src/form/field.rs

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

use std::{ptr, fmt, convert::{TryFrom, TryInto}, hash::{Hash, Hasher}};

use ncursesw::{normal, form, form::{FieldOptions, FieldJustification, FIELD}};
use crate::{Origin, NCurseswWinError};
use crate::form::{FieldType, FieldInfo, FieldParameters};

/// Form field.
pub struct Field {
    handle:       FIELD, // pointer to ncurses field type internal structure
    free_on_drop: bool
}

impl Field {
    // make a new instance from the passed ncurses pointer.
    pub(in crate::form) fn _from(handle: FIELD, free_on_drop: bool) -> Self {
        assert!(!handle.is_null(), "Field::_from() : handle.is_null()");

        Self { handle, free_on_drop }
    }

    pub(in crate::form) fn _handle(&self) -> FIELD {
        self.handle
    }
}

impl Field {
    pub fn new(parameters: FieldParameters) -> result!(Self) {
        Ok(Self::_from(form::new_field(parameters.try_into()?)?, true))
    }

    #[deprecated(since = "0.4.1", note = "Use Field::new() instead")]
    pub fn new_field(parameters: FieldParameters) -> result!(Self) {
        Self::new(parameters)
    }

    pub fn dup_field(&self, origin: Origin) -> result!(Self) {
        Ok(Self::_from(form::dup_field(self.handle, origin.try_into()?)?, true))
    }

    pub fn dynamic_field_info(&self) -> result!(FieldInfo) {
        Ok(FieldInfo::try_from(form::dynamic_field_info(self.handle)?)?)
    }

    pub fn field_arg(&self) -> result!(*mut libc::c_void) {
        Ok(form::field_arg(self.handle)?)
    }

    pub fn field_back(&self) -> normal::Attributes {
        form::field_back(self.handle)
    }

    pub fn field_buffer(&self, buffer_number: u8) -> result!(Vec<i8>) {
        Ok(form::field_buffer(self.handle, i32::from(buffer_number))?)
    }

    pub fn field_fore(&self) -> normal::Attributes {
        form::field_fore(self.handle)
    }

    pub fn field_index(&self) -> result!(usize) {
        Ok(usize::try_from(form::field_index(self.handle)?)?)
    }

    pub fn field_info(&self) -> result!(FieldParameters) {
        Ok(FieldParameters::try_from(form::field_info(self.handle)?)?)
    }

    pub fn field_just(&self) -> result!(FieldJustification) {
        Ok(form::field_just(self.handle)?)
    }

    pub fn field_opts(&self) -> FieldOptions {
        form::field_opts(self.handle)
    }

    pub fn field_opts_off(&self, opts: FieldOptions) -> result!(()) {
        Ok(form::field_opts_off(self.handle, opts)?)
    }

    pub fn field_opts_on(&self, opts: FieldOptions) -> result!(()) {
        Ok(form::field_opts_on(self.handle, opts)?)
    }

    pub fn field_pad(&self) -> char {
        form::field_pad(self.handle)
    }

    pub fn field_status(&self) -> bool {
        form::field_status(self.handle)
    }

    pub fn field_type(&self) -> result!(FieldType) {
        Ok(FieldType::_from(form::field_type(self.handle)?, false))
    }

    pub fn field_userptr<T>(&self) -> result!(Option<Box<T>>) {
        let ptr = form::field_userptr(self.handle)?;

        if !ptr.is_null() {
            Ok(Some(unsafe { Box::from_raw(ptr as *mut T) }))
        } else {
            Ok(None)
        }
    }

    pub fn link_field(&self, origin: Origin) -> result!(Self) {
        Ok(Self::_from(form::link_field(self.handle, origin.try_into()?)?, true))
    }

    pub fn move_field(&self, origin: Origin) -> result!(()) {
        Ok(form::move_field(self.handle, origin.try_into()?)?)
    }

    pub fn new_page(&self) -> bool {
        form::new_page(self.handle)
    }

    pub fn set_field_back(&self, attr: normal::Attributes) -> result!(()) {
        Ok(form::set_field_back(self.handle, attr)?)
    }

    pub fn set_field_buffer(&self, buffer_number: u8, buffer: &[i8]) -> result!(()) {
        Ok(form::set_field_buffer(self.handle, i32::from(buffer_number), buffer)?)
    }

    pub fn set_field_fore(&self, attr: normal::Attributes) -> result!(()) {
        Ok(form::set_field_fore(self.handle, attr)?)
    }

    pub fn set_field_just(&self, justification: FieldJustification) -> result!(()) {
        Ok(form::set_field_just(self.handle, justification)?)
    }

    pub fn set_field_opts(&self, opts: FieldOptions) -> result!(()) {
        Ok(form::set_field_opts(self.handle, opts)?)
    }

    pub fn set_field_pad(&self, pad: char) -> result!(()) {
        Ok(form::set_field_pad(self.handle, pad)?)
    }

    pub fn set_field_status(&self, status: bool) -> result!(()) {
        Ok(form::set_field_status(self.handle, status)?)
    }

    //pub fn set_field_type(&self, fieldtype: FieldType) -> result!(()) {

    pub fn set_field_userptr<T>(&self, userptr: Option<Box<&T>>) -> result!(()) {
        Ok(form::set_field_userptr(self.handle, match userptr {
            Some(ptr) => Some(Box::into_raw(ptr) as *mut libc::c_void),
            None      => None
        })?)
    }

    pub fn set_max_field(&self, max: usize) -> result!(()) {
        Ok(form::set_max_field(self.handle, i32::try_from(max)?)?)
    }

    pub fn set_new_page(&self, new_page_flag: bool) -> result!(()) {
        Ok(form::set_new_page(self.handle, new_page_flag)?)
    }
}

impl Drop for Field {
    fn drop(&mut self) {
        if self.free_on_drop {
            if let Err(source) = form::free_field(self.handle) {
                panic!("{} @ {:?}", source, self)
            }
        }
    }
}

unsafe impl Send for Field { } // too make thread safe
unsafe impl Sync for Field { } // too make thread safe

impl PartialEq for Field {
    fn eq(&self, rhs: &Self) -> bool {
        ptr::eq(self.handle, rhs.handle)
    }
}

impl Eq for Field { }

impl Hash for Field {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Field {{ handle: {:p}, free_on_drop: {} }}", self.handle, self.free_on_drop)
    }
}
