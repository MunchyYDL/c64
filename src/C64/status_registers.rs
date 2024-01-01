use std::ops::{BitAnd, BitOr, BitOrAssign};

use crate::C64::Byte;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StatusRegisters(pub Byte);

impl StatusRegisters {
    pub fn get(&self) -> Byte {
        self.0
    }

    pub fn get_flag(&self, flag: StatusFlags) -> Byte {
        self.0 | flag as u8
    }

    pub fn set_flag(&mut self, flag: StatusFlags) {
        self.0 |= flag as u8
    }

    pub fn clear_flag(&mut self, flag: StatusFlags) {
        self.0 &= !(flag as u8);
    }
}

impl BitAnd<Byte> for StatusRegisters {
    type Output = Byte;

    fn bitand(self, rhs: u8) -> Self::Output {
        self.0 & rhs
    }
}

impl BitOr<Byte> for StatusRegisters {
    type Output = Byte;

    fn bitor(self, rhs: Byte) -> Self::Output {
        self.0 | rhs
    }
}

impl BitOrAssign<Byte> for StatusRegisters {
    fn bitor_assign(&mut self, rhs: u8) {
        self.0 = self.0 | rhs
    }
}

impl BitOr<StatusFlags> for StatusRegisters {
    type Output = Byte;

    fn bitor(self, rhs: StatusFlags) -> Self::Output {
        self.0 | rhs as u8
    }
}

impl BitOrAssign<StatusFlags> for StatusRegisters {
    fn bitor_assign(&mut self, rhs: StatusFlags) {
        self.0 = self.0 | rhs as u8
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum StatusFlags {
    /// Carry bit
    C = (1 << 0),
    /// Zero
    Z = (1 << 1),
    /// Disable Interrupts
    I = (1 << 2),
    /// Decimal Mode
    D = (1 << 3),
    /// Break
    B = (1 << 4),
    // Unused
    U = (1 << 5),
    /// Overflow
    V = (1 << 6),
    /// Negative
    N = (1 << 7),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_should_set_flag() {
        let mut sr = StatusRegisters(0x00);
        sr.set_flag(StatusFlags::C);
        assert_eq!(sr.0, 1)
    }

    #[test]
    fn public_should_clear_flag() {
        let mut sr = StatusRegisters(0x01);
        sr.clear_flag(StatusFlags::C);
        assert_eq!(sr.0, 0)
    }
}
