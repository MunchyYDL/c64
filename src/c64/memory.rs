use super::block::Block;

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

    //* These are some convenience functions, used for
    //* direct memory manipulation, only used from the
    //* outside, never from the C64 itself

    // Simple function to read a part of memory out to a block
    // FIXME: This function doesn't check for buffer overflow
    pub fn read_block(&self, address: u16, length: u16) -> Block {
        let len = length as usize;
        let start = address as usize;
        let end = start + len;

        let mut memory = vec![0u8; len];
        memory.clone_from_slice(&self.internal[start..end]);

        Block {
            start: address,
            instructions: memory,
        }
    }

    // Simple function to write a block to memory
    // FIXME: This function doesn't check for buffer overflow
    pub fn write_block(&mut self, block: &Block) {
        let mut current = block.start as usize;

        for b in block.instructions.iter() {
            self.internal[current] = *b;
            current += 1;
        }
    }
}
