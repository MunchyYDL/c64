use std::fmt::Display;

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
    /// Unused
    // U = (1 << 5),
    /// Overflow
    V = (1 << 6),
    /// Negative
    N = (1 << 7),
}

#[derive(Debug)]
pub struct StatusRegister(pub u8);

impl StatusRegister {
    pub fn set(&mut self, flag: StatusFlags) {
        self.0 |= flag as u8;
    }

    pub fn clear(&mut self, flag: StatusFlags) {
        self.0 &= !(flag as u8)
    }

    pub fn get(&self, flag: StatusFlags) -> bool {
        (self.0 & flag as u8) > 0
    }
}

impl Display for StatusRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sr = {
            use StatusFlags::*;
            let n = self.get(N);
            let v = self.get(V);
            let b = self.get(B);
            let d = self.get(D);
            let i = self.get(I);
            let z = self.get(Z);
            let c = self.get(C);

            let mut sr = String::new();
            sr.push(if n { 'N' } else { 'n' });
            sr.push(if v { 'V' } else { 'v' });
            sr.push('-');
            sr.push(if b { 'B' } else { 'b' });
            sr.push(if d { 'D' } else { 'd' });
            sr.push(if i { 'I' } else { 'i' });
            sr.push(if z { 'Z' } else { 'z' });
            sr.push(if c { 'C' } else { 'c' });

            sr
        };
        write!(f, "{} ({:08b})", sr, self.0)
    }
}
