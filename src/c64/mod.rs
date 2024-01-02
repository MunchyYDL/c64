#![allow(dead_code)]

pub mod bus;
pub mod cpu;

use self::bus::Bus;
use self::cpu::Cpu;

#[derive(Debug)]
pub struct Block {
    pub start: u16,
    pub instructions: Vec<u8>,
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
    use super::C64;

    #[test]
    fn should_be_able_to_init_the_machine() {
        let mut c64 = C64::new();
        c64.reset();
        assert_eq!(c64.cpu.PC, 0xfffc);
    }
}
