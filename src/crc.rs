#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC_CRCLL register."]
    pub crcll: CRCLL,
    #[doc = "0x01 - CRC_CRCLU register."]
    pub crclu: CRCLU,
    #[doc = "0x02 - CRC_CRCHL register."]
    pub crchl: CRCHL,
    #[doc = "0x03 - CRC_CRCHU register."]
    pub crchu: CRCHU,
    #[doc = "0x04 - CRC Polynomial Register"]
    pub gpoly: GPOLY,
    #[doc = "0x08 - CRC Control Register"]
    pub ctrl: CTRL,
}
#[doc = "CRC_CRCLL register."]
pub struct CRCLL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_CRCLL register."]
pub mod crcll;
#[doc = "CRC Data Register"]
pub struct CRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Data Register"]
pub mod crc;
#[doc = "CRC_CRCL register."]
pub struct CRCL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC_CRCL register."]
pub mod crcl;
#[doc = "CRC_CRCLU register."]
pub struct CRCLU {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_CRCLU register."]
pub mod crclu;
#[doc = "CRC_CRCHL register."]
pub struct CRCHL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_CRCHL register."]
pub mod crchl;
#[doc = "CRC_CRCH register."]
pub struct CRCH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC_CRCH register."]
pub mod crch;
#[doc = "CRC_CRCHU register."]
pub struct CRCHU {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_CRCHU register."]
pub mod crchu;
#[doc = "CRC Polynomial Register"]
pub struct GPOLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Polynomial Register"]
pub mod gpoly;
#[doc = "CRC_GPOLYLL register."]
pub struct GPOLYLL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_GPOLYLL register."]
pub mod gpolyll;
#[doc = "CRC_GPOLYL register."]
pub struct GPOLYL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC_GPOLYL register."]
pub mod gpolyl;
#[doc = "CRC_GPOLYLU register."]
pub struct GPOLYLU {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_GPOLYLU register."]
pub mod gpolylu;
#[doc = "CRC_GPOLYH register."]
pub struct GPOLYH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC_GPOLYH register."]
pub mod gpolyh;
#[doc = "CRC_GPOLYHL register."]
pub struct GPOLYHL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_GPOLYHL register."]
pub mod gpolyhl;
#[doc = "CRC_GPOLYHU register."]
pub struct GPOLYHU {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_GPOLYHU register."]
pub mod gpolyhu;
#[doc = "CRC Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Control Register"]
pub mod ctrl;
#[doc = "CRC_CTRLHU register."]
pub struct CTRLHU {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_CTRLHU register."]
pub mod ctrlhu;
