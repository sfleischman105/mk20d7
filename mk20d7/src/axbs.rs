#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Priority Registers Slave"]
    pub prs0: PRS,
    _reserved0: [u8; 12usize],
    #[doc = "0x10 - Control Register"]
    pub crs0: CRS,
    _reserved1: [u8; 236usize],
    #[doc = "0x100 - Priority Registers Slave"]
    pub prs1: PRS,
    _reserved2: [u8; 12usize],
    #[doc = "0x110 - Control Register"]
    pub crs1: CRS,
    _reserved3: [u8; 236usize],
    #[doc = "0x200 - Priority Registers Slave"]
    pub prs2: PRS,
    _reserved4: [u8; 12usize],
    #[doc = "0x210 - Control Register"]
    pub crs2: CRS,
    _reserved5: [u8; 236usize],
    #[doc = "0x300 - Priority Registers Slave"]
    pub prs3: PRS,
    _reserved6: [u8; 12usize],
    #[doc = "0x310 - Control Register"]
    pub crs3: CRS,
    _reserved7: [u8; 1260usize],
    #[doc = "0x800 - Master General Purpose Control Register"]
    pub mgpcr0: MGPCR,
    _reserved8: [u8; 252usize],
    #[doc = "0x900 - Master General Purpose Control Register"]
    pub mgpcr1: MGPCR,
    _reserved9: [u8; 252usize],
    #[doc = "0xa00 - Master General Purpose Control Register"]
    pub mgpcr2: MGPCR,
    _reserved10: [u8; 252usize],
    #[doc = "0xb00 - Master General Purpose Control Register"]
    pub mgpcr3: MGPCR,
}
#[doc = "Priority Registers Slave"]
pub struct PRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Registers Slave"]
pub mod prs;
#[doc = "Control Register"]
pub struct CRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod crs;
#[doc = "Master General Purpose Control Register"]
pub struct MGPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master General Purpose Control Register"]
pub mod mgpcr;
