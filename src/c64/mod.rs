#![allow(dead_code)]

pub mod block;
pub mod bus;
pub mod cpu;

use once_cell::sync::Lazy;
use std::collections::HashMap;

use self::block::Block;
use self::bus::Bus;
use self::cpu::Cpu;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum AddressingMode {
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Implied,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}

pub(crate) struct Instruction {
    code: u8,
    mode: AddressingMode,
    name: &'static str,
    length: u8,
    cycles: u8,
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
        ("AND", vec![
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
        ("ASL", vec![
            (Accumulator, 0x0a, 1, 2, 0),
            (ZeroPage,    0x06, 2, 5, 0),
            (ZeroPageX,   0x16, 2, 6, 0),
            (Absolute,    0x0e, 3, 6, 0),
            (AbsoluteX,   0x1e, 3, 7, 0),
        ]),

        //* EOR - Bitwise Exclusive-OR with Accumulator - Flags: Nv-bdiZc
        ("EOR", vec![
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
        ("ORA", vec![
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
        ("LSR", vec![
            (Accumulator, 0x44, 1, 2, 0),
            (ZeroPage,    0x46, 2, 5, 0),
            (ZeroPageX,   0x56, 2, 6, 0),
            (Absolute,    0x4e, 3, 6, 0),
            (AbsoluteX,   0x5e, 3, 7, 0),
        ]),

        //* ROL - Rotate Left - Flags: Nv-bdiZC
        ("ROL", vec![
            (Accumulator, 0x2a, 1, 2, 0),
            (ZeroPage,    0x26, 2, 5, 0),
            (ZeroPageX,   0x36, 2, 6, 0),
            (Absolute,    0x2e, 3, 6, 0),
            (AbsoluteX,   0x3e, 3, 7, 0),
        ]),

        //* ROR - Rotate Right - Flags: Nv-bdiZC
        ("ROR", vec![
            (Accumulator, 0x6a, 1, 2, 0),
            (ZeroPage,    0x66, 2, 5, 0),
            (ZeroPageX,   0x76, 2, 6, 0),
            (Absolute,    0x6e, 3, 6, 0),
            (AbsoluteX,   0x7e, 3, 7, 0),
        ]),


        //* Branch Instructions
        ("BPL", vec![(Relative, 0x10, 2, 2, 2)]), //* BPL - Branch on Plus
        ("BMI", vec![(Relative, 0x30, 2, 2, 2)]), //* BMI - Branch on Minus

        ("BVC", vec![(Relative, 0x50, 2, 2, 2)]), //* BVC - Branch on Overflow Clear
        ("BVS", vec![(Relative, 0x70, 2, 2, 2)]), //* BVS - Branch on Overflow Set

        ("BCC", vec![(Relative, 0x90, 2, 2, 2)]), //* BCC - Branch on Carry Clear
        ("BCS", vec![(Relative, 0xb0, 2, 2, 2)]), //* BCS - Branch on Carry Set

        ("BNE", vec![(Relative, 0xd0, 2, 2, 2)]), //* BNE - Branch on Not Equal
        ("BEQ", vec![(Relative, 0xf0, 2, 2, 2)]), //* BEQ - Branch on Equal


        //* Compare Instructions

        //* CMP - Compare Accumulator - Flags: Nv-bdiZC
        ("CMP", vec![
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
        ("CPX", vec![
            (Immediate,   0xe0, 2, 2, 0),
            (ZeroPage,    0xe4, 2, 3, 0),
            (Absolute,    0xec, 3, 4, 0),
        ]),

        //* CPY - Compare Y Register - Flags: Nv-bdiZC
        ("CPY", vec![
            (Immediate,   0xc0, 2, 2, 0),
            (ZeroPage,    0xc4, 2, 3, 0),
            (Absolute,    0xcc, 3, 4, 0),
        ]),

        //* BIT - Test Bits - Flags: NV-bdiZc
        ("BIT", vec![
            (ZeroPage,    0x24, 2, 3, 0),
            (Absolute,    0x2c, 3, 4, 0),
        ]),


        //* Flag Instructions
        ("CLC", vec![(Implied, 0x18, 1, 2, 0)]), //* CLC - Clear Carry
        ("SEC", vec![(Implied, 0x38, 1, 2, 0)]), //* SEC - Set Carry

        ("CLD", vec![(Implied, 0xd8, 1, 2, 0)]), //* CLC - Clear Decimal
        ("SED", vec![(Implied, 0xf8, 1, 2, 0)]), //* SEC - Set Decimal

        ("CLI", vec![(Implied, 0x58, 1, 2, 0)]), //* CLC - Clear Interrupt
        ("SEI", vec![(Implied, 0x78, 1, 2, 0)]), //* SEC - Set Interrupt

        ("CLV", vec![(Implied, 0xb8, 1, 2, 0)]), //* CLC - Clear Overflow


        //* Jump Instructions
        //* JMP - Jump
        ("JMP", vec![
            (Absolute, 0x4c, 3, 3, 0),
            (Indirect, 0x6c, 3, 5, 0),
        ]),

        //* JSR - Jump Saving Return
        ("JSR", vec![(Absolute, 0x20, 3, 6, 0)]),

        //* RTS - Return to Saved
        ("RTS", vec![(Implied, 0x60, 1, 6, 0)]),

        //* RTI - Return from Interrupt
        ("RTI", vec![(Implied, 0x40, 1, 6, 0)]),


        //* Math Instructions
        //* ADC - Add with Carry - Flags: NV-bdiZC
        ("ADC",
        vec![
            (Immediate, 0x69, 2, 2, 0),
            (ZeroPage,  0x65, 2, 3, 0),
            (ZeroPageX, 0x75, 2, 4, 0),
            (Absolute,  0x6d, 3, 4, 0),
            (AbsoluteX, 0x7d, 3, 4, 1),
            (AbsoluteY, 0x79, 3, 4, 1),
            (IndirectX, 0x61, 2, 6, 0),
            (IndirectY, 0x71, 2, 5, 1),
        ]),

        //* ABC - Subtract with Carry - Flags: NV-bdiZC
        ("SBC",
        vec![
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
        ("LDA",
        vec![
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
        ("STA",
        vec![
            (ZeroPage,  0x85, 2, 3, 0),
            (ZeroPageX, 0x95, 2, 4, 0),
            (Absolute,  0x8d, 3, 4, 0),
            (AbsoluteX, 0x9d, 3, 5, 0),
            (AbsoluteY, 0x99, 3, 5, 0),
            (IndirectX, 0x81, 2, 6, 0),
            (IndirectY, 0x91, 2, 6, 0),
        ]),

        //* LDX - Load X Register - Flags: Nv-bdiZc
        ("LDX",
        vec![
            (Immediate, 0xa2, 2, 2, 0),
            (ZeroPage,  0xa6, 2, 3, 0),
            (ZeroPageY, 0xb6, 2, 4, 0),
            (Absolute,  0xae, 3, 4, 0),
            (AbsoluteY, 0xbe, 3, 4, 1),
        ]),

        //* STX - Store X Register
        ("STX",
        vec![
            (ZeroPage,  0x86, 2, 3, 0),
            (ZeroPageY, 0x96, 2, 4, 0),
            (Absolute,  0x8e, 3, 4, 0),
        ]),


        //* LDY - Load Y Register - Flags: Nv-bdiZc
        ("LDY",
        vec![
            (Immediate, 0xa0, 2, 2, 0),
            (ZeroPage,  0xa4, 2, 3, 0),
            (ZeroPageX, 0xb4, 2, 4, 0),
            (Absolute,  0xac, 3, 4, 0),
            (AbsoluteX, 0xbc, 3, 4, 1),
        ]),

        //* STY - Store Y Register
        ("STY",
        vec![
            (ZeroPage,  0x84, 2, 3, 0),
            (ZeroPageX, 0x94, 2, 4, 0),
            (Absolute,  0x8c, 3, 4, 0),
        ]),

        //* DEC - Decrement Memory - Flags: Nv-bdiZc
        ("DEC",
        vec![
            (ZeroPage,  0xc6, 2, 5, 0),
            (ZeroPageX, 0xd6, 2, 6, 0),
            (Absolute,  0xce, 3, 6, 0),
            (AbsoluteX, 0xde, 3, 7, 0),
        ]),

        //* INC - Increment Memory - Flags: Nv-bdiZc
        ("INC",
        vec![
            (ZeroPage,  0xe6, 2, 5, 0),
            (ZeroPageX, 0xf6, 2, 6, 0),
            (Absolute,  0xee, 3, 6, 0),
            (AbsoluteX, 0xfe, 3, 7, 0),
        ]),


        //* Register Instructions

        ("TAX", vec![(Implied, 0xaa, 1, 2, 0)]), //* TAX - Transfer A to X
        ("TAY", vec![(Implied, 0xa8, 1, 2, 0)]), //* TAY - Transfer A to Y
        ("TXA", vec![(Implied, 0x8a, 1, 2, 0)]), //* TXA - Transfer X to A
        ("TYA", vec![(Implied, 0x98, 1, 2, 0)]), //* TYA - Transfer Y to A

        ("DEX", vec![(Implied, 0xca, 1, 2, 0)]), //* DEX - Decrement X
        ("DEY", vec![(Implied, 0x88, 1, 2, 0)]), //* DEY - Decrement Y
        ("INX", vec![(Implied, 0xe8, 1, 2, 0)]), //* INX - Increment X
        ("INY", vec![(Implied, 0xc8, 1, 2, 0)]), //* INY - Increment Y


        //* Stack Instructions

        ("PHA", vec![(Implied, 0x48, 1, 3, 0)]), //* PHA - Push Accumulator
        ("PHP", vec![(Implied, 0x08, 1, 3, 0)]), //* PHP - Push Processor Status

        ("PLA", vec![(Implied, 0x68, 1, 4, 0)]), //* PLA - Pull Accumulator
        ("PLP", vec![(Implied, 0x28, 1, 4, 0)]), //* PLP - Pull Processor Status

        ("TXS", vec![(Implied, 0x9a, 1, 2, 0)]), //* TXS - Transfer X to Stack Pointer
        ("TSX", vec![(Implied, 0xba, 1, 2, 0)]), //* TSX - Transfer Stack Pointer to X


        //* Other Instructions

        ("BRK", vec![(Implied, 0x00, 1, 7, 0)]),
        ("NOP", vec![(Implied, 0xea, 1, 2, 0)]),
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

#[rustfmt::skip]
static MNEMONICS: Lazy<HashMap<(&str, AddressingMode), u8>> = Lazy::new(|| {
    use AddressingMode::*;
    HashMap::from([
        (("LDX", Immediate), 0xa2),
        (("SEI", Implied),   0x78),
        (("TXS", Implied),   0x9a),
        (("CLD", Implied),   0xd8),
        (("JSR", Absolute),  0x20),
        (("BNE", Absolute),  0xd0),
        (("CLI", Implied),   0x58),
        (("STX", Absolute),  0x8e),
        (("JMP", Indirect),  0x6c),
        // (("LDA", Absolute), 0x00), (("LDA", Immediate), 0x00)
    ])
});

pub(crate) struct C64 {
    memory: Memory,
    cpu: Cpu,
    bus: Bus,
}

type Memory = [u8; 0xffff];

impl C64 {
    // FIXME: This is a shitty implementation for now,
    // lets get back to this and fix it...
    pub fn new() -> Self {
        let mut cpu = Cpu::new();
        let memory: Memory = [0; 0xffff];
        let bus = Bus::new(cpu.clone(), memory);

        cpu.connect_bus(bus.clone());

        C64 { cpu, memory, bus }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn run(&self, _block: Block) {
        todo!()
    }
}

impl Default for C64 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use tests::{block::parse_params, cpu::StatusFlags};

    use crate::show;

    use super::*;

    #[test]
    fn should_be_able_to_init_the_machine() {
        let mut c64 = C64::new();
        c64.reset();
        c64.cpu.set_flag(StatusFlags::I);
        println!("CPU - {}", c64.cpu);
        assert_eq!(c64.cpu.PC, 0xfffc);
    }

    // rust-fmt disable
    #[rustfmt::skip]
    #[test]
    fn should_disassemble_block() {
        let block = Block {
            start: 0xFCE2,
            instructions: vec![
                0xa2, 0xff,
                0x78,
                0x9a,
                0xd8,
                0x20, 0x02, 0xfd,
                0xd0, 0x03,
                0x6c, 0x00, 0x80,
                0x8e, 0x16, 0xd0,
                0x20, 0xa3, 0xfd,
                0x20, 0x50, 0xfd,
                0x20, 0x15, 0xfd,
                0x20, 0x5b, 0xff,
                0x58,
                0x6c, 0x00, 0xa0,
            ],
        };

        // cspell: disable
        let expected: Vec<String> = vec![
            "FCE2   A2 FF      LDX #$FF   ".trim().into(),
            "FCE4   78         SEI        ".trim().into(),
            "FCE5   9A         TXS        ".trim().into(),
            "FCE6   D8         CLD        ".trim().into(),
            "FCE7   20 02 FD   JSR $FD02  ".trim().into(),
            "FCEA   D0 03      BNE $FCEF  ".trim().into(),
            "FCEC   6C 00 80   JMP ($8000)".trim().into(),
            "FCEF   8E 16 D0   STX $D016  ".trim().into(),
            "FCF2   20 A3 FD   JSR $FDA3  ".trim().into(),
            "FCF5   20 50 FD   JSR $FD50  ".trim().into(),
            "FCF8   20 15 FD   JSR $FD15  ".trim().into(),
            "FCFB   20 5B FF   JSR $FF5B  ".trim().into(),
            "FCFE   58         CLI        ".trim().into(),
            "FCFF   6C 00 A0   JMP ($A000)".trim().into(),
        ];

        let result = block.disassemble();

        assert_eq!(result.len(), expected.len());
        for line in 0..result.len() {
            assert_eq!(result[line], expected[line]);
        }
        // cspell: enable
    }

    #[test]
    fn should_assemble_block() {
        let block = Block::assemble(
            r"
            *= $fce2    ; start here
            LDX #$FF
            SEI

            TXS
            CLD
            ; Try this
            JSR $FD02
            BNE $FCEF
            JMP ($8000)
            STX $D016
; And this
            JSR $FDA3
            JSR $FD50
            JSR $FD15
            JSR $FF5B
            CLI
            JMP ($A000)
        ",
        );

        let expected = Block {
            start: 0xFCE2,
            instructions: vec![
                0xa2, 0xff, 0x78, 0x9a, 0xd8, 0x20, 0x02, 0xfd, 0xd0, 0x03, 0x6c, 0x00, 0x80, 0x8e,
                0x16, 0xd0, 0x20, 0xa3, 0xfd, 0x20, 0x50, 0xfd, 0x20, 0x15, 0xfd, 0x20, 0x5b, 0xff,
                0x58, 0x6c, 0x00, 0xa0,
            ],
        };

        println!("{}", expected);
        println!("{}", block);

        assert_eq!(block.start, expected.start);
        assert_eq!(block.instructions.len(), expected.instructions.len());
    }

    #[test]
    fn should_parse_params() {
        let raw = "($a000)";
        let result = parse_params(raw);

        assert_eq!(vec![0x00, 0xa0], result);
    }

    #[test]
    fn should_copy_block_to_memory() {
        let mut c64 = C64::new();
        c64.reset();

        let block = Block {
            start: 0x1000,
            instructions: vec![0x78, 0x58, 0x00],
        };

        show(&block);
        let mut address = block.start as usize;
        for b in block.instructions.iter() {
            // println!("writing {:02x} to {:04x}", b, address);
            c64.cpu.write(address, *b);
            address += 1;
        }

        let byte1 = c64.cpu.read(0x1000);
        let byte2 = c64.cpu.read(0x1001);
        let byte3 = c64.cpu.read(0x1002);

        println!("CPU Read: {:02x} {:02x} {:02x}", byte1, byte2, byte3);
    }

    #[test]
    fn should_be_able_to_modify_memory_directly() {
        let mut c64 = C64::new();
        c64.memory[0x1000] = 0xca;
        c64.memory[0x1001] = 0xfe;
        c64.memory[0x1002] = 0xba;
        c64.memory[0x1003] = 0xbe;

        let byte1 = c64.cpu.read(0x1000);
        let byte2 = c64.cpu.read(0x1001);
        let byte3 = c64.cpu.read(0x1002);
        let byte4 = c64.cpu.read(0x1003);

        println!(
            "CPU Read: {:02x} {:02x} {:02x} {:02x}",
            byte1, byte2, byte3, byte4
        );
    }
}
