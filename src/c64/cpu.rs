#![allow(non_snake_case)]
/*
  The 6510 microprocessor is a relatively simple 8 bit CPU with only a few internal
  registers capable of addressing at most 64kb of memory via it's 16 bit address bus.
  The processor is little endian and expects addresses to be stored in memory least
  significant byte first.

  The first 256 byte page of memory ($0000-$00ff) is referred to as 'Zero Page'
  and is the focus of a number of special addressing modes that result in shorter
  (and quicker) instructions or allow indirect access to the memory.

  The second page of memory ($0100-$01ff) is reserved for the system stack and
  which cannot be relocated.

  The only other reserved locations in the memory map are the very last 6 bytes
  of the memory $fffa-$ffff which must be programmed with the addresses of the
  non-maskable interrupt handler ($fffa/b), the power on reset location ($fffc/d)
  and the BRK/interrupt request handler ($fffe/f) respectively.

  The 6510 does not have any special support of hardware devices so they must be
  mapped to regions of memory in order to exchanges data with the hardware latches.
*/

use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use once_cell::sync::Lazy;

use super::{block::Block, bus::Bus};

#[derive(Debug)]
pub struct Cpu {
    /// Program Counter
    ///
    /// The program counter is a 16 bit register which points to the next instruction
    /// to be executed. The value of the PC is modified automatically as instructions
    /// are executed.
    pub PC: u16,
    /// Stack Pointer
    pub SP: u8,
    /// Accumulator
    pub A: u8,
    /// Index register X
    pub X: u8,
    /// Index register Y
    pub Y: u8,
    /// Status Registers
    pub SR: u8,

    // The amount of cycles still left for the last operation to complete
    cycles: u8,

    // The connected bus
    bus: Rc<RefCell<Bus>>,
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sr = {
            use StatusFlags::*;
            let n = self.get_flag(N);
            let v = self.get_flag(V);
            let b = self.get_flag(B);
            let d = self.get_flag(D);
            let i = self.get_flag(I);
            let z = self.get_flag(Z);
            let c = self.get_flag(C);

            let mut sr = String::new();
            sr.push(if n { 'N' } else { 'n' });
            sr.push(if v { 'V' } else { 'v' });
            sr.push('-');
            sr.push(if d { 'D' } else { 'd' });
            sr.push(if i { 'I' } else { 'i' });
            sr.push(if z { 'Z' } else { 'z' });
            sr.push(if c { 'C' } else { 'c' });

            sr
        };

        writeln!(
            f,
            "PC: {:04x}, SP: {:04x}, A: {:02x}, X: {:02x}, Y: {:02x} - SR: {} ({:08b})",
            self.PC, self.SP, self.A, self.X, self.Y, sr, self.SR
        )
    }
}

impl Cpu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Cpu {
            PC: 0xffc,
            SP: 0x00,
            A: 0x00,
            X: 0x00,
            Y: 0x00,
            SR: 0x00,
            cycles: 0,
            bus,
        }
    }
    pub fn reset(&mut self) {
        self.PC = 0xfffc;
        self.SP = 0x00;
        self.A = 0xaa;
        self.X = 0x00;
        self.Y = 0x00;
        self.clear_flag(StatusFlags::D);
    }

    pub fn clock() {
        // Should this function handle the logic of fetching an op-code and it's
        // corresponding fetching of data?

        // Maybe a simple start could be to keep a counter of the remaining cycles
        // to work through for the last op?
    }

    // Status Register - SR - Manipulation
    pub fn clear_flag(&mut self, flag: StatusFlags) {
        self.SR &= !(flag as u8)
    }

    pub fn get_flag(&self, flag: StatusFlags) -> bool {
        (self.SR & flag as u8) > 0
    }

    pub fn set_flag(&mut self, flag: StatusFlags) {
        self.SR |= flag as u8;
    }

    // // Bus related
    // pub fn connect_bus(&mut self, bus: Bus) {
    //     self.bus = bus;
    // }

    pub fn read(&self, address: usize) -> u8 {
        self.bus.borrow().read(address)
    }

    pub fn write(&mut self, address: usize, value: u8) {
        self.bus.borrow_mut().write(address, value);
    }

    fn step(&self, block: &Block) {

        //* 1. Copy block to memory
        //* 2. Set PC to start of block?
        //*
    }

    fn op_sei(&mut self) {
        self.set_flag(StatusFlags::I);
    }

    fn op_cli(&mut self) {
        self.clear_flag(StatusFlags::I)
    }
}

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
pub(crate) static MNEMONICS: Lazy<HashMap<(&str, AddressingMode), u8>> = Lazy::new(|| {
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

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::c64::Memory;

    use super::*;

    fn setup() -> Cpu {
        let memory = Rc::new(RefCell::new(Memory::new()));
        let bus = Rc::new(RefCell::new(Bus::new(memory)));
        Cpu::new(bus)
    }

    #[test]
    fn test_reset() {
        let mut cpu = setup();
        cpu.reset();

        assert_eq!(cpu.PC, 0xfffc);
        assert_eq!(cpu.A, 0xaa);
    }

    #[test]
    fn should_set_flag() {
        let mut cpu = setup();
        cpu.set_flag(StatusFlags::D);
        assert!(cpu.get_flag(StatusFlags::D));
    }

    #[test]
    fn should_clear_flag() {
        let mut cpu = setup();
        cpu.SR = StatusFlags::D as u8;
        assert!(cpu.get_flag(StatusFlags::D));

        cpu.clear_flag(StatusFlags::D);
        assert!(!cpu.get_flag(StatusFlags::D));
    }

    #[test]
    fn test_op_sei() {
        let mut cpu = setup();
        println!("CPU - {}", cpu);

        cpu.op_sei();
        println!("CPU - {}", cpu);

        cpu.op_cli();
        println!("CPU - {}", cpu);
    }
}
