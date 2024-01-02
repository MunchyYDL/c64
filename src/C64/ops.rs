#[derive(Debug)]
#[repr(u8)]
pub enum Op {
    cli = 0x58,
    sei = 0x78,
    lda = 0x33,
    sta = 0x54,
}

pub enum AddressingMode {
    Implied,
    Immediate,
}

impl From<Op> for u8 {
    fn from(op: Op) -> Self {
        op as u8
    }
}
