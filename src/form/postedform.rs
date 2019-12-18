/*
    src/form/postedform.rs

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

use std::{fmt, hash::{Hash, Hasher}};

use ncursesw::form;
use crate::{NCurseswWinError, WideChar, gen::NCurseswWindow, form::Form};

pub use ncursesw::form::FormRequest;

/// A Posted (Visible) Form.
pub struct PostedForm<'a> {
    form:   &'a Form,
    posted: bool
}

impl<'a> PostedForm<'a> {
    pub(in crate::form) fn new(form: &'a Form, refresh: bool) -> result!(Self) {
        form::post_form(form._handle())?;

        if refresh {
            form.form_win()?.refresh()?;
        }

        Ok(Self { form, posted: true })
    }

    /// Command processing of `Form` events.
    pub fn form_driver(&self, request: FormRequest) -> result!(Option<i32>) {
        Ok(form::form_driver(self.form._handle(), request)?)
    }

    /// Command processing of `Form` events (wide character mode).
    pub fn form_driver_w(&self, request: FormRequest, wch: WideChar) -> result!(Option<i32>) {
        Ok(form::form_driver_w(self.form._handle(), request, wch)?)
    }

    /// Repost (make visible) this instance, will error if the instance is already posted.
    pub fn repost(&mut self) -> result!(()) {
        form::post_form(self.form._handle())?;

        self.posted = true;

        Ok(())
    }

    /// Unpost (make invisible) this instance, will error if the instance is not posted.
    pub fn unpost(&mut self) -> result!(()) {
        form::unpost_form(self.form._handle())?;

        self.posted = false;

        Ok(())
    }

    /// Has the instance been posted i.e. is it visible.
    pub fn posted(&self) -> bool {
        self.posted
    }

    /// Refresh the form's main window.
    pub fn refresh(&self) -> result!(()) {
        self.form.form_win()?.refresh()
    }

    /// Restore the cursor to te current position associated with the form's selected field.
    pub fn pos_form_cursor(&self) -> result!(()) {
        Ok(form::pos_form_cursor(self.form._handle())?)
    }
}

impl<'a> Drop for PostedForm<'a> {
    fn drop(&mut self) {
        if self.posted {
            if let Err(source) = form::unpost_form(self.form._handle()) {
                panic!("{} @ {:?}", source, self)
            }
        }
    }
}

unsafe impl<'a> Send for PostedForm<'a> { } // too make thread safe
unsafe impl<'a> Sync for PostedForm<'a> { } // too make thread safe

impl<'a> Hash for PostedForm<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.form.hash(state);
    }
}

impl<'a> fmt::Debug for PostedForm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PostedForm {{ form: {:?}, posted: {} }}", self.form, self.posted)
    }
}
