/*
    src/form/fieldtypes/alphanumeric.rs

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

use std::fmt;
use crate::form::{FieldType, FIELDTYPE_ALNUM, IsFieldType};

/// This field type accepts alphabetic data and digits; no blanks, no special
/// characters (this is checked at character-entry time).
#[derive(PartialEq, Eq, Hash)]
pub struct AlphaNumeric<'a> {
    fieldtype: &'a FieldType,
    arguments: u8,
    width:     u16
}

impl<'a> AlphaNumeric<'a> {
    pub fn new(width: u16) -> Self {
        Self { fieldtype: &*FIELDTYPE_ALNUM, arguments: 1, width }
    }
}

impl<'a> IsFieldType<'a, i32, i32, i32> for AlphaNumeric<'a> {
    fn fieldtype(&self) -> &'a FieldType {
        self.fieldtype
    }

    fn arguments(&self) -> u8 {
        self.arguments
    }

    fn arg1(&self) -> i32 {
        i32::from(self.width)
    }

    fn arg2(&self) -> i32 { 0 }
    fn arg3(&self) -> i32 { 0 }
}

unsafe impl<'a> Send for AlphaNumeric<'a> { } // too make thread safe
unsafe impl<'a> Sync for AlphaNumeric<'a> { } // too make thread safe

impl <'a>AsRef<AlphaNumeric<'a>> for AlphaNumeric<'a> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl <'a>AsMut<AlphaNumeric<'a>> for AlphaNumeric<'a> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'a> fmt::Debug for AlphaNumeric<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ fieldtype: {:?}, arguments: {}, width: {} }}", self.fieldtype, self.arguments, self.width)
    }
}
