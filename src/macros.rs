/*
    src/macros.rs

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

#![macro_use]

macro_rules! result { ($t: ty) => { Result<$t, NCurseswWinError> } }

macro_rules! nonblocking_get {
    ($fname: ident, $func: ident, $str: expr, $result: ident) => {
        pub fn $fname(&self, timeout: Option<time::Duration>) -> result!(Option<CharacterResult<$result>>) {
            match timeout {
                None       => self.timeout(time::Duration::new(0, 0))?,
                Some(time) => self.timeout(time)?
            }

            let result = match self.$func() {
                Err(source) => {
                    if source == crate::timeout_error(&$str) {
                        Ok(None)
                    } else {
                        Err(source)
                    }
                },
                Ok(result) => Ok(Some(result))
            };

            self.set_blocking_mode();

            result
        }
    }
}

macro_rules! nonblocking_get_with_origin {
    ($fname: ident, $func: ident, $str: expr, $result: ident) => {
        pub fn $fname(&self, origin: Origin, timeout: Option<time::Duration>) -> result!(Option<CharacterResult<$result>>) {
            match timeout {
                None       => self.timeout(time::Duration::new(0, 0))?,
                Some(time) => self.timeout(time)?
            }

            let result = match self.$func(origin) {
                Err(source) => {
                    if source == crate::timeout_error(&$str) {
                        Ok(None)
                    } else {
                        Err(source)
                    }
                },
                Ok(result) => Ok(Some(result))
            };

            self.set_blocking_mode();

            result
        }
    }
}
