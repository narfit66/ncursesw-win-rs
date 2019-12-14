/*
    src/form/fieldparameters.rs

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

use std::{fmt, convert::{TryFrom, TryInto}};

use crate::{Size, Origin, NCurseswWinError};

pub struct FieldParameters {
    size:      Size,
    origin:    Origin,
    offscreen: u16,
    nbuffers:  u8
}

impl FieldParameters {
    pub fn new(size: Size, origin: Origin, offscreen: u16, nbuffers: u8) -> Self {
        Self { size, origin, offscreen, nbuffers }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn origin(&self) -> Origin {
        self.origin
    }

    pub fn offscreen(&self) -> u16 {
        self.offscreen
    }

    pub fn nbuffers(&self) -> u8 {
        self.nbuffers
    }
}


impl TryInto<ncursesw::form::FieldParameters> for FieldParameters {
    type Error = NCurseswWinError;

    fn try_into(self) -> Result<ncursesw::form::FieldParameters, Self::Error> {
        Ok(ncursesw::form::FieldParameters::new(self.size().try_into()?, self.origin().try_into()?, u16::try_into(self.offscreen)?, u8::try_into(self.nbuffers)?))
    }
}

impl TryFrom<ncursesw::form::FieldParameters> for FieldParameters {
    type Error = NCurseswWinError;

    fn try_from(parameters: ncursesw::form::FieldParameters) -> Result<Self, Self::Error> {
        Ok(Self { size: parameters.size().try_into()?, origin: parameters.origin().try_into()?, offscreen: u16::try_from(parameters.offscreen())?, nbuffers: u8::try_from(parameters.nbuffers())? })
    }
}

impl fmt::Display for FieldParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(size: {}, origin: {}, offscreen: {}, nbuffers: {})", self.size, self.origin, self.offscreen, self.nbuffers)
    }
}
