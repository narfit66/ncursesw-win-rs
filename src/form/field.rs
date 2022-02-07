/*
    src/form/field.rs

    Copyright (c) 2019-2022 Stephen Whittle  All rights reserved.

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
use ncursesw::{
    normal, form,
    form::{FieldOptions, FieldJustification, FIELD, FIELDTYPE, ncursesw_form_error_from_rc},
    shims::bindings, shims::constants::E_OK
};
use crate::{
    Origin, HasHandle, NCurseswWinError,
    form::{FieldBuffer, FieldType, FieldInfo, FieldParameters, fieldtypes::IsFieldType}
};

/// Form field.
pub struct Field {
    handle:       FIELD, // pointer to NCurses field type internal structure
    free_on_drop: bool
}

impl Field {
    pub(in crate::form) fn _from(handle: FIELD, free_on_drop: bool) -> Self {
        assert!(!handle.is_null(), "Field::_from() : handle.is_null()");

        Self { handle, free_on_drop }
    }

    pub(in crate::form) fn _handle(&self) -> FIELD {
        self.handle
    }
}

impl Field {
    /// Allocates a new field and initializes it from the contents of a `FieldPrameters`
    /// type given: height, width, row of upper-left corner, column of upper-left corner,
    /// number off-screen rows, and number of additional working buffers.
    pub fn new(parameters: FieldParameters) -> result!(Self) {
        Ok(Self::_from(form::new_field(parameters.try_into()?)?, true))
    }

    #[deprecated(since = "0.5.0", note = "Use Field::new() instead")]
    /// Allocates a new field and initializes it from the contents of a `FieldPrameters`
    /// type given: height, width, row of upper-left corner, column of upper-left corner,
    /// number off-screen rows, and number of additional working buffers.
    pub fn new_field(parameters: FieldParameters) -> result!(Self) {
        Self::new(parameters)
    }

    /// Duplicates a field at a new location. Most attributes (including current
    /// contents, size, validation type, buffer count, growth threshold, justification,
    /// foreground, background, pad character, options, and user pointer) are copied.
    /// Field status and the field page bit are not copied.
    pub fn dup_field(&self, origin: Origin) -> result!(Self) {
        Ok(Self::_from(form::dup_field(self.handle, origin.try_into()?)?, true))
    }

    /// Returns the actual size of the field, and its maximum possible size.
    /// If the field has no size limit, the return max will be set to 0.
    /// A field can be made dynamic by turning off the `FieldOptions::Static`
    /// option with `self.field_opts_off()`.
    pub fn dynamic_field_info(&self) -> result!(FieldInfo) {
        FieldInfo::try_from(form::dynamic_field_info(self.handle)?)
    }

    pub fn field_arg(&self) -> result!(*mut libc::c_void) {
        Ok(form::field_arg(Some(self.handle))?)
    }

    /// Returns the background attribute. The default is `normal::Attributes::Normal`.
    pub fn field_back(&self) -> normal::Attributes {
        form::field_back(Some(self.handle))
    }

    /// Returns a vector of the contents of the given `FieldBuffer`:
    ///
    /// - The buffer contents always have the same length, and are padded with trailing
    ///   spaces as needed to ensure this length is the same.
    /// - The buffer may contain leading spaces, depending on how it was set.
    /// - The buffer contents are set with `self.set_field_buffer()`, or as a side effect of
    ///   any editing operations on the corresponding field.
    /// - Editing operations are based on the window which displays the field, rather
    ///   than a string. The window contains only printable characters, and is filled
    ///   with blanks. If you want the raw data, you must write your own routine that
    ///   copies the value out of the buffer and removes the leading and trailing spaces.
    /// - Because editing operations change the content of the buffer to correspond to
    ///   the window, you should not rely on using buffers for long-term storage of form data.
    pub fn field_buffer(&self, field_buffer: FieldBuffer) -> result!(Vec<i8>) {
        Ok(form::field_buffer(self.handle, field_buffer.number())?)
    }

    /// Returns the foreground attribute. The default is `normal::Attributes::Standout`.
    pub fn field_fore(&self) -> normal::Attributes {
        form::field_fore(Some(self.handle))
    }

    /// Returns the index of the field in the field array of the form it is connected to.
    pub fn field_index(&self) -> result!(usize) {
        Ok(usize::try_from(form::field_index(self.handle)?)?)
    }

    /// Returns the sizes and other attributes passed in to the field at its creation time.
    /// The attributes are: height, width, row of upper-left corner, column of upper-left
    /// corner, number off-screen rows, and number of working buffers.
    pub fn field_info(&self) -> result!(FieldParameters) {
        FieldParameters::try_from(form::field_info(self.handle)?)
    }

    /// Returns a field's justification attribute.
    pub fn field_just(&self) -> result!(FieldJustification) {
        Ok(form::field_just(Some(self.handle))?)
    }

    /// Returns the field's current options.
    pub fn field_opts(&self) -> FieldOptions {
        form::field_opts(Some(self.handle))
    }

    /// Turns off the given options, and leaves others alone.
    pub fn field_opts_off(&self, opts: FieldOptions) -> result!(()) {
        Ok(form::field_opts_off(Some(self.handle), opts)?)
    }

    /// Turns on the given options, and leaves others alone.
    pub fn field_opts_on(&self, opts: FieldOptions) -> result!(()) {
        Ok(form::field_opts_on(Some(self.handle), opts)?)
    }

    /// Returns the given form's pad character. The default is a blank.
    pub fn field_pad(&self) -> result!(char) {
        Ok(form::field_pad(Some(self.handle))?)
    }

    /// Gets the current field status value. The status is `true` whenever the field changes.
    pub fn field_status(&self) -> bool {
        form::field_status(Some(self.handle))
    }

    /// Returns the data type validation for fields.
    pub fn field_type(&self) -> result!(FieldType) {
        Ok(FieldType::_from(form::field_type(Some(self.handle))?, false))
    }

    /// Returns the fields user pointer.
    // TODO: needs testing!
    pub fn field_userptr<T>(&self) -> result!(Option<Box<T>>) {
        Ok(unsafe { form::field_userptr(Some(self.handle))?.as_mut().map(|userptr| Box::from_raw(userptr as *mut libc::c_void as *mut T)) })
    }

    /// Acts like `self.dup_field()`, but the new field shares buffers with its parent.
    /// Attribute data is separate.
    pub fn link_field(&self, origin: Origin) -> result!(Self) {
        Ok(Self::_from(form::link_field(self.handle, origin.try_into()?)?, true))
    }

    /// Moves the given field (which must be disconnected) to a specified location on the screen.
    pub fn move_field(&self, origin: Origin) -> result!(()) {
        Ok(form::move_field(self.handle, origin.try_into()?)?)
    }

    /// The function `new_page()` is a predicate which tests if a given field
    /// marks a page beginning on its form.
    pub fn new_page(&self) -> bool {
        form::new_page(Some(self.handle))
    }

    /// Sets the background attribute of form. This is the highlight used to
    /// display the extent fields in the form.
    pub fn set_field_back(&self, attr: normal::Attributes) -> result!(()) {
        Ok(form::set_field_back(Some(self.handle), attr)?)
    }

    /// Sets the buffer of the given field to contain a given string:
    ///
    /// - `FieldBuffer::Display` is the displayed value of the field.
    /// - Other buffers of type `FieldBuffer::Buffer()` may be allocated by
    ///   applications through but are not manipulated by the forms library.
    pub fn set_field_buffer(&self, field_buffer: FieldBuffer, buffer: &[i8]) -> result!(()) {
        Ok(form::set_field_buffer(self.handle, field_buffer.number(), buffer)?)
    }

    /// Sets the foreground attribute of field. This is the highlight used to
    /// display the field contents.
    pub fn set_field_fore(&self, attr: normal::Attributes) -> result!(()) {
        Ok(form::set_field_fore(Some(self.handle), attr)?)
    }

    /// Sets the justification attribute of a field.
    pub fn set_field_just(&self, justification: FieldJustification) -> result!(()) {
        Ok(form::set_field_just(Some(self.handle), justification)?)
    }

    /// Sets all the given field's options.
    pub fn set_field_opts(&self, opts: FieldOptions) -> result!(()) {
        Ok(form::set_field_opts(Some(self.handle), opts)?)
    }

    /// Sets the character used to fill the field.
    pub fn set_field_pad(&self, pad: char) -> result!(()) {
        Ok(form::set_field_pad(Some(self.handle), pad)?)
    }

    /// Sets the associated status flag of field.
    pub fn set_field_status(&self, status: bool) -> result!(()) {
        Ok(form::set_field_status(Some(self.handle), status)?)
    }

    /// Declares a data type for a given form field.
    /// This is the type checked by validation functions.
    // TODO: needs testing!
    pub fn set_field_type<'a, A, B, C, T>(&self, fieldtype: &T) -> result!(())
        where T: IsFieldType<'a, A, B, C> + HasHandle<FIELDTYPE>
    {
        match match fieldtype.arguments() {
            0    => unsafe { bindings::set_field_type(self.handle, fieldtype._handle()) },
            1    => unsafe { bindings::set_field_type(self.handle, fieldtype._handle(), fieldtype.arg1()) },
            2    => unsafe { bindings::set_field_type(self.handle, fieldtype._handle(), fieldtype.arg1(), fieldtype.arg2()) },
            3    => unsafe { bindings::set_field_type(self.handle, fieldtype._handle(), fieldtype.arg1(), fieldtype.arg2(), fieldtype.arg3()) },
            args => return Err(NCurseswWinError::FieldTypeArguments { func: "set_field_type".to_string(), args })
        } {
            E_OK => Ok(()),
            rc   => Err(NCurseswWinError::from(ncursesw_form_error_from_rc("set_field_type", rc)))
        }
    }

    /// Sets the fields user pointer.
    // TODO: needs testing!
    pub fn set_field_userptr<T>(&self, userptr: Option<Box<&T>>) -> result!(()) {
        Ok(form::set_field_userptr(Some(self.handle), userptr.map(|userptr| Box::into_raw(userptr) as *mut libc::c_void))?)
    }

    /// Sets the maximum size for a dynamic field. An argument of 0 turns off any
    /// maximum size threshold for that field.
    pub fn set_max_field(&self, max: usize) -> result!(()) {
        Ok(form::set_max_field(self.handle, i32::try_from(max)?)?)
    }

    /// Sets or resets a flag marking the given field as the beginning of a new page on its form.
    pub fn set_new_page(&self, new_page_flag: bool) -> result!(()) {
        Ok(form::set_new_page(Some(self.handle), new_page_flag)?)
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

impl AsRef<Field> for Field {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Field> for Field {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Clone for Field {
    fn clone(&self) -> Self {
        Self::_from(self.handle, false)
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Field {{ handle: {:p}, free_on_drop: {} }}", self.handle, self.free_on_drop)
    }
}
