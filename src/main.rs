use crate::C64::Block;

mod C64;

fn main() {
    let block = Block {
        start: 0x1000,
        instructions: vec![0x78],
    };
    println!("{}: {:#x?}", stringify!(block), block);
}
