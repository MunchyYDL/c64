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

pub mod addressing_modes;
pub mod instructions;
pub mod status_register;

pub use addressing_modes::AddressingMode;
pub use instructions::*;
pub use status_register::*;

use super::bus::Bus;

use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug)]
#[allow(non_snake_case)]
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
    pub SR: StatusRegister,

    // The amount of cycles still left for the last operation to complete
    cycles: u8,

    // The connected bus
    bus: Rc<RefCell<Bus>>,
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PC: {:04x}, SP: {:04x}, A: {:02x}, X: {:02x}, Y: {:02x} - SR: {}",
            self.PC, self.SP, self.A, self.X, self.Y, self.SR
        )
    }
}

impl Cpu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Cpu {
            PC: 0xfffc,
            SP: 0x00,
            A: 0x00,
            X: 0x00,
            Y: 0x00,
            SR: StatusRegister(0),
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

    //* Status Register - SR - Manipulation
    pub fn clear_flag(&mut self, flag: StatusFlags) {
        // self.SR &= !(flag as u8)
        self.SR.clear(flag);
    }

    pub fn get_flag(&self, flag: StatusFlags) -> bool {
        // (self.SR & flag as u8) > 0
        self.SR.get(flag)
    }

    fn set_flag(&mut self, flag: StatusFlags) {
        // self.SR |= flag as u8;
        self.SR.set(flag);
    }

    //* Memory Access
    // TODO: Should these be private instead?
    pub fn read(&self, address: u16) -> u8 {
        self.bus.borrow().read(address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.bus.borrow_mut().write(address, value);
    }

    //* Addressing Modes

    //* Operations
    fn op_sei(&mut self) {
        self.set_flag(StatusFlags::I);
    }

    fn op_cli(&mut self) {
        self.clear_flag(StatusFlags::I)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::c64::Memory;
    use std::cell::RefCell;

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
        cpu.SR = StatusRegister(StatusFlags::D as u8);
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
