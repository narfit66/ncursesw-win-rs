/*
    src/form/fieldtype.rs

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

use std::{ptr, fmt, hash::{Hash, Hasher}};

use ncursesw::{
    form, form::{FIELD, FIELDTYPE},
    shims::bindings::{
        TYPE_ALPHA, TYPE_ALNUM, TYPE_ENUM, TYPE_INTEGER, TYPE_NUMERIC, TYPE_REGEXP, TYPE_IPV4,
        va_list
    }
};
use crate::NCurseswWinError;

lazy_static! {
    /// Alpha field type.
    pub static ref FIELDTYPE_ALPHA:   FieldType = FieldType::_from(unsafe { TYPE_ALPHA }, false);
    /// Alphanumeric field type.
    pub static ref FIELDTYPE_ALNUM:   FieldType = FieldType::_from(unsafe { TYPE_ALNUM }, false);
    /// Enumerated field type.
    pub static ref FIELDTYPE_ENUM:    FieldType = FieldType::_from(unsafe { TYPE_ENUM }, false);
    /// Integer field type.
    pub static ref FIELDTYPE_INTEGER: FieldType = FieldType::_from(unsafe { TYPE_INTEGER }, false);
    /// Numeric (decimal) field type.
    pub static ref FIELDTYPE_NUMERIC: FieldType = FieldType::_from(unsafe { TYPE_NUMERIC }, false);
    /// Regular expresion field type.
    pub static ref FIELDTYPE_REGEXP:  FieldType = FieldType::_from(unsafe { TYPE_REGEXP }, false);
    /// IpV4 field type.
    pub static ref FIELDTYPE_IPV4:    FieldType = FieldType::_from(unsafe { TYPE_IPV4 }, false);
}

/// Field type.
pub struct FieldType {
    handle:       FIELDTYPE, // pointer to ncurses field type internal structure
    free_on_drop: bool
}

impl FieldType {
    // make a new instance from the passed ncurses menu item pointer.
    pub(in crate::form) fn _from(handle: FIELDTYPE, free_on_drop: bool) -> Self {
        assert!(!handle.is_null(), "FieldType::_from() : handle.is_null()");

        Self { handle, free_on_drop }
    }

    pub(in crate::form) fn _handle(&self) -> FIELDTYPE {
        self.handle
    }
}

impl FieldType {
    pub fn new(
        field_check: unsafe extern "C" fn(_: FIELD, _: *const libc::c_void) -> bool,
        char_check:  unsafe extern "C" fn(_: i32, _: *const libc::c_void) -> bool
    ) -> result!(Self)
    {
        Ok(Self::_from(form::new_fieldtype(field_check, char_check)?, true))
    }

    #[deprecated(since = "0.4.1", note = "Use FieldType::new() instead")]
    pub fn new_fieldtype(
        field_check: unsafe extern "C" fn(_: FIELD, _: *const libc::c_void) -> bool,
        char_check:  unsafe extern "C" fn(_: i32, _: *const libc::c_void) -> bool
    ) -> result!(Self)
    {
        Self::new(field_check, char_check)
    }

    pub fn link_fieldtype(&self, fieldtype: &Self) -> result!(Self) {
        Ok(Self::_from(form::link_fieldtype(self.handle, fieldtype._handle())?, true))
    }

    pub fn set_fieldtype_arg(
        &self,
        make_arg: unsafe extern "C" fn(_: *mut va_list) -> *mut libc::c_void,
        copy_arg: Option<unsafe extern "C" fn(_: *const libc::c_void) -> *mut libc::c_void>,
        free_arg: Option<unsafe extern "C" fn(_: *mut libc::c_void)>
    ) -> result!(())
    {
        Ok(form::set_fieldtype_arg(self.handle, make_arg, copy_arg, free_arg)?)
    }

    pub fn set_fieldtype_choice(
        &self,
        next_choice: unsafe extern "C" fn(_: FIELD, _: *const libc::c_void) -> bool,
        prev_choice: unsafe extern "C" fn(_: FIELD, _: *const libc::c_void) -> bool
    ) -> result!(())
    {
        Ok(form::set_fieldtype_choice(self.handle, next_choice, prev_choice)?)
    }
}

impl Drop for FieldType {
    fn drop(&mut self) {
        if self.free_on_drop {
            if let Err(source) = form::free_fieldtype(self.handle) {
                panic!("{} @ {:?}", source, self)
            }
        }
    }
}

unsafe impl Send for FieldType { } // too make thread safe
unsafe impl Sync for FieldType { } // too make thread safe

impl PartialEq for FieldType {
    fn eq(&self, rhs: &Self) -> bool {
        ptr::eq(self.handle, rhs.handle)
    }
}

impl Eq for FieldType { }

impl Hash for FieldType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl fmt::Debug for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldType {{ handle: {:p}, free_on_drop: {} }}", self.handle, self.free_on_drop)
    }
}
