#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status and Control Register"]
    pub sc: SC,
    #[doc = "0x04 - Modulus Register"]
    pub mod_: MOD,
    #[doc = "0x08 - Counter Register"]
    pub cnt: CNT,
    #[doc = "0x0c - Interrupt Delay Register"]
    pub idly: IDLY,
    #[doc = "0x10 - Channel n Control Register 1"]
    pub ch0c1: CHC1,
    #[doc = "0x14 - Channel n Status Register"]
    pub ch0s: CHS,
    #[doc = "0x18 - Channel n Delay 0 Register"]
    pub ch0dly0: CHDLY0,
    #[doc = "0x1c - Channel n Delay 1 Register"]
    pub ch0dly1: CHDLY1,
    _reserved0: [u8; 24usize],
    #[doc = "0x38 - Channel n Control Register 1"]
    pub ch1c1: CHC1,
    #[doc = "0x3c - Channel n Status Register"]
    pub ch1s: CHS,
    #[doc = "0x40 - Channel n Delay 0 Register"]
    pub ch1dly0: CHDLY0,
    #[doc = "0x44 - Channel n Delay 1 Register"]
    pub ch1dly1: CHDLY1,
    _reserved1: [u8; 264usize],
    #[doc = "0x150 - DAC Interval Trigger n Control Register"]
    pub dacintc: DACINTC,
    #[doc = "0x154 - DAC Interval n Register"]
    pub dacint: DACINT,
    _reserved2: [u8; 56usize],
    #[doc = "0x190 - Pulse-Out n Enable Register"]
    pub poen: POEN,
    #[doc = "0x194 - Pulse-Out n Delay Register"]
    pub podly: [PODLY; 3],
}
#[doc = "Status and Control Register"]
pub struct SC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status and Control Register"]
pub mod sc;
#[doc = "Modulus Register"]
pub struct MOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Modulus Register"]
pub mod mod_;
#[doc = "Counter Register"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Register"]
pub mod cnt;
#[doc = "Interrupt Delay Register"]
pub struct IDLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Delay Register"]
pub mod idly;
#[doc = "Channel n Control Register 1"]
pub struct CHC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Control Register 1"]
pub mod chc1;
#[doc = "Channel n Status Register"]
pub struct CHS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Status Register"]
pub mod chs;
#[doc = "Channel n Delay 0 Register"]
pub struct CHDLY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Delay 0 Register"]
pub mod chdly0;
#[doc = "Channel n Delay 1 Register"]
pub struct CHDLY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Delay 1 Register"]
pub mod chdly1;
#[doc = "DAC Interval Trigger n Control Register"]
pub struct DACINTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC Interval Trigger n Control Register"]
pub mod dacintc;
#[doc = "DAC Interval n Register"]
pub struct DACINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC Interval n Register"]
pub mod dacint;
#[doc = "Pulse-Out n Enable Register"]
pub struct POEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse-Out n Enable Register"]
pub mod poen;
#[doc = "Pulse-Out n Delay Register"]
pub struct PODLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse-Out n Delay Register"]
pub mod podly;
