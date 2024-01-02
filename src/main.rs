mod c64;
use crate::c64::{Block, C64};

fn main() {
    let block = Block {
        start: 0x1000,
        instructions: vec![0x78],
    };
    println!("{:#x?}", block);

    let mut c64 = C64::new();
    c64.reset();
}
