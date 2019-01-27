#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Crossbar switch (AXBS) slave configuration"]
    pub plasc: PLASC,
    #[doc = "0x0a - Crossbar switch (AXBS) master configuration"]
    pub plamc: PLAMC,
    #[doc = "0x0c - Control register"]
    pub cr: CR,
}
#[doc = "Crossbar switch (AXBS) slave configuration"]
pub struct PLASC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar switch (AXBS) slave configuration"]
pub mod plasc;
#[doc = "Crossbar switch (AXBS) master configuration"]
pub struct PLAMC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar switch (AXBS) master configuration"]
pub mod plamc;
#[doc = "Control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod cr;
