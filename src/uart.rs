#[allow(unused_imports)]
use core::arch::asm;
use hippomenes_derive::CSRAccess;

// Field definitions

// Output pins
#[derive(CSRAccess)]
#[width = 8]
#[offset = 0]
#[address = 0x050]
pub enum Uart {}

// CSR as a whole
pub struct Bits;

impl Bits {
    read_csr_as_usize!(0x050);
    write_csr_as!(0x050);
    set!(0x050);
    clear!(0x050);
}
