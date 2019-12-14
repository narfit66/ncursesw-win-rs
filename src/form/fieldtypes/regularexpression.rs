/*
    src/form/fieldtypes/regularexpression.rs

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

use std::fmt;

use crate::{NCurseswWinError, form::{FieldType, FIELDTYPE_REGEXP, IsFieldType}};
use crate::cstring::*;

/// This field type accepts data matching a regular expression.
#[derive(PartialEq, Eq, Hash)]
pub struct RegularExpression<'a> {
    fieldtype: &'a FieldType,
    arguments: u8,
    regexp:    &'a [i8]
}

impl<'a> RegularExpression<'a> {
    pub fn new(regexp: &str) -> result!(Self) {
        Ok(Self { fieldtype: &*FIELDTYPE_REGEXP, arguments: 1, regexp: c_str_with_nul!(regexp) })
    }
}

impl<'a> IsFieldType<'a, *const i8, i32, i32> for RegularExpression<'a> {
    fn fieldtype(&self) -> &'a FieldType {
        self.fieldtype
    }

    fn arguments(&self) -> u8 {
        self.arguments
    }

    fn arg1(&self) -> *const i8 {
        self.regexp.as_ptr()
    }

    fn arg2(&self) -> i32 { 0 }
    fn arg3(&self) -> i32 { 0 }
}

unsafe impl<'a> Send for RegularExpression<'a> { } // too make thread safe
unsafe impl<'a> Sync for RegularExpression<'a> { } // too make thread safe

impl<'a> fmt::Debug for RegularExpression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ fieldtype: {:?}, arguments: {}, regexp: {:?} }}", self.fieldtype, self.arguments, self.regexp)
    }
}
