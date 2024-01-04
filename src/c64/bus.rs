use super::{cpu::Cpu, Memory};

#[derive(Clone, Debug)]
pub(crate) struct Bus {
    cpu: Cpu,
    memory: Memory,
}

impl Bus {
    pub fn new(cpu: Cpu, memory: Memory) -> Self {
        Bus { cpu, memory }
    }

    pub fn read(&self, address: usize) -> u8 {
        if (0x0000..=0xffff).contains(&address) {
            return self.memory[address];
        }
        0x00 // Default
    }

    pub fn write(&mut self, address: usize, value: u8) {
        if (0x0000..=0xffff).contains(&address) {
            self.memory[address] = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_and_write() {
        let mut bus = Bus {
            memory: [0xff; 0xffff],
            cpu: Cpu::new(),
        };

        let address = 0x1000;

        let value = bus.read(address);
        assert_eq!(value, 0xff);

        bus.write(address, 0x80);
        let value = bus.read(address);
        assert_eq!(value, 0x80);
    }
}
