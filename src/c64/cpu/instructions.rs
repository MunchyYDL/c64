use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::addressing_modes::AddressingMode;

pub(crate) struct Instruction {
    pub code: u8,
    pub mode: AddressingMode,
    pub name: &'static str,
    pub length: u8,
    pub cycles: u8,
}

impl Instruction {
    pub const fn new(
        code: u8,
        mode: AddressingMode,
        name: &'static str,
        length: u8,
        cycles: u8,
    ) -> Self {
        Instruction {
            code,
            mode,
            name,
            length,
            cycles,
        }
    }

    pub fn unknown(code: u8) -> Self {
        Instruction::new(code, AddressingMode::Implied, "???", 1, 0)
    }
}

pub fn decode(opcode: &u8) -> &'static Instruction {
    INSTRUCTIONS.get(opcode).unwrap()
}

// FIXME: Add proper names and types
type Op<'a> = &'a str;
type AddMo = (AddressingMode, u8, u8, u8, u8);
type Inst<'a> = (Op<'a>, Vec<AddMo>);

// https://c64os.com/post/6502instructions
#[rustfmt::skip]
static ALL_INSTRUCTIONS: Lazy<Vec<Inst<'static>>> = Lazy::new(|| {
    use AddressingMode::*;

    vec![
        //* Bitwise Instructions

        //* AND - Bitwise AND with Accumulator - Flags: Nv-bdiZc
        ("and", vec![
            (Immediate,   0x29, 2, 2, 0),
            (ZeroPage,    0x25, 2, 3, 0),
            (ZeroPageX,   0x35, 2, 4, 0),
            (Absolute,    0x2d, 3, 4, 0),
            (AbsoluteX,   0x3d, 3, 4, 1),
            (AbsoluteY,   0x39, 3, 4, 1),
            (IndirectX,   0x21, 2, 6, 0),
            (IndirectY,   0x31, 2, 5, 1),
        ]),

        //* ASL - Arithmetic Shift Left - Flags: Nv-bdiZC
        ("asl", vec![
            (Accumulator, 0x0a, 1, 2, 0),
            (ZeroPage,    0x06, 2, 5, 0),
            (ZeroPageX,   0x16, 2, 6, 0),
            (Absolute,    0x0e, 3, 6, 0),
            (AbsoluteX,   0x1e, 3, 7, 0),
        ]),

        //* EOR - Bitwise Exclusive-OR with Accumulator - Flags: Nv-bdiZc
        ("eor", vec![
            (Immediate,   0x49, 2, 2, 0),
            (ZeroPage,    0x45, 2, 3, 0),
            (ZeroPageX,   0x55, 2, 4, 0),
            (Absolute,    0x4d, 3, 4, 0),
            (AbsoluteX,   0x5d, 3, 4, 1),
            (AbsoluteY,   0x59, 3, 4, 1),
            (IndirectX,   0x41, 2 ,6, 0),
            (IndirectY,   0x51, 2 ,5, 1),
        ]),

        //* ORA - Bitwise OR with Accumulator - Flags: Nv-bdiZc
        ("ora", vec![
            (Immediate,   0x09, 2, 2, 0),
            (ZeroPage,    0x05, 2, 3, 0),
            (ZeroPageX,   0x15, 2, 4, 0),
            (Absolute,    0x0d, 3, 4, 0),
            (AbsoluteX,   0x1d, 3, 4, 1),
            (AbsoluteY,   0x19, 3, 4, 1),
            (IndirectX,   0x01, 2 ,6, 0),
            (IndirectY,   0x11, 2 ,5, 1),
        ]),

        //* LSR - Logical Shift Right - Flags: Nv-bdiZC
        ("lsr", vec![
            (Accumulator, 0x44, 1, 2, 0),
            (ZeroPage,    0x46, 2, 5, 0),
            (ZeroPageX,   0x56, 2, 6, 0),
            (Absolute,    0x4e, 3, 6, 0),
            (AbsoluteX,   0x5e, 3, 7, 0),
        ]),

        //* ROL - Rotate Left - Flags: Nv-bdiZC
        ("rol", vec![
            (Accumulator, 0x2a, 1, 2, 0),
            (ZeroPage,    0x26, 2, 5, 0),
            (ZeroPageX,   0x36, 2, 6, 0),
            (Absolute,    0x2e, 3, 6, 0),
            (AbsoluteX,   0x3e, 3, 7, 0),
        ]),

        //* ROR - Rotate Right - Flags: Nv-bdiZC
        ("ror", vec![
            (Accumulator, 0x6a, 1, 2, 0),
            (ZeroPage,    0x66, 2, 5, 0),
            (ZeroPageX,   0x76, 2, 6, 0),
            (Absolute,    0x6e, 3, 6, 0),
            (AbsoluteX,   0x7e, 3, 7, 0),
        ]),


        //* Branch Instructions
        ("bpl", vec![(Relative, 0x10, 2, 2, 2)]), //* BPL - Branch on Plus
        ("bmi", vec![(Relative, 0x30, 2, 2, 2)]), //* BMI - Branch on Minus

        ("bvc", vec![(Relative, 0x50, 2, 2, 2)]), //* BVC - Branch on Overflow Clear
        ("bvs", vec![(Relative, 0x70, 2, 2, 2)]), //* BVS - Branch on Overflow Set

        ("bcc", vec![(Relative, 0x90, 2, 2, 2)]), //* BCC - Branch on Carry Clear
        ("bcs", vec![(Relative, 0xb0, 2, 2, 2)]), //* BCS - Branch on Carry Set

        ("bne", vec![(Relative, 0xd0, 2, 2, 2)]), //* BNE - Branch on Not Equal
        ("beq", vec![(Relative, 0xf0, 2, 2, 2)]), //* BEQ - Branch on Equal


        //* Compare Instructions

        //* CMP - Compare Accumulator - Flags: Nv-bdiZC
        ("cmp", vec![
            (Immediate,   0xc9, 2, 2, 0),
            (ZeroPage,    0xc5, 2, 3, 0),
            (ZeroPageX,   0xd5, 2, 4, 0),
            (Absolute,    0xcd, 3, 4, 0),
            (AbsoluteX,   0xdd, 3, 4, 1),
            (AbsoluteY,   0xd9, 3, 4, 1),
            (IndirectX,   0xc1, 2 ,6, 0),
            (IndirectY,   0xd1, 2 ,5, 1),
        ]),

        //* CPX - Compare X Register - Flags: Nv-bdiZC
        ("cpx", vec![
            (Immediate,   0xe0, 2, 2, 0),
            (ZeroPage,    0xe4, 2, 3, 0),
            (Absolute,    0xec, 3, 4, 0),
        ]),

        //* CPY - Compare Y Register - Flags: Nv-bdiZC
        ("cpy", vec![
            (Immediate,   0xc0, 2, 2, 0),
            (ZeroPage,    0xc4, 2, 3, 0),
            (Absolute,    0xcc, 3, 4, 0),
        ]),

        //* BIT - Test Bits - Flags: NV-bdiZc
        ("bit", vec![
            (ZeroPage,    0x24, 2, 3, 0),
            (Absolute,    0x2c, 3, 4, 0),
        ]),


        //* Flag Instructions
        ("clc", vec![(Implied, 0x18, 1, 2, 0)]), //* CLC - Clear Carry
        ("sec", vec![(Implied, 0x38, 1, 2, 0)]), //* SEC - Set Carry

        ("cld", vec![(Implied, 0xd8, 1, 2, 0)]), //* CLD - Clear Decimal
        ("sed", vec![(Implied, 0xf8, 1, 2, 0)]), //* SED - Set Decimal

        ("cli", vec![(Implied, 0x58, 1, 2, 0)]), //* CLC - Clear Interrupt
        ("sei", vec![(Implied, 0x78, 1, 2, 0)]), //* SEC - Set Interrupt

        ("clv", vec![(Implied, 0xb8, 1, 2, 0)]), //* CLC - Clear Overflow


        //* Jump Instructions
        //* JMP - Jump
        ("jmp", vec![
            (Absolute, 0x4c, 3, 3, 0),
            (Indirect, 0x6c, 3, 5, 0),
        ]),

        //* JSR - Jump Saving Return
        ("jsr", vec![(Absolute, 0x20, 3, 6, 0)]),

        //* RTS - Return to Saved
        ("rts", vec![(Implied, 0x60, 1, 6, 0)]),

        //* RTI - Return from Interrupt
        ("rti", vec![(Implied, 0x40, 1, 6, 0)]),


        //* Math Instructions
        //* ADC - Add with Carry - Flags: NV-bdiZC
        ("adc", vec![
            (Immediate, 0x69, 2, 2, 0),
            (ZeroPage,  0x65, 2, 3, 0),
            (ZeroPageX, 0x75, 2, 4, 0),
            (Absolute,  0x6d, 3, 4, 0),
            (AbsoluteX, 0x7d, 3, 4, 1),
            (AbsoluteY, 0x79, 3, 4, 1),
            (IndirectX, 0x61, 2, 6, 0),
            (IndirectY, 0x71, 2, 5, 1),
        ]),

        //* SBC - Subtract with Carry - Flags: NV-bdiZC
        ("sbc", vec![
            (Immediate, 0xe9, 2, 2, 0),
            (ZeroPage,  0xe5, 2, 3, 0),
            (ZeroPageX, 0xf5, 2, 4, 0),
            (Absolute,  0xed, 3, 4, 0),
            (AbsoluteX, 0xfd, 3, 4, 1),
            (AbsoluteY, 0xf9, 3, 4, 1),
            (IndirectX, 0xe1, 2, 6, 0),
            (IndirectY, 0xf1, 2, 5, 1),
        ]),

        //* Memory Instructions

        //* LDA - Load Accumulator - Flags: Nv-bdiZc
        ("lda", vec![
            (Immediate, 0xa9, 2, 2, 0),
            (ZeroPage,  0xa5, 2, 3, 0),
            (ZeroPageX, 0xb5, 2, 4, 0),
            (Absolute,  0xad, 3, 4, 0),
            (AbsoluteX, 0xbd, 3, 4, 1),
            (AbsoluteY, 0xb9, 3, 4, 1),
            (IndirectX, 0xa1, 2, 6, 0),
            (IndirectY, 0xb1, 2, 5, 1),
        ]),

        //* STA - Store Accumulator
        ("sta", vec![
            (ZeroPage,  0x85, 2, 3, 0),
            (ZeroPageX, 0x95, 2, 4, 0),
            (Absolute,  0x8d, 3, 4, 0),
            (AbsoluteX, 0x9d, 3, 5, 0),
            (AbsoluteY, 0x99, 3, 5, 0),
            (IndirectX, 0x81, 2, 6, 0),
            (IndirectY, 0x91, 2, 6, 0),
        ]),

        //* LDX - Load X Register - Flags: Nv-bdiZc
        ("ldx", vec![
            (Immediate, 0xa2, 2, 2, 0),
            (ZeroPage,  0xa6, 2, 3, 0),
            (ZeroPageY, 0xb6, 2, 4, 0),
            (Absolute,  0xae, 3, 4, 0),
            (AbsoluteY, 0xbe, 3, 4, 1),
        ]),

        //* STX - Store X Register
        ("stx", vec![
            (ZeroPage,  0x86, 2, 3, 0),
            (ZeroPageY, 0x96, 2, 4, 0),
            (Absolute,  0x8e, 3, 4, 0),
        ]),


        //* LDY - Load Y Register - Flags: Nv-bdiZc
        ("ldy", vec![
            (Immediate, 0xa0, 2, 2, 0),
            (ZeroPage,  0xa4, 2, 3, 0),
            (ZeroPageX, 0xb4, 2, 4, 0),
            (Absolute,  0xac, 3, 4, 0),
            (AbsoluteX, 0xbc, 3, 4, 1),
        ]),

        //* STY - Store Y Register
        ("sty", vec![
            (ZeroPage,  0x84, 2, 3, 0),
            (ZeroPageX, 0x94, 2, 4, 0),
            (Absolute,  0x8c, 3, 4, 0),
        ]),

        //* DEC - Decrement Memory - Flags: Nv-bdiZc
        ("dec", vec![
            (ZeroPage,  0xc6, 2, 5, 0),
            (ZeroPageX, 0xd6, 2, 6, 0),
            (Absolute,  0xce, 3, 6, 0),
            (AbsoluteX, 0xde, 3, 7, 0),
        ]),

        //* INC - Increment Memory - Flags: Nv-bdiZc
        ("inc", vec![
            (ZeroPage,  0xe6, 2, 5, 0),
            (ZeroPageX, 0xf6, 2, 6, 0),
            (Absolute,  0xee, 3, 6, 0),
            (AbsoluteX, 0xfe, 3, 7, 0),
        ]),


        //* Register Instructions

        ("tax", vec![(Implied, 0xaa, 1, 2, 0)]), //* TAX - Transfer A to X
        ("tay", vec![(Implied, 0xa8, 1, 2, 0)]), //* TAY - Transfer A to Y
        ("txa", vec![(Implied, 0x8a, 1, 2, 0)]), //* TXA - Transfer X to A
        ("tya", vec![(Implied, 0x98, 1, 2, 0)]), //* TYA - Transfer Y to A

        ("dex", vec![(Implied, 0xca, 1, 2, 0)]), //* DEX - Decrement X
        ("dey", vec![(Implied, 0x88, 1, 2, 0)]), //* DEY - Decrement Y
        ("inx", vec![(Implied, 0xe8, 1, 2, 0)]), //* INX - Increment X
        ("iny", vec![(Implied, 0xc8, 1, 2, 0)]), //* INY - Increment Y


        //* Stack Instructions

        ("pha", vec![(Implied, 0x48, 1, 3, 0)]), //* PHA - Push Accumulator
        ("php", vec![(Implied, 0x08, 1, 3, 0)]), //* PHP - Push Processor Status

        ("pla", vec![(Implied, 0x68, 1, 4, 0)]), //* PLA - Pull Accumulator
        ("plp", vec![(Implied, 0x28, 1, 4, 0)]), //* PLP - Pull Processor Status

        ("txs", vec![(Implied, 0x9a, 1, 2, 0)]), //* TXS - Transfer X to Stack Pointer
        ("tsx", vec![(Implied, 0xba, 1, 2, 0)]), //* TSX - Transfer Stack Pointer to X


        //* Other Instructions

        ("brk", vec![(Implied, 0x00, 1, 7, 0)]),
        ("nop", vec![(Implied, 0xea, 1, 2, 0)]),
    ]
});

static INSTRUCTIONS: Lazy<HashMap<u8, Instruction>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(0xff);

    // First initialize it to hold unknown values in all places.
    for code in 0..=0xff {
        map.insert(code, Instruction::unknown(code));
    }

    // Then overwrite with the valid instructions.
    for x in ALL_INSTRUCTIONS.iter() {
        let name = x.0;
        for am in x.1.iter() {
            let (mode, code, length, cycles, _): (AddressingMode, u8, u8, u8, u8) = *am;

            map.insert(
                code,
                Instruction {
                    code,
                    mode,
                    name,
                    length,
                    cycles,
                },
            );
        }
    }

    map
});

pub(crate) static MNEMONICS: Lazy<HashMap<(&str, AddressingMode), u8>> = Lazy::new(|| {
    let mut map = HashMap::new();

    for x in ALL_INSTRUCTIONS.iter() {
        let name = x.0;
        for am in x.1.iter() {
            let (mode, code, _, _, _): (AddressingMode, u8, u8, u8, u8) = *am;
            map.insert((name, mode), code);
        }
    }

    map
});
