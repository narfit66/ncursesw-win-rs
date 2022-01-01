/*
    src/form/fieldtypes/numeric.rs

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

use std::{fmt, hash::{Hash, Hasher}};
use crate::form::{FieldType, FIELDTYPE_NUMERIC, IsFieldType};

/// This field type accepts a decimal number.
pub struct Numeric<'a> {
    fieldtype: &'a FieldType,
    arguments: u8,
    padding:   i32,
    minimum:   libc::c_double,
    maximum:   libc::c_double
}

impl<'a> Numeric<'a> {
    pub fn new(padding: i32, minimum: libc::c_double, maximum: libc::c_double) -> Self {
        Self { fieldtype: &*FIELDTYPE_NUMERIC, arguments: 3, padding, minimum, maximum }
    }
}

impl<'a> IsFieldType<'a, i32, libc::c_double, libc::c_double> for Numeric<'a> {
    fn fieldtype(&self) -> &'a FieldType {
        self.fieldtype
    }

    fn arguments(&self) -> u8 {
        self.arguments
    }

    fn arg1(&self) -> i32 {
        self.padding
    }

    fn arg2(&self) -> libc::c_double {
        self.minimum
    }

    fn arg3(&self) -> libc::c_double {
        self.maximum
    }
}

unsafe impl<'a> Send for Numeric<'a> { } // too make thread safe
unsafe impl<'a> Sync for Numeric<'a> { } // too make thread safe

// have to implement PartialEq and Eq manually otherwise we get the following:
// 33 |     minimum:   libc::c_double,
//    |     ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::cmp::Eq` is not implemented for `f64`
//    |
//    = note: required by `std::cmp::AssertParamIsEq`
impl<'a> PartialEq for Numeric<'a> {
    fn eq(&self, rhs: &Self) -> bool {
        self.fieldtype == rhs.fieldtype &&
        self.arguments == rhs.arguments &&
        self.padding == rhs.padding &&
        canonicalize(self.minimum) == canonicalize(rhs.minimum) &&
        canonicalize(self.maximum) == canonicalize(rhs.maximum)
    }
}

impl<'a> Eq for Numeric<'a> { }

impl<'a> Hash for Numeric<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fieldtype.hash(state);
        self.arguments.hash(state);
        self.padding.hash(state);
        transmute_value(self.minimum).hash(state);
        transmute_value(self.maximum).hash(state);
    }
}

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

fn canonicalize(value: libc::c_double) -> libc::c_long {
    (value * 1024.0 * 1024.0).round() as libc::c_long
}

// as a f64 does not support hash() transmute to an i64 and
// hash on this bit pattern instead (this is for 64-bit systems,
// on a 32-bit system it would be a f32 and i32).
fn transmute_value(value: libc::c_double) -> libc::c_long {
    value.to_bits() as libc::c_long
}
