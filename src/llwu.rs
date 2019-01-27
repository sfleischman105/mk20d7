#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LLWU Pin Enable 1 Register"]
    pub pe1: PE1,
    #[doc = "0x01 - LLWU Pin Enable 2 Register"]
    pub pe2: PE2,
    #[doc = "0x02 - LLWU Pin Enable 3 Register"]
    pub pe3: PE3,
    #[doc = "0x03 - LLWU Pin Enable 4 Register"]
    pub pe4: PE4,
    #[doc = "0x04 - LLWU Module Enable Register"]
    pub me: ME,
    #[doc = "0x05 - LLWU Flag 1 Register"]
    pub f1: F1,
    #[doc = "0x06 - LLWU Flag 2 Register"]
    pub f2: F2,
    #[doc = "0x07 - LLWU Flag 3 Register"]
    pub f3: F3,
    #[doc = "0x08 - LLWU Pin Filter 1 Register"]
    pub filt1: FILT1,
    #[doc = "0x09 - LLWU Pin Filter 2 Register"]
    pub filt2: FILT2,
    #[doc = "0x0a - LLWU Reset Enable Register"]
    pub rst: RST,
}
#[doc = "LLWU Pin Enable 1 Register"]
pub struct PE1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 1 Register"]
pub mod pe1;
#[doc = "LLWU Pin Enable 2 Register"]
pub struct PE2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 2 Register"]
pub mod pe2;
#[doc = "LLWU Pin Enable 3 Register"]
pub struct PE3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 3 Register"]
pub mod pe3;
#[doc = "LLWU Pin Enable 4 Register"]
pub struct PE4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 4 Register"]
pub mod pe4;
#[doc = "LLWU Module Enable Register"]
pub struct ME {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Module Enable Register"]
pub mod me;
#[doc = "LLWU Flag 1 Register"]
pub struct F1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Flag 1 Register"]
pub mod f1;
#[doc = "LLWU Flag 2 Register"]
pub struct F2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Flag 2 Register"]
pub mod f2;
#[doc = "LLWU Flag 3 Register"]
pub struct F3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Flag 3 Register"]
pub mod f3;
#[doc = "LLWU Pin Filter 1 Register"]
pub struct FILT1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Filter 1 Register"]
pub mod filt1;
#[doc = "LLWU Pin Filter 2 Register"]
pub struct FILT2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Filter 2 Register"]
pub mod filt2;
#[doc = "LLWU Reset Enable Register"]
pub struct RST {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Reset Enable Register"]
pub mod rst;
