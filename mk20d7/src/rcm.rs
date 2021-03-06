#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Reset Status Register 0"]
    pub srs0: SRS0,
    #[doc = "0x01 - System Reset Status Register 1"]
    pub srs1: SRS1,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - Reset Pin Filter Control Register"]
    pub rpfc: RPFC,
    #[doc = "0x05 - Reset Pin Filter Width Register"]
    pub rpfw: RPFW,
    _reserved1: [u8; 1usize],
    #[doc = "0x07 - Mode Register"]
    pub mr: MR,
}
#[doc = "System Reset Status Register 0"]
pub struct SRS0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "System Reset Status Register 0"]
pub mod srs0;
#[doc = "System Reset Status Register 1"]
pub struct SRS1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "System Reset Status Register 1"]
pub mod srs1;
#[doc = "Reset Pin Filter Control Register"]
pub struct RPFC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Reset Pin Filter Control Register"]
pub mod rpfc;
#[doc = "Reset Pin Filter Width Register"]
pub struct RPFW {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Reset Pin Filter Width Register"]
pub mod rpfw;
#[doc = "Mode Register"]
pub struct MR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Mode Register"]
pub mod mr;
