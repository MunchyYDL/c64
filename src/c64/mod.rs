#![allow(dead_code)]

pub mod block;
pub mod bus;
pub mod cpu;

use self::block::Block;
use self::bus::Bus;
use self::cpu::Cpu;

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
