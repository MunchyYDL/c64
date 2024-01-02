use crate::C64::{Byte, Word};

use super::status_registers::{StatusRegisters, StatusFlags};

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

#[derive(Clone, Copy, Default)]
pub struct Cpu {
    /// Program Counter
    ///
    /// The program counter is a 16 bit register which points to the next instruction
    /// to be executed. The value of the PC is modified automatically as instructions
    /// are executed
    PC: Word,
    /// Stack Pointer
    SP: Word,
    /// Accumulator
    A: Byte,
    /// Index register X
    X: Byte,
    /// Index register Y
    Y: Byte,
    /// Status Registers
    SR: StatusRegisters,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clock() {
        // Should this function handle the logic of fetching an op-code and it's
        // corresponding fetching of data?

        // Maybe a simple start could be to keep a counter of the remaining cycles
        // to work through for the last op?
    }
}

// Ops
impl Cpu {
    fn op_sei(&mut self) {
        self.SR.set(StatusFlags::I);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_sei() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.SR, StatusRegisters::new(0));

        cpu.op_sei();
        assert!(cpu.SR.get(StatusFlags::I));
    }
}
