/*
    src/screen.rs

    Copyright (c) 2020-2022 Stephen Whittle  All rights reserved.

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
use ncursesw::{SCREEN, panels, mouse};
use crate::{
    ColorType, ColorsType, ColorAttributeTypes, HasHandle, NCurseswWinError,
    ChtypeChar, WideChar, ComplexChar, Panel, InputMode, CursorType, KeyBinding,
    Window, Size, Origin, Legacy
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
    pub fn new<O, I>(term: Option<&str>, output: &O, input: &I) -> result!(Self)
        where O: AsRawFd + Write,
              I: AsRawFd + Read
    {
        Ok(Screen::_from(ncursesw::newterm(term, output, input)?, true))
    }

    #[deprecated(since = "0.5.0", note = "Use Screen::new() instead")]
    pub fn newterm<O, I>(term: Option<&str>, output: &O, input: &I) -> result!(Self)
        where O: AsRawFd + Write,
              I: AsRawFd + Read
    {
        Self::new(term, output, input)
    }

    /// # Safety
    /// 
    /// convert a 'SCREEN' pointer from the underlying 'ncursesw' crate
    /// to a 'ncurseswwin::Screen'.
    pub unsafe fn from_ptr(screen: SCREEN) -> Self {
        Self::_from(screen, false)
    }

    /// # Safety
    /// 
    /// Return's the 'Screen' as a pointer so we can use some of underlying 'ncursesw'
    /// crates functions, for example 'ncursesw::extend::ColorPair::new_sp()' (as imported
    /// in this crate as 'crate::extend::ColorPair::new_sp()')
    pub unsafe fn as_ptr(&self) -> SCREEN {
        self.handle
    }

    /// Set the input mode to use within NCurses on this Screen.
    ///
    /// The terminal gets input from the user. Then it's sometimes buffered up. At
    /// some point it's passed into the program's input buffer.
    ///
    /// - Character: Input is passed in 1 character at a time, but special
    ///   characters (such as Ctrl+C and Ctrl+S) are automatically processed for
    ///   you by the terminal.
    /// - Cooked: Input is passed in 1 line at a time, with the special character
    ///   processing mentioned above enabled.
    /// - RawCharacter: Input is passed in 1 character at a time, and special
    ///   character sequences are not processed automatically.
    /// - RawCooked: Input is passed in 1 line at a time, and special
    ///   character sequences are not processed automatically.
    ///
    /// The default mode is inherited from the terminal that started the program
    /// (usually Cooked), so you should _always_ set the desired mode explicitly
    /// at the start of your program.
    pub fn set_input_mode(&self, mode: InputMode) -> result!(()) {
        match match mode {
            InputMode::Character    => ncursesw::cbreak_sp(self.handle),
            InputMode::Cooked       => ncursesw::nocbreak_sp(self.handle),
            InputMode::RawCharacter => ncursesw::raw_sp(self.handle),
            InputMode::RawCooked    => ncursesw::noraw_sp(self.handle)
        } {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(_)       => Ok(())
        }
    }

    /// Set echo on or off within NCurses on this Screen.
    ///
    /// Enables or disables the automatic echoing of input into the window as
    /// the user types. Default to on, but you probably want it to be off most
    /// of the time.
    pub fn set_echo(&self, flag: bool) -> result!(()) {
        match if flag {
            ncursesw::echo_sp(self.handle)
        } else {
            ncursesw::noecho_sp(self.handle)
        } {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(_)       => Ok(())
        }
    }

    pub fn set_filter(&self, flag: bool) {
        if flag {
            ncursesw::filter_sp(self.handle)
        } else {
            ncursesw::nofilter_sp(self.handle)
        }
    }

    /// Control whether NCurses translates the return key into newline on input on this Screen.
    ///
    /// This determines wether ncurses translates newline into return and line-feed on output (in either
    /// case, the call addch('\n') does the equivalent of return and line feed on the virtual screen).
    /// Initially, these translations do occur. If you disable then ncurses will be able to make
    /// better use of the line-feed capability, resulting in faster cursor motion.
    /// Also, ncurses will then be able to detect the return key.
    pub fn set_newline(&self, flag: bool) -> result!(()) {
        match if flag {
            ncursesw::nl_sp(self.handle)
        } else {
            ncursesw::nonl_sp(self.handle)
        } {
            Err(source) => Err(NCurseswWinError::NCurseswError { source }),
            Ok(_)       => Ok(())
        }
    }

    pub fn set_qiflush(&self, flag: bool) {
        if flag {
            ncursesw::qiflush_sp(self.handle)
        } else {
            ncursesw::noqiflush_sp(self.handle)
        }
    }

    pub fn assume_default_colors<S, C, T>(&self, colors: S) -> result!(())
        where S: ColorsType<C, T>,
              C: ColorType<T>,
              T: ColorAttributeTypes
    {
        Ok(ncursesw::assume_default_colors_sp(self.handle, colors)?)
    }

    pub fn baudrate(&self) -> result!(u32) {
        Ok(u32::try_from(ncursesw::baudrate_sp(self.handle))?)
    }

    pub fn beep(&self) -> result!(()) {
        Ok(ncursesw::beep_sp(self.handle)?)
    }

    pub fn can_change_color(&self) -> bool {
        ncursesw::can_change_color_sp(self.handle)
    }

    pub fn cursor_set(&self, cursor: CursorType) -> result!(CursorType) {
        Ok(ncursesw::curs_set_sp(self.handle, cursor)?)
    }

    #[deprecated(since = "0.6.0", note = "Use Screen::cursor_set() instead")]
    pub fn curs_set(&self, cursor: CursorType) -> result!(CursorType) {
        self.cursor_set(cursor)
    }

    pub fn define_key(&self, definition: Option<&str>, keycode: KeyBinding) -> result!(()) {
        Ok(ncursesw::define_key_sp(self.handle, definition, keycode)?)
    }

    pub fn def_prog_mode(&self) -> result!(()) {
        Ok(ncursesw::def_prog_mode_sp(self.handle)?)
    }

    pub fn def_shell_mode(&self) -> result!(()) {
        Ok(ncursesw::def_shell_mode_sp(self.handle)?)
    }

    pub fn delay_output(&self, ms: time::Duration) -> result!(()) {
        Ok(ncursesw::delay_output_sp(self.handle, ms)?)
    }

    pub fn doupdate(&self) -> result!(()) {
        Ok(ncursesw::doupdate_sp(self.handle)?)
    }

    pub fn erasechar(&self) -> result!(char) {
        Ok(ncursesw::erasechar_sp(self.handle)?)
    }

    pub fn flash(&self) -> result!(()) {
        Ok(ncursesw::flash_sp(self.handle)?)
    }

    pub fn flushinp(&self) -> result!(()) {
        Ok(ncursesw::flushinp_sp(self.handle)?)
    }

    pub fn get_escdelay(&self) -> result!(time::Duration) {
        Ok(ncursesw::get_escdelay_sp(self.handle)?)
    }

    pub fn getwin<I: AsRawFd + Read>(&self, file: &I) -> result!(Window) {
        Ok(Window::_from(Some(self.handle), ncursesw::getwin_sp(self.handle, file)?, true))
    }

    pub fn halfdelay(&self, tenths: time::Duration) -> result!(()) {
        Ok(ncursesw::halfdelay_sp(self.handle, tenths)?)
    }

    pub fn has_colors(&self) -> bool {
        ncursesw::has_colors_sp(self.handle)
    }

    pub fn has_ic(&self) -> bool {
        ncursesw::has_ic_sp(self.handle)
    }

    pub fn has_il(&self) -> bool {
        ncursesw::has_il_sp(self.handle)
    }

    pub fn has_key(&self, ch: KeyBinding) -> bool {
        ncursesw::has_key_sp(self.handle, ch)
    }

    pub fn has_mouse(&self) -> bool {
        mouse::has_mouse_sp(self.handle)
    }

    pub fn intrflush(&self, flag: bool) -> result!(()) {
        Ok(ncursesw::intrflush_sp(self.handle, flag)?)
    }

    pub fn is_term_resized(&self, size: Size) -> result!(bool) {
        Ok(ncursesw::is_term_resized_sp(self.handle, size.try_into()?))
    }

    pub fn keybound(&self, keycode: KeyBinding, count: i32) -> Option<String> {
        ncursesw::keybound_sp(self.handle, keycode, count)
    }

    pub fn key_defined(&self, definition: &str) -> result!(Option<KeyBinding>) {
        Ok(ncursesw::key_defined_sp(self.handle, definition)?)
    }

    pub fn keyname(&self, c: KeyBinding) -> result!(String) {
        Ok(ncursesw::keyname_sp(self.handle, c)?)
    }

    pub fn keyok(&self, keycode: KeyBinding, enable: bool) -> result!(()) {
        Ok(ncursesw::keyok_sp(self.handle, keycode, enable)?)
    }

    pub fn killchar(&self) -> result!(char) {
        Ok(ncursesw::killchar_sp(self.handle)?)
    }

    pub fn longname(&self) -> result!(String) {
        Ok(ncursesw::longname_sp(self.handle)?)
    }

    pub fn mcprint(&self, data: &[i8], len: i32) -> result!(i32) {
        Ok(ncursesw::mcprint_sp(self.handle, data, len)?)
    }

    pub fn mvcur(&self, old: Origin, new: Origin) -> result!(()) {
        Ok(ncursesw::mvcur_sp(self.handle, old.try_into()?, new.try_into()?)?)
    }

    #[deprecated(since = "0.5.0", note = "ncurses library call superseeded by native rust call. Use std::thread::sleep(dur: std::time::Duration) instead")]
    pub fn napms(&self, ms: time::Duration) -> result!(()) {
        Ok(ncursesw::napms_sp(self.handle, ms)?)
    }

    #[deprecated(since = "0.5.0", note = "use with caution as this routine will reset all color pairs potentially before they go out of scope and the color pairs will default to terminal default foreground and backgound colors.")]
    /// Reset all defined color pairs.
    pub fn reset_color_pairs(&self) {
        ncursesw::reset_color_pairs_sp(self.handle)
    }

    pub fn reset_prog_mode(&self) -> result!(()) {
        Ok(ncursesw::reset_prog_mode_sp(self.handle)?)
    }

    pub fn reset_shell_mode(&self) -> result!(()) {
        Ok(ncursesw::reset_shell_mode_sp(self.handle)?)
    }

    pub fn resetty(&self) -> result!(()) {
        Ok(ncursesw::resetty_sp(self.handle)?)
    }

    pub fn resize_term(&self, size: Size) -> result!(()) {
        Ok(ncursesw::resize_term_sp(self.handle, size.try_into()?)?)
    }

    pub fn resizeterm(&self, size: Size) -> result!(()) {
        Ok(ncursesw::resizeterm_sp(self.handle, size.try_into()?)?)
    }

    pub fn savetty(&self) -> result!(()) {
        Ok(ncursesw::savetty_sp(self.handle)?)
    }

    pub fn scr_init<P: AsRef<Path>>(&self, path: P) -> result!(()) {
        Ok(ncursesw::scr_init_sp(self.handle, path)?)
    }

    pub fn scr_restore<P: AsRef<Path>>(&self, path: P) -> result!(()) {
        Ok(ncursesw::scr_restore_sp(self.handle, path)?)
    }

    pub fn scr_set<P: AsRef<Path>>(&self, path: P) -> result!(()) {
        Ok(ncursesw::scr_set_sp(self.handle, path)?)
    }

    pub fn set_escdelay(&self, ms: time::Duration) -> result!(()) {
        Ok(ncursesw::set_escdelay_sp(self.handle, ms)?)
    }

    pub fn set_tabsize(&self, size: i32) -> result!(()) {
        Ok(ncursesw::set_tabsize_sp(self.handle, size)?)
    }

    pub fn set_term(&self) -> result!(Screen) {
        Ok(Screen::_from(ncursesw::set_term(self.handle)?, false))
    }

    pub fn start_color(&self) -> result!(()) {
        Ok(ncursesw::start_color_sp(self.handle)?)
    }

    pub fn termname(&self) -> result!(String) {
        Ok(ncursesw::termname_sp(self.handle)?)
    }

    pub fn typeahead<FD: AsRawFd + Read>(&self, file: Option<FD>) -> result!(()) {
        Ok(ncursesw::typeahead_sp(self.handle, file)?)
    }

    pub fn unctrl(&self, c: ChtypeChar) -> result!(String) {
        Ok(ncursesw::unctrl_sp(self.handle, c)?)
    }

    pub fn ungetch(&self, ch: char) -> result!(()) {
        Ok(ncursesw::ungetch_sp(self.handle, ch)?)
    }

    pub fn unget_wch(&self, ch: WideChar) -> result!(()) {
        Ok(ncursesw::unget_wch_sp(self.handle, ch)?)
    }

    pub fn use_default_colors(&self) -> result!(()) {
        Ok(ncursesw::use_default_colors_sp(self.handle)?)
    }

    pub fn use_env(&self, flag: bool) {
        ncursesw::use_env_sp(self.handle, flag)
    }

    pub fn use_tioctl(&self, flag: bool) {
        ncursesw::use_tioctl_sp(self.handle, flag)
    }

    pub fn use_legacy_coding(&self, level: Legacy) -> result!(Legacy) {
        Ok(ncursesw::use_legacy_coding_sp(self.handle, level)?)
    }

    pub fn wunctrl(&self, ch: ComplexChar) -> result!(WideChar) {
        Ok(ncursesw::wunctrl_sp(self.handle, ch)?)
    }

    pub fn erasewchar(&self) -> result!(WideChar) {
        Ok(ncursesw::erasewchar_sp(self.handle)?)
    }

    pub fn killwchar(&self) -> result!(WideChar) {
        Ok(ncursesw::killwchar_sp(self.handle)?)
    }

    /// Returns the topmost panel in the given screen.
    pub fn ceiling_panel(&self) -> result!(Panel) {
        Ok(Panel::_from(Some(self.handle), panels::ceiling_panel(self.handle)?, false))
    }

    /// Returns the lowest panel in the given screen.
    pub fn ground_panel(&self) -> result!(Panel) {
        Ok(Panel::_from(Some(self.handle), panels::ground_panel(self.handle)?, false))
    }

    pub fn update_panels(&self) {
        panels::update_panels_sp(self.handle)
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        if self.free_on_drop {
            if let Err(source) = ncursesw::endwin_sp(self.handle) {
                panic!("{} @ {:?}", source, self)
            }

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

impl AsRef<Screen> for Screen {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Screen> for Screen {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Screen {{ handle: {:p}, free_on_drop: {} }}", self.handle, self.free_on_drop)
    }
}
