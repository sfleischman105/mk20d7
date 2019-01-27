#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral ID Register"]
    pub perid: PERID,
    _reserved0: [u8; 3usize],
    #[doc = "0x04 - Peripheral ID Complement Register"]
    pub idcomp: IDCOMP,
    _reserved1: [u8; 3usize],
    #[doc = "0x08 - Peripheral Revision Register"]
    pub rev: REV,
    _reserved2: [u8; 3usize],
    #[doc = "0x0c - Peripheral Additional Info Register"]
    pub addinfo: ADDINFO,
    _reserved3: [u8; 3usize],
    #[doc = "0x10 - OTG Interrupt Status Register"]
    pub otgistat: OTGISTAT,
    _reserved4: [u8; 3usize],
    #[doc = "0x14 - OTG Interrupt Control Register"]
    pub otgicr: OTGICR,
    _reserved5: [u8; 3usize],
    #[doc = "0x18 - OTG Status Register"]
    pub otgstat: OTGSTAT,
    _reserved6: [u8; 3usize],
    #[doc = "0x1c - OTG Control Register"]
    pub otgctl: OTGCTL,
    _reserved7: [u8; 99usize],
    #[doc = "0x80 - Interrupt Status Register"]
    pub istat: ISTAT,
    _reserved8: [u8; 3usize],
    #[doc = "0x84 - Interrupt Enable Register"]
    pub inten: INTEN,
    _reserved9: [u8; 3usize],
    #[doc = "0x88 - Error Interrupt Status Register"]
    pub errstat: ERRSTAT,
    _reserved10: [u8; 3usize],
    #[doc = "0x8c - Error Interrupt Enable Register"]
    pub erren: ERREN,
    _reserved11: [u8; 3usize],
    #[doc = "0x90 - Status Register"]
    pub stat: STAT,
    _reserved12: [u8; 3usize],
    #[doc = "0x94 - Control Register"]
    pub ctl: CTL,
    _reserved13: [u8; 3usize],
    #[doc = "0x98 - Address Register"]
    pub addr: ADDR,
    _reserved14: [u8; 3usize],
    #[doc = "0x9c - BDT Page Register 1"]
    pub bdtpage1: BDTPAGE1,
    _reserved15: [u8; 3usize],
    #[doc = "0xa0 - Frame Number Register Low"]
    pub frmnuml: FRMNUML,
    _reserved16: [u8; 3usize],
    #[doc = "0xa4 - Frame Number Register High"]
    pub frmnumh: FRMNUMH,
    _reserved17: [u8; 3usize],
    #[doc = "0xa8 - Token Register"]
    pub token: TOKEN,
    _reserved18: [u8; 3usize],
    #[doc = "0xac - SOF Threshold Register"]
    pub softhld: SOFTHLD,
    _reserved19: [u8; 3usize],
    #[doc = "0xb0 - BDT Page Register 2"]
    pub bdtpage2: BDTPAGE2,
    _reserved20: [u8; 3usize],
    #[doc = "0xb4 - BDT Page Register 3"]
    pub bdtpage3: BDTPAGE3,
    _reserved21: [u8; 11usize],
    #[doc = "0xc0 - Endpoint Control Register"]
    pub endpt0: ENDPT,
    _reserved22: [u8; 3usize],
    #[doc = "0xc4 - Endpoint Control Register"]
    pub endpt1: ENDPT,
    _reserved23: [u8; 3usize],
    #[doc = "0xc8 - Endpoint Control Register"]
    pub endpt2: ENDPT,
    _reserved24: [u8; 3usize],
    #[doc = "0xcc - Endpoint Control Register"]
    pub endpt3: ENDPT,
    _reserved25: [u8; 3usize],
    #[doc = "0xd0 - Endpoint Control Register"]
    pub endpt4: ENDPT,
    _reserved26: [u8; 3usize],
    #[doc = "0xd4 - Endpoint Control Register"]
    pub endpt5: ENDPT,
    _reserved27: [u8; 3usize],
    #[doc = "0xd8 - Endpoint Control Register"]
    pub endpt6: ENDPT,
    _reserved28: [u8; 3usize],
    #[doc = "0xdc - Endpoint Control Register"]
    pub endpt7: ENDPT,
    _reserved29: [u8; 3usize],
    #[doc = "0xe0 - Endpoint Control Register"]
    pub endpt8: ENDPT,
    _reserved30: [u8; 3usize],
    #[doc = "0xe4 - Endpoint Control Register"]
    pub endpt9: ENDPT,
    _reserved31: [u8; 3usize],
    #[doc = "0xe8 - Endpoint Control Register"]
    pub endpt10: ENDPT,
    _reserved32: [u8; 3usize],
    #[doc = "0xec - Endpoint Control Register"]
    pub endpt11: ENDPT,
    _reserved33: [u8; 3usize],
    #[doc = "0xf0 - Endpoint Control Register"]
    pub endpt12: ENDPT,
    _reserved34: [u8; 3usize],
    #[doc = "0xf4 - Endpoint Control Register"]
    pub endpt13: ENDPT,
    _reserved35: [u8; 3usize],
    #[doc = "0xf8 - Endpoint Control Register"]
    pub endpt14: ENDPT,
    _reserved36: [u8; 3usize],
    #[doc = "0xfc - Endpoint Control Register"]
    pub endpt15: ENDPT,
    _reserved37: [u8; 3usize],
    #[doc = "0x100 - USB Control Register"]
    pub usbctrl: USBCTRL,
    _reserved38: [u8; 3usize],
    #[doc = "0x104 - USB OTG Observe Register"]
    pub observe: OBSERVE,
    _reserved39: [u8; 3usize],
    #[doc = "0x108 - USB OTG Control Register"]
    pub control: CONTROL,
    _reserved40: [u8; 3usize],
    #[doc = "0x10c - USB Transceiver Control Register 0"]
    pub usbtrc0: USBTRC0,
    _reserved41: [u8; 7usize],
    #[doc = "0x114 - Frame Adjust Register"]
    pub usbfrmadjust: USBFRMADJUST,
}
#[doc = "Peripheral ID Register"]
pub struct PERID {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Peripheral ID Register"]
pub mod perid;
#[doc = "Peripheral ID Complement Register"]
pub struct IDCOMP {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Peripheral ID Complement Register"]
pub mod idcomp;
#[doc = "Peripheral Revision Register"]
pub struct REV {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Peripheral Revision Register"]
pub mod rev;
#[doc = "Peripheral Additional Info Register"]
pub struct ADDINFO {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Peripheral Additional Info Register"]
pub mod addinfo;
#[doc = "OTG Interrupt Status Register"]
pub struct OTGISTAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "OTG Interrupt Status Register"]
pub mod otgistat;
#[doc = "OTG Interrupt Control Register"]
pub struct OTGICR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "OTG Interrupt Control Register"]
pub mod otgicr;
#[doc = "OTG Status Register"]
pub struct OTGSTAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "OTG Status Register"]
pub mod otgstat;
#[doc = "OTG Control Register"]
pub struct OTGCTL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "OTG Control Register"]
pub mod otgctl;
#[doc = "Interrupt Status Register"]
pub struct ISTAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Status Register"]
pub mod istat;
#[doc = "Interrupt Enable Register"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "Error Interrupt Status Register"]
pub struct ERRSTAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Error Interrupt Status Register"]
pub mod errstat;
#[doc = "Error Interrupt Enable Register"]
pub struct ERREN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Error Interrupt Enable Register"]
pub mod erren;
#[doc = "Status Register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status Register"]
pub mod stat;
#[doc = "Control Register"]
pub struct CTL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control Register"]
pub mod ctl;
#[doc = "Address Register"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Address Register"]
pub mod addr;
#[doc = "BDT Page Register 1"]
pub struct BDTPAGE1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "BDT Page Register 1"]
pub mod bdtpage1;
#[doc = "Frame Number Register Low"]
pub struct FRMNUML {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Frame Number Register Low"]
pub mod frmnuml;
#[doc = "Frame Number Register High"]
pub struct FRMNUMH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Frame Number Register High"]
pub mod frmnumh;
#[doc = "Token Register"]
pub struct TOKEN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Token Register"]
pub mod token;
#[doc = "SOF Threshold Register"]
pub struct SOFTHLD {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SOF Threshold Register"]
pub mod softhld;
#[doc = "BDT Page Register 2"]
pub struct BDTPAGE2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "BDT Page Register 2"]
pub mod bdtpage2;
#[doc = "BDT Page Register 3"]
pub struct BDTPAGE3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "BDT Page Register 3"]
pub mod bdtpage3;
#[doc = "Endpoint Control Register"]
pub struct ENDPT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Endpoint Control Register"]
pub mod endpt;
#[doc = "USB Control Register"]
pub struct USBCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Control Register"]
pub mod usbctrl;
#[doc = "USB OTG Observe Register"]
pub struct OBSERVE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB OTG Observe Register"]
pub mod observe;
#[doc = "USB OTG Control Register"]
pub struct CONTROL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB OTG Control Register"]
pub mod control;
#[doc = "USB Transceiver Control Register 0"]
pub struct USBTRC0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transceiver Control Register 0"]
pub mod usbtrc0;
#[doc = "Frame Adjust Register"]
pub struct USBFRMADJUST {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Frame Adjust Register"]
pub mod usbfrmadjust;
