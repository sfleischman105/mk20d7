#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIT Module Control Register"]
    pub mcr: MCR,
    _reserved0: [u8; 252usize],
    #[doc = "0x100 - Timer Load Value Register"]
    pub ldval0: LDVAL,
    #[doc = "0x104 - Current Timer Value Register"]
    pub cval0: CVAL,
    #[doc = "0x108 - Timer Control Register"]
    pub tctrl0: TCTRL,
    #[doc = "0x10c - Timer Flag Register"]
    pub tflg0: TFLG,
    #[doc = "0x110 - Timer Load Value Register"]
    pub ldval1: LDVAL,
    #[doc = "0x114 - Current Timer Value Register"]
    pub cval1: CVAL,
    #[doc = "0x118 - Timer Control Register"]
    pub tctrl1: TCTRL,
    #[doc = "0x11c - Timer Flag Register"]
    pub tflg1: TFLG,
    #[doc = "0x120 - Timer Load Value Register"]
    pub ldval2: LDVAL,
    #[doc = "0x124 - Current Timer Value Register"]
    pub cval2: CVAL,
    #[doc = "0x128 - Timer Control Register"]
    pub tctrl2: TCTRL,
    #[doc = "0x12c - Timer Flag Register"]
    pub tflg2: TFLG,
    #[doc = "0x130 - Timer Load Value Register"]
    pub ldval3: LDVAL,
    #[doc = "0x134 - Current Timer Value Register"]
    pub cval3: CVAL,
    #[doc = "0x138 - Timer Control Register"]
    pub tctrl3: TCTRL,
    #[doc = "0x13c - Timer Flag Register"]
    pub tflg3: TFLG,
}
#[doc = "PIT Module Control Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PIT Module Control Register"]
pub mod mcr;
#[doc = "Timer Load Value Register"]
pub struct LDVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Load Value Register"]
pub mod ldval;
#[doc = "Current Timer Value Register"]
pub struct CVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Timer Value Register"]
pub mod cval;
#[doc = "Timer Control Register"]
pub struct TCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Control Register"]
pub mod tctrl;
#[doc = "Timer Flag Register"]
pub struct TFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Flag Register"]
pub mod tflg;
