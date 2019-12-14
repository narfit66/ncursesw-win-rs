/*
    src/form/fieldtypes/integer.rs

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

use crate::form::{FieldType, FIELDTYPE_INTEGER, IsFieldType};

/// This field type accepts an integer.
pub struct Integer<'a> {
    fieldtype: &'a FieldType,
    arguments: u8,
    padding:   i32,
    minimum:   i32,
    maximum:   i32
}

impl<'a> Integer<'a> {
    pub fn new(padding: i32, minimum: i32, maximum: i32) -> Self {
        Self { fieldtype: &*FIELDTYPE_INTEGER, arguments: 3, padding, minimum, maximum }
    }
}

impl<'a> IsFieldType<'a, i32, i32, i32> for Integer<'a> {
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
