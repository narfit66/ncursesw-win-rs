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
#![allow(clippy::never_loop)]

use std::{path, time};
use std::convert::TryFrom;

use ncursesw::{
    AttributesType, ColorPairType, ColorAttributeTypes, AttributesColorPairType,
    Origin, Size, CharacterResult, AttributesColorPairSet, Region,
    Changed, ChtypeChar, ChtypeString, ComplexChar, ComplexString,
    WideChar, WideString, WINDOW, NCurseswError,
    getcchar
};
use ncursesw::normal;
use ncursesw::mouse::{wenclose, wmouse_trafo, OriginResult};
use crate::graphics::{
    WIDEBOXDRAWING, complex_box_graphic, BoxDrawingType, BoxDrawingGraphic,
    HorizontalGraphic, VerticalGraphic
};
use crate::Timeout;
use crate::ncurseswwinerror::NCurseswWinError;

// constant to control remaping during BoxDrawingGraphic.transform()
const BOX_DRAWING_GRAPHIC_REMAP: bool = true;

/// A moveable window canvas.
///
/// All methods are either there original ncurses name or were specificlly passed a pointer
/// to `_win_st` the 'w' has been removed for example the ncurses function `mvwgetn_wstr()`
/// has become the method `self.mvgetn_wstr()`.
pub struct Window {
    handle:       WINDOW, // pointer to ncurses _win_st internal structure
    free_on_drop: bool    // free WINDOW handle on drop of structure
}

impl Window {
    /// Create a new instance of a Window
    pub fn newwin(size: Size, origin: Origin) -> result!(Self) {
        match ncursesw::newwin(size, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Self::from(handle, true))
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
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Self::from(handle, true))
        }
    }

    pub fn dupwin(&self) -> result!(Self) {
        match ncursesw::dupwin(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Self::from(handle, true))
        }
    }

    pub fn subpad(&self, size: Size, origin: Origin) -> result!(Self) {
        match ncursesw::subpad(self.handle, size, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Self::from(handle, true))
        }
    }

    pub fn subwin(&self, size: Size, origin: Origin) -> result!(Self) {
        match ncursesw::subwin(self.handle, size, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(handle)  => Ok(Self::from(handle, true))
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

unsafe impl Send for Window { } // too make thread safe
unsafe impl Sync for Window { } // too make thread safe

impl Window {
    pub fn addchnstr(&self, chstr: &ChtypeString, number: i32) -> result!(()) {
        ncursesw::waddchnstr(self.handle, chstr, number)?;

        Ok(())
    }

    pub fn addch(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::waddch(self.handle, ch)?;

        Ok(())
    }

    pub fn addchstr(&self, chstr: &ChtypeString) -> result!(()) {
        ncursesw::waddchstr(self.handle, chstr)?;

        Ok(())
    }

    pub fn addnstr(&self, str: &str, number: i32) -> result!(()) {
        ncursesw::waddnstr(self.handle, str, number)?;

        Ok(())
    }

    pub fn addnwstr(&self, wstr: &WideString, number: i32) -> result!(()) {
        ncursesw::waddnwstr(self.handle, wstr, number)?;

        Ok(())
    }

    pub fn addstr(&self, str: &str) -> result!(()) {
        ncursesw::waddstr(self.handle, str)?;

        Ok(())
    }

    pub fn add_wchnstr(&self, wchstr: &ComplexString, number: i32) -> result!(()) {
        ncursesw::wadd_wchnstr(self.handle, wchstr, number)?;

        Ok(())
    }

    pub fn add_wch(&self, wch: ComplexChar) -> result!(()) {
        ncursesw::wadd_wch(self.handle, wch)?;

        Ok(())
    }

    pub fn add_wchstr(&self, wchstr: &ComplexString) -> result!(()) {
        ncursesw::wadd_wchstr(self.handle, wchstr)?;

        Ok(())
    }

    pub fn addwstr(&self, wstr: &WideString) -> result!(()) {
        ncursesw::waddwstr(self.handle, wstr)?;

        Ok(())
    }

    pub fn attr_get(&self) -> result!(AttributesColorPairSet) {
        match ncursesw::wattr_get(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(set)     => Ok(set)
        }
    }

    pub fn attr_off<A, T>(&self, attrs: A) -> result!(())
        where A: AttributesType<T>,
              T: ColorAttributeTypes
    {
        ncursesw::wattr_off(self.handle, attrs)?;

        Ok(())
    }

    pub fn attr_on<A, T>(&self, attrs: A) -> result!(())
        where A: AttributesType<T>,
              T: ColorAttributeTypes
    {
        ncursesw::wattr_on(self.handle, attrs)?;

        Ok(())
    }

    pub fn attr_set<A, P, T>(&self, attrs: A, color_pair: P) -> result!(())
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        ncursesw::wattr_set(self.handle, attrs, color_pair)?;

        Ok(())
    }

    pub fn attroff(&self, attrs: normal::Attributes) -> result!(()) {
        ncursesw::wattroff(self.handle, attrs)?;

        Ok(())
    }

    pub fn attron(&self, attrs: normal::Attributes) -> result!(()) {
        ncursesw::wattron(self.handle, attrs)?;

        Ok(())
    }

    pub fn attrset(&self, attrs: normal::Attributes) -> result!(()) {
        ncursesw::wattrset(self.handle, attrs)?;

        Ok(())
    }

    pub fn bkgd(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::wbkgd(self.handle, ch)?;

        Ok(())
    }

    pub fn bkgdset(&self, ch: ChtypeChar) {
        ncursesw::wbkgdset(self.handle, ch)
    }

    pub fn bkgrnd(&self, wch: ComplexChar) -> result!(()) {
        ncursesw::wbkgrnd(self.handle, wch)?;

        Ok(())
    }

    pub fn bkgrndset(&self, wch: ComplexChar) {
        ncursesw::wbkgrndset(self.handle, wch)
    }

    pub fn border(
        &self,
        ls: ChtypeChar,
        rs: ChtypeChar,
        ts: ChtypeChar,
        bs: ChtypeChar,
        tl: ChtypeChar,
        tr: ChtypeChar,
        bl: ChtypeChar,
        br: ChtypeChar) -> result!(())
    {
        ncursesw::wborder(self.handle, ls, rs, ts, bs, tl, tr, bl, br)?;

        Ok(())
    }

    pub fn border_set(
        &self,
        ls: ComplexChar,
        rs: ComplexChar,
        ts: ComplexChar,
        bs: ComplexChar,
        tl: ComplexChar,
        tr: ComplexChar,
        bl: ComplexChar,
        br: ComplexChar) -> result!(())
    {
        ncursesw::wborder_set(self.handle, ls, rs, ts, bs, tl, tr, bl, br)?;

        Ok(())
    }

    pub fn box_set(&self, verch: ComplexChar, horch: ComplexChar) -> result!(()) {
        ncursesw::box_set(self.handle, verch, horch)?;

        Ok(())
    }

    pub fn chgat<A, P, T>(&self, number: i32, attrs: A, color_pair: P) -> result!(())
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        ncursesw::wchgat(self.handle, number, attrs, color_pair)?;

        Ok(())
    }

    pub fn clearok(&self, bf: bool) -> result!(()) {
        ncursesw::clearok(self.handle, bf)?;

        Ok(())
    }

    pub fn clear(&self) -> result!(()) {
        ncursesw::wclear(self.handle)?;

        Ok(())
    }

    pub fn clrtobot(&self) -> result!(()) {
        ncursesw::wclrtobot(self.handle)?;

        Ok(())
    }

    pub fn clrtoeol(&self) -> result!(()) {
        ncursesw::wclrtoeol(self.handle)?;

        Ok(())
    }

    pub fn color_set<P, T>(&self, color_pair: P) -> result!(())
        where P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        ncursesw::wcolor_set(self.handle, color_pair)?;

        Ok(())
    }

    pub fn copywin(
        &self,
        dstwin: &Self,
        smin: Origin,
        dmin: Origin,
        dmax: Origin,
        overlay: bool) -> result!(())
    {
        ncursesw::copywin(self.handle, dstwin.handle, smin, dmin, dmax, overlay)?;

        Ok(())
    }

    pub fn cursyncup(&self) {
        ncursesw::wcursyncup(self.handle);
    }

    pub fn delch(&self) -> result!(()) {
        ncursesw::wdelch(self.handle)?;

        Ok(())
    }

    pub fn echochar(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::wechochar(self.handle, ch)?;

        Ok(())
    }

    pub fn echo_wchar(&self, wch: ComplexChar) -> result!(()) {
        ncursesw::wecho_wchar(self.handle, wch)?;

        Ok(())
    }

    pub fn erase(&self) -> result!(()) {
        ncursesw::werase(self.handle)?;

        Ok(())
    }

    pub fn getattrs(&self) -> normal::Attributes {
        ncursesw::getattrs(self.handle)
    }

    pub fn getbegx(&self) -> result!(i32) {
        match ncursesw::getbegx(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(x)       => Ok(x)
        }
    }

    pub fn getbegy(&self) -> result!(i32) {
        match ncursesw::getbegy(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(y)       => Ok(y)
        }
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use origin() instead")]
    pub fn getbegyx(&self) -> result!(Origin) {
        match ncursesw::getbegyx(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(origin)  => Ok(origin)
        }
    }

    pub fn getbkgd(&self) -> ChtypeChar {
        ncursesw::getbkgd(self.handle)
    }

    pub fn getbkgrnd(&self) -> result!(ComplexChar) {
        match ncursesw::wgetbkgrnd(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(ch)      => Ok(ch)
        }
    }

    pub fn getch(&self) -> result!(CharacterResult<char>) {
        match ncursesw::wgetch(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(result)  => Ok(result)
        }
    }

    pub fn getcurx(&self) -> result!(i32) {
        match ncursesw::getcurx(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(x)       => Ok(x)
        }
    }

    pub fn getcury(&self) -> result!(i32) {
        match ncursesw::getcury(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(y)       => Ok(y)
        }
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use cursor() instead")]
    pub fn getcuryx(&self) -> result!(Origin) {
        match ncursesw::getcuryx(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(origin)  => Ok(origin)
        }
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use get_timeout() instead")]
    pub fn getdelay(&self) -> result!(time::Duration) {
        match ncursesw::wgetdelay(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(delay)   => Ok(delay)
        }
    }

    pub fn getmaxx(&self) -> result!(i32) {
        match ncursesw::getmaxx(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(x)       => Ok(x)
        }
    }

    pub fn getmaxy(&self) -> result!(i32) {
        match ncursesw::getmaxy(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(y)       => Ok(y)
        }
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use size() instead")]
    pub fn getmaxyx(&self) -> result!(Size) {
        match ncursesw::getmaxyx(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(size)    => Ok(size)
        }
    }

    pub fn getnstr(&self, number: i32) -> result!(String) {
        match ncursesw::wgetnstr(self.handle, number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(str)     => Ok(str)
        }
    }

    pub fn getn_wstr(&self, number: i32) -> result!(WideString) {
        match ncursesw::wgetn_wstr(self.handle, number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(wstr)    => Ok(wstr)
        }
    }

    /// returns the parent Window for subwindows, or None if their is no parent.
    pub fn getparent(&self) -> Option<Self> {
        match ncursesw::wgetparent(self.handle) {
            None         => None,
            Some(handle) => Some(Self::from(handle, false))
        }
    }

    pub fn getparx(&self) -> result!(i32) {
        match ncursesw::getparx(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(x)       => Ok(x)
        }
    }

    pub fn getpary(&self) -> result!(i32) {
        match ncursesw::getpary(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(y)       => Ok(y)
        }
    }

    pub fn getparyx(&self) -> result!(Origin) {
        match ncursesw::getparyx(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(origin)  => Ok(origin)
        }
    }

    pub fn getscrreg(&self) -> result!(Region) {
        match ncursesw::wgetscrreg(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(region)  => Ok(region)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use getnstr() instead")]
    pub fn getstr(&self) -> result!(String) {
        match ncursesw::wgetstr(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(str)     => Ok(str)
        }
    }

    pub fn get_wch(&self) -> result!(CharacterResult<WideChar>) {
        match ncursesw::wget_wch(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(result)  => Ok(result)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use getn_wstr() instead")]
    pub fn get_wstr(&self) -> result!(WideString) {
        match ncursesw::wget_wstr(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(wstr)    => Ok(wstr)
        }
    }

    pub fn hline(&self, ch: ChtypeChar, number: i32) -> result!(()) {
        ncursesw::whline(self.handle, ch, number)?;

        Ok(())
    }

    pub fn hline_set(&self, wch: ComplexChar, number: i32) -> result!(()) {
        ncursesw::whline_set(self.handle, wch, number)?;

        Ok(())
    }

    pub fn idcok(&self, bf: bool) {
        ncursesw::idcok(self.handle, bf)
    }

    pub fn idlok(&self, bf: bool) -> result!(()) {
        ncursesw::idlok(self.handle, bf)?;

        Ok(())
    }

    pub fn immedok(&self, bf: bool) {
        ncursesw::immedok(self.handle, bf)
    }

    pub fn inchnstr(&self, number: i32) -> result!(ChtypeString) {
        match ncursesw::winchnstr(self.handle, number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(chstr)   => Ok(chstr)
        }
    }

    pub fn inch(&self) -> ChtypeChar {
        ncursesw::winch(self.handle)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use inchnstr() instead")]
    pub fn inchstr(&self) -> result!(ChtypeString) {
        match ncursesw::winchstr(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(chstr)   => Ok(chstr)
        }
    }

    pub fn innstr(&self, number: i32) -> result!(String) {
        match ncursesw::winnstr(self.handle, number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(str)     => Ok(str)
        }
    }

    pub fn innwstr(&self, number: i32) -> result!(WideString) {
        match ncursesw::winnwstr(self.handle, number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(wstr)    => Ok(wstr)
        }
    }

    pub fn insch(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::winsch(self.handle, ch)?;

        Ok(())
    }

    pub fn insdelln(&self, number: i32) -> result!(()) {
        ncursesw::winsdelln(self.handle, number)?;

        Ok(())
    }

    pub fn insertln(&self) -> result!(()) {
        ncursesw::winsertln(self.handle)?;

        Ok(())
    }

    pub fn insnstr(&self, str: &str, number: i32) -> result!(()) {
        ncursesw::winsnstr(self.handle, str, number)?;

        Ok(())
    }

    pub fn ins_nwstr(&self, wstr: &WideString, number: i32) -> result!(()) {
        ncursesw::wins_nwstr(self.handle, wstr, number)?;

        Ok(())
    }

    pub fn insstr(&self, str: &str) -> result!(()) {
        ncursesw::winsstr(self.handle, str)?;

        Ok(())
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use innstr() instead")]
    pub fn instr(&self) -> result!(String) {
        match ncursesw::winstr(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(str)     => Ok(str)
        }
    }

    pub fn ins_wch(&self, wch: ComplexChar) -> result!(()) {
        ncursesw::wins_wch(self.handle, wch)?;

        Ok(())
    }

    pub fn ins_wstr(&self, wstr: &WideString) -> result!(()) {
        ncursesw::wins_wstr(self.handle, wstr)?;

        Ok(())
    }

    pub fn intrflush(&self, bf: bool) -> result!(()) {
        ncursesw::intrflush(self.handle, bf)?;

        Ok(())
    }

    pub fn in_wchnstr(&self, number: i32) -> result!(ComplexString) {
        match ncursesw::win_wchnstr(self.handle, number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(cstr)    => Ok(cstr)
        }
    }

    pub fn in_wch(&self) -> result!(ComplexChar) {
        match ncursesw::win_wch(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(cc)      => Ok(cc)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use in_wchnstr() instead")]
    pub fn in_wchstr(&self) -> result!(ComplexString) {
        match ncursesw::win_wchstr(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(cstr)    => Ok(cstr)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use innwstr() instead")]
    pub fn inwstr(&self) -> result!(WideString) {
        match ncursesw::winwstr(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(wstr)    => Ok(wstr)
        }
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
        ncursesw::keypad(self.handle, bf)?;

        Ok(())
    }

    pub fn leaveok(&self, bf: bool) -> result!(()) {
        ncursesw::leaveok(self.handle, bf)?;

        Ok(())
    }

    pub fn meta(&self, bf: bool) -> result!(()) {
        ncursesw::meta(self.handle, bf)?;

        Ok(())
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use set_cursor() instead")]
    pub fn r#move(&self, origin: Origin) -> result!(()) {
        ncursesw::wmove(self.handle, origin)?;

        Ok(())
    }

    pub fn mvaddchnstr(&self, origin: Origin, chstr: &ChtypeString, number: i32) -> result!(()) {
        ncursesw::mvwaddchnstr(self.handle, origin, chstr, number)?;

        Ok(())
    }

    pub fn mvaddch(&self, origin: Origin, ch: ChtypeChar) -> result!(()) {
        ncursesw::mvwaddch(self.handle, origin, ch)?;

        Ok(())
    }

    pub fn mvaddchstr(&self, origin: Origin, chstr: &ChtypeString) -> result!(()) {
        ncursesw::mvwaddchstr(self.handle, origin, chstr)?;

        Ok(())
    }

    pub fn mvaddnstr(&self, origin: Origin, str: &str, number: i32) -> result!(()) {
        ncursesw::mvwaddnstr(self.handle, origin, str, number)?;

        Ok(())
    }

    pub fn mvaddnwstr(&self, origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
        ncursesw::mvwaddnwstr(self.handle, origin, wstr, number)?;

        Ok(())
    }

    pub fn mvaddstr(&self, origin: Origin, str: &str) -> result!(()) {
        ncursesw::mvwaddstr(self.handle, origin, str)?;

        Ok(())
    }

    pub fn mvadd_wchnstr(&self, origin: Origin, wchstr: &ComplexString, number: i32) -> result!(()) {
        ncursesw::mvwadd_wchnstr(self.handle, origin, wchstr, number)?;

        Ok(())
    }

    pub fn mvadd_wch(&self, origin: Origin, wch: ComplexChar) -> result!(()) {
        ncursesw::mvwadd_wch(self.handle, origin, wch)?;

        Ok(())
    }

    pub fn mvadd_wchstr(&self, origin: Origin, wchstr: &ComplexString) -> result!(()) {
        ncursesw::mvwadd_wchstr(self.handle, origin, wchstr)?;

        Ok(())
    }

    pub fn mvaddwstr(&self, origin: Origin, wstr: &WideString) -> result!(()) {
        ncursesw::mvwaddwstr(self.handle, origin, wstr)?;

        Ok(())
    }

    pub fn mvchgat<A, P, T>(&self, origin: Origin, number: i32, attrs: A, color_pair: P) -> result!(())
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        ncursesw::mvwchgat(self.handle, origin, number, attrs, color_pair)?;

        Ok(())
    }

    pub fn mvdelch(&self, origin: Origin) -> result!(()) {
        ncursesw::mvwdelch(self.handle, origin)?;

        Ok(())
    }

    pub fn mvderwin(&self, origin: Origin) -> result!(()) {
        ncursesw::mvderwin(self.handle, origin)?;

        Ok(())
    }

    pub fn mvgetch(&self, origin: Origin) -> result!(CharacterResult<char>) {
        match ncursesw::mvwgetch(self.handle, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(result)  => Ok(result)
        }
    }

    pub fn mvgetnstr(&self, origin: Origin, number: i32) -> result!(String) {
        match ncursesw::mvwgetnstr(self.handle, origin, number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(str)     => Ok(str)
        }
    }

    pub fn mvgetn_wstr(&self, origin: Origin, number: i32) -> result!(WideString) {
        match ncursesw::mvwgetn_wstr(self.handle, origin, number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(wstr)    => Ok(wstr)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvgetnstr() instead")]
    pub fn mvgetstr(&self, origin: Origin) -> result!(String) {
        match ncursesw::mvwgetstr(self.handle, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(str)     => Ok(str)
        }
    }

    pub fn mvget_wch(&self, origin: Origin) -> result!(CharacterResult<WideChar>) {
        match ncursesw::mvwget_wch(self.handle, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(result)  => Ok(result)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvgetn_wstr() instead")]
    pub fn mvget_wstr(&self, origin: Origin) -> result!(WideString) {
        match ncursesw::mvwget_wstr(self.handle, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(wstr)    => Ok(wstr)
        }
    }

    pub fn mvhline(&self, origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
        ncursesw::mvwhline(self.handle, origin, ch, number)?;

        Ok(())
    }

    pub fn mvhline_set(&self, origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
        ncursesw::mvwhline_set(self.handle, origin, wch, number)?;

        Ok(())
    }

    pub fn mvinchnstr(&self, origin: Origin, number: i32) -> result!(ChtypeString) {
        match ncursesw::mvwinchnstr(self.handle, origin, number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(chstr)   => Ok(chstr)
        }
    }

    pub fn mvinch(&self, origin: Origin) -> ChtypeChar {
        ncursesw::mvwinch(self.handle, origin)
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvinchnstr() instead")]
    pub fn mvinchstr(&self, origin: Origin) -> result!(ChtypeString) {
        match ncursesw::mvwinchstr(self.handle, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(chstr)   => Ok(chstr)
        }
    }

    pub fn mvinnstr(&self, origin: Origin, number: i32) -> result!(String) {
        match ncursesw::mvwinnstr(self.handle, origin, number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(str)     => Ok(str)
        }
    }

    pub fn mvinnwstr(&self, origin: Origin, number: i32) -> result!(WideString) {
        match ncursesw::mvwinnwstr(self.handle, origin, number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(wstr)    => Ok(wstr)
        }
    }

    pub fn mvinsch(&self, origin: Origin, ch: ChtypeChar) -> result!(()) {
        ncursesw::mvwinsch(self.handle, origin, ch)?;

        Ok(())
    }

    pub fn mvinsnstr(&self, origin: Origin, str: &str, number: i32) -> result!(()) {
        ncursesw::mvwinsnstr(self.handle, origin, str, number)?;

        Ok(())
    }

    pub fn mvins_nwstr(&self, origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
        ncursesw::mvwins_nwstr(self.handle, origin, wstr, number)?;

        Ok(())
    }

    pub fn mvinsstr(&self, origin: Origin, str: &str) -> result!(()) {
        ncursesw::mvwinsstr(self.handle, origin, str)?;

        Ok(())
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvinnstr() instead")]
    pub fn mvinstr(&self, origin: Origin) -> result!(String) {
        match ncursesw::mvwinstr(self.handle, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(str)     => Ok(str)
        }
    }

    pub fn mvins_wch(&self, origin: Origin, wch: ComplexChar) -> result!(()) {
        ncursesw::mvwins_wch(self.handle, origin, wch)?;

        Ok(())
    }

    pub fn mvins_wstr(&self, origin: Origin, wstr: &WideString) -> result!(()) {
        ncursesw::mvwins_wstr(self.handle, origin, wstr)?;

        Ok(())
    }

    pub fn mvin_wchnstr(&self, origin: Origin, number: i32) -> result!(ComplexString) {
        match ncursesw::mvwin_wchnstr(self.handle, origin, number) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(cstr)    => Ok(cstr)
        }
    }

    pub fn mvin_wch(&self, origin: Origin) -> result!(ComplexChar) {
        match ncursesw::mvwin_wch(self.handle, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(cc)      => Ok(cc)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvin_wchnstr() instead")]
    pub fn mvin_wchstr(&self, origin: Origin) -> result!(ComplexString) {
        match ncursesw::mvwin_wchstr(self.handle, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(cstr)    => Ok(cstr)
        }
    }

    #[deprecated(since = "0.1.1", note = "underlying native function can cause issues. Use mvinnwstr() instead")]
    pub fn mvinwstr(&self, origin: Origin) -> result!(WideString) {
        match ncursesw::mvwinwstr(self.handle, origin) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(wstr)    => Ok(wstr)
        }
    }

    pub fn mvvline(&self, origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
        ncursesw::mvwvline(self.handle, origin, ch, number)?;

        Ok(())
    }

    pub fn mvvline_set(&self, origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
        ncursesw::mvwvline_set(self.handle, origin, wch, number)?;

        Ok(())
    }

    pub fn mvwin(&self, origin: Origin) -> result!(()) {
        ncursesw::mvwin(self.handle, origin)?;

        Ok(())
    }

    pub fn nodelay(&self, bf: bool) -> result!(()) {
        ncursesw::nodelay(self.handle, bf)?;

        Ok(())
    }

    pub fn notimeout(&self, bf: bool) -> result!(()) {
        ncursesw::notimeout(self.handle, bf)?;

        Ok(())
    }

    pub fn noutrefresh(&self) -> result!(()) {
        ncursesw::wnoutrefresh(self.handle)?;

        Ok(())
    }

    pub fn overlay(&self, srcwin: &Self) -> result!(()) {
        ncursesw::overlay(srcwin.handle, self.handle)?;

        Ok(())
    }

    pub fn overwrite(&self, srcwin: &Self) -> result!(()) {
        ncursesw::overwrite(srcwin.handle, self.handle)?;

        Ok(())
    }

    pub fn pechochar(&self, ch: ChtypeChar) -> result!(()) {
        ncursesw::pechochar(self.handle, ch)?;

        Ok(())
    }

    pub fn pecho_wchar(&self, wch: ComplexChar) -> result!(()) {
        ncursesw::pecho_wchar(self.handle, wch)?;

        Ok(())
    }

    pub fn pnoutrefresh(&self, pmin: Origin, smin: Origin, smax: Origin) -> result!(()) {
        ncursesw::pnoutrefresh(self.handle, pmin, smin, smax)?;

        Ok(())
    }

    pub fn prefresh(&self, pmin: Origin, smin: Origin, smax: Origin) -> result!(()) {
        ncursesw::prefresh(self.handle, pmin, smin, smax)?;

        Ok(())
    }

    pub fn putwin(&self, path: &path::Path) -> result!(()) {
        ncursesw::putwin(self.handle, path)?;

        Ok(())
    }

    pub fn r#box(&self, verch: ChtypeChar, horch: ChtypeChar) -> result!(()) {
        ncursesw::r#box(self.handle, verch, horch)?;

        Ok(())
    }

    pub fn redrawln(&self, beg_line: i32, num_lines: i32) -> result!(()) {
        ncursesw::wredrawln(self.handle, beg_line, num_lines)?;

        Ok(())
    }

    pub fn redrawwin(&self) -> result!(()) {
        ncursesw::redrawwin(self.handle)?;

        Ok(())
    }

    pub fn refresh(&self) -> result!(()) {
        ncursesw::wrefresh(self.handle)?;

        Ok(())
    }

    pub fn resize(&self, size: Size) -> result!(()) {
        ncursesw::wresize(self.handle, size)?;

        Ok(())
    }

    pub fn scrl(&self, n: i32) -> result!(()) {
        ncursesw::wscrl(self.handle, n)?;

        Ok(())
    }

    pub fn scrollok(&self, bf: bool) -> result!(()) {
        ncursesw::scrollok(self.handle, bf)?;

        Ok(())
    }

    pub fn scroll(&self) -> result!(()) {
        ncursesw::scroll(self.handle)?;

        Ok(())
    }

    pub fn setscrreg(&self, region: Region) -> result!(()) {
        ncursesw::wsetscrreg(self.handle, region)?;

        Ok(())
    }

    pub fn standend(&self) -> result!(()) {
        ncursesw::wstandend(self.handle)?;

        Ok(())
    }

    pub fn standout(&self) -> result!(()) {
        ncursesw::wstandout(self.handle)?;

        Ok(())
    }

    pub fn syncdown(&self) {
        ncursesw::wsyncdown(self.handle);
    }

    pub fn syncok(&self, bf: bool) -> result!(()) {
        ncursesw::syncok(self.handle, bf)?;

        Ok(())
    }

    pub fn syncup(&self) {
        ncursesw::wsyncup(self.handle);
    }

    #[deprecated(since = "0.1.0", note = "ambiguous function name. Use set_timeout() instead")]
    pub fn timeout(&self, ms: time::Duration) -> result!(()) {
        ncursesw::wtimeout(self.handle, ms)?;

        Ok(())
    }

    pub fn touchline(&self, start: i32, count: i32) -> result!(()) {
        ncursesw::touchline(self.handle, start, count)?;

        Ok(())
    }

    pub fn touchln(&self, line: i32, n: i32, changed: Changed) -> result!(()) {
        ncursesw::wtouchln(self.handle, line, n, changed)?;

        Ok(())
    }

    pub fn touchwin(&self) -> result!(()) {
        ncursesw::touchwin(self.handle)?;

        Ok(())
    }

    pub fn untouchwin(&self) -> result!(()) {
        ncursesw::untouchwin(self.handle)?;

        Ok(())
    }

    pub fn vline(&self, ch: ChtypeChar, number: i32) -> result!(()) {
        ncursesw::wvline(self.handle, ch, number)?;

        Ok(())
    }

    pub fn vline_set(&self, wch: ComplexChar, number: i32) -> result!(()) {
        ncursesw::wvline_set(self.handle, wch, number)?;

        Ok(())
    }
}

impl Window {
    /// get the origin of the window.
    pub fn origin(&self) -> result!(Origin) {
        match ncursesw::getbegyx(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(origin)  => Ok(origin)
        }
    }

    /// get the size of the window.
    pub fn size(&self) -> result!(Size) {
        match ncursesw::getmaxyx(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(size)    => Ok(size)
        }
    }

    /// get the cursor origin on the window.
    pub fn cursor(&self) -> result!(Origin) {
        match ncursesw::getcuryx(self.handle) {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(origin)  => Ok(origin)
        }
    }

    /// set the cursor origin on the window.
    pub fn set_cursor(&self, origin: Origin) -> result!(()) {
        ncursesw::wmove(self.handle, origin)?;

        Ok(())
    }
}

/// Transformative box drawing.
impl Window {
    /// Draw a horizontal line at current cursor of a length using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    pub fn thline_set(&self, box_drawing_type: BoxDrawingType, graphic: HorizontalGraphic, length: i32) -> result!(()) {
        assert!(length > 0, "thline_set() : length={} > 0", length);

        self.mvthline_set(self.cursor()?, box_drawing_type, graphic, length)
    }

    /// Draw a horizontal line at origin of a length using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    pub fn mvthline_set(&self, origin: Origin, box_drawing_type: BoxDrawingType, graphic: HorizontalGraphic, length: i32) -> result!(()) {
        assert!(length > 0, "mvthline_set() : length={} > 0", length);

        // build a vector of the complex characters we are going to overwrite
        let complex_chars: Vec<ComplexChar> = ComplexString::into(self.mvin_wchnstr(origin, length)?);
        let mut line_origin = origin;

        // iterate over the vector of complex characters
        for &complex_char in &complex_chars {
            // get the elements of our complex character i.e. wide character, attributes and color pair
            let char_attr_pair = getcchar(complex_char)?;
            // get the unicode value of our complex characters character
            let wchar: u32 = WideChar::into(char_attr_pair.character());

            // define our default graphic character to use
            let mut box_drawing_graphic = match graphic {
                HorizontalGraphic::Upper  => BoxDrawingGraphic::UpperHorizontalLine,
                HorizontalGraphic::Center => BoxDrawingGraphic::HorizontalLine,
                HorizontalGraphic::Lower  => BoxDrawingGraphic::LowerHorizontalLine
            };

            // iterate and filter wide character graphic characters of the specified box drawing type that have the same unicode value
            for (key, _) in WIDEBOXDRAWING.iter().filter(|(k, v)| k.box_drawing_type() == box_drawing_type && **v == wchar) {
                // transform our selected graphic character with our default graphic character then...
                // if we've transformed into a plus or left/right tee graphic then...
                // if we are in the upper or lower edge of the window then change to the appropriate tee or corner character
                box_drawing_graphic = self.transform_by_position(
                    box_drawing_graphic.transform(key.box_drawing_graphic(), BOX_DRAWING_GRAPHIC_REMAP),
                    line_origin,
                    Direction::Horizontal
                )?;

                break;
            }

            // place our complex graphics character onto the window with the existing attributes and color pair
            self.mvadd_wch(line_origin, match char_attr_pair.attributes_and_color_pair() {
                AttributesColorPairSet::Normal(set)   => complex_box_graphic(box_drawing_type, box_drawing_graphic, &set.attributes(), &set.color_pair())?,
                AttributesColorPairSet::Extended(set) => complex_box_graphic(box_drawing_type, box_drawing_graphic, &set.attributes(), &set.color_pair())?
            })?;

            line_origin.x += 1;

            // check if we've reached the right edge of the window
            if line_origin.x >= self.getmaxx()? {
                break;
            }
        }

        Ok(())
    }

    /// Draw a vertical line at the current cursor of a length using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    pub fn tvline_set(&self, box_drawing_type: BoxDrawingType, graphic: VerticalGraphic, length: i32) -> result!(()) {
        assert!(length > 0, "tvline_set() : length={} > 0", length);

        self.mvtvline_set(self.cursor()?, box_drawing_type, graphic, length)
    }

    /// Draw a vertical line at origin of a length using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    pub fn mvtvline_set(&self, origin: Origin, box_drawing_type: BoxDrawingType, graphic: VerticalGraphic, length: i32) -> result!(()) {
        assert!(length > 0, "mvtvline_set() : length={} > 0", length);

        let mut complex_chars = vec!();
        let mut line_origin = origin;

        // build a vector of the complex characters we are going to overwrite
        for _ in 0..length {
            complex_chars.push(self.mvin_wch(line_origin)?);
            line_origin.y += 1;
        }

        line_origin = origin;

        // iterate over the vector of complex characters
        for &complex_char in &complex_chars {
            // get the elements of our complex character i.e. wide character, attributes and color pair
            let char_attr_pair = getcchar(complex_char)?;
            // get the unicode value of our complex characters character
            let wchar: u32 = WideChar::into(char_attr_pair.character());

            // define our default graphic character to use
            let mut box_drawing_graphic = match graphic {
                VerticalGraphic::Left   => BoxDrawingGraphic::LeftVerticalLine,
                VerticalGraphic::Center => BoxDrawingGraphic::VerticalLine,
                VerticalGraphic::Right  => BoxDrawingGraphic::RightVerticalLine
            };

            // iterate and filter wide character graphic characters of the specified box drawing type that have the same unicode value
            for (key, _) in WIDEBOXDRAWING.iter().filter(|(k, v)| k.box_drawing_type() == box_drawing_type && **v == wchar) {
                // transform our selected graphic character with our default graphic character then...
                // if we've transformed into a plus or left/right tee graphic then...
                // if we are in the left or right edge of the window then change to the appropriate tee or corner character
                box_drawing_graphic = self.transform_by_position(
                    box_drawing_graphic.transform(key.box_drawing_graphic(), BOX_DRAWING_GRAPHIC_REMAP),
                    line_origin,
                    Direction::Vertical
                )?;

                break;
            }

            // place our complex graphics character onto the window with the existing attributes and color pair
            self.mvadd_wch(line_origin, match char_attr_pair.attributes_and_color_pair() {
                AttributesColorPairSet::Normal(set)   => complex_box_graphic(box_drawing_type, box_drawing_graphic, &set.attributes(), &set.color_pair())?,
                AttributesColorPairSet::Extended(set) => complex_box_graphic(box_drawing_type, box_drawing_graphic, &set.attributes(), &set.color_pair())?
            })?;

            line_origin.y += 1;

            // check if we've reached the bottom edge of the window
            if line_origin.y >= self.getmaxy()? {
                break;
            }
        }

        Ok(())
    }

    /// Draw a box at current cursor of a size using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    pub fn tbox_set(&self, size: Size, box_drawing_type: BoxDrawingType) -> result!(()) {
        self.mvtbox_set(self.cursor()?, size, box_drawing_type)
    }

    /// Draw a box at origin of a size using the box drawing type.
    ///
    /// The original attributes and color pairs are retained from characters that are overwritten.
    pub fn mvtbox_set(&self, origin: Origin, size: Size, box_drawing_type: BoxDrawingType) -> result!(()) {
        let get_corner_char = |corner_origin: Origin, graphic: BoxDrawingGraphic| -> result!(ComplexChar) {
            let char_attr_pair = getcchar(self.mvin_wch(corner_origin)?)?;
            let mut box_drawing_graphic = graphic;

            for (key, _) in WIDEBOXDRAWING.iter().filter(|(k, v)| k.box_drawing_type() == box_drawing_type && **v == WideChar::into(char_attr_pair.character())) {
                box_drawing_graphic = box_drawing_graphic.transform(key.box_drawing_graphic(), BOX_DRAWING_GRAPHIC_REMAP);

                break;
            }

            let cchar = match char_attr_pair.attributes_and_color_pair() {
                AttributesColorPairSet::Normal(set)   => complex_box_graphic(box_drawing_type, box_drawing_graphic, &set.attributes(), &set.color_pair())?,
                AttributesColorPairSet::Extended(set) => complex_box_graphic(box_drawing_type, box_drawing_graphic, &set.attributes(), &set.color_pair())?
            };

            Ok(cchar)
        };

        let mut corner_origin = origin;
        self.mvadd_wch(corner_origin, get_corner_char(corner_origin, BoxDrawingGraphic::UpperLeftCorner)?)?;

        corner_origin = Origin { y: origin.y, x: origin.x + (size.columns - 1) };
        self.mvadd_wch(corner_origin, get_corner_char(corner_origin, BoxDrawingGraphic::UpperRightCorner)?)?;

        corner_origin = Origin { y: origin.y + (size.lines - 1), x: origin.x };
        self.mvadd_wch(corner_origin, get_corner_char(corner_origin, BoxDrawingGraphic::LowerLeftCorner)?)?;

        corner_origin = Origin { y: origin.y + (size.lines - 1), x: origin.x + (size.columns - 1) };
        let screen_size = Origin { y: ncursesw::LINES() - 1, x: ncursesw::COLS() - 1 };
        if corner_origin == screen_size {
            self.mvins_wch(corner_origin, get_corner_char(corner_origin, BoxDrawingGraphic::LowerRightCorner)?)?;
        } else {
            self.mvadd_wch(corner_origin, get_corner_char(corner_origin, BoxDrawingGraphic::LowerRightCorner)?)?;
        }

        self.mvthline_set(Origin { y: origin.y, x: origin.x + 1}, box_drawing_type, HorizontalGraphic::Upper, size.columns - 2)?;
        self.mvthline_set(Origin { y: origin.y + (size.lines - 1), x: origin.x + 1}, box_drawing_type, HorizontalGraphic::Lower, size.columns - 2)?;
        self.mvtvline_set(Origin { y: origin.y + 1, x: origin.x }, box_drawing_type, VerticalGraphic::Left, size.lines - 2)?;
        self.mvtvline_set(Origin { y: origin.y + 1, x: origin.x + (size.columns - 1)}, box_drawing_type, VerticalGraphic::Right, size.lines - 2)?;

        Ok(())
    }

    // if we are in the left or right edge of the window then change to the appropriate tee or corner character
    fn transform_by_position(
        &self,
        box_drawing_graphic: BoxDrawingGraphic,
        origin: Origin,
        direction: Direction
    ) -> result!(BoxDrawingGraphic) {
        // can we transform our box_drawing_graphic
        if if box_drawing_graphic == BoxDrawingGraphic::Plus {
            true
        } else {
            match direction {
                Direction::Vertical   => box_drawing_graphic == BoxDrawingGraphic::LeftTee || box_drawing_graphic == BoxDrawingGraphic::RightTee,
                Direction::Horizontal => box_drawing_graphic == BoxDrawingGraphic::UpperTee || box_drawing_graphic == BoxDrawingGraphic::LowerTee
            }
        } {
            let max_y = self.getmaxy()?;
            let max_x = self.getmaxx()?;

            Ok(if origin.x == 0 && origin.y != max_y {
                if origin.y == 0 {
                    BoxDrawingGraphic::UpperLeftCorner
                } else if origin.y == max_y {
                    BoxDrawingGraphic::LowerLeftCorner
                } else {
                    match direction {
                        Direction::Vertical   => BoxDrawingGraphic::UpperTee,
                        Direction::Horizontal => BoxDrawingGraphic::LeftTee
                    }
                }
            } else if origin.y == max_y {
                if origin.x == 0 {
                    BoxDrawingGraphic::UpperRightCorner
                } else if origin.x == max_x {
                    BoxDrawingGraphic::LowerRightCorner
                } else {
                    match direction {
                        Direction::Vertical   => BoxDrawingGraphic::LowerTee,
                        Direction::Horizontal => BoxDrawingGraphic::RightTee
                    }
                }
            } else {
                box_drawing_graphic
            })
        } else {
            Ok(box_drawing_graphic)
        }
    }
}

impl Window {
    pub fn enclose(&self, origin: Origin) -> bool {
        wenclose(self.handle, origin)
    }

    pub fn mouse_trafo(&self, origin: Origin, to_screen: bool) -> OriginResult {
        wmouse_trafo(self.handle, origin, to_screen)
    }
}

impl Window {
    /// get the non-blocking read timeout in milliseconds.
    pub fn get_timeout(&self) -> result!(Timeout) {
        match ncursesw::shims::ncurses::wgetdelay(self.handle) {
            -1 => Ok(None),
            rc => {
                if rc < 0 {
                    Err(NCurseswWinError::from(NCurseswError::NCursesFunction { func: "wgetdelay".to_string(), rc }))
                } else {
                    let delay = time::Duration::from_millis(u64::try_from(rc)?);

                    Ok(Some(delay))
                }
            }
        }
    }

    /// set the non-blocking read timeout in milliseconds, use `ms: None` to set blocking read mode.
    pub fn set_timeout(&self, ms: Timeout) -> result!(()) {
        match ms {
            None     => ncursesw::shims::ncurses::wtimeout(self.handle, -1),
            Some(ms) => ncursesw::wtimeout(self.handle, ms)?
        }

        Ok(())
    }

    nonblocking_get!(getch_nonblocking, getch, "wgetch", char);
    nonblocking_get!(get_wch_nonblocking, get_wch, "wget_wch", WideChar);
    nonblocking_get_with_origin!(mvgetch_nonblocking, mvgetch, "mvwgetch", char);
    nonblocking_get_with_origin!(mvget_wch_nonblocking, mvget_wch, "mvwget_wch", WideChar);
}

/// Create a Window instance from a previous saved file.
///
/// This uses the file previously generated using the Window.putwin() routine.
pub fn getwin(path: &path::Path) -> result!(Window) {
    match ncursesw::getwin(path) {
        Err(source) => Err(NCurseswWinError::NCurseswError { source }),
        Ok(handle)  => Ok(Window::from(handle, true))
    }
}

/// Create a new instance of a Window that will act as a pad.
pub fn newpad(size: Size) -> result!(Window) {
    match ncursesw::newpad(size) {
        Err(source) => Err(NCurseswWinError::NCurseswError { source }),
        Ok(handle)  => Ok(Window::from(handle, true))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Horizontal,
    Vertical
}
