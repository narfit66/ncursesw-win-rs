/*
    src/form/fieldtypes/integer.rs

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

use std::fmt;
use crate::form::{FieldType, FIELDTYPE_INTEGER, IsFieldType};

/// This field type accepts an integer.
#[derive(PartialEq, Eq, Hash)]
pub struct Integer<'a> {
    fieldtype: &'a FieldType,
    arguments: u8,
    padding:   i32,
    minimum:   libc::c_long,
    maximum:   libc::c_long
}

impl<'a> Integer<'a> {
    pub fn new(padding: i32, minimum: libc::c_long, maximum: libc::c_long) -> Self {
        Self { fieldtype: &*FIELDTYPE_INTEGER, arguments: 3, padding, minimum, maximum }
    }
}

impl<'a> IsFieldType<'a, i32, libc::c_long, libc::c_long> for Integer<'a> {
    fn fieldtype(&self) -> &'a FieldType {
        self.fieldtype
    }

    fn arguments(&self) -> u8 {
        self.arguments
    }

    fn arg1(&self) -> i32 {
        self.padding
    }

    fn arg2(&self) -> libc::c_long {
        self.minimum
    }

    fn arg3(&self) -> libc::c_long {
        self.maximum
    }
}

unsafe impl<'a> Send for Integer<'a> { } // too make thread safe
unsafe impl<'a> Sync for Integer<'a> { } // too make thread safe

impl<'a> fmt::Debug for Integer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ fieldtype: {:?}, arguments: {}, padding: {}, minimum: {}, maximum: {} }}",
            self.fieldtype,
            self.arguments,
            self.padding,
            self.minimum,
            self.maximum
        )
    }
}
