/*
    src/form/fieldtypes/enumerate.rs

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

#![allow(clippy::forget_ref)]
#![allow(clippy::forget_copy)]

use std::{fmt, mem, ptr, convert::TryFrom};

use crate::{NCurseswWinError, cstring::*, form::{FieldType, FIELDTYPE_ENUM, IsFieldType}};

type ENTRY = *const i8;

/// This type allows you to restrict a field's values to be among a specified
/// set of string values (for example, the two-letter postal codes for U.S. states).
#[derive(PartialEq, Eq, Hash)]
pub struct Enumerate<'a> {
    fieldtype:    &'a FieldType,
    arguments:    u8,
    value_list:   *const ENTRY,
    check_case:   bool,
    check_unique: bool
}

impl<'a> Enumerate<'a> {
    pub fn new(value_list: &[&str], check_case: bool, check_unique: bool) -> result!(Self) {
        let entry_handles = unsafe { libc::calloc(value_list.len() + 1, mem::size_of::<ENTRY>()) as *const ENTRY };

        if entry_handles.is_null() {
            Err(NCurseswWinError::OutOfMemory { func: "Enumerate::new".to_string() })
        } else {
            for (offset, value) in value_list.iter().enumerate() {
                let entry = c_str_with_nul!(value);

                unsafe { ptr::write(entry_handles.offset(isize::try_from(offset)?) as *mut _, entry.as_ptr()) };

                mem::forget(entry);
            }

            mem::forget(entry_handles);

            Ok(Self { fieldtype: &*FIELDTYPE_ENUM, arguments: 3, value_list: entry_handles, check_case, check_unique })
        }
    }
}

impl<'a> IsFieldType<'a, *const ENTRY, i32, i32> for Enumerate<'a> {
    fn fieldtype(&self) -> &'a FieldType {
        self.fieldtype
    }

    fn arguments(&self) -> u8 {
        self.arguments
    }

    fn arg1(&self) -> *const ENTRY {
        self.value_list
    }

    fn arg2(&self) -> i32 {
        i32::from(self.check_case)
    }

    fn arg3(&self) -> i32 {
        i32::from(self.check_unique)
    }
}

impl<'a> Drop for Enumerate<'a> {
    fn drop(&mut self) {
        let mut offset = 0;

        unsafe {
            let mut entry_ptr = ptr::read(self.value_list);

            while !entry_ptr.is_null() {
                libc::free(self.value_list.offset(offset) as *mut libc::c_void);

                offset += 1;

                entry_ptr = ptr::read(self.value_list.offset(offset));
            }

            libc::free(self.value_list as *mut libc::c_void);
        }
    }
}

unsafe impl<'a> Send for Enumerate<'a> { } // too make thread safe
unsafe impl<'a> Sync for Enumerate<'a> { } // too make thread safe

impl<'a> fmt::Debug for Enumerate<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ fieldtype: {:?}, arguments: {}, value_list: {:p}, check_case: {}, check_unique: {} }}",
            self.fieldtype,
            self.arguments,
            self.value_list,
            self.check_case,
            self.check_unique
        )
    }
}
