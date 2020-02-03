/*
    src/extend/softlabels.rs

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

use std::{fmt, sync::{Mutex, atomic::Ordering}, collections::HashSet};

use ncursesw;
use ncursesw::{SCREEN, extend::{ColorPair, Attributes}};

use crate::{
    Screen,
    AttributesType, ColorPairType, ColorAttributeTypes,
    NCurseswWinError,
    SoftLabelType,
    Justification,
    WideString,
    ncurses::INITSCR_CALLED
};

static MODULE_PATH: &str = "ncurseswwin::extend::softlabels::";

lazy_static! {
    static ref SOFTLABELS: Mutex<HashSet<Option<Screen>>> = Mutex::new(HashSet::new());
}

pub struct SoftLabels {
    screen: Option<SCREEN>
}

impl SoftLabels {
    pub fn new(fmt: SoftLabelType) -> result!(Self) {
        check_softlabel_init(None)?;

        ncursesw::slk_init(fmt)?;

        Ok(Self { screen: None })
    }

    #[deprecated(since = "0.5.0", note = "Use SoftLabels::new() instead")]
    pub fn slk_init(fmt: SoftLabelType) -> result!(Self) {
        Self::new(fmt)
    }

    pub fn new_sp(screen: &Screen, fmt: SoftLabelType) -> result!(Self) {
        check_softlabel_init(Some(Screen::_from(screen._handle(), false)))?;

        ncursesw::slk_init_sp(screen._handle(), fmt)?;

        Ok(Self { screen: Some(screen._handle()) })
    }

    #[deprecated(since = "0.5.0", note = "Use SoftLabels::new_sp() instead")]
    pub fn slk_init_sp(screen: &Screen, fmt: SoftLabelType) -> result!(Self) {
        Self::new_sp(screen, fmt)
    }

    pub fn screen(&self) -> Option<Screen> {
        self.screen.map_or_else(|| None, |ptr| Some(Screen::_from(ptr, false)))
    }

    pub fn slk_attr(&self) -> Attributes {
        Attributes::from(self.screen.map_or_else(|| ncursesw::slk_attr(), |screen| ncursesw::slk_attr_sp(screen)))
    }

    pub fn slk_attr_off<A, T>(&self, attrs: A) -> result!(())
        where A: AttributesType<T>,
              T: ColorAttributeTypes
    {
        assert!(self.screen.is_some(), "{}slk_attr_off() : not supported on screen defined SoftLabels!!!", MODULE_PATH);

        Ok(ncursesw::slk_attr_off(attrs)?)
    }

    pub fn slk_attr_on<A, T>(&self, attrs: A) -> result!(())
        where A: AttributesType<T>,
              T: ColorAttributeTypes
    {
        assert!(self.screen.is_some(), "{}slk_attr_on() : not supported on screen defined SoftLabels!!!", MODULE_PATH);

        Ok(ncursesw::slk_attr_on(attrs)?)
    }

    pub fn slk_attr_set<A, P, T>(&self, attrs: A, color_pair: P) -> result!(())
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        Ok(if let Some(screen) = self.screen {
            ncursesw::slk_attr_set_sp(screen, attrs, color_pair)
        } else {
            ncursesw::slk_attr_set(attrs, color_pair)
        }?)
    }

    pub fn slk_clear(&self) -> result!(()) {
        Ok(self.screen.map_or_else(|| ncursesw::slk_clear(), |screen| ncursesw::slk_clear_sp(screen))?)
    }

    pub fn slk_color(&self, color_pair: ColorPair) -> result!(()) {
        Ok(if let Some(screen) = self.screen {
            ncursesw::extended_slk_color_sp(screen, color_pair)
        } else {
            ncursesw::extended_slk_color(color_pair)
        }?)
    }

    pub fn slk_label(&self, labnum: u8) -> result!(String) {
        Ok(self.screen.map_or_else(|| ncursesw::slk_label(i32::from(labnum)), |screen| ncursesw::slk_label_sp(screen, i32::from(labnum)))?)
    }

    pub fn slk_noutrefresh(&self) -> result!(()) {
        Ok(self.screen.map_or_else(|| ncursesw::slk_noutrefresh(), |screen| ncursesw::slk_noutrefresh_sp(screen))?)
    }

    pub fn slk_refresh(&self) -> result!(()) {
        Ok(self.screen.map_or_else(|| ncursesw::slk_refresh(), |screen| ncursesw::slk_refresh_sp(screen))?)
    }

    pub fn slk_restore(&self) -> result!(()) {
        Ok(self.screen.map_or_else(|| ncursesw::slk_restore(), |screen| ncursesw::slk_restore_sp(screen))?)
    }

    pub fn slk_set(&self, labnum: u8, label: &str, fmt: Justification) -> result!(()) {
        Ok(self.screen.map_or_else(|| ncursesw::slk_set(i32::from(labnum), label, fmt), |screen| ncursesw::slk_set_sp(screen, i32::from(labnum), label, fmt))?)
    }

    pub fn slk_touch(&self) -> result!(()) {
        Ok(self.screen.map_or_else(|| ncursesw::slk_touch(), |screen| ncursesw::slk_touch_sp(screen))?)
    }

    pub fn slk_wset(&self, labnum: u8, label: &WideString, fmt: Justification) -> result!(()) {
        assert!(self.screen.is_some(), "{}slk_wset() : not supported on screen defined SoftLabels!!!", MODULE_PATH);

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

fn check_softlabel_init(screen: Option<Screen>) -> result!(()) {
    if screen.is_none() && INITSCR_CALLED.load(Ordering::SeqCst) {
        Err(NCurseswWinError::InitscrAlreadyCalled)
    } else if !SOFTLABELS
        .lock()
        .unwrap_or_else(|_| panic!("SoftLabel::new() : SOFTLABEL.lock() failed!!!"))
        .insert(screen)
    {
        Err(NCurseswWinError::SoftLabelAlreadyDefined)
    } else {
        Ok(())
    }
}
