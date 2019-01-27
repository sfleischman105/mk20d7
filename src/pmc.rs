#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low Voltage Detect Status and Control 1 Register"]
    pub lvdsc1: LVDSC1,
    #[doc = "0x01 - Low Voltage Detect Status and Control 2 Register"]
    pub lvdsc2: LVDSC2,
    #[doc = "0x02 - Regulator Status and Control Register"]
    pub regsc: REGSC,
}
#[doc = "Low Voltage Detect Status and Control 1 Register"]
pub struct LVDSC1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Low Voltage Detect Status and Control 1 Register"]
pub mod lvdsc1;
#[doc = "Low Voltage Detect Status and Control 2 Register"]
pub struct LVDSC2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Low Voltage Detect Status and Control 2 Register"]
pub mod lvdsc2;
#[doc = "Regulator Status and Control Register"]
pub struct REGSC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Regulator Status and Control Register"]
pub mod regsc;
