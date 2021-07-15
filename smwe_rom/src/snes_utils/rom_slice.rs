use std::{fmt, fmt::Formatter};

use crate::snes_utils::addr::{Addr, AddrPc, AddrSnes};

#[derive(Copy, Clone, Debug)]
pub struct RomSlice<A: Addr> {
    pub begin: A,
    pub size:  usize,
}

pub type PcSlice = RomSlice<AddrPc>;
pub type SnesSlice = RomSlice<AddrSnes>;

impl PcSlice {
    pub const fn new(begin: AddrPc, size: usize) -> Self {
        Self { begin, size }
    }
}

impl SnesSlice {
    pub const fn new(begin: AddrSnes, size: usize) -> Self {
        Self { begin, size }
    }
}

impl<A: Addr> RomSlice<A> {
    pub fn end(&self) -> A {
        self.begin + self.size
    }

    pub fn shift_forward(self, offset: usize) -> Self {
        Self { begin: self.begin + offset, ..self }
    }

    pub fn shift_backward(self, offset: usize) -> Self {
        Self { begin: self.begin - offset, ..self }
    }

    pub fn skip_forward(self, times_size: usize) -> Self {
        Self { begin: self.begin + (self.size * times_size), ..self }
    }

    pub fn skip_backward(self, times_size: usize) -> Self {
        Self { begin: self.begin - (self.size * times_size), ..self }
    }

    pub fn move_to(self, new_address: A) -> Self {
        Self { begin: new_address, ..self }
    }

    pub fn expand(self, diff: usize) -> Self {
        Self { size: self.size + diff, ..self }
    }

    pub fn shrink(self, diff: usize) -> Self {
        Self { size: self.size - diff, ..self }
    }

    pub fn resize(self, new_size: usize) -> Self {
        Self { size: new_size, ..self }
    }

    pub fn infinite(self) -> Self {
        Self { size: 0, ..self }
    }

    pub fn is_infinite(&self) -> bool {
        self.size == 0
    }
}

impl<A: Addr> fmt::Display for RomSlice<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "RomSlice {{ begin: {:X}, size: {} }}", self.begin, self.size)
    }
}
