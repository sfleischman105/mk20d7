#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Mode Protection Register"]
    pub pmprot: PMPROT,
    #[doc = "0x01 - Power Mode Control Register"]
    pub pmctrl: PMCTRL,
    #[doc = "0x02 - VLLS Control Register"]
    pub vllsctrl: VLLSCTRL,
    #[doc = "0x03 - Power Mode Status Register"]
    pub pmstat: PMSTAT,
}
#[doc = "Power Mode Protection Register"]
pub struct PMPROT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Power Mode Protection Register"]
pub mod pmprot;
#[doc = "Power Mode Control Register"]
pub struct PMCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Power Mode Control Register"]
pub mod pmctrl;
#[doc = "VLLS Control Register"]
pub struct VLLSCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "VLLS Control Register"]
pub mod vllsctrl;
#[doc = "Power Mode Status Register"]
pub struct PMSTAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Power Mode Status Register"]
pub mod pmstat;
