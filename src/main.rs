use c64::C64::{Block, Op};

fn main() {
    let block = Block {
        start: 0x1000,
        instructions: vec![Op::sei, Op::lda, Op::sta],
    };
    println!("{}: {:#x?}", stringify!(block), block);
}
