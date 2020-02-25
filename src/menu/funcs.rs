/*
    src/menu/funcs.rs

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

use std::convert::{TryFrom, TryInto};

use ncursesw::menu::{Menu_Hook, MenuRequest, MenuOptions, ItemOptions};
use crate::{
    normal, Screen, Window, HasHandle, NCurseswWinError,
    menu::{
        Menu, MenuSize, MenuSpacing,
        callbacks::{
            set_menu_callback, extern_item_init, extern_item_term,
            extern_menu_init, extern_menu_term, CallbackType
        }
    }
};

#[deprecated(since = "0.6.0")]
pub fn item_init() -> result!(Menu_Hook) {
    Ok(ncursesw::menu::item_init(None)?)
}

/// The menu items options.
pub fn item_opts() -> ItemOptions {
    ncursesw::menu::item_opts(None)
}

/// Set the menu items passed options off.
pub fn item_opts_off(opts: ItemOptions) -> result!(()) {
    Ok(ncursesw::menu::item_opts_off(None, opts)?)
}

/// Set the menu items passed options on.
pub fn item_opts_on(opts: ItemOptions) -> result!(()) {
    Ok(ncursesw::menu::item_opts_on(None, opts)?)
}

#[deprecated(since = "0.6.0")]
pub fn item_term() -> result!(Menu_Hook) {
    Ok(ncursesw::menu::item_term(None)?)
}

/// The menu items user client code defined value.
// TODO: needs testing!
pub fn item_userptr<T>() -> Option<Box<T>> {
    ncursesw::menu::item_userptr(None).as_mut().map(|userptr| unsafe { Box::from_raw(*userptr as *mut T) })
}

pub fn menu_back() -> normal::Attributes {
    ncursesw::menu::menu_back(None)
}

pub fn menu_fore() -> normal::Attributes {
    ncursesw::menu::menu_fore(None)
}

pub fn menu_format() -> result!(MenuSize) {
    Ok(MenuSize::try_from(ncursesw::menu::menu_format(None))?)
}

pub fn menu_grey() -> normal::Attributes {
    ncursesw::menu::menu_grey(None)
}

#[deprecated(since = "0.6.0")]
pub fn menu_init() -> result!(Menu_Hook) {
    Ok(ncursesw::menu::menu_init(None)?)
}

pub fn menu_mark() -> result!(String) {
    Ok(ncursesw::menu::menu_mark(None)?)
}

pub fn menu_opts() -> MenuOptions {
    ncursesw::menu::menu_opts(None)
}

pub fn menu_opts_off(opts: MenuOptions) -> result!(()) {
    Ok(ncursesw::menu::menu_opts_off(None, opts)?)
}

pub fn menu_opts_on(opts: MenuOptions) -> result!(()) {
    Ok(ncursesw::menu::menu_opts_on(None, opts)?)
}

pub fn menu_pad() -> result!(char) {
    Ok(ncursesw::menu::menu_pad(None)?)
}

pub fn menu_request_by_name(name: &str) -> result!(Option<MenuRequest>) {
    Ok(ncursesw::menu::menu_request_by_name(name)?)
}

pub fn menu_request_name(request: MenuRequest) -> result!(String) {
    Ok(ncursesw::menu::menu_request_name(request)?)
}

pub fn menu_spacing() -> result!(MenuSpacing) {
    Ok(MenuSpacing::try_from(ncursesw::menu::menu_spacing(None)?)?)
}

pub fn menu_sub() -> result!(Window) {
    Ok(Window::_from(None, ncursesw::menu::menu_sub(None)?, false))
}

#[deprecated(since = "0.6.0")]
pub fn menu_term() -> result!(Menu_Hook) {
    Ok(ncursesw::menu::menu_term(None)?)
}

// TODO: needs testing!
pub fn menu_userptr<T>() -> Option<Box<T>> {
    ncursesw::menu::menu_userptr(None).as_mut().map(|userptr| unsafe { Box::from_raw(*userptr as *mut T) })
}

pub fn menu_win() -> result!(Window) {
    Ok(Window::_from(None, ncursesw::menu::menu_win(None)?, false))
}

pub fn set_item_init<F>(func: F) -> result!(())
    where F: Fn(&Menu) + 'static + Send
{
    set_menu_callback(None, CallbackType::ItemInit, func);

    Ok(ncursesw::menu::set_item_init(None, Some(extern_item_init))?)
}

/// Set the menu items passed options.
pub fn set_item_opts(opts: ItemOptions) -> result!(()) {
    Ok(ncursesw::menu::set_item_opts(None, opts)?)
}

pub fn set_item_term<F>(func: F) -> result!(())
    where F: Fn(&Menu) + 'static + Send
{
    set_menu_callback(None, CallbackType::ItemTerm, func);

    Ok(ncursesw::menu::set_item_term(None, Some(extern_item_term))?)
}

/// Set the menu items user client code defined value.
// TODO: needs testing!
pub fn set_item_userptr<T>(userptr: Option<Box<&T>>) {
    ncursesw::menu::set_item_userptr(None, userptr.and_then(|userptr| Some(Box::into_raw(userptr) as *mut libc::c_void)))
}

pub fn set_menu_back(attrs: normal::Attributes) -> result!(()) {
    Ok(ncursesw::menu::set_menu_back(None, attrs)?)
}

pub fn set_menu_fore(attrs: normal::Attributes) -> result!(()) {
    Ok(ncursesw::menu::set_menu_fore(None, attrs)?)
}

pub fn set_menu_format(menu_size: MenuSize) -> result!(()) {
    Ok(ncursesw::menu::set_menu_format(None, menu_size.try_into()?)?)
}

pub fn set_menu_grey(attrs: normal::Attributes) -> result!(()) {
    Ok(ncursesw::menu::set_menu_grey(None, attrs)?)
}

pub fn set_menu_init<F>(func: F) -> result!(())
    where F: Fn(&Menu) + 'static + Send
{
    set_menu_callback(None, CallbackType::MenuInit, func);

    Ok(ncursesw::menu::set_menu_init(None, Some(extern_menu_init))?)
}

pub fn set_menu_mark(mark: &str) -> result!(()) {
    Ok(ncursesw::menu::set_menu_mark(None, mark)?)
}

pub fn set_menu_opts(opts: MenuOptions) -> result!(()) {
    Ok(ncursesw::menu::set_menu_opts(None, opts)?)
}

pub fn set_menu_pad(pad: char) -> result!(()) {
    Ok(ncursesw::menu::set_menu_pad(None, pad)?)
}

pub fn set_menu_spacing(menu_spacing: MenuSpacing) -> result!(()) {
    Ok(ncursesw::menu::set_menu_spacing(None, menu_spacing.try_into()?)?)
}

pub fn set_menu_sub(window: Option<&Window>) -> result!(()) {
    Ok(ncursesw::menu::set_menu_sub(None, window.and_then(|window| Some(window._handle())))?)
}

pub fn set_menu_term<F>(func: F) -> result!(())
    where F: Fn(&Menu) + 'static + Send
{
    set_menu_callback(None, CallbackType::MenuTerm, func);

    Ok(ncursesw::menu::set_menu_term(None, Some(extern_menu_term))?)
}

// TODO: needs testing!
pub fn set_menu_userptr<T>(userptr: Option<Box<&T>>) {
    ncursesw::menu::set_menu_userptr(None, userptr.and_then(|userptr| Some(Box::into_raw(userptr) as *mut libc::c_void)))
}

pub fn set_menu_win(window: Option<&Window>) -> result!(()) {
    Ok(ncursesw::menu::set_menu_win(None, window.and_then(|window| Some(window._handle())))?)
}

// screen functions.

pub fn menu_back_sp(screen: &Screen) -> normal::Attributes {
    normal::Attributes::new_sp(screen._handle(), menu_back().into())
}

pub fn menu_fore_sp(screen: &Screen) -> normal::Attributes {
    normal::Attributes::new_sp(screen._handle(), menu_fore().into())
}

pub fn menu_grey_sp(screen: &Screen) -> normal::Attributes {
    normal::Attributes::new_sp(screen._handle(), menu_grey().into())
}

pub fn menu_sub_sp(screen: &Screen) -> result!(Window) {
    Ok(Window::_from(Some(screen._handle()), ncursesw::menu::menu_sub(None)?, false))
}

pub fn menu_win_sp(screen: &Screen) -> result!(Window) {
    Ok(Window::_from(Some(screen._handle()), ncursesw::menu::menu_win(None)?, false))
}
