#![allow(dead_code, unused_variables)]
mod c64;
use crate::c64::Block;

#[rustfmt::skip]
fn main() {
    let block = Block {
        start: 0x0801,
        instructions: vec![
            0x78, 0x77, 0x78, 0x20, 0x02, 0x34
        ],
    };
    show(block);

    let block = Block {
        start: 0xFCE2,
        instructions: vec![
            0xa2, 0xff, 0x78, 0x9a, 0xd8, 0x20, 0x02, 0xfd,
            0xd0, 0x03, 0x6c, 0x00, 0x80, 0x8e, 0x16, 0xd0,
            0x20, 0xa3, 0xfd, 0x20, 0x50, 0xfd, 0x20, 0x15,
            0xfd, 0x20, 0x5b, 0xff, 0x58, 0x6c, 0x00, 0xa0,
        ],
    };
    show(block);
}

fn show(block: Block) {
    show_vec(&block.memory());
    show_vec(&block.disassemble());
}

fn show_vec(vec: &[String]) {
    for line in vec.iter() {
        println!("{}", line);
    }
    println!();
}
