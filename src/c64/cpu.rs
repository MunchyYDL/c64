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
pub mod operations;
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

    /// IR - Instruction Register, the current instruction the cpu is executing
    pub IR: Option<Instruction>,
    // The amount of cycles still left for the last operation to complete
    cycles: u8,

    // The connected bus
    bus: Rc<RefCell<Bus>>,
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
            IR: None,
        }
    }

    pub fn reset(&mut self) {
        self.PC = 0xfffc;
        self.SP = 0x00;
        self.A = 0x00;
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
    fn clear_flag(&mut self, flag: StatusFlags) {
        self.SR.clear(flag);
    }

    fn get_flag(&self, flag: StatusFlags) -> bool {
        self.SR.get(flag)
    }

    fn set_flag(&mut self, flag: StatusFlags) {
        self.SR.set(flag);
    }

    //* Memory Access
    fn fetch(&mut self) -> u8 {
        let data = self.read(self.PC);
        self.PC += 1;
        data
    }

    // FIXME: Change these to private, when the tests are changed.
    pub(crate) fn read(&self, address: u16) -> u8 {
        self.bus.borrow().read(address)
    }

    pub(crate) fn write(&mut self, address: u16, value: u8) {
        self.bus.borrow_mut().write(address, value);
    }

    //* Addressing Modes

    // This function steps an entire instruction at a time.
    // This is a naive first implementation, I guess I will
    // need to make this a state machine later on, for better
    // cycle accuracy. I will most likely also need a more
    // detailed model of each instruction, like when it reads
    // and writes during it's execution, for knowing when
    // it will be blocked by the VIC-II chip or not. 🤔
    pub(crate) fn step(&mut self) {
        // Fetch
        let opcode = self.fetch();
        let op = decode(&opcode);

        println!("{}", op);

        let (address, value) = match op.mode {
            AddressingMode::Accumulator => (0, 0),
            AddressingMode::Absolute => {
                let lo = self.fetch() as u16;
                let hi = self.fetch() as u16;
                let address: u16 = (hi << 8) + lo;
                let value = self.read(address);
                println!("  Absolute read: ${:04x} = {:02x}", address, value);
                (address, value)
            }
            AddressingMode::AbsoluteX => todo!(),
            AddressingMode::AbsoluteY => todo!(),
            AddressingMode::Immediate => {
                let address = self.PC;
                print!("   Immediate: ${:04x}", address);
                let value = self.fetch();
                println!(" = {:02x}", value);
                (address, value)
            }
            AddressingMode::Implied => (0, 0),
            AddressingMode::Indirect => todo!(),
            AddressingMode::IndirectX => todo!(),
            AddressingMode::IndirectY => todo!(),
            AddressingMode::Relative => todo!(),
            AddressingMode::ZeroPage => todo!(),
            AddressingMode::ZeroPageX => todo!(),
            AddressingMode::ZeroPageY => todo!(),
        };

        match (op.name, op.code) {
            ("sei", _) => self.op_sei(),
            ("cli", _) => self.op_cli(),
            ("lda", _) => self.op_lda(value),
            ("ldx", _) => self.op_ldx(value),
            ("ldy", _) => self.op_ldy(value),
            ("sta", _) => self.op_sta(address),
            ("stx", _) => self.op_stx(address),
            ("sty", _) => self.op_sty(address),
            // 0x00 => self.op_brk(),
            _ => {
                println!("Not implemented yet: {}", op)
            }
        }
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PC: {:04x}, SP: {:02x}, A: {:02x}, X: {:02x}, Y: {:02x} - SR: {}",
            self.PC, self.SP, self.A, self.X, self.Y, self.SR
        )
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
        cpu.PC = 0x0801;
        cpu.A = 0xaa;
        assert_eq!(cpu.PC, 0x0801);
        assert_eq!(cpu.A, 0xaa);

        cpu.reset();
        assert_eq!(cpu.PC, 0xfffc);
        assert_eq!(cpu.A, 0x00);
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
    fn test_ops_sei_cli() {
        let mut cpu = setup();
        println!("CPU - {}", cpu);

        cpu.op_sei();
        println!("CPU - {}", cpu);

        cpu.op_cli();
        println!("CPU - {}", cpu);
    }
}
