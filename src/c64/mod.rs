#![allow(dead_code)]

pub mod bus;
pub mod cpu;

use std::collections::HashMap;

use self::bus::Bus;
use self::cpu::Cpu;

#[derive(Debug)]
pub struct Block {
    pub start: u16,
    pub instructions: Vec<u8>,
}

struct Instruction {
    code: u8,
    mode: AddressingMode,
    name: String,
    length: u8,
    cycles: u8,
}

impl Instruction {
    pub const fn new(code: u8, mode: AddressingMode, name: String, length: u8, cycles: u8) -> Self {
        Instruction {
            code,
            mode,
            name,
            length,
            cycles,
        }
    }
}

#[derive(Clone, Copy)]
enum AddressingMode {
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

impl Block {
    pub fn memory(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        let step = 8;
        let max = self.instructions.len();

        (0..max).step_by(step).for_each(|pos| {
            let addr = self.start + pos as u16;

            let mut bytes = String::from("");
            let mut decoded = String::from("");

            let inner_max = std::cmp::min(pos + step, max);

            for pos in pos..inner_max {
                let byte = self.instructions[pos];
                bytes += &format!("{byte:02X} ");
                decoded += &format!("{}", byte as char);

                // Add inner spacing
                if pos % 4 == 3 {
                    bytes += "  ";
                }
            }
            result.push(format!("{addr:04X}   {bytes:28}{decoded}",));
        });
        result
    }

    #[rustfmt::skip]
    pub fn disassemble(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        let mut ins: HashMap<u8, Instruction> = HashMap::new();
        ins.insert(0xa2, Instruction::new(0xa2, AddressingMode::Immediate, "LDX".into(), 2, 0));
        ins.insert(0x78, Instruction::new(0x78, AddressingMode::Implied,   "SEI".into(), 1, 0));
        ins.insert(0x9a, Instruction::new(0x9a, AddressingMode::Implied,   "TXS".into(), 1, 0));
        ins.insert(0xd8, Instruction::new(0xd8, AddressingMode::Implied,   "CLD".into(), 1, 0));
        ins.insert(0x20, Instruction::new(0x20, AddressingMode::Absolute,  "JSR".into(), 3, 0));
        ins.insert(0xd0, Instruction::new(0xd0, AddressingMode::Relative,  "BNE".into(), 2, 0));
        ins.insert(0x58, Instruction::new(0x58, AddressingMode::Implied,   "CLI".into(), 1, 0));
        ins.insert(0x8e, Instruction::new(0x8e, AddressingMode::Absolute,  "STX".into(), 3, 0));
        ins.insert(0x6c, Instruction::new(0x6c, AddressingMode::Indirect,  "JMP".into(), 3, 0));

        // Memory Instructions
        ins.insert(0xa9, Instruction::new(0xa9, AddressingMode::Immediate,  "LDA".into(), 2, 2));
        ins.insert(0xa5, Instruction::new(0xa5, AddressingMode::ZeroPage,   "LDA".into(), 2, 3));
        ins.insert(0xb5, Instruction::new(0xb5, AddressingMode::ZeroPageX,  "LDA".into(), 2, 4));
        ins.insert(0xad, Instruction::new(0xad, AddressingMode::Absolute,   "LDA".into(), 3, 4));
        ins.insert(0xbd, Instruction::new(0xbd, AddressingMode::AbsoluteX,  "LDA".into(), 3, 4));
        ins.insert(0xb9, Instruction::new(0xb9, AddressingMode::AbsoluteY,  "LDA".into(), 3, 4));
        ins.insert(0xa1, Instruction::new(0xa1, AddressingMode::IndirectX,  "LDA".into(), 2, 6));
        ins.insert(0xb1, Instruction::new(0xb1, AddressingMode::IndirectY,  "LDA".into(), 2, 5));

        ins.insert(0x85, Instruction::new(0x85, AddressingMode::ZeroPage,   "STA".into(), 2, 3));
        ins.insert(0x95, Instruction::new(0x95, AddressingMode::ZeroPageX,  "STA".into(), 2, 4));
        ins.insert(0x8d, Instruction::new(0x8d, AddressingMode::Absolute,   "STA".into(), 3, 4));
        ins.insert(0x9d, Instruction::new(0x9d, AddressingMode::AbsoluteX,  "STA".into(), 3, 5));
        ins.insert(0x99, Instruction::new(0x99, AddressingMode::AbsoluteY,  "STA".into(), 3, 5));
        ins.insert(0x81, Instruction::new(0x81, AddressingMode::IndirectX,  "STA".into(), 2, 6));
        ins.insert(0x91, Instruction::new(0x91, AddressingMode::IndirectY,  "STA".into(), 2, 6));

        // Register Instructions
        ins.insert(0xaa, Instruction::new(0xaa, AddressingMode::Implied,    "TAX".into(), 1, 2));
        ins.insert(0xa8, Instruction::new(0xa8, AddressingMode::Implied,    "TAY".into(), 1, 2));
        ins.insert(0x8a, Instruction::new(0x8a, AddressingMode::Implied,    "TXA".into(), 1, 2));
        ins.insert(0x98, Instruction::new(0x98, AddressingMode::Implied,    "TYA".into(), 1, 2));

        ins.insert(0xca, Instruction::new(0xca, AddressingMode::Implied,    "DEX".into(), 1, 2));
        ins.insert(0x88, Instruction::new(0x88, AddressingMode::Implied,    "DEY".into(), 1, 2));
        ins.insert(0xe8, Instruction::new(0xe8, AddressingMode::Implied,    "INX".into(), 1, 2));
        ins.insert(0xc8, Instruction::new(0xc8, AddressingMode::Implied,    "INY".into(), 1, 2));


        // Stack Instructions
        ins.insert(0x48, Instruction::new(0x48, AddressingMode::Implied,  "PHA".into(), 1, 3));
        ins.insert(0x08, Instruction::new(0x08, AddressingMode::Implied,  "PHP".into(), 1, 3));
        ins.insert(0x9a, Instruction::new(0x9a, AddressingMode::Implied,  "TXS".into(), 1, 2));

        ins.insert(0x68, Instruction::new(0x68, AddressingMode::Implied,  "PLA".into(), 1, 4));
        ins.insert(0xba, Instruction::new(0xba, AddressingMode::Implied,  "TSX".into(), 1, 2));

        ins.insert(0x28, Instruction::new(0x28, AddressingMode::Implied,  "PLP".into(), 1, 4));

        // Other Instructions
        ins.insert(0x00, Instruction::new(0x00, AddressingMode::Implied,  "BRK".into(), 1, 7));
        ins.insert(0xea, Instruction::new(0xea, AddressingMode::Implied,  "NOP".into(), 1, 2));


        let mut pos = 0;

        while pos < self.instructions.len() {
            let inst = self.instructions[pos];

            let mut code = inst;
            let mut name = String::from("???");
            let mut length = 1;
            let mut mode = AddressingMode::Implied;

            if let Some(op) = ins.get(&inst) {
                code = op.code;
                name = op.name.clone();
                length = op.length;
                mode = op.mode;
            }

            let addr = self.start + pos as u16;

            let (bytes, decoded) = match length {
                1 => {
                    let bytes = format!("{code:02X}      ");
                    let decoded = name;
                    (bytes, decoded)
                }
                2 => {
                    let lo = self.instructions[pos + 1];
                    let bytes = format!("{code:02X} {lo:02X}   ");
                    let decoded = match mode {
                        AddressingMode::Relative => format!("{name} ${:04X}", addr + lo as u16 + 2),
                        _ => format!("{name} #${lo:02X}"),
                    };
                    (bytes, decoded)
                }
                3 => {
                    let lo = self.instructions[pos + 1];
                    let hi = self.instructions[pos + 2];
                    let bytes = format!("{code:02X} {lo:02X} {hi:02X}");
                    let decoded = match mode {
                        AddressingMode::Indirect => format!("{name} (${hi:02X}{lo:02X})"),
                        _ => format!("{name} ${hi:02X}{lo:02X}"),
                    };
                    (bytes, decoded)
                }
                _ => panic!(),
            };

            result.push(format!("{addr:04X}   {bytes}   {decoded}",));
            pos += length as usize;
        }
        result
    }
}

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
    use super::*;

    #[test]
    fn should_be_able_to_init_the_machine() {
        let mut c64 = C64::new();
        c64.reset();
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
}
