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

    pub fn read(&self, address: usize) -> u8 {
        *self.internal.get(address).unwrap()
    }

    pub fn write(&mut self, address: usize, value: u8) {
        self.internal[address] = value;
    }
}
