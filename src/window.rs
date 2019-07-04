/*
    src/window.rs

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

#![allow(deprecated)]
#![allow(clippy::too_many_arguments)]

use std::{path, time};

use ncursesw::{WINDOW, Origin, Size, CharacterResult, AttributesColorPairSet, Region, Changed, ChtypeChar, ChtypeString, ComplexChar, ComplexString, WideString, WideCharResult, NCurseswError};
use ncursesw::normal;
use ncursesw::gen::{AttributesType, ColorPairType, ColorAttributeTypes};

pub struct Window {
    handle:       WINDOW, // pointer to ncurses _win_st internal structure
    free_on_drop: bool    // free WINDOW handle on drop of structure
}

unsafe impl Send for Window { } // too make thread safe
unsafe impl Sync for Window { } // too make thread safe

impl Window {
    pub fn newwin(size: Size, origin: Origin) -> result!(Self) {
        match ncursesw::newwin(size, origin) {
            Err(e)     => Err(e),
            Ok(handle) => Ok(Self::from(handle, true))
        }
    }

    // make a new instance from the passed ncurses _win_st pointer and specify
    // if the handle is to be free'd when the structure is dropped.
    //
    // free_on_drop is false in call's such as getparent(&self) where we are
    // 'peeking' the Window but it would be invalid to free the handle when
    // our instance goes out of scope.
    pub(crate) fn from(handle: WINDOW, free_on_drop: bool) -> Self {
        Self { handle, free_on_drop }
    }

    // get the ncurses _win_st pointer for this Window structure.
    pub(crate) fn handle(&self) -> WINDOW {
        self.handle
    }

    pub fn derwin(&self, size: Size, origin: Origin) -> result!(Self) {
        match ncursesw::derwin(self.handle, size, origin) {
            Err(e)     => Err(e),
            Ok(handle) => Ok(Self::from(handle, true))
        }
    }

    pub fn dupwin(&self) -> result!(Self) {
        match ncursesw::dupwin(self.handle) {
            Err(e)     => Err(e),
            Ok(handle) => Ok(Self::from(handle, true))
        }
    }

    pub fn subpad(&self, size: Size, origin: Origin) -> result!(Self) {
        match ncursesw::subpad(self.handle, size, origin) {
            Err(e)     => Err(e),
            Ok(handle) => Ok(Self::from(handle, true))
        }
    }

    pub fn subwin(&self, size: Size, origin: Origin) -> result!(Self) {
        match ncursesw::subwin(self.handle, size, origin) {
            Err(e)     => Err(e),
            Ok(handle) => Ok(Self::from(handle, true))
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if self.free_on_drop {
            if let Err(e) = ncursesw::delwin(self.handle) {
                panic!(e.to_string())
            }
        }
    }
}

impl Window {
    pub fn addchnstr(&self, chstr: &ChtypeString, number: i32) -> result!(()) {
        ncursesw::waddchnstr(self.handle, chstr, number)
    }

    pub fn addch(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::waddch(self.handle, ch)
    }

    pub fn addchstr(&self, chstr: &ChtypeString) -> result!(()) {
        ncursesw::waddchstr(self.handle, chstr)
    }

    pub fn addnstr(&self, str: &str, number: i32) -> result!(()) {
        ncursesw::waddnstr(self.handle, str, number)
    }

    pub fn addnwstr(&self, wstr: &WideString, number: i32) -> result!(()) {
        ncursesw::waddnwstr(self.handle, wstr, number)
    }

    pub fn addstr(&self, str: &str) -> result!(()) {
        ncursesw::waddstr(self.handle, str)
    }

    pub fn add_wchnstr(&self, wchstr: &ComplexString, number: i32) -> result!(()) {
        ncursesw::wadd_wchnstr(self.handle, wchstr, number)
    }

    pub fn add_wch(&self, wch: ComplexChar) -> result!(()) {
        ncursesw::wadd_wch(self.handle, wch)
    }

    pub fn add_wchstr(&self, wchstr: &ComplexString) -> result!(()) {
        ncursesw::wadd_wchstr(self.handle, wchstr)
    }

    pub fn addwstr(&self, wstr: &WideString) -> result!(()) {
        ncursesw::waddwstr(self.handle, wstr)
    }

    pub fn attr_get(&self) -> result!(AttributesColorPairSet) {
        ncursesw::wattr_get(self.handle)
    }

    pub fn attr_off<A, T>(&self, attrs: A) -> result!(()) where A: AttributesType<T>, T: ColorAttributeTypes {
        ncursesw::wattr_off(self.handle, attrs)
    }

    pub fn attr_on<A, T>(&self, attrs: A) -> result!(()) where A: AttributesType<T>, T: ColorAttributeTypes {
        ncursesw::wattr_on(self.handle, attrs)
    }

    pub fn attr_set<A, P, T>(&self, attrs: A, color_pair: P) -> result!(()) where A: AttributesType<T>, P: ColorPairType<T>, T: ColorAttributeTypes {
        ncursesw::wattr_set(self.handle, attrs, color_pair)
    }

    pub fn attroff(&self, attrs: normal::Attributes) -> result!(()) {
        ncursesw::wattroff(self.handle, attrs)
    }

    pub fn attron(&self, attrs: normal::Attributes) -> result!(()) {
        ncursesw::wattron(self.handle, attrs)
    }

    pub fn attrset(&self, attrs: normal::Attributes) -> result!(()) {
        ncursesw::wattrset(self.handle, attrs)
    }

    pub fn bkgd(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::wbkgd(self.handle, ch)
    }

    pub fn bkgdset(&self, ch: ChtypeChar) {
        ncursesw::wbkgdset(self.handle, ch)
    }

    pub fn bkgrnd(&self, wch: ComplexChar) -> result!(()) {
        ncursesw::wbkgrnd(self.handle, wch)
    }

    pub fn bkgrndset(&self, wch: ComplexChar) {
        ncursesw::wbkgrndset(self.handle, wch)
    }

    pub fn border(&self, ls: ChtypeChar, rs: ChtypeChar, ts: ChtypeChar, bs: ChtypeChar, tl: ChtypeChar, tr: ChtypeChar, bl: ChtypeChar, br: ChtypeChar) -> result!(()) {
        ncursesw::wborder(self.handle, ls, rs, ts, bs, tl, tr, bl, br)
    }

    pub fn border_set(&self, ls: ComplexChar, rs: ComplexChar, ts: ComplexChar, bs: ComplexChar, tl: ComplexChar, tr: ComplexChar, bl: ComplexChar, br: ComplexChar) -> result!(()) {
        ncursesw::wborder_set(self.handle, ls, rs, ts, bs, tl, tr, bl, br)
    }

    pub fn box_set(&self, verch: ComplexChar, horch: ComplexChar) -> result!(()) {
        ncursesw::box_set(self.handle, verch, horch)
    }

    pub fn chgat<A, P, T>(&self, number: i32, attrs: A, color_pair: P) -> result!(()) where A: AttributesType<T>, P: ColorPairType<T>, T: ColorAttributeTypes {
        ncursesw::wchgat(self.handle, number, attrs, color_pair)
    }

    pub fn clearok(&self, bf: bool) -> result!(()) {
        ncursesw::clearok(self.handle, bf)
    }

    pub fn clear(&self) -> result!(()) {
        ncursesw::wclear(self.handle)
    }

    pub fn clrtobot(&self) -> result!(()) {
        ncursesw::wclrtobot(self.handle)
    }

    pub fn clrtoeol(&self) -> result!(()) {
        ncursesw::wclrtoeol(self.handle)
    }

    pub fn color_set<P, T>(&self, color_pair: P) -> result!(()) where P: ColorPairType<T>, T: ColorAttributeTypes {
        ncursesw::wcolor_set(self.handle, color_pair)
    }

    pub fn copywin(
        &self,
        dstwin: &Self,
        smin: Origin,
        dmin: Origin,
        dmax: Origin,
        overlay: bool) -> result!(())
    {
        ncursesw::copywin(self.handle, dstwin.handle, smin, dmin, dmax, overlay)
    }

    pub fn cursyncup(&self) {
        ncursesw::wcursyncup(self.handle);
    }

    pub fn delch(&self) -> result!(()) {
        ncursesw::wdelch(self.handle)
    }

    pub fn echochar(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::wechochar(self.handle, ch)
    }

    pub fn echo_wchar(&self, wch: ComplexChar) -> result!(()) {
        ncursesw::wecho_wchar(self.handle, wch)
    }

    pub fn erase(&self) -> result!(()) {
        ncursesw::werase(self.handle)
    }

    pub fn getattrs(&self) -> normal::Attributes {
        ncursesw::getattrs(self.handle)
    }

    pub fn getbegx(&self) -> result!(i32) {
        ncursesw::getbegx(self.handle)
    }

    pub fn getbegy(&self) -> result!(i32) {
        ncursesw::getbegy(self.handle)
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use origin() instead")]
    pub fn getbegyx(&self) -> result!(Origin) {
        ncursesw::getbegyx(self.handle)
    }

    pub fn getbkgd(&self) -> ChtypeChar {
        ncursesw::getbkgd(self.handle)
    }

    pub fn getbkgrnd(&self) -> result!(ComplexChar) {
        ncursesw::wgetbkgrnd(self.handle)
    }

    pub fn getch(&self) -> result!(CharacterResult) {
        ncursesw::wgetch(self.handle)
    }

    pub fn getcurx(&self) -> result!(i32) {
        ncursesw::getcurx(self.handle)
    }

    pub fn getcury(&self) -> result!(i32) {
        ncursesw::getcury(self.handle)
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use cursor() instead")]
    pub fn getcuryx(&self) -> result!(Origin) {
        ncursesw::getcuryx(self.handle)
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use get_timeout() instead")]
    pub fn getdelay(&self) -> result!(time::Duration) {
        ncursesw::wgetdelay(self.handle)
    }

    pub fn getmaxx(&self) -> result!(i32) {
        ncursesw::getmaxx(self.handle)
    }

    pub fn getmaxy(&self) -> result!(i32) {
        ncursesw::getmaxy(self.handle)
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use size() instead")]
    pub fn getmaxyx(&self) -> result!(Size) {
        ncursesw::getmaxyx(self.handle)
    }

    pub fn getnstr(&self, number: i32) -> result!(String) {
        ncursesw::wgetnstr(self.handle, number)
    }

    pub fn getn_wstr(&self, number: i32) -> result!(WideString) {
        ncursesw::wgetn_wstr(self.handle, number)
    }

    /// returns the parent Window for subwindows, or None if their is no parent.
    pub fn getparent(&self) -> Option<Self> {
        match ncursesw::wgetparent(self.handle) {
            None         => None,
            Some(handle) => Some(Self::from(handle, false))
        }
    }

    pub fn getparx(&self) -> result!(i32) {
        ncursesw::getparx(self.handle)
    }

    pub fn getpary(&self) -> result!(i32) {
        ncursesw::getpary(self.handle)
    }

    pub fn getparyx(&self) -> result!(Origin) {
        ncursesw::getparyx(self.handle)
    }

    pub fn getscrreg(&self) -> result!(Region) {
        ncursesw::wgetscrreg(self.handle)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use getnstr() instead")]
    pub fn getstr(&self) -> result!(String) {
        ncursesw::wgetstr(self.handle)
    }

    pub fn get_wch(&self) -> result!(WideCharResult) {
        ncursesw::wget_wch(self.handle)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use getn_wstr() instead")]
    pub fn get_wstr(&self) -> result!(WideString) {
        ncursesw::wget_wstr(self.handle)
    }

    pub fn hline(&self, ch: ChtypeChar, number: i32) -> result!(()) {
        ncursesw::whline(self.handle, ch, number)
    }

    pub fn hline_set(&self, wch: ComplexChar, number: i32) -> result!(()) {
        ncursesw::whline_set(self.handle, wch, number)
    }

    pub fn idcok(&self, bf: bool) {
        ncursesw::idcok(self.handle, bf)
    }

    pub fn idlok(&self, bf: bool) -> result!(()) {
        ncursesw::idlok(self.handle, bf)
    }

    pub fn immedok(&self, bf: bool) {
        ncursesw::immedok(self.handle, bf)
    }

    pub fn inchnstr(&self, number: i32) -> result!(ChtypeString) {
        ncursesw::winchnstr(self.handle, number)
    }

    pub fn inch(&self) -> ChtypeChar {
        ncursesw::winch(self.handle)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use inchnstr() instead")]
    pub fn inchstr(&self) -> result!(ChtypeString) {
        ncursesw::winchstr(self.handle)
    }

    pub fn innstr(&self, number: i32) -> result!(String) {
        ncursesw::winnstr(self.handle, number)
    }

    pub fn innwstr(&self, number: i32) -> result!(WideString) {
        ncursesw::winnwstr(self.handle, number)
    }

    pub fn insch(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::winsch(self.handle, ch)
    }

    pub fn insdelln(&self, number: i32) -> result!(()) {
        ncursesw::winsdelln(self.handle, number)
    }

    pub fn insertln(&self) -> result!(()) {
        ncursesw::winsertln(self.handle)
    }

    pub fn insnstr(&self, str: &str, number: i32) -> result!(()) {
        ncursesw::winsnstr(self.handle, str, number)
    }

    pub fn ins_nwstr(&self, wstr: &WideString, number: i32) -> result!(()) {
        ncursesw::wins_nwstr(self.handle, wstr, number)
    }

    pub fn insstr(&self, str: &str) -> result!(()) {
        ncursesw::winsstr(self.handle, str)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use innstr() instead")]
    pub fn instr(&self) -> result!(String) {
        ncursesw::winstr(self.handle)
    }

    pub fn ins_wch(&self, wch: &ComplexString) -> result!(()) {
        ncursesw::wins_wch(self.handle, wch)
    }

    pub fn ins_wstr(&self, wstr: &WideString) -> result!(()) {
        ncursesw::wins_wstr(self.handle, wstr)
    }

    pub fn intrflush(&self, bf: bool) -> result!(()) {
        ncursesw::intrflush(self.handle, bf)
    }

    pub fn in_wchnstr(&self, number: i32) -> result!(ComplexString) {
        ncursesw::win_wchnstr(self.handle, number)
    }

    pub fn in_wch(&self) -> result!(ComplexChar) {
        ncursesw::win_wch(self.handle)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use in_wchnstr() instead")]
    pub fn in_wchstr(&self) -> result!(ComplexString) {
        ncursesw::win_wchstr(self.handle)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use innwstr() instead")]
    pub fn inwstr(&self) -> result!(WideString) {
        ncursesw::winwstr(self.handle)
    }

    pub fn is_cleared(&self) -> bool {
        ncursesw::is_cleared(self.handle)
    }

    pub fn is_idcok(&self) -> bool {
        ncursesw::is_idcok(self.handle)
    }

    pub fn is_idlok(&self) -> bool {
        ncursesw::is_idlok(self.handle)
    }

    pub fn is_immedok(&self) -> bool {
        ncursesw::is_immedok(self.handle)
    }

    pub fn is_keypad(&self) -> bool {
        ncursesw::is_keypad(self.handle)
    }

    pub fn is_leaveok(&self) -> bool {
        ncursesw::is_leaveok(self.handle)
    }

    pub fn is_linetouched(&self, line: i32) -> bool {
        ncursesw::is_linetouched(self.handle, line)
    }

    pub fn is_nodelay(&self) -> bool {
        ncursesw::is_nodelay(self.handle)
    }

    pub fn is_notimeout(&self) -> bool {
        ncursesw::is_notimeout(self.handle)
    }

    pub fn is_pad(&self) -> bool {
        ncursesw::is_pad(self.handle)
    }

    pub fn is_scrollok(&self) -> bool {
        ncursesw::is_scrollok(self.handle)
    }

    pub fn is_syncok(&self) -> bool {
        ncursesw::is_syncok(self.handle)
    }

    pub fn is_wintouched(&self) -> bool {
        ncursesw::is_wintouched(self.handle)
    }

    pub fn keypad(&self, bf: bool) -> result!(()) {
        ncursesw::keypad(self.handle, bf)
    }

    pub fn leaveok(&self, bf: bool) -> result!(()) {
        ncursesw::leaveok(self.handle, bf)
    }

    pub fn meta(&self, bf: bool) -> result!(()) {
        ncursesw::meta(self.handle, bf)
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use set_cursor() instead")]
    pub fn r#move(&self, origin: Origin) -> result!(()) {
        ncursesw::wmove(self.handle, origin)
    }

    pub fn mvaddchnstr(&self, origin: Origin, chstr: &ChtypeString, number: i32) -> result!(()) {
        ncursesw::mvwaddchnstr(self.handle, origin, chstr, number)
    }

    pub fn mvaddch(&self, origin: Origin, ch: ChtypeChar) -> result!(()) {
        ncursesw::mvwaddch(self.handle, origin, ch)
    }

    pub fn mvaddchstr(&self, origin: Origin, chstr: &ChtypeString) -> result!(()) {
        ncursesw::mvwaddchstr(self.handle, origin, chstr)
    }

    pub fn mvaddnstr(&self, origin: Origin, str: &str, number: i32) -> result!(()) {
        ncursesw::mvwaddnstr(self.handle, origin, str, number)
    }

    pub fn mvaddnwstr(&self, origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
        ncursesw::mvwaddnwstr(self.handle, origin, wstr, number)
    }

    pub fn mvaddstr(&self, origin: Origin, str: &str) -> result!(()) {
        ncursesw::mvwaddstr(self.handle, origin, str)
    }

    pub fn mvadd_wchnstr(&self, origin: Origin, wchstr: &ComplexString, number: i32) -> result!(()) {
        ncursesw::mvwadd_wchnstr(self.handle, origin, wchstr, number)
    }

    pub fn mvadd_wch(&self, origin: Origin, wch: ComplexChar) -> result!(()) {
        ncursesw::mvwadd_wch(self.handle, origin, wch)
    }

    pub fn mvadd_wchstr(&self, origin: Origin, wchstr: &ComplexString) -> result!(()) {
        ncursesw::mvwadd_wchstr(self.handle, origin, wchstr)
    }

    pub fn mvaddwstr(&self, origin: Origin, wstr: &WideString) -> result!(()) {
        ncursesw::mvwaddwstr(self.handle, origin, wstr)
    }

    pub fn mvchgat<A, P, T>(&self, origin: Origin, number: i32, attrs: A, color_pair: P) -> result!(()) where A: AttributesType<T>, P: ColorPairType<T>, T: ColorAttributeTypes {
        ncursesw::mvwchgat(self.handle, origin, number, attrs, color_pair)
    }

    pub fn mvdelch(&self, origin: Origin) -> result!(()) {
        ncursesw::mvwdelch(self.handle, origin)
    }

    pub fn mvderwin(&self, origin: Origin) -> result!(()) {
        ncursesw::mvderwin(self.handle, origin)
    }

    pub fn mvgetch(&self, origin: Origin) -> result!(CharacterResult) {
        ncursesw::mvwgetch(self.handle, origin)
    }

    pub fn mvgetnstr(&self, origin: Origin, number: i32) -> result!(String) {
        ncursesw::mvwgetnstr(self.handle, origin, number)
    }

    pub fn mvgetn_wstr(&self, origin: Origin, number: i32) -> result!(WideString) {
        ncursesw::mvwgetn_wstr(self.handle, origin, number)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvgetnstr() instead")]
    pub fn mvgetstr(&self, origin: Origin) -> result!(String) {
        ncursesw::mvwgetstr(self.handle, origin)
    }

    pub fn mvget_wch(&self, origin: Origin) -> result!(WideCharResult) {
        ncursesw::mvwget_wch(self.handle, origin)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvgetn_wstr() instead")]
    pub fn mvget_wstr(&self, origin: Origin) -> result!(WideString) {
        ncursesw::mvwget_wstr(self.handle, origin)
    }

    pub fn mvhline(&self, origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
        ncursesw::mvwhline(self.handle, origin, ch, number)
    }

    pub fn mvhline_set(&self, origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
        ncursesw::mvwhline_set(self.handle, origin, wch, number)
    }

    pub fn mvinchnstr(&self, origin: Origin, number: i32) -> result!(ChtypeString) {
        ncursesw::mvwinchnstr(self.handle, origin, number)
    }

    pub fn mvinch(&self, origin: Origin) -> ChtypeChar {
        ncursesw::mvwinch(self.handle, origin)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvinchnstr() instead")]
    pub fn mvinchstr(&self, origin: Origin) -> result!(ChtypeString) {
        ncursesw::mvwinchstr(self.handle, origin)
    }

    pub fn mvinnstr(&self, origin: Origin, number: i32) -> result!(String) {
        ncursesw::mvwinnstr(self.handle, origin, number)
    }

    pub fn mvinnwstr(&self, origin: Origin, number: i32) -> result!(WideString) {
        ncursesw::mvwinnwstr(self.handle, origin, number)
    }

    pub fn mvinsch(&self, origin: Origin, ch: ChtypeChar) -> result!(()) {
        ncursesw::mvwinsch(self.handle, origin, ch)
    }

    pub fn mvinsnstr(&self, origin: Origin, str: &str, number: i32) -> result!(()) {
        ncursesw::mvwinsnstr(self.handle, origin, str, number)
    }

    pub fn mvins_nwstr(&self, origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
        ncursesw::mvwins_nwstr(self.handle, origin, wstr, number)
    }

    pub fn mvinsstr(&self, origin: Origin, str: &str) -> result!(()) {
        ncursesw::mvwinsstr(self.handle, origin, str)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvinnstr() instead")]
    pub fn mvinstr(&self, origin: Origin) -> result!(String) {
        ncursesw::mvwinstr(self.handle, origin)
    }

    pub fn mvins_wch(&self, origin: Origin, wch: &ComplexString) -> result!(()) {
        ncursesw::mvwins_wch(self.handle, origin, wch)
    }

    pub fn mvins_wstr(&self, origin: Origin, wstr: &WideString) -> result!(()) {
        ncursesw::mvwins_wstr(self.handle, origin, wstr)
    }

    pub fn mvin_wchnstr(&self, origin: Origin, number: i32) -> result!(ComplexString) {
        ncursesw::mvwin_wchnstr(self.handle, origin, number)
    }

    pub fn mvin_wch(&self, origin: Origin) -> result!(ComplexChar) {
        ncursesw::mvwin_wch(self.handle, origin)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvin_wchnstr() instead")]
    pub fn mvin_wchstr(&self, origin: Origin) -> result!(ComplexString) {
        ncursesw::mvwin_wchstr(self.handle, origin)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvinnwstr() instead")]
    pub fn mvinwstr(&self, origin: Origin) -> result!(WideString) {
        ncursesw::mvwinwstr(self.handle, origin)
    }

    pub fn mvvline(&self, origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
        ncursesw::mvwvline(self.handle, origin, ch, number)
    }

    pub fn mvvline_set(&self, origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
        ncursesw::mvwvline_set(self.handle, origin, wch, number)
    }

    pub fn mvwin(&self, origin: Origin) -> result!(()) {
        ncursesw::mvwin(self.handle, origin)
    }

    pub fn nodelay(&self, bf: bool) -> result!(()) {
        ncursesw::nodelay(self.handle, bf)
    }

    pub fn notimeout(&self, bf: bool) -> result!(()) {
        ncursesw::notimeout(self.handle, bf)
    }

    pub fn noutrefresh(&self) -> result!(()) {
        ncursesw::wnoutrefresh(self.handle)
    }

    pub fn overlay(&self, srcwin: &Self) -> result!(()) {
        ncursesw::overlay(srcwin.handle, self.handle)
    }

    pub fn overwrite(&self, srcwin: &Self) -> result!(()) {
        ncursesw::overwrite(srcwin.handle, self.handle)
    }

    pub fn pechochar(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::pechochar(self.handle, ch)
    }

    pub fn pecho_wchar(&self, wch: ComplexChar) -> result!(()) {
        ncursesw::pecho_wchar(self.handle, wch)
    }

    pub fn pnoutrefresh(&self, pmin: Origin, smin: Origin, smax: Origin) -> result!(()) {
        ncursesw::pnoutrefresh(self.handle, pmin, smin, smax)
    }

    pub fn prefresh(&self, pmin: Origin, smin: Origin, smax: Origin) -> result!(()) {
        ncursesw::prefresh(self.handle, pmin, smin, smax)
    }

    pub fn putwin(&self, path: &path::Path) -> result!(()) {
        ncursesw::putwin(self.handle, path)
    }

    pub fn r#box(&self, verch: ChtypeChar, horch: ChtypeChar) -> result!(()) {
        ncursesw::r#box(self.handle, verch, horch)
    }

    pub fn redrawln(&self, beg_line: i32, num_lines: i32) -> result!(()) {
        ncursesw::wredrawln(self.handle, beg_line, num_lines)
    }

    pub fn redrawwin(&self) -> result!(()) {
        ncursesw::redrawwin(self.handle)
    }

    pub fn refresh(&self) -> result!(()) {
        ncursesw::wrefresh(self.handle)
    }

    pub fn resize(&self, size: Size) -> result!(()) {
        ncursesw::wresize(self.handle, size)
    }

    pub fn scrl(&self, n: i32) -> result!(()) {
        ncursesw::wscrl(self.handle, n)
    }

    pub fn scrollok(&self, bf: bool) -> result!(()) {
        ncursesw::scrollok(self.handle, bf)
    }

    pub fn scroll(&self) -> result!(()) {
        ncursesw::scroll(self.handle)
    }

    pub fn setscrreg(&self, region: Region) -> result!(()) {
        ncursesw::wsetscrreg(self.handle, region)
    }

    pub fn standend(&self) -> result!(()) {
        ncursesw::wstandend(self.handle)
    }

    pub fn standout(&self) -> result!(()) {
        ncursesw::wstandout(self.handle)
    }

    pub fn syncdown(&self) {
        ncursesw::wsyncdown(self.handle);
    }

    pub fn syncok(&self, bf: bool) -> result!(()) {
        ncursesw::syncok(self.handle, bf)
    }

    pub fn syncup(&self) {
        ncursesw::wsyncup(self.handle);
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use set_timeout() instead")]
    pub fn timeout(&self, ms: time::Duration) -> result!(()) {
        ncursesw::wtimeout(self.handle, ms)
    }

    pub fn touchline(&self, start: i32, count: i32) -> result!(()) {
        ncursesw::touchline(self.handle, start, count)
    }

    pub fn touchln(&self, line: i32, n: i32, changed: Changed) -> result!(()) {
        ncursesw::wtouchln(self.handle, line, n, changed)
    }

    pub fn touchwin(&self) -> result!(()) {
        ncursesw::touchwin(self.handle)
    }

    pub fn untouchwin(&self) -> result!(()) {
        ncursesw::untouchwin(self.handle)
    }

    pub fn vline(&self, ch: ChtypeChar, number: i32) -> result!(()) {
        ncursesw::wvline(self.handle, ch, number)
    }

    pub fn vline_set(&self, wch: ComplexChar, number: i32) -> result!(()) {
        ncursesw::wvline_set(self.handle, wch, number)
    }
}

impl Window {
    /// get the origin of the window.
    pub fn origin(&self) -> result!(Origin) {
        ncursesw::getbegyx(self.handle)
    }

    /// get the size of the window.
    pub fn size(&self) -> result!(Size) {
        ncursesw::getmaxyx(self.handle)
    }

    /// get the cursor origin on the window.
    pub fn cursor(&self) -> result!(Origin) {
        ncursesw::getcuryx(self.handle)
    }

    /// set the cursor origin on the window.
    pub fn set_cursor(&self, origin: Origin) -> result!(()) {
        ncursesw::wmove(self.handle, origin)
    }

    /// get the non-blocking read timeout in milliseconds.
    pub fn get_timeout(&self) -> result!(time::Duration) {
        ncursesw::wgetdelay(self.handle)
    }

    /// set the non-blocking read timeout in milliseconds, use `set_blocking_mode()` is set blocking read mode.
    pub fn set_timeout(&self, ms: time::Duration) -> result!(()) {
        ncursesw::wtimeout(self.handle, ms)
    }

    /// check if Window is blocking or non-blocking read mode.
    pub fn is_blocking_mode(&self) -> bool {
        ncursesw::shims::ncurses::wgetdelay(self.handle) == -1
    }

    /// set Window to be in blocking read mode.
    pub fn set_blocking_mode(&self) {
        ncursesw::shims::ncurses::wtimeout(self.handle, -1)
    }
}

pub fn getwin(path: &path::Path) -> result!(Window) {
    match ncursesw::getwin(path) {
        Err(e)     => Err(e),
        Ok(handle) => Ok(Window::from(handle, true))
    }
}

pub fn newpad(size: Size) -> result!(Window) {
    match ncursesw::newpad(size) {
        Err(e)     => Err(e),
        Ok(handle) => Ok(Window::from(handle, true))
    }
}
