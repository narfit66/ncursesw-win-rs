/*
    src/ncurses/funcs.rs

    Copyright (c) 2019-2021 Stephen Whittle  All rights reserved.

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

#![allow(deprecated)]

use std::{sync::atomic::AtomicBool, panic::{UnwindSafe, catch_unwind}};
use anyhow::{Result, Error};
use crate::{Window, NCurseswWinError, ncurses::ncurses::NCurses};

lazy_static! {
    pub(in crate) static ref INITSCR_CALLED: AtomicBool = AtomicBool::new(false);
    pub(in crate) static ref COLOR_STARTED:  AtomicBool = AtomicBool::new(false);
}

/// Safely initialise NCurses, panic's will be caught correctly and
/// passed back as `NCurseswWinError::Panic`.
/// NCurses should free (as best it can) memory etc correctly.
pub fn ncursesw_entry<F: FnOnce(&Window) -> Result<T> + UnwindSafe, T>(func: F) -> Result<T> {
    // We wrap all our use of ncurseswin with this function.
    match ncursesw_init(|stdscr| {
        // In here we get an initialized Window structure (stdscr) and pass that
        // to our closure, `catch_unwind()` as called in `ncursesw_init()` will
        // return a `Result` of `Ok` so we will wrap our return of `func()`
        // in that.
        match func(stdscr) {
            Err(source) => Ok(Err(source)),
            Ok(value)   => Ok(Ok(value))
        }
    }).unwrap_or_else(|source| Err(match source {
        // This block only runs if there was an error. We might or might not
        // have been able to recover an error message. You technically can pass
        // any value into a panic, but we only get an error message if the panic
        // value was a `String` or `&str`.
        Some(message) => NCurseswWinError::Panic { message },
        None          => NCurseswWinError::Panic { message: "There was a panic, but no message!".to_string() }
    })) {
        // The `Err` branch matches against the `NCurseswWinError::Panic` error
        // from the above `unwrap_or_else()`.
        Err(source) => Err(Error::new(source)),
        // The `Ok` branch unwraps and matches against `ncursesw_init()` error
        // or return value.
        Ok(result)  => result
    }
}

#[deprecated(since = "0.3.0", note = "Use ncursesw_entry() instead")]
/// Safely initialise NCurses, panic will be caught correctly and NCurses free (as best it can) correctly.
pub fn ncursesw_init<F: FnOnce(&Window) -> R + UnwindSafe, R>(func: F) -> Result<R, Option<String>> {
    // use `catch_unwind()` to catch panic's, an error will be a panic
    // so try and convert it into a string.
    catch_unwind(|| {
        // initilise ncurses.
        let ncurses = match NCurses::new() {
            Err(_)     => panic!("ncursesw::initscr() has failed!"),
            Ok(handle) => handle
        };

        func(&ncurses.stdscr())
    }).map_err(|source| match source.downcast_ref::<&str>() {
        Some(andstr) => Some((*andstr).to_string()),
        None         => source.downcast_ref::<String>().map(|string| string.to_string())
    })
}

/// Safely create an application entry point, unlike `ncursesw_entry()` this does
/// not initialise the NCurses library by calling `initscr()`. panic's will be caught
/// correctly and passed back as `NCurseswWinError::Panic`. NCurses should free
/// (as best it can) memory etc correctly.
pub fn safe_entry<F: FnOnce() -> Result<T> + UnwindSafe, T>(func: F) -> Result<T> {
    // We wrap all our use of ncurseswin with this function.
    match safe_init(|| {
        // The `catch_unwind()` in `safe_init()` will return a `Result` of `Ok
        // so we will wrap our return of `func()` in that.
        match func() {
            Err(source) => Ok(Err(source)),
            Ok(value)   => Ok(Ok(value))
        }
    }).unwrap_or_else(|source| Err(match source {
        // This block only runs if there was an error. We might or might not
        // have been able to recover an error message. You technically can pass
        // any value into a panic, but we only get an error message if the panic
        // value was a `String` or `&str`.
        Some(message) => NCurseswWinError::Panic { message },
        None          => NCurseswWinError::Panic { message: "There was a panic, but no message!".to_string() }
    })) {
        // The `Err` branch matches against the `NCurseswWinError::Panic` error
        // from the above `unwrap_or_else()`.
        Err(source) => Err(Error::new(source)),
        // The `Ok` branch unwraps and matches against `safe_init()` error
        // or return value
        Ok(result)  => result
    }
}

// Create an application entry point, panic will be caught correctly.
fn safe_init<F: FnOnce() -> R + UnwindSafe, R>(func: F) -> Result<R, Option<String>> {
    // use `catch_unwind()` to catch panic's, an error will be a panic
    // so try and convert it into a string.
    catch_unwind(|| {
        func()
    }).map_err(|source| match source.downcast_ref::<&str>() {
        Some(andstr) => Some((*andstr).to_string()),
        None         => source.downcast_ref::<String>().map(|string| string.to_string())
    })
}
