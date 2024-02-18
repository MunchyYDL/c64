#![allow(dead_code)]

pub mod bus;
pub mod cpu;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt::Display;

use self::bus::Bus;
use self::cpu::Cpu;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub start: u16,
    pub instructions: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum AddressingMode {
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

    pub fn unknown(code: u8) -> Self {
        Instruction::new(code, AddressingMode::Implied, String::from("???"), 1, 0)
    }
}

pub fn decode(opcode: &u8) -> &'static Instruction {
    INSTRUCTIONS
        .get(opcode)
        .unwrap_or_else(|| &UNKNOWN_INSTRUCTION)
}

static UNKNOWN_INSTRUCTION: Lazy<Instruction> =
    Lazy::new(|| Instruction::new(0xff, AddressingMode::Implied, "???".into(), 1, 0));

#[rustfmt::skip]
static INSTRUCTIONS: Lazy<HashMap<u8, Instruction>> = Lazy::new(|| {
    use AddressingMode::*;
    HashMap::from([
        (0xa2, Instruction::new(0xa2, Immediate, "LDX".into(), 2, 0)),
        (0x78, Instruction::new(0x78, Implied,   "SEI".into(), 1, 0)),
        (0x9a, Instruction::new(0x9a, Implied,   "TXS".into(), 1, 0)),
        (0xd8, Instruction::new(0xd8, Implied,   "CLD".into(), 1, 0)),
        (0x20, Instruction::new(0x20, Absolute,  "JSR".into(), 3, 0)),
        (0xd0, Instruction::new(0xd0, Relative,  "BNE".into(), 2, 0)),
        (0x58, Instruction::new(0x58, Implied,   "CLI".into(), 1, 0)),
        (0x8e, Instruction::new(0x8e, Absolute,  "STX".into(), 3, 0)),
        (0x6c, Instruction::new(0x6c, Indirect,  "JMP".into(), 3, 0)),

        // Bitwise Instructions

        // Branch Instructions

        // Compare Instructions

        // Flag Instructions

        // Jump Instructions

        // Math Instructions



        // Memory Instructions
        (0xa9, Instruction::new(0xa9, Immediate, "LDA".into(), 2, 2)),
        (0xa5, Instruction::new(0xa5, ZeroPage,  "LDA".into(), 2, 3)),
        (0xb5, Instruction::new(0xb5, ZeroPageX, "LDA".into(), 2, 4)),
        (0xad, Instruction::new(0xad, Absolute,  "LDA".into(), 3, 4)),
        (0xbd, Instruction::new(0xbd, AbsoluteX, "LDA".into(), 3, 4)),
        (0xb9, Instruction::new(0xb9, AbsoluteY, "LDA".into(), 3, 4)),
        (0xa1, Instruction::new(0xa1, IndirectX, "LDA".into(), 2, 6)),
        (0xb1, Instruction::new(0xb1, IndirectY, "LDA".into(), 2, 5)),

        (0x85, Instruction::new(0x85, ZeroPage,  "STA".into(), 2, 3)),
        (0x95, Instruction::new(0x95, ZeroPageX, "STA".into(), 2, 4)),
        (0x8d, Instruction::new(0x8d, Absolute,  "STA".into(), 3, 4)),
        (0x9d, Instruction::new(0x9d, AbsoluteX, "STA".into(), 3, 5)),
        (0x99, Instruction::new(0x99, AbsoluteY, "STA".into(), 3, 5)),
        (0x81, Instruction::new(0x81, IndirectX, "STA".into(), 2, 6)),
        (0x91, Instruction::new(0x91, IndirectY, "STA".into(), 2, 6)),

        // more...
        
        // Register Instructions
        (0xaa, Instruction::new(0xaa, Implied,    "TAX".into(), 1, 2)),
        (0xa8, Instruction::new(0xa8, Implied,    "TAY".into(), 1, 2)),
        (0x8a, Instruction::new(0x8a, Implied,    "TXA".into(), 1, 2)),
        (0x98, Instruction::new(0x98, Implied,    "TYA".into(), 1, 2)),

        (0xca, Instruction::new(0xca, Implied,    "DEX".into(), 1, 2)),
        (0x88, Instruction::new(0x88, Implied,    "DEY".into(), 1, 2)),
        (0xe8, Instruction::new(0xe8, Implied,    "INX".into(), 1, 2)),
        (0xc8, Instruction::new(0xc8, Implied,    "INY".into(), 1, 2)),

        // Stack Instructions
        (0x48, Instruction::new(0x48, Implied,  "PHA".into(), 1, 3)),
        (0x08, Instruction::new(0x08, Implied,  "PHP".into(), 1, 3)),
        (0x9a, Instruction::new(0x9a, Implied,  "TXS".into(), 1, 2)),
        
        (0x68, Instruction::new(0x68, Implied,  "PLA".into(), 1, 4)),
        (0xba, Instruction::new(0xba, Implied,  "TSX".into(), 1, 2)),
        
        (0x28, Instruction::new(0x28, Implied,  "PLP".into(), 1, 4)),
        
        // Other Instructions
        (0x00, Instruction::new(0x00, Implied,  "BRK".into(), 1, 7)),
        (0xea, Instruction::new(0xea, Implied,  "NOP".into(), 1, 2)),      

        // FIXME: Unknown Instructions
        (0x77, Instruction::unknown(0x77)),  
    ])
});

static MNEMONICS: Lazy<HashMap<(&str, AddressingMode), u8>> = Lazy::new(|| {
    use AddressingMode::*;
    HashMap::from([
        (("LDX", Immediate), 0xa2),
        (("SEI", Implied), 0x78),
        (("TXS", Implied), 0x9a),
        (("CLD", Implied), 0xd8),
        (("JSR", Absolute), 0x20),
        (("BNE", Absolute), 0xd0),
        (("CLI", Implied), 0x58),
        (("STX", Absolute), 0x8e),
        (("JMP", Indirect), 0x6c),
        // (("LDA", Absolute), 0x00), (("LDA", Immediate), 0x00)
    ])
});

// #[rustfmt::skip]
// static LOOKUP: Lazy<HashMap<>

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

    pub fn disassemble(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        let mut pos = 0;

        while pos < self.instructions.len() {
            let inst = self.instructions[pos];
            let Instruction {
                code,
                name,
                length,
                mode,
                cycles,
            } = decode(&inst);

            let addr = self.start + pos as u16;

            let (bytes, decoded) = match length {
                1 => {
                    let bytes = format!("{code:02X}      ");
                    let decoded = name.to_string();
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
            pos += *length as usize;
        }
        result
    }

    // A really simple assembler function, to be able to
    // enter some code easily into the emulator, to test
    // it out a bit simpler during development.
    fn assemble(source: &str) -> Self {
        let mut found_start = false;
        let mut start: u16 = 0x0000;
        let mut instructions: Vec<u8> = vec![];

        // For now, require the start-address to be the first "instruction".
        // E.g. *= $0810
        for line in source.lines() {
            let instruction = {
                let l: Vec<&str> = line.split(';').collect();
                l[0].trim()
            };

            // Find the start address first
            if !found_start {
                let op = instruction.trim_start_matches("*= $");
                if let Ok(val) = u16::from_str_radix(op, 16) {
                    start = val;
                    found_start = true;
                }
            } else {
                // We're not interested in empty lines or comments
                if instruction.is_empty() {
                    continue;
                }

                let (mnemonic, params) = {
                    let l: Vec<&str> = instruction.split(' ').collect();
                    if l.len() == 1 {
                        (l[0], "")
                    } else {
                        (l[0], l[1])
                    }
                };

                // Let's deduce the addressing mode
                use AddressingMode::*;
                let mode = match params {
                    "" => AddressingMode::Implied,
                    x if x.starts_with('#') => Immediate,
                    x if x.starts_with('(') => match x {
                        x if x.ends_with(')') => Indirect,
                        x if x.ends_with("),x") => IndirectX,
                        x if x.ends_with("),y") => IndirectY,
                        _ => panic!(),
                    },
                    x if x.starts_with('$') => Absolute,
                    _ => panic!(),
                };

                let mut decoded = parse_params(params);

                let unknown = 0xef;
                let code = MNEMONICS.get(&(&mnemonic, mode)).unwrap_or(&unknown);

                println!("{instruction:16} -> {mnemonic} {params:8} - {mode:12?} -> {code:4x} {decoded:x?}");

                instructions.push(*code);
                if mnemonic == "BNE" {
                    instructions.push(0x03);
                } else {
                    instructions.append(&mut decoded);
                }

                // match mode {
                //     Absolute => todo!(),
                //     AbsoluteX => todo!(),
                //     AbsoluteY => todo!(),
                //     Immediate => instructions.append(decoded.clone().as_mut()),
                //     Implied => continue,
                //     Indirect => todo!(),
                //     IndirectX => todo!(),
                //     IndirectY => todo!(),
                //     Relative => todo!(),
                //     ZeroPage => todo!(),
                //     ZeroPageX => todo!(),
                //     ZeroPageY => todo!(),
                // }
            }
        }

        Block {
            start,
            instructions,
        }
    }
}

fn parse_params(params: &str) -> Vec<u8> {
    let without_prefix = params
        .trim_start_matches('(')
        .trim_start_matches('#')
        .trim_start_matches('$')
        .trim_end_matches(')');

    match without_prefix.len() {
        0 => vec![],
        1 | 2 => {
            let lo = u8::from_str_radix(without_prefix, 16).unwrap();
            vec![lo]
        }
        3 => {
            let hi_str = &without_prefix[0..1];
            let lo_str = &without_prefix[1..3];
            let hi = u8::from_str_radix(hi_str, 16).unwrap();
            let lo = u8::from_str_radix(lo_str, 16).unwrap();

            // println!(
            //     "{params}, {len} - {hi_str} {lo_str}, {lo} {hi}",
            //     len = without_prefix.len()
            // );
            vec![lo, hi]
        }
        4 => {
            let hi_str = &without_prefix[0..2];
            let lo_str = &without_prefix[2..4];
            let hi = u8::from_str_radix(hi_str, 16).unwrap();
            let lo = u8::from_str_radix(lo_str, 16).unwrap();

            // println!(
            //     "{params}, {len} - {hi_str} {lo_str}, {lo} {hi}",
            //     len = without_prefix.len()
            // );
            vec![lo, hi]
        }
        _ => panic!(),
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block [\n  Start: {:04x}\n  Instructions: {:02x?}\n]",
            self.start, self.instructions
        )
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
}
