#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub control: CONTROL,
    #[doc = "0x04 - Clock Register"]
    pub clock: CLOCK,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - TIMER0 Register"]
    pub timer0: TIMER0,
    #[doc = "0x14 - no description available"]
    pub timer1: TIMER1,
    #[doc = "0x18 - no description available"]
    pub timer2: TIMER2,
}
#[doc = "Control Register"]
pub struct CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod control;
#[doc = "Clock Register"]
pub struct CLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Register"]
pub mod clock;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "TIMER0 Register"]
pub struct TIMER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIMER0 Register"]
pub mod timer0;
#[doc = "no description available"]
pub struct TIMER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod timer1;
#[doc = "no description available"]
pub struct TIMER2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod timer2;
