use std::fmt::Display;

use super::cpu::{decode, AddressingMode, Instruction, MNEMONICS};

#[derive(Debug, PartialEq)]
pub(crate) struct Block {
    pub start: u16,
    pub instructions: Vec<u8>,
}

#[allow(clippy::zero_prefixed_literal)]
impl Block {
    pub fn memory(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        let step = 16;
        let max = self.instructions.len();

        (0..max).step_by(step).for_each(|pos| {
            let addr = self.start + pos as u16;

            let mut bytes = String::from("");
            let mut decoded = String::from("");

            let inner_max = std::cmp::min(pos + step, max);

            for pos in pos..inner_max {
                let byte = self.instructions[pos];
                let mut ch = byte as char;

                bytes += &format!("{byte:02x} ");

                // Handle unprintable chars
                if byte <= 0x20 || byte >= 0x7f {
                    ch = '.';
                }
                decoded += &format!("{}", ch);

                // Add inner spacing
                if pos % 4 == 3 {
                    bytes += "  ";
                }
            }
            result.push(format!("{addr:04x}   {bytes:56}{decoded}",));
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
                    let bytes = format!("{code:02x}      ");
                    let decoded = name.to_string();
                    (bytes, decoded)
                }
                2 => {
                    let lo = self.instructions[pos + 1];
                    let bytes = format!("{code:02x} {lo:02x}   ");
                    let decoded = match mode {
                        AddressingMode::Relative => format!("{name} ${:04x}", addr + lo as u16 + 2),
                        _ => format!("{name} #${lo:02x}"),
                    };
                    (bytes, decoded)
                }
                3 => {
                    let lo = self.instructions[pos + 1];
                    let hi = self.instructions[pos + 2];
                    let bytes = format!("{code:02x} {lo:02x} {hi:02x}");
                    let decoded = match mode {
                        AddressingMode::Indirect => format!("{name} (${hi:02x}{lo:02x})"),
                        _ => format!("{name} ${hi:02x}{lo:02x}"),
                    };
                    (bytes, decoded)
                }
                _ => panic!(),
            };

            result.push(format!("{addr:04x}   {bytes}   {decoded}",));
            pos += *length as usize;
        }
        result
    }

    // A really simple assembler function, to be able to
    // enter some code easily into the emulator, to test
    // it out a bit simpler during development.
    pub fn assemble(source: &str) -> Self {
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
                let code = MNEMONICS.get(&(mnemonic, mode)).unwrap_or(&unknown);

                println!("{instruction:16} -> {mnemonic} {params:8} - {mode:12?} -> {code:4x} {decoded:x?}");

                instructions.push(*code);
                if mnemonic == "bne" {
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

    pub fn from_prg(content: Vec<u8>) -> Self {
        let first = content[0];
        let second = content[1];
        let instructions = Vec::from(&content[2..content.len()]);

        let start: u16 = ((second as u16) << 8) + (first as u16);

        Block {
            start,
            instructions,
        }
    }
}

pub fn parse_params(params: &str) -> Vec<u8> {
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
