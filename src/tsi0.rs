#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Control and Status Register"]
    pub gencs: GENCS,
    #[doc = "0x04 - SCAN Control Register"]
    pub scanc: SCANC,
    #[doc = "0x08 - Pin Enable Register"]
    pub pen: PEN,
    #[doc = "0x0c - Wake-Up Channel Counter Register"]
    pub wucntr: WUCNTR,
    _reserved0: [u8; 240usize],
    #[doc = "0x100 - Counter Register"]
    pub cntr1: CNTR,
    #[doc = "0x104 - Counter Register"]
    pub cntr3: CNTR,
    #[doc = "0x108 - Counter Register"]
    pub cntr5: CNTR,
    #[doc = "0x10c - Counter Register"]
    pub cntr7: CNTR,
    #[doc = "0x110 - Counter Register"]
    pub cntr9: CNTR,
    #[doc = "0x114 - Counter Register"]
    pub cntr11: CNTR,
    #[doc = "0x118 - Counter Register"]
    pub cntr13: CNTR,
    #[doc = "0x11c - Counter Register"]
    pub cntr15: CNTR,
    #[doc = "0x120 - Low Power Channel Threshold Register"]
    pub threshold: THRESHOLD,
}
#[doc = "General Control and Status Register"]
pub struct GENCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Control and Status Register"]
pub mod gencs;
#[doc = "SCAN Control Register"]
pub struct SCANC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCAN Control Register"]
pub mod scanc;
#[doc = "Pin Enable Register"]
pub struct PEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Enable Register"]
pub mod pen;
#[doc = "Wake-Up Channel Counter Register"]
pub struct WUCNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake-Up Channel Counter Register"]
pub mod wucntr;
#[doc = "Counter Register"]
pub struct CNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Register"]
pub mod cntr;
#[doc = "Low Power Channel Threshold Register"]
pub struct THRESHOLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Power Channel Threshold Register"]
pub mod threshold;
