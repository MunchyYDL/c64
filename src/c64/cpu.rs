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

use std::fmt::Display;

use super::{block::Block, bus::Bus};

#[derive(Clone, Default, Debug)]
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
    bus: Option<Box<Bus>>,
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
    pub fn new() -> Self {
        Self::default()
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

    // Bus related
    pub fn connect_bus(&mut self, bus: Bus) {
        self.bus = Some(Box::new(bus))
    }

    pub fn read(&self, address: usize) -> u8 {
        if let Some(bus) = &self.bus {
            bus.read(address)
        } else {
            0x00
        }
    }

    pub fn write(&mut self, address: usize, value: u8) {
        if let Some(bus) = &mut self.bus {
            bus.write(address, value);
        }
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
    use super::*;

    #[test]
    fn test_reset() {
        let mut cpu = Cpu::new();
        cpu.reset();

        assert_eq!(cpu.PC, 0xfffc);
        assert_eq!(cpu.A, 0xaa);
    }

    #[test]
    fn should_set_flag() {
        let mut cpu = Cpu::new();

        cpu.set_flag(StatusFlags::D);
        assert!(cpu.get_flag(StatusFlags::D));
    }

    #[test]
    fn should_clear_flag() {
        let mut cpu = Cpu::new();

        cpu.SR = StatusFlags::D as u8;
        assert!(cpu.get_flag(StatusFlags::D));

        cpu.clear_flag(StatusFlags::D);
        assert!(!cpu.get_flag(StatusFlags::D));
    }

    #[test]
    fn test_op_sei() {
        let mut cpu = Cpu::new();
        println!("CPU - {}", cpu);

        cpu.op_sei();
        println!("CPU - {}", cpu);

        cpu.op_cli();
        println!("CPU - {}", cpu);
    }
}
