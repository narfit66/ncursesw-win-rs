/*
    src/form/funcs.rs

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

use std::convert::TryFrom;
use ncursesw::form::{
    FormRequest, FormOptions, FieldJustification, FieldOptions
};
use crate::{
    Screen, Window, HasHandle, NCurseswWinError, normal,
    form::{
        Form, Field, FieldType, Form_Hook,
        callbacks::{
            set_form_callback, extern_field_init, extern_field_term,
            extern_form_init, extern_form_term, CallbackType
        }
    }
};

/// Returns the default current field of the given form.
pub fn current_field() -> result!(Field) {
    Ok(Field::_from(ncursesw::form::current_field(None)?, false))
}

pub fn field_arg() -> result!(*mut libc::c_void) {
    Ok(ncursesw::form::field_arg(None)?)
}

/// Returns the default field background attribute. The default is `normal::Attributes::Normal`.
pub fn field_back() -> normal::Attributes {
    ncursesw::form::field_back(None)
}

/// Returns the default field count of fields in form.
pub fn field_count() -> result!(usize) {
    Ok(usize::try_from(ncursesw::form::field_count(None)?)?)
}

/// Returns the default field foreground attribute. The default is `normal::Attributes::Standout`.
pub fn field_fore() -> normal::Attributes {
    ncursesw::form::field_fore(None)
}

#[deprecated(since = "0.6.0")]
/// Returns the default current field init hook.
pub fn field_init() -> result!(Form_Hook) {
    Ok(ncursesw::form::field_init(None)?)
}

/// Returns a field's default justification attribute.
pub fn field_just() -> result!(FieldJustification) {
    Ok(ncursesw::form::field_just(None)?)
}

/// Returns the field's default current options.
pub fn field_opts() -> FieldOptions {
    ncursesw::form::field_opts(None)
}

/// Turns off the given default options, and leaves others alone.
pub fn field_opts_off(opts: FieldOptions) -> result!(()) {
    Ok(ncursesw::form::field_opts_off(None, opts)?)
}

/// Turns on the given default options, and leaves others alone.
pub fn field_opts_on(opts: FieldOptions) -> result!(()) {
    Ok(ncursesw::form::field_opts_on(None, opts)?)
}

/// Returns the given form's default pad character. The default is a blank.
pub fn field_pad() -> result!(char) {
    Ok(ncursesw::form::field_pad(None)?)
}

/// Gets the current field default status value. The status is `true` whenever the field changes.
pub fn field_status() -> bool {
    ncursesw::form::field_status(None)
}

#[deprecated(since = "0.6.0")]
/// Returns the current default field term hook.
pub fn field_term() -> result!(Form_Hook) {
    Ok(ncursesw::form::field_term(None)?)
}

/// Returns the default data type validation for fields.
pub fn field_type() -> result!(FieldType) {
    Ok(FieldType::_from(ncursesw::form::field_type(None)?, false))
}

/// Returns the default fields user pointer.
// TODO: needs testing!
pub fn field_userptr<T>() -> result!(Option<Box<T>>) {
    Ok(unsafe { ncursesw::form::field_userptr(None)?.as_mut().map(|userptr| Box::from_raw(userptr as *mut libc::c_void as *mut T)) })
}

/// The function `new_page()` is a predicate which tests if a given default field
/// marks a page beginning on its form.
pub fn new_page() -> bool {
    ncursesw::form::new_page(None)
}

/// Returns a default vector of the fields of the given form.
pub fn form_fields() -> result!(Vec<Field>) {
    Ok(ncursesw::form::form_fields(None)?.iter().map(|handle| Field::_from(*handle, false)).collect())
}

#[deprecated(since = "0.6.0")]
/// Returns the default current form init hook.
pub fn form_init() -> result!(Form_Hook) {
    Ok(ncursesw::form::form_init(None)?)
}

/// Returns the default form's current options.
pub fn form_opts() -> FormOptions {
    ncursesw::form::form_opts(None)
}

/// Turns off the given default options, and leaves others alone.
pub fn form_opts_off(opts: FormOptions) -> result!(()) {
    Ok(ncursesw::form::form_opts_off(None, opts)?)
}

/// Turns on the given default options, and leaves others alone.
pub fn form_opts_on(opts: FormOptions) -> result!(()) {
    Ok(ncursesw::form::form_opts_on(None, opts)?)
}

/// Returns the form's default current page number.
pub fn form_page() -> result!(usize) {
    Ok(usize::try_from(ncursesw::form::form_page(None)?)?)
}

/// Searches in the name-table for a request with the given name and returns
/// its request code as a `Some`. Otherwise `None` is returned.
pub fn form_request_by_name(name: &str) -> result!(Option<FormRequest>) {
    Ok(ncursesw::form::form_request_by_name(name)?)
}

/// Returns the printable name of a form request code.
pub fn form_request_name(request: FormRequest) -> result!(String) {
    Ok(ncursesw::form::form_request_name(request)?)
}

/// Return the forms default sub-window.
pub fn form_sub() -> result!(Window) {
    Ok(Window::_from(None, ncursesw::form::form_sub(None)?, false))
}

#[deprecated(since = "0.6.0")]
/// Returns the default current form term hook.
pub fn form_term() -> result!(Form_Hook) {
    Ok(ncursesw::form::form_term(None)?)
}

/// Returns the forms default user pointer.
// TODO: needs testing!
pub fn form_userptr<T>() -> result!(Option<Box<T>>) {
    Ok(unsafe { ncursesw::form::form_userptr(None)?.as_mut().map(|userptr| Box::from_raw(userptr as *mut libc::c_void as *mut T)) })
}

/// Return the default forms main window.
pub fn form_win() -> result!(Window) {
    Ok(Window::_from(None, ncursesw::form::form_win(None)?, false))
}

/// Sets the background attribute of form. This is the highlight used to
/// display the extent fields in the form.
pub fn set_field_back(attr: normal::Attributes) -> result!(()) {
    Ok(ncursesw::form::set_field_back(None, attr)?)
}

/// Sets the foreground attribute of field. This is the highlight used to
/// display the field contents.
pub fn set_field_fore(attr: normal::Attributes) -> result!(()) {
    Ok(ncursesw::form::set_field_fore(None, attr)?)
}

/// Sets a default callback to be called at form-post time and each time
/// the selected field changes (after the change).
pub fn set_field_init<F>(func: F) -> result!(())
    where F: Fn(&Form) + 'static + Send
{
    set_form_callback(None, CallbackType::FieldInit, func);

    Ok(ncursesw::form::set_field_init(None, Some(extern_field_init))?)
}

/// Sets the justification attribute of a field.
pub fn set_field_just(justification: FieldJustification) -> result!(()) {
    Ok(ncursesw::form::set_field_just(None, justification)?)
}

/// Sets all the given field's options.
pub fn set_field_opts(opts: FieldOptions) -> result!(()) {
    Ok(ncursesw::form::set_field_opts(None, opts)?)
}

/// Sets the character used to fill the field.
pub fn set_field_pad(pad: char) -> result!(()) {
    Ok(ncursesw::form::set_field_pad(None, pad)?)
}

/// Sets the associated status flag of field.
pub fn set_field_status(status: bool) -> result!(()) {
    Ok(ncursesw::form::set_field_status(None, status)?)
}

/// Sets a default callback to be called at form-unpost time and each time
/// the selected field changes (before the change).
pub fn set_field_term<F>(func: F) -> result!(())
    where F: Fn(&Form) + 'static + Send
{
    set_form_callback(None, CallbackType::FieldTerm, func);

    Ok(ncursesw::form::set_field_term(None, Some(extern_field_term))?)
}

/// Sets the fields user pointer.
// TODO: needs testing!
pub fn set_field_userptr<T>(userptr: Option<Box<&T>>) -> result!(()) {
    Ok(ncursesw::form::set_field_userptr(None, userptr.map(|userptr| Box::into_raw(userptr) as *mut libc::c_void))?)
}

/// Sets or resets a flag marking the given field as the beginning of a new page on its form.
pub fn set_new_page(new_page_flag: bool) -> result!(()) {
    Ok(ncursesw::form::set_new_page(None, new_page_flag)?)
}

/// Sets a default callback to be called at form-post time and just after
/// a page change once it is posted.
pub fn set_form_init<F>(func: F) -> result!(())
    where F: Fn(&Form) + 'static + Send
{
    set_form_callback(None, CallbackType::FormInit, func);

    Ok(ncursesw::form::set_form_init(None, Some(extern_form_init))?)
}

/// Sets all the given form's options.
pub fn set_form_opts(opts: FormOptions) -> result!(()) {
    Ok(ncursesw::form::set_form_opts(None, opts)?)
}

/// Sets the forms sub-window.
pub fn set_form_sub(window: Option<&Window>) -> result!(()) {
    Ok(ncursesw::form::set_form_sub(None, window.map(|window| window._handle()))?)
}

/// Sets a default callback to be called at form-unpost time and just before
/// a page change once it is posted.
pub fn set_form_term<F>(func: F) -> result!(())
    where F: Fn(&Form) + 'static + Send
{
    set_form_callback(None, CallbackType::FormTerm, func);

    Ok(ncursesw::form::set_form_term(None, Some(extern_form_term))?)
}

/// Sets the forms user pointer.
// TODO: needs testing!
pub fn set_form_userptr<T>(userptr: Option<Box<&T>>) -> result!(()) {
    Ok(ncursesw::form::set_form_userptr(None, userptr.map(|userptr| Box::into_raw(userptr) as *mut libc::c_void))?)
}

/// Set the forms main window.
pub fn set_form_win(window: Option<&Window>) -> result!(()) {
    Ok(ncursesw::form::set_form_win(None, window.map(|window| window._handle()))?)
}

// screen functions.

/// Returns the default field background attribute. The default is `normal::Attributes::Normal`.
pub fn field_back_sp(screen: &Screen) -> normal::Attributes {
    normal::Attributes::new_sp(screen._handle(), ncursesw::form::field_back(None).into())
}

/// Returns the default field foreground attribute. The default is `normal::Attributes::Standout`.
pub fn field_fore_sp(screen: &Screen) -> normal::Attributes {
    normal::Attributes::new_sp(screen._handle(), ncursesw::form::field_fore(None).into())
}

/// Return the forms sub-window.
pub fn form_sub_sp(screen: &Screen) -> result!(Window) {
    Ok(Window::_from(Some(screen._handle()), ncursesw::form::form_sub(None)?, false))
}

/// Return the forms sub-window.
pub fn form_win_sp(screen: &Screen) -> result!(Window) {
    Ok(Window::_from(Some(screen._handle()), ncursesw::form::form_win(None)?, false))
}
