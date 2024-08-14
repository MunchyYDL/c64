#![allow(dead_code)]

pub mod block;
pub mod bus;
pub mod cpu;
pub mod memory;
pub mod vic_ii;

use std::cell::RefCell;
use std::rc::Rc;

use self::block::Block;
use self::bus::Bus;
use self::cpu::Cpu;
use self::memory::Memory;

pub(crate) struct C64 {
    cpu: Cpu,
    memory: Rc<RefCell<Memory>>,
    bus: Rc<RefCell<Bus>>,
}

impl C64 {
    pub fn new() -> Self {
        let memory = Rc::new(RefCell::new(Memory::new()));
        let bus = Rc::new(RefCell::new(Bus::new(memory.clone())));
        let cpu = Cpu::new(bus.clone());

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
    use tests::block::parse_params;

    use crate::show;

    use super::*;

    #[test]
    fn should_be_able_to_init_the_machine() {
        let mut c64 = C64::new();
        c64.reset();
        println!("CPU - {}", c64.cpu);
        assert_eq!(c64.cpu.PC, 0xfffc);
    }

    // rust-fmt disable
    #[rustfmt::skip]
    #[test]
    fn should_disassemble_block() {
        let block = Block {
            start: 0xfce2,
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
            "fce2   a2 ff      ldx #$ff   ".trim().into(),
            "fce4   78         sei        ".trim().into(),
            "fce5   9a         txs        ".trim().into(),
            "fce6   d8         cld        ".trim().into(),
            "fce7   20 02 fd   jsr $fd02  ".trim().into(),
            "fcea   d0 03      bne $fcef  ".trim().into(),
            "fcec   6c 00 80   jmp ($8000)".trim().into(),
            "fcef   8e 16 d0   stx $d016  ".trim().into(),
            "fcf2   20 a3 fd   jsr $fda3  ".trim().into(),
            "fcf5   20 50 fd   jsr $fd50  ".trim().into(),
            "fcf8   20 15 fd   jsr $fd15  ".trim().into(),
            "fcfb   20 5b ff   jsr $ff5b  ".trim().into(),
            "fcfe   58         cli        ".trim().into(),
            "fcff   6c 00 a0   jmp ($a000)".trim().into(),
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
            ldx #$ff
            sei

            txs
            cld
            ; Try this
            jsr $fd02
            bne $fcef
            jmp ($8000)
            stx $d016
; And this
            jsr $fda3
            jsr $fd50
            jsr $fd15
            jsr $ff5b
            cli
            jmp ($a000)
        ",
        );

        let expected = Block {
            start: 0xfce2,
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
        let mut address = block.start;
        for b in block.instructions.iter() {
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
        let c64 = C64::new();
        {
            let mut mem = c64.memory.borrow_mut();
            mem.write(0x1000, 0xca);
            mem.write(0x1001, 0xfe);
            mem.write(0x1002, 0xba);
            mem.write(0x1003, 0xbe);

            let byte1 = mem.read(0x1000);
            let byte2 = mem.read(0x1001);
            let byte3 = mem.read(0x1002);
            let byte4 = mem.read(0x1003);

            println!(
                "MEM Read: {:02x} {:02x} {:02x} {:02x}",
                byte1, byte2, byte3, byte4
            );
        }

        let byte1 = c64.cpu.read(0x1000);
        let byte2 = c64.cpu.read(0x1001);
        let byte3 = c64.cpu.read(0x1002);
        let byte4 = c64.cpu.read(0x1003);

        println!(
            "CPU Read: {:02x} {:02x} {:02x} {:02x}",
            byte1, byte2, byte3, byte4
        );
    }

    #[test]
    fn should_copy_block_from_memory() {
        let c64 = C64::new();

        let block = c64.memory.borrow().read_block(0x1000, 0x04);

        assert_eq!(block.start, 0x1000);
        assert_eq!(block.instructions, vec![0; 4]);

        // Modify the memory a bit
        {
            let mut mem = c64.memory.borrow_mut();
            mem.write(0x1000, 0xca);
            mem.write(0x1001, 0xfe);
            mem.write(0x1002, 0xba);
            mem.write(0x1003, 0xbe);

            let byte1 = mem.read(0x1000);
            let byte2 = mem.read(0x1001);
            let byte3 = mem.read(0x1002);
            let byte4 = mem.read(0x1003);

            println!(
                "MEM Read: {:02x} {:02x} {:02x} {:02x}",
                byte1, byte2, byte3, byte4
            );
        }

        let block = c64.memory.borrow().read_block(0x1000, 0x04);

        assert_eq!(block.start, 0x1000);
        assert_eq!(block.instructions, vec![0xca, 0xfe, 0xba, 0xbe]);
    }

    #[test]
    fn should_write_block_to_memory() {
        let c64 = C64::new();

        let block1 = Block {
            start: 0xc000,
            instructions: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa],
        };

        c64.memory.borrow_mut().write_block(&block1);

        let block2 = c64.memory.borrow().read_block(0xc000, 10);

        show(&block1);
        show(&block2);

        assert_eq!(&block1, &block2);
    }

    #[test]
    fn should_execute_a_small_program() {
        let mut c64 = C64::new();
        let source = r"
            *= $2000    ; start here
            lda #$03
            ora #$0f
        ";
        let prg = Block::assemble(source);

        show(&prg);

        c64.memory.borrow_mut().write_block(&prg);
        c64.cpu.PC = 0x2000;

        println!("CPU - {}\n", c64.cpu);

        for x in 0..2 {
            c64.cpu.step();
            println!("CPU - {}\n", c64.cpu);
        }
    }
}
