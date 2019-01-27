#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OSC Control Register"]
    pub cr: CR,
}
#[doc = "OSC Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "OSC Control Register"]
pub mod cr;
