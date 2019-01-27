#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip select address register"]
    pub csar0: CSAR,
    #[doc = "0x04 - Chip select mask register"]
    pub csmr0: CSMR,
    #[doc = "0x08 - Chip select control register"]
    pub cscr0: CSCR,
    #[doc = "0x0c - Chip select address register"]
    pub csar1: CSAR,
    #[doc = "0x10 - Chip select mask register"]
    pub csmr1: CSMR,
    #[doc = "0x14 - Chip select control register"]
    pub cscr1: CSCR,
    #[doc = "0x18 - Chip select address register"]
    pub csar2: CSAR,
    #[doc = "0x1c - Chip select mask register"]
    pub csmr2: CSMR,
    #[doc = "0x20 - Chip select control register"]
    pub cscr2: CSCR,
    #[doc = "0x24 - Chip select address register"]
    pub csar3: CSAR,
    #[doc = "0x28 - Chip select mask register"]
    pub csmr3: CSMR,
    #[doc = "0x2c - Chip select control register"]
    pub cscr3: CSCR,
    #[doc = "0x30 - Chip select address register"]
    pub csar4: CSAR,
    #[doc = "0x34 - Chip select mask register"]
    pub csmr4: CSMR,
    #[doc = "0x38 - Chip select control register"]
    pub cscr4: CSCR,
    #[doc = "0x3c - Chip select address register"]
    pub csar5: CSAR,
    #[doc = "0x40 - Chip select mask register"]
    pub csmr5: CSMR,
    #[doc = "0x44 - Chip select control register"]
    pub cscr5: CSCR,
    _reserved0: [u8; 24usize],
    #[doc = "0x60 - Chip select port multiplexing control register"]
    pub cspmcr: CSPMCR,
}
#[doc = "Chip select address register"]
pub struct CSAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip select address register"]
pub mod csar;
#[doc = "Chip select mask register"]
pub struct CSMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip select mask register"]
pub mod csmr;
#[doc = "Chip select control register"]
pub struct CSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip select control register"]
pub mod cscr;
#[doc = "Chip select port multiplexing control register"]
pub struct CSPMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip select port multiplexing control register"]
pub mod cspmcr;
