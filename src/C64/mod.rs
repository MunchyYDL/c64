#![allow(dead_code, non_snake_case, non_camel_case_types)]

pub mod cpu;
pub mod ops;
pub mod status_registers;

type Byte = u8;
type Word = u16;

pub struct Memory {}

#[derive(Debug)]
pub struct Block {
    pub start: Word,
    pub instructions: Vec<Byte>,
}

