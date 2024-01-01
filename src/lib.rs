#![allow(dead_code, non_snake_case, non_camel_case_types)]

pub mod C64 {

    pub mod Mos6510;
    pub mod VicII;
    pub mod status_registers;

    type Byte = u8;
    type Word = u16;

    pub struct Memory {}

    #[derive(Debug)]
    pub struct Block {
        pub start: Word,
        pub instructions: Vec<Op>,
    }

    #[derive(Debug)]
    #[repr(u8)]
    pub enum Op {
        cli = 0x58,
        sei = 0x78,
        lda = 0x33,
        sta = 0x54,
    }

    pub enum AddressingMode {
        Immediate,
        Implied,
    }

    impl From<Op> for u8 {
        fn from(op: Op) -> Self {
            op as u8
        }
    }
}
