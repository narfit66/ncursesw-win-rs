/*
    src/form/fieldtypes/numeric.rs

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

use crate::form::{FieldType, FIELDTYPE_NUMERIC, IsFieldType};

/// This field type accepts a decimal number.
#[derive(PartialEq, Eq, Hash)]
pub struct Numeric<'a> {
    fieldtype: &'a FieldType,
    arguments: u8,
    padding:   i32,
    minimum:   i32,
    maximum:   i32
}

impl<'a> Numeric<'a> {
    pub fn new(padding: i32, minimum: i32, maximum: i32) -> Self {
        Self { fieldtype: &*FIELDTYPE_NUMERIC, arguments: 3, padding, minimum, maximum }
    }
}

impl<'a> IsFieldType<'a, i32, i32, i32> for Numeric<'a> {
    fn fieldtype(&self) -> &'a FieldType {
        self.fieldtype
    }

    fn arguments(&self) -> u8 {
        self.arguments
    }

    fn arg1(&self) -> i32 {
        self.padding
    }

    fn arg2(&self) -> i32 {
        self.minimum
    }

    fn arg3(&self) -> i32 {
        self.maximum
    }
}

unsafe impl<'a> Send for Numeric<'a> { } // too make thread safe
unsafe impl<'a> Sync for Numeric<'a> { } // too make thread safe

impl<'a> fmt::Debug for Numeric<'a> {
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
