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
macro_rules! character_result { ($t: ty) => { Option<CharacterResult<$t>> } }

macro_rules! nonblocking_get {
    ($fname: ident, $func: ident, $str: expr, $result: ty) => {
        fn $fname(&self, timeout: Timeout) -> result!(character_result!($result)) {
            // remember the original timeout
            let orig_timeout = self.get_timeout()?;

            // set the timeout to what has been specified.
            self.set_timeout(timeout)?;

            // call $func, if it returned Err and the error is a timeout error
            // then return Ok(None) otherwise return the error.
            // if $func returned Ok then return Ok(Some(result))
            let result = match self.$func() {
                Err(source) => {
                    if source == crate::ncurseswwinerror::timeout_error(&$str) {
                        Ok(None)
                    } else {
                        Err(source)
                    }
                },
                Ok(result) => Ok(Some(result))
            };

            // if we didn't error above then set the timeout back to what it
            // originally was.
            if result.is_ok() {
                self.set_timeout(orig_timeout)?
            }

            result
        }
    }
}

macro_rules! nonblocking_get_with_origin {
    ($fname: ident, $func: ident, $str: expr, $result: ident) => {
        fn $fname(&self, origin: Origin, timeout: Timeout) -> result!(character_result!($result)) {
            // remember the original timeout
            let orig_timeout = self.get_timeout()?;

            // set the timeout to what has been specified.
            self.set_timeout(timeout)?;

            // call $func, if it returned Err and the error is a timeout error
            // then return Ok(None) otherwise return the error.
            // if $func returned Ok then return Ok(Some(result))
            let result = match self.$func(origin) {
                Err(source) => {
                    if source == crate::ncurseswwinerror::timeout_error(&$str) {
                        Ok(None)
                    } else {
                        Err(source)
                    }
                },
                Ok(result) => Ok(Some(result))
            };

            // if we didn't error above then set the timeout back to what it
            // originally was.
            if result.is_ok() {
                self.set_timeout(orig_timeout)?
            }

            result
        }
    }
}
