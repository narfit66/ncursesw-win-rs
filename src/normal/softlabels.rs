/*
    src/normal/softlabels.rs

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
use ncursesw::{attr_t, normal::{ColorPair, Attributes}};

use crate::{
    Screen,
    AttributesType, ColorPairType, ColorAttributeTypes,
    NCurseswWinError,
    SoftLabelType,
    Justification,
    WideString,
    ncurses::INITSCR_CALLED
};

static MODULE_PATH: &str = "ncurseswwin::normal::softlabels::";

lazy_static! {
    static ref SOFTLABELS: Mutex<HashSet<Option<Screen>>> = Mutex::new(HashSet::new());
}


pub struct SoftLabels {
    screen: Option<ncursesw::SCREEN>
}

impl SoftLabels {
    pub fn new(fmt: SoftLabelType) -> result!(Self) {
        if INITSCR_CALLED.load(Ordering::SeqCst) {
            return Err(NCurseswWinError::InitscrAlreadyCalled)
        }

        if !SOFTLABELS
            .lock()
            .unwrap_or_else(|_| panic!("SoftLabel::new() : SOFTLABEL.lock() failed!!!"))
            .insert(None)
        {
            return Err(NCurseswWinError::SoftLabelAlreadyDefined { screen: None })
        }

        ncursesw::slk_init(fmt)?;

        Ok(Self { screen: None })
    }

    pub fn new_sp(screen: &Screen, fmt: SoftLabelType) -> result!(Self) {
        if INITSCR_CALLED.load(Ordering::SeqCst) {
            return Err(NCurseswWinError::InitscrAlreadyCalled)
        }

        if !SOFTLABELS
            .lock()
            .unwrap_or_else(|_| panic!("SoftLabel::new_sp() : SOFTLABEL.lock() failed!!!"))
            .insert(None)
        {
            return Err(NCurseswWinError::SoftLabelAlreadyDefined { screen: Some(Screen::_from(screen._handle(), false)) })
        }

        ncursesw::slk_init_sp(screen._handle(), fmt)?;

        Ok(Self { screen: Some(screen._handle()) })
    }

    pub fn screen(&self) -> Option<Screen> {
        if let Some(screen) = self.screen {
            Some(Screen::_from(screen, false))
        } else {
            None
        }
    }

    pub fn attr(&self) -> attr_t {
        if let Some(screen) = self.screen {
            ncursesw::slk_attr_sp(screen)
        } else {
            ncursesw::slk_attr()
        }
    }

    pub fn attr_off<A, T>(&self, attrs: A) -> result!(())
        where A: AttributesType<T>,
              T: ColorAttributeTypes
    {
        assert!(self.screen.is_some(), "{}attr_off() : not supported on screen defined SoftLabels!!!", MODULE_PATH);

        Ok(ncursesw::slk_attr_off(attrs)?)
    }

    pub fn attr_on<A, T>(&self, attrs: A) -> result!(())
        where A: AttributesType<T>,
              T: ColorAttributeTypes
    {
        assert!(self.screen.is_some(), "{}attr_on() : not supported on screen defined SoftLabels!!!", MODULE_PATH);

        Ok(ncursesw::slk_attr_on(attrs)?)
    }

    pub fn attr_set<A, P, T>(&self, attrs: A, color_pair: P) -> result!(())
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        Ok(if let Some(screen) = self.screen {
            ncursesw::slk_attr_set_sp(screen, attrs, color_pair)?
        } else {
            ncursesw::slk_attr_set(attrs, color_pair)?
        })
    }

    pub fn attroff(&self, attrs: Attributes) -> result!(()) {
        Ok(if let Some(screen) = self.screen {
            ncursesw::slk_attroff_sp(screen, attrs)?
        } else {
            ncursesw::slk_attroff(attrs)?
        })
    }

    pub fn attron(&self, attrs: Attributes) -> result!(()) {
        Ok(if let Some(screen) = self.screen {
            ncursesw::slk_attron_sp(screen, attrs)?
        } else {
            ncursesw::slk_attron(attrs)?
        })
    }

    pub fn attrset(&self, attrs: Attributes) -> result!(()) {
        Ok(if let Some(screen) = self.screen {
            ncursesw::slk_attrset_sp(screen, attrs)?
        } else {
            ncursesw::slk_attrset(attrs)?
        })
    }

    pub fn clear(&self) -> result!(()) {
        Ok(if let Some(screen) = self.screen {
            ncursesw::slk_clear_sp(screen)?
        } else {
            ncursesw::slk_clear()?
        })
    }

    pub fn color(&self, color_pair: ColorPair) -> result!(()) {
        Ok(if let Some(screen) = self.screen {
            ncursesw::slk_color_sp(screen, color_pair)?
        } else {
            ncursesw::slk_color(color_pair)?
        })
    }

    pub fn label(&self, labnum: i32) -> result!(String) {
        Ok(if let Some(screen) = self.screen {
            ncursesw::slk_label_sp(screen, labnum)?
        } else {
            ncursesw::slk_label(labnum)?
        })
    }

    pub fn noutrefresh(&self) -> result!(()) {
        Ok(if let Some(screen) = self.screen {
            ncursesw::slk_noutrefresh_sp(screen)?
        } else {
            ncursesw::slk_noutrefresh()?
        })
    }

    pub fn refresh(&self) -> result!(()) {
        Ok(if let Some(screen) = self.screen {
            ncursesw::slk_refresh_sp(screen)?
        } else {
            ncursesw::slk_refresh()?
        })
    }

    pub fn restore(&self) -> result!(()) {
        Ok(if let Some(screen) = self.screen {
            ncursesw::slk_restore_sp(screen)?
        } else {
            ncursesw::slk_restore()?
        })
    }

    pub fn set(&self, labnum: i32, label: &str, fmt: Justification) -> result!(()) {
        Ok(if let Some(screen) = self.screen {
            ncursesw::slk_set_sp(screen, labnum, label, fmt)?
        } else {
            ncursesw::slk_set(labnum, label, fmt)?
        })
    }

    pub fn touch(&self) -> result!(()) {
        Ok(if let Some(screen) = self.screen {
            ncursesw::slk_touch_sp(screen)?
        } else {
            ncursesw::slk_touch()?
        })
    }

    pub fn wset(&self, labnum: i32, label: &WideString, fmt: Justification) -> result!(()) {
        assert!(self.screen.is_some(), "{}wset() : not supported on screen defined SoftLabels!!!", MODULE_PATH);

        Ok(ncursesw::slk_wset(labnum, label, fmt)?)
    }
}

unsafe impl Send for SoftLabels { } // too make thread safe
unsafe impl Sync for SoftLabels { } // too make thread safe

impl fmt::Debug for SoftLabels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SoftLabels {{ screen: {:?} }}", self.screen)
    }
}
