#[derive(Debug)]
pub(crate) struct Memory {
    internal: [u8; 0xffff],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            internal: [0x00; 0xffff],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        *self.internal.get(address as usize).unwrap()
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.internal[address as usize] = value;
    }
}
