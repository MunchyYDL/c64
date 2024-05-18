use std::{cell::RefCell, rc::Rc};

use super::Memory;

#[derive(Debug)]
pub(crate) struct Bus {
    memory: Rc<RefCell<Memory>>,
}

impl Bus {
    pub fn new(memory: Rc<RefCell<Memory>>) -> Self {
        Bus { memory }
    }

    pub fn read(&self, address: u16) -> u8 {
        if (0x0000..=0xffff).contains(&address) {
            self.memory.borrow().read(address)
        } else {
            0x00 // Default
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if (0x0000..=0xffff).contains(&address) {
            self.memory.borrow_mut().write(address, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_and_write() {
        let memory = Rc::new(RefCell::new(Memory::new()));
        let mut bus = Bus { memory };

        let address = 0x1000;
        let value = bus.read(address);
        assert_eq!(value, 0x00);

        bus.write(address, 0x80);
        let value = bus.read(address);
        assert_eq!(value, 0x80);
    }
}
