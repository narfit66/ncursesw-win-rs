/*
    src/screen.rs

    Copyright (c) 2020 Stephen Whittle  All rights reserved.

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

use std::{
    ptr, fmt, time, hash::{Hash, Hasher}, convert::{TryFrom, TryInto},
    path::Path, os::unix::io::AsRawFd, io::{Write, Read}
};

use ncursesw::SCREEN;
use crate::{
    ColorType, ColorsType, ColorAttributeTypes, AttributesType, ColorPairType,
    HasHandle, NCurseswWinError,
    normal, ChtypeChar, WideChar, ComplexChar,
    CursorType, KeyBinding, Window, Size, Origin, SoftLabelType, Justification, Legacy
};

pub struct Screen {
    handle:       SCREEN, // pointer to NCurses screen internal structure
    free_on_drop: bool    // free SCREEN handle on drop of structure
}

impl Screen {
    pub(in crate) fn _from(handle: SCREEN, free_on_drop: bool) -> Self {
        assert!(!handle.is_null(), "Screen::_from() : handle.is_null()");

        Self { handle, free_on_drop }
    }

    pub(in crate) fn _handle(&self) -> SCREEN {
        self.handle
    }
}

impl Screen {
    pub fn new<O, I>(term: Option<&str>, output: O, input: I) -> result!(Self)
        where O: AsRawFd + Write,
              I: AsRawFd + Read
    {
        Ok(Screen::_from(ncursesw::newterm(term, output, input)?, true))
    }

    #[deprecated(since = "0.5.0", note = "Use Screen::new() instead")]
    pub fn newterm<O, I>(term: Option<&str>, output: O, input: I) -> result!(Self)
        where O: AsRawFd + Write,
              I: AsRawFd + Read
    {
        Self::new(term, output, input)
    }

    pub fn assume_default_colors<S, C, T>(&self, colors: S) -> result!(())
        where S: ColorsType<C, T>,
              C: ColorType<T>,
              T: ColorAttributeTypes
    {
        Ok(ncursesw::assume_default_colors_sp(self._handle(), colors)?)
    }

    pub fn baudrate(&self) -> result!(u32) {
        Ok(u32::try_from(ncursesw::baudrate_sp(self._handle()))?)
    }

    pub fn beep(&self) -> result!(()) {
        Ok(ncursesw::beep_sp(self._handle())?)
    }

    pub fn can_change_color(&self) -> bool {
        ncursesw::can_change_color_sp(self._handle())
    }

    pub fn cbreak(&self) -> result!(()) {
        Ok(ncursesw::cbreak_sp(self._handle())?)
    }

    pub fn curs_set(&self, cursor: CursorType) -> result!(CursorType) {
        Ok(ncursesw::curs_set_sp(self._handle(), cursor)?)
    }

    pub fn define_key(&self, definition: Option<&str>, keycode: KeyBinding) -> result!(()) {
        Ok(ncursesw::define_key_sp(self._handle(), definition, keycode)?)
    }

    pub fn def_prog_mode(&self) -> result!(()) {
        Ok(ncursesw::def_prog_mode_sp(self._handle())?)
    }

    pub fn def_shell_mode(&self) -> result!(()) {
        Ok(ncursesw::def_shell_mode_sp(self._handle())?)
    }

    pub fn delay_output(&self, ms: time::Duration) -> result!(()) {
        Ok(ncursesw::delay_output_sp(self._handle(), ms)?)
    }

    pub fn doupdate(&self) -> result!(()) {
        Ok(ncursesw::doupdate_sp(self._handle())?)
    }

    pub fn echo(&self) -> result!(()) {
        Ok(ncursesw::echo_sp(self._handle())?)
    }

    pub fn erasechar(&self) -> result!(char) {
        Ok(ncursesw::erasechar_sp(self._handle())?)
    }

    pub fn filter(&self) {
        ncursesw::filter_sp(self._handle())
    }

    pub fn flash(&self) -> result!(()) {
        Ok(ncursesw::flash_sp(self._handle())?)
    }

    pub fn flushinp(&self) -> result!(()) {
        Ok(ncursesw::flushinp_sp(self._handle())?)
    }

    pub fn get_escdelay(&self) -> result!(time::Duration) {
        Ok(ncursesw::get_escdelay_sp(self._handle())?)
    }

    pub fn getwin<I: AsRawFd + Read>(&self, file: I) -> result!(Window) {
        Ok(Window::_from(Some(self._handle()), ncursesw::getwin_sp(self._handle(), file)?, true))
    }

    pub fn halfdelay(&self, tenths: time::Duration) -> result!(()) {
        Ok(ncursesw::halfdelay_sp(self._handle(), tenths)?)
    }

    pub fn has_colors(&self) -> bool {
        ncursesw::has_colors_sp(self._handle())
    }

    pub fn has_ic(&self) -> bool {
        ncursesw::has_ic_sp(self._handle())
    }

    pub fn has_il(&self) -> bool {
        ncursesw::has_il_sp(self._handle())
    }

    pub fn has_key(&self, ch: KeyBinding) -> bool {
        ncursesw::has_key_sp(self._handle(), ch)
    }

    pub fn intrflush(&self, window: &Window, bf: bool) -> result!(()) {
        Ok(ncursesw::intrflush_sp(self._handle(), window._handle(), bf)?)
    }

    pub fn is_term_resized(&self, size: Size) -> result!(bool) {
        Ok(ncursesw::is_term_resized_sp(self._handle(), size.try_into()?))
    }

    pub fn keybound(&self, keycode: KeyBinding, count: i32) -> result!(String) {
        Ok(ncursesw::keybound_sp(self._handle(), keycode, count)?)
    }

    pub fn key_defined(&self, definition: &str) -> result!(KeyBinding) {
        Ok(ncursesw::key_defined_sp(self._handle(), definition)?)
    }

    pub fn keyname(&self, c: KeyBinding) -> result!(String) {
        Ok(ncursesw::keyname_sp(self._handle(), c)?)
    }

    pub fn keyok(&self, keycode: KeyBinding, enable: bool) -> result!(()) {
        Ok(ncursesw::keyok_sp(self._handle(), keycode, enable)?)
    }

    pub fn killchar(&self) -> result!(char) {
        Ok(ncursesw::killchar_sp(self._handle())?)
    }

    pub fn longname(&self) -> result!(String) {
        Ok(ncursesw::longname_sp(self._handle())?)
    }

    pub fn mcprint(&self, data: &[i8], len: i32) -> result!(i32) {
        Ok(ncursesw::mcprint_sp(self._handle(), data, len)?)
    }

    pub fn mvcur(&self, old: Origin, new: Origin) -> result!(()) {
        Ok(ncursesw::mvcur_sp(self._handle(), old.try_into()?, new.try_into()?)?)
    }

    #[deprecated(since = "0.5.0", note = "ncurses library call superseeded by native rust call. Use std::thread::sleep(dur: std::time::Duration) instead")]
    pub fn napms(&self, ms: time::Duration) -> result!(()) {
        Ok(ncursesw::napms_sp(self._handle(), ms)?)
    }

    pub fn nl(&self) -> result!(()) {
        Ok(ncursesw::nl_sp(self._handle())?)
    }

    pub fn nocbreak(&self) -> result!(()) {
        Ok(ncursesw::nocbreak_sp(self._handle())?)
    }

    pub fn noecho(&self) -> result!(()) {
        Ok(ncursesw::noecho_sp(self._handle())?)
    }

    pub fn nofilter(&self) {
        ncursesw::nofilter_sp(self._handle())
    }

    pub fn nonl(&self) -> result!(()) {
        Ok(ncursesw::nonl_sp(self._handle())?)
    }

    pub fn noqiflush(&self) {
        ncursesw::noqiflush_sp(self._handle())
    }

    pub fn noraw(&self) -> result!(()) {
        Ok(ncursesw::noraw_sp(self._handle())?)
    }

    pub fn qiflush(&self) {
        ncursesw::qiflush_sp(self._handle())
    }

    pub fn raw(&self) -> result!(()) {
        Ok(ncursesw::raw_sp(self._handle())?)
    }

    pub fn reset_color_pairs(&self) {
        ncursesw::reset_color_pairs_sp(self._handle())
    }

    pub fn reset_prog_mode(&self) -> result!(()) {
        Ok(ncursesw::reset_prog_mode_sp(self._handle())?)
    }

    pub fn reset_shell_mode(&self) -> result!(()) {
        Ok(ncursesw::reset_shell_mode_sp(self._handle())?)
    }

    pub fn resetty(&self) -> result!(()) {
        Ok(ncursesw::resetty_sp(self._handle())?)
    }

    pub fn resize_term(&self, size: Size) -> result!(()) {
        Ok(ncursesw::resize_term_sp(self._handle(), size.try_into()?)?)
    }

    pub fn resizeterm(&self, size: Size) -> result!(()) {
        Ok(ncursesw::resizeterm_sp(self._handle(), size.try_into()?)?)
    }

    pub fn savetty(&self) -> result!(()) {
        Ok(ncursesw::savetty_sp(self._handle())?)
    }

    pub fn scr_init(&self, filename: &Path) -> result!(()) {
        Ok(ncursesw::scr_init_sp(self._handle(), filename)?)
    }

    pub fn scr_restore(&self, filename: &Path) -> result!(()) {
        Ok(ncursesw::scr_restore_sp(self._handle(), filename)?)
    }

    pub fn scr_set(&self, filename: &Path) -> result!(()) {
        Ok(ncursesw::scr_set_sp(self._handle(), filename)?)
    }

    pub fn set_escdelay(&self, ms: time::Duration) -> result!(()) {
        Ok(ncursesw::set_escdelay_sp(self._handle(), ms)?)
    }

    pub fn set_tabsize(&self, size: i32) -> result!(()) {
        Ok(ncursesw::set_tabsize_sp(self._handle(), size)?)
    }

    pub fn slk_attroff(&self, attrs: normal::Attributes) -> result!(()) {
        Ok(ncursesw::slk_attroff_sp(self._handle(), attrs)?)
    }

    pub fn slk_attron(&self, attrs: normal::Attributes) -> result!(()) {
        Ok(ncursesw::slk_attron_sp(self._handle(), attrs)?)
    }

    pub fn slk_attr_set<A, P, T>(&self, attrs: A, color_pair: P) -> result!(())
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        Ok(ncursesw::slk_attr_set_sp(self._handle(), attrs, color_pair)?)
    }

    pub fn slk_attrset(&self, attrs: normal::Attributes) -> result!(()) {
        Ok(ncursesw::slk_attrset_sp(self._handle(), attrs)?)
    }

    /*
    pub fn slk_attr(&self) -> attr_t {
        ncursesw::slk_attr_sp(self._handle())
    }
    */

    pub fn slk_clear(&self) -> result!(()) {
        Ok(ncursesw::slk_clear_sp(self._handle())?)
    }

    pub fn slk_color(&self, color_pair: normal::ColorPair) -> result!(()) {
        Ok(ncursesw::slk_color_sp(self._handle(), color_pair)?)
    }

    pub fn slk_init(&self, fmt: SoftLabelType) -> result!(()) {
        Ok(ncursesw::slk_init_sp(self._handle(), fmt)?)
    }

    pub fn slk_label(&self, number: i32) -> result!(String) {
        Ok(ncursesw::slk_label_sp(self._handle(), number)?)
    }

    pub fn slk_noutrefresh(&self) -> result!(()) {
        Ok(ncursesw::slk_noutrefresh_sp(self._handle())?)
    }

    pub fn slk_refresh(&self) -> result!(()) {
        Ok(ncursesw::slk_refresh_sp(self._handle())?)
    }

    pub fn slk_restore(&self) -> result!(()) {
        Ok(ncursesw::slk_restore_sp(self._handle())?)
    }

    pub fn slk_set(&self, label_number: i32, label: &str, fmt: Justification) -> result!(()) {
        Ok(ncursesw::slk_set_sp(self._handle(), label_number, label, fmt)?)
    }

    pub fn slk_touch(&self) -> result!(()) {
        Ok(ncursesw::slk_touch_sp(self._handle())?)
    }

    pub fn start_color(&self) -> result!(()) {
        Ok(ncursesw::start_color_sp(self._handle())?)
    }

    /*
    pub fn term_attrs(&self) -> attr_t {
        ncursesw::term_attrs_sp(self._handle())
    }

    pub fn termattrs(&self) -> chtype {
        ncursesw::termattrs_sp(self._handle())
    }
    */

    pub fn termname(&self) -> result!(String) {
        Ok(ncursesw::termname_sp(self._handle())?)
    }

    pub fn typeahead<FD: AsRawFd + Read>(&self, file: Option<FD>) -> result!(()) {
        Ok(ncursesw::typeahead_sp(self._handle(), file)?)
    }

    pub fn unctrl(&self, c: ChtypeChar) -> result!(String) {
        Ok(ncursesw::unctrl_sp(self._handle(), c)?)
    }

    pub fn ungetch(&self, ch: char) -> result!(()) {
        Ok(ncursesw::ungetch_sp(self._handle(), ch)?)
    }

    pub fn unget_wch(&self, ch: WideChar) -> result!(()) {
        Ok(ncursesw::unget_wch_sp(self._handle(), ch)?)
    }

    pub fn use_default_colors(&self) -> result!(()) {
        Ok(ncursesw::use_default_colors_sp(self._handle())?)
    }

    pub fn use_env(&self, f: bool) {
        ncursesw::use_env_sp(self._handle(), f)
    }

    pub fn use_tioctl(&self, f: bool) {
        ncursesw::use_tioctl_sp(self._handle(), f)
    }

    pub fn use_legacy_coding(&self, level: Legacy) -> result!(Legacy) {
        Ok(ncursesw::use_legacy_coding_sp(self._handle(), level)?)
    }

    /*
    pub fn vid_attr(&self, attrs: attr_t, pair: short_t) -> i32 {
        ncursesw::vid_attr_sp(self._handle(), attrs, pair)
    }

    pub fn vidattr(&self, attrs: chtype) -> i32 {
        ncursesw::vidattr_sp(self._handle(), attrs)
    }
    */

    pub fn wunctrl_sp(&self, ch: ComplexChar) -> result!(WideChar) {
        Ok(ncursesw::wunctrl_sp(self._handle(), ch)?)
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        if self.free_on_drop {
            ncursesw::delscreen(self.handle);
        }
    }
}

unsafe impl Send for Screen { } // too make thread safe
unsafe impl Sync for Screen { } // too make thread safe

impl PartialEq for Screen {
    fn eq(&self, rhs: &Self) -> bool {
        ptr::eq(self.handle, rhs.handle)
    }
}

impl Eq for Screen { }

impl Hash for Screen {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Screen {{ handle: {:p}, free_on_drop: {} }}", self.handle, self.free_on_drop)
    }
}

pub fn new_prescr() -> result!(Screen) {
    Ok(Screen::_from(ncursesw::new_prescr()?, true))
}

pub fn newterm<O, I>(screen: &Screen, term: Option<&str>, output: O, input: I) -> result!(Screen)
    where O: AsRawFd + Write,
          I: AsRawFd + Read
{
    Ok(Screen::_from(ncursesw::newterm_sp(screen._handle(), term, output, input)?, true))
}

pub fn set_term(screen: &Screen) -> result!(Screen) {
    Ok(Screen::_from(ncursesw::set_term(screen._handle())?, false))
}
