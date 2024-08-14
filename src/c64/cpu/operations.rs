use super::{Cpu, StatusFlags};

//* Operations

//* Bitwise Operations
impl Cpu {
    fn op_and(&mut self, value: u8) {}

    pub fn op_ora(&mut self, value: u8) {
        self.A &= value;
    }
}

//* Flag Operations */
impl Cpu {
    fn op_clc(&mut self) {
        self.clear_flag(StatusFlags::C);
    }
    fn op_sec(&mut self) {
        self.set_flag(StatusFlags::C)
    }

    fn op_cld(&mut self) {
        self.clear_flag(StatusFlags::D);
    }
    fn op_sed(&mut self) {
        self.set_flag(StatusFlags::D)
    }

    pub fn op_cli(&mut self) {
        self.clear_flag(StatusFlags::I)
    }
    pub fn op_sei(&mut self) {
        self.set_flag(StatusFlags::I);
    }

    fn op_clv(&mut self) {
        self.clear_flag(StatusFlags::V);
    }
}

//* Memory Operations */
impl Cpu {
    pub fn op_lda(&mut self, value: u8) {
        self.A = value;
    }
    pub fn op_ldx(&mut self, value: u8) {
        self.X = value;
    }
    pub fn op_ldy(&mut self, value: u8) {
        self.Y = value;
    }

    pub fn op_sta(&mut self, address: u16) {
        self.write(address, self.A);
    }
    pub fn op_stx(&mut self, address: u16) {
        self.write(address, self.X);
    }
    pub fn op_sty(&mut self, address: u16) {
        self.write(address, self.Y);
    }
}

//* Other Operations */
impl Cpu {
    pub fn op_brk(&mut self) {
        // todo!();
    }

    pub fn op_nop(&mut self) {
        // todo!();
    }
}
