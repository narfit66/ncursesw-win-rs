/*
    src/include/softlabels.rs

    Copyright (c) 2020-2021 Stephen Whittle  All rights reserved.

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

use std::{fmt, sync::{Mutex, atomic::Ordering}, collections::HashSet};
use ncursesw;
use ncursesw::SCREEN;
use crate::{
    Screen, NCurseswWinError, SoftLabelType, Justification, WideString,
    AttributesType, ColorPairType,
    ncurses::INITSCR_CALLED
};

lazy_static! {
    static ref SOFTLABELS: Mutex<HashSet<Option<Screen>>> = Mutex::new(HashSet::new());
}

pub struct SoftLabels {
    screen:     Option<SCREEN>,
    label_type: SoftLabelType
}

impl SoftLabels {
    pub(in crate) fn _from(screen: Option<SCREEN>, label_type: SoftLabelType) -> Self {
        assert!(screen.map_or_else(|| true, |screen| !screen.is_null()), "SoftLabels::_from() : screen.is_null()");

        Self { screen, label_type }
    }
}

impl SoftLabels {
    pub fn new(label_type: SoftLabelType) -> result!(Self) {
        check_softlabel_init(None);

        ncursesw::slk_init(label_type)?;

        Ok(Self::_from(None, label_type))
    }

    #[deprecated(since = "0.5.0", note = "Use SoftLabels::new() instead")]
    pub fn slk_init(label_type: SoftLabelType) -> result!(Self) {
        Self::new(label_type)
    }

    pub fn new_sp(screen: &Screen, label_type: SoftLabelType) -> result!(Self) {
        check_softlabel_init(Some(Screen::_from(screen._handle(), false)));

        ncursesw::slk_init_sp(screen._handle(), label_type)?;

        Ok(Self::_from(Some(screen._handle()), label_type))
    }

    #[deprecated(since = "0.5.0", note = "Use SoftLabels::new_sp() instead")]
    pub fn slk_init_sp(screen: &Screen, label_type: SoftLabelType) -> result!(Self) {
        Self::new_sp(screen, label_type)
    }

    /// The screen associated with the soft labels.
    pub fn screen(&self) -> Option<Screen> {
        self.screen.map(|screen| Screen::_from(screen, false))
    }

    pub fn label_type(&self) -> SoftLabelType {
        self.label_type
    }

    pub fn slk_attr(&self) -> Attributes {
        let attrs = self.screen.map_or_else(ncursesw::slk_attr, ncursesw::slk_attr_sp);

        self.screen.map_or_else(|| Attributes::new(attrs.into()), |screen| Attributes::new_sp(screen, attrs.into()))
    }

    pub fn slk_attr_off(&self, attrs: Attributes) -> result!(()) {
        assert!(self.screen.is_none(), "{}slk_attr_off() : not supported on screen defined SoftLabels!!!", MODULE_PATH);
        assert!(self.screen == attrs.screen());

        Ok(ncursesw::slk_attr_off(attrs)?)
    }

    pub fn slk_attr_on(&self, attrs: Attributes) -> result!(()) {
        assert!(self.screen.is_none(), "{}slk_attr_on() : not supported on screen defined SoftLabels!!!", MODULE_PATH);
        assert!(self.screen == attrs.screen());

        Ok(ncursesw::slk_attr_on(attrs)?)
    }

    pub fn slk_attr_set(&self, attrs: Attributes, color_pair: ColorPair) -> result!(()) {
        assert!(self.screen == attrs.screen());
        assert!(self.screen == color_pair.screen());

        Ok(self.screen.map_or_else(|| ncursesw::slk_attr_set(attrs, color_pair), |screen| ncursesw::slk_attr_set_sp(screen, attrs, color_pair))?)
    }

    pub fn slk_clear(&self) -> result!(()) {
        Ok(self.screen.map_or_else(ncursesw::slk_clear, ncursesw::slk_clear_sp)?)
    }

    pub fn slk_label(&self, labnum: u8) -> Option<String> {
        assert!(labnum >= self.label_type.min_label() as u8 && labnum <= self.label_type.max_label() as u8);

        self.screen.map_or_else(|| ncursesw::slk_label(i32::from(labnum)), |screen| ncursesw::slk_label_sp(screen, i32::from(labnum)))
    }

    pub fn slk_noutrefresh(&self) -> result!(()) {
        Ok(self.screen.map_or_else(ncursesw::slk_noutrefresh, ncursesw::slk_noutrefresh_sp)?)
    }

    pub fn slk_refresh(&self) -> result!(()) {
        Ok(self.screen.map_or_else(ncursesw::slk_refresh, ncursesw::slk_refresh_sp)?)
    }

    pub fn slk_restore(&self) -> result!(()) {
        Ok(self.screen.map_or_else(ncursesw::slk_restore, ncursesw::slk_restore_sp)?)
    }

    pub fn slk_set(&self, labnum: u8, label: Option<&str>, fmt: Justification) -> result!(()) {
        assert!(labnum >= self.label_type.min_label() as u8 && labnum <= self.label_type.max_label() as u8);

        Ok(self.screen.map_or_else(|| ncursesw::slk_set(i32::from(labnum), label, fmt), |screen| ncursesw::slk_set_sp(screen, i32::from(labnum), label, fmt))?)
    }

    pub fn slk_touch(&self) -> result!(()) {
        Ok(self.screen.map_or_else(ncursesw::slk_touch, ncursesw::slk_touch_sp)?)
    }

    pub fn slk_wset(&self, labnum: u8, label: Option<&WideString>, fmt: Justification) -> result!(()) {
        assert!(self.screen.is_none(), "{}slk_wset() : not supported on screen defined SoftLabels!!!", MODULE_PATH);
        assert!(labnum >= self.label_type.min_label() as u8 && labnum <= self.label_type.max_label() as u8);

        Ok(ncursesw::slk_wset(i32::from(labnum), label, fmt)?)
    }
}

unsafe impl Send for SoftLabels { } // too make thread safe
unsafe impl Sync for SoftLabels { } // too make thread safe

impl fmt::Debug for SoftLabels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SoftLabels {{ screen: {:?} }}", self.screen)
    }
}

fn check_softlabel_init(screen: Option<Screen>) {
    if screen.is_none() && INITSCR_CALLED.load(Ordering::SeqCst) {
        panic!("{}check_softlabel_init() : initscr() already called!!!", MODULE_PATH);
    } else if !SOFTLABELS
        .lock()
        .unwrap_or_else(|_| panic!("{}check_softlabel_init() : SOFTLABEL.lock() failed!!!", MODULE_PATH))
        .insert(screen)
    {
        panic!("{}check_softlabel_init() : SoftLabel already defined!!!", MODULE_PATH);
    }
}
