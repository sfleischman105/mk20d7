#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC status and control registers 1"]
    pub sc1a: SC1,
    #[doc = "0x04 - ADC status and control registers 1"]
    pub sc1b: SC1,
    #[doc = "0x08 - ADC configuration register 1"]
    pub cfg1: CFG1,
    #[doc = "0x0c - Configuration register 2"]
    pub cfg2: CFG2,
    #[doc = "0x10 - ADC data result register"]
    pub ra: R,
    #[doc = "0x14 - ADC data result register"]
    pub rb: R,
    #[doc = "0x18 - Compare value registers"]
    pub cv1: CV,
    #[doc = "0x1c - Compare value registers"]
    pub cv2: CV,
    #[doc = "0x20 - Status and control register 2"]
    pub sc2: SC2,
    #[doc = "0x24 - Status and control register 3"]
    pub sc3: SC3,
    #[doc = "0x28 - ADC offset correction register"]
    pub ofs: OFS,
    #[doc = "0x2c - ADC plus-side gain register"]
    pub pg: PG,
    #[doc = "0x30 - ADC minus-side gain register"]
    pub mg: MG,
    #[doc = "0x34 - ADC plus-side general calibration value register"]
    pub clpd: CLPD,
    #[doc = "0x38 - ADC plus-side general calibration value register"]
    pub clps: CLPS,
    #[doc = "0x3c - ADC plus-side general calibration value register"]
    pub clp4: CLP4,
    #[doc = "0x40 - ADC plus-side general calibration value register"]
    pub clp3: CLP3,
    #[doc = "0x44 - ADC plus-side general calibration value register"]
    pub clp2: CLP2,
    #[doc = "0x48 - ADC plus-side general calibration value register"]
    pub clp1: CLP1,
    #[doc = "0x4c - ADC plus-side general calibration value register"]
    pub clp0: CLP0,
    #[doc = "0x50 - ADC PGA register"]
    pub pga: PGA,
    #[doc = "0x54 - ADC minus-side general calibration value register"]
    pub clmd: CLMD,
    #[doc = "0x58 - ADC minus-side general calibration value register"]
    pub clms: CLMS,
    #[doc = "0x5c - ADC minus-side general calibration value register"]
    pub clm4: CLM4,
    #[doc = "0x60 - ADC minus-side general calibration value register"]
    pub clm3: CLM3,
    #[doc = "0x64 - ADC minus-side general calibration value register"]
    pub clm2: CLM2,
    #[doc = "0x68 - ADC minus-side general calibration value register"]
    pub clm1: CLM1,
    #[doc = "0x6c - ADC minus-side general calibration value register"]
    pub clm0: CLM0,
}
#[doc = "ADC status and control registers 1"]
pub struct SC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC status and control registers 1"]
pub mod sc1;
#[doc = "ADC configuration register 1"]
pub struct CFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC configuration register 1"]
pub mod cfg1;
#[doc = "Configuration register 2"]
pub struct CFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register 2"]
pub mod cfg2;
#[doc = "ADC data result register"]
pub struct R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC data result register"]
pub mod r;
#[doc = "Compare value registers"]
pub struct CV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare value registers"]
pub mod cv;
#[doc = "Status and control register 2"]
pub struct SC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status and control register 2"]
pub mod sc2;
#[doc = "Status and control register 3"]
pub struct SC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status and control register 3"]
pub mod sc3;
#[doc = "ADC offset correction register"]
pub struct OFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC offset correction register"]
pub mod ofs;
#[doc = "ADC plus-side gain register"]
pub struct PG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC plus-side gain register"]
pub mod pg;
#[doc = "ADC minus-side gain register"]
pub struct MG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC minus-side gain register"]
pub mod mg;
#[doc = "ADC plus-side general calibration value register"]
pub struct CLPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC plus-side general calibration value register"]
pub mod clpd;
#[doc = "ADC plus-side general calibration value register"]
pub struct CLPS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC plus-side general calibration value register"]
pub mod clps;
#[doc = "ADC plus-side general calibration value register"]
pub struct CLP4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC plus-side general calibration value register"]
pub mod clp4;
#[doc = "ADC plus-side general calibration value register"]
pub struct CLP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC plus-side general calibration value register"]
pub mod clp3;
#[doc = "ADC plus-side general calibration value register"]
pub struct CLP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC plus-side general calibration value register"]
pub mod clp2;
#[doc = "ADC plus-side general calibration value register"]
pub struct CLP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC plus-side general calibration value register"]
pub mod clp1;
#[doc = "ADC plus-side general calibration value register"]
pub struct CLP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC plus-side general calibration value register"]
pub mod clp0;
#[doc = "ADC PGA register"]
pub struct PGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC PGA register"]
pub mod pga;
#[doc = "ADC minus-side general calibration value register"]
pub struct CLMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC minus-side general calibration value register"]
pub mod clmd;
#[doc = "ADC minus-side general calibration value register"]
pub struct CLMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC minus-side general calibration value register"]
pub mod clms;
#[doc = "ADC minus-side general calibration value register"]
pub struct CLM4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC minus-side general calibration value register"]
pub mod clm4;
#[doc = "ADC minus-side general calibration value register"]
pub struct CLM3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC minus-side general calibration value register"]
pub mod clm3;
#[doc = "ADC minus-side general calibration value register"]
pub struct CLM2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC minus-side general calibration value register"]
pub mod clm2;
#[doc = "ADC minus-side general calibration value register"]
pub struct CLM1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC minus-side general calibration value register"]
pub mod clm1;
#[doc = "ADC minus-side general calibration value register"]
pub struct CLM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC minus-side general calibration value register"]
pub mod clm0;
