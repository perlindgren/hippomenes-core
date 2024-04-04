#[allow(unused_imports)]
use core::arch::asm;
use hippomenes_derive::CSRAccess;

// Field definitions
#[derive(CSRAccess)]
#[width = 1]
#[offset = 0]
#[address = 0x0]
pub enum Pin0 {}

#[derive(CSRAccess)]
#[width = 1]
#[offset = 1]
#[address = 0x0]
pub enum Pin1 {}

#[derive(CSRAccess)]
#[width = 1]
#[offset = 2]
#[address = 0x0]
pub enum Pin2 {}

#[derive(CSRAccess)]
#[width = 1]
#[offset = 3]
#[address = 0x0]
pub enum Pin3 {}

#[derive(CSRAccess)]
#[width = 1]
#[offset = 4]
#[address = 0x0]
pub enum Pin4 {}

// CSR as a whole
pub struct Bits;

impl Bits {
    read_csr_as_usize!(0x000);
    write_csr_as!(0x000);
    set!(0x000);
    clear!(0x000);
}
