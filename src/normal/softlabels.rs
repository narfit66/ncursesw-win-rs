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

use ncursesw::normal::{ColorPair, Attributes};

static MODULE_PATH: &str = "ncurseswwin::normal::softlabels::";

include!("../include/softlabels.rs");

impl SoftLabels {
    pub fn slk_attroff(&self, attrs: Attributes) -> result!(()) {
        assert!(self.screen == attrs.screen());

        Ok(self.screen.map_or_else(|| ncursesw::slk_attroff(attrs), |screen| ncursesw::slk_attroff_sp(screen, attrs))?)
    }

    pub fn slk_attron(&self, attrs: Attributes) -> result!(()) {
        assert!(self.screen == attrs.screen());

        Ok(self.screen.map_or_else(|| ncursesw::slk_attron(attrs), |screen| ncursesw::slk_attron_sp(screen, attrs))?)
    }

    pub fn slk_attrset(&self, attrs: Attributes) -> result!(()) {
        assert!(self.screen == attrs.screen());

        Ok(self.screen.map_or_else(|| ncursesw::slk_attrset(attrs), |screen| ncursesw::slk_attrset_sp(screen, attrs))?)
    }

    pub fn slk_color(&self, color_pair: ColorPair) -> result!(()) {
        assert!(self.screen == color_pair.screen());

        Ok(self.screen.map_or_else(|| ncursesw::slk_color(color_pair), |screen| ncursesw::slk_color_sp(screen, color_pair))?)
    }
}
