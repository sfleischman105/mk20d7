#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Access Protection Register"]
    pub pfapr: PFAPR,
    #[doc = "0x04 - Flash Bank 0 Control Register"]
    pub pfb0cr: PFB0CR,
    #[doc = "0x08 - Flash Bank 1 Control Register"]
    pub pfb1cr: PFB1CR,
    _reserved0: [u8; 244usize],
    #[doc = "0x100 - Cache Tag Storage"]
    pub tagvdw0s: [TAGVDW0S; 8],
    #[doc = "0x120 - Cache Tag Storage"]
    pub tagvdw1s: [TAGVDW1S; 8],
    #[doc = "0x140 - Cache Tag Storage"]
    pub tagvdw2s: [TAGVDW2S; 8],
    #[doc = "0x160 - Cache Tag Storage"]
    pub tagvdw3s: [TAGVDW3S; 8],
    _reserved1: [u8; 128usize],
    #[doc = "0x200 - Cache Data Storage (upper word)"]
    pub dataw0s0u: DATAW0SU,
    #[doc = "0x204 - Cache Data Storage (lower word)"]
    pub dataw0s0l: DATAW0SL,
    #[doc = "0x208 - Cache Data Storage (upper word)"]
    pub dataw0s1u: DATAW0SU,
    #[doc = "0x20c - Cache Data Storage (lower word)"]
    pub dataw0s1l: DATAW0SL,
    #[doc = "0x210 - Cache Data Storage (upper word)"]
    pub dataw0s2u: DATAW0SU,
    #[doc = "0x214 - Cache Data Storage (lower word)"]
    pub dataw0s2l: DATAW0SL,
    #[doc = "0x218 - Cache Data Storage (upper word)"]
    pub dataw0s3u: DATAW0SU,
    #[doc = "0x21c - Cache Data Storage (lower word)"]
    pub dataw0s3l: DATAW0SL,
    #[doc = "0x220 - Cache Data Storage (upper word)"]
    pub dataw0s4u: DATAW0SU,
    #[doc = "0x224 - Cache Data Storage (lower word)"]
    pub dataw0s4l: DATAW0SL,
    #[doc = "0x228 - Cache Data Storage (upper word)"]
    pub dataw0s5u: DATAW0SU,
    #[doc = "0x22c - Cache Data Storage (lower word)"]
    pub dataw0s5l: DATAW0SL,
    #[doc = "0x230 - Cache Data Storage (upper word)"]
    pub dataw0s6u: DATAW0SU,
    #[doc = "0x234 - Cache Data Storage (lower word)"]
    pub dataw0s6l: DATAW0SL,
    #[doc = "0x238 - Cache Data Storage (upper word)"]
    pub dataw0s7u: DATAW0SU,
    #[doc = "0x23c - Cache Data Storage (lower word)"]
    pub dataw0s7l: DATAW0SL,
    #[doc = "0x240 - Cache Data Storage (upper word)"]
    pub dataw1s0u: DATAW1SU,
    #[doc = "0x244 - Cache Data Storage (lower word)"]
    pub dataw1s0l: DATAW1SL,
    #[doc = "0x248 - Cache Data Storage (upper word)"]
    pub dataw1s1u: DATAW1SU,
    #[doc = "0x24c - Cache Data Storage (lower word)"]
    pub dataw1s1l: DATAW1SL,
    #[doc = "0x250 - Cache Data Storage (upper word)"]
    pub dataw1s2u: DATAW1SU,
    #[doc = "0x254 - Cache Data Storage (lower word)"]
    pub dataw1s2l: DATAW1SL,
    #[doc = "0x258 - Cache Data Storage (upper word)"]
    pub dataw1s3u: DATAW1SU,
    #[doc = "0x25c - Cache Data Storage (lower word)"]
    pub dataw1s3l: DATAW1SL,
    #[doc = "0x260 - Cache Data Storage (upper word)"]
    pub dataw1s4u: DATAW1SU,
    #[doc = "0x264 - Cache Data Storage (lower word)"]
    pub dataw1s4l: DATAW1SL,
    #[doc = "0x268 - Cache Data Storage (upper word)"]
    pub dataw1s5u: DATAW1SU,
    #[doc = "0x26c - Cache Data Storage (lower word)"]
    pub dataw1s5l: DATAW1SL,
    #[doc = "0x270 - Cache Data Storage (upper word)"]
    pub dataw1s6u: DATAW1SU,
    #[doc = "0x274 - Cache Data Storage (lower word)"]
    pub dataw1s6l: DATAW1SL,
    #[doc = "0x278 - Cache Data Storage (upper word)"]
    pub dataw1s7u: DATAW1SU,
    #[doc = "0x27c - Cache Data Storage (lower word)"]
    pub dataw1s7l: DATAW1SL,
    #[doc = "0x280 - Cache Data Storage (upper word)"]
    pub dataw2s0u: DATAW2SU,
    #[doc = "0x284 - Cache Data Storage (lower word)"]
    pub dataw2s0l: DATAW2SL,
    #[doc = "0x288 - Cache Data Storage (upper word)"]
    pub dataw2s1u: DATAW2SU,
    #[doc = "0x28c - Cache Data Storage (lower word)"]
    pub dataw2s1l: DATAW2SL,
    #[doc = "0x290 - Cache Data Storage (upper word)"]
    pub dataw2s2u: DATAW2SU,
    #[doc = "0x294 - Cache Data Storage (lower word)"]
    pub dataw2s2l: DATAW2SL,
    #[doc = "0x298 - Cache Data Storage (upper word)"]
    pub dataw2s3u: DATAW2SU,
    #[doc = "0x29c - Cache Data Storage (lower word)"]
    pub dataw2s3l: DATAW2SL,
    #[doc = "0x2a0 - Cache Data Storage (upper word)"]
    pub dataw2s4u: DATAW2SU,
    #[doc = "0x2a4 - Cache Data Storage (lower word)"]
    pub dataw2s4l: DATAW2SL,
    #[doc = "0x2a8 - Cache Data Storage (upper word)"]
    pub dataw2s5u: DATAW2SU,
    #[doc = "0x2ac - Cache Data Storage (lower word)"]
    pub dataw2s5l: DATAW2SL,
    #[doc = "0x2b0 - Cache Data Storage (upper word)"]
    pub dataw2s6u: DATAW2SU,
    #[doc = "0x2b4 - Cache Data Storage (lower word)"]
    pub dataw2s6l: DATAW2SL,
    #[doc = "0x2b8 - Cache Data Storage (upper word)"]
    pub dataw2s7u: DATAW2SU,
    #[doc = "0x2bc - Cache Data Storage (lower word)"]
    pub dataw2s7l: DATAW2SL,
    #[doc = "0x2c0 - Cache Data Storage (upper word)"]
    pub dataw3s0u: DATAW3SU,
    #[doc = "0x2c4 - Cache Data Storage (lower word)"]
    pub dataw3s0l: DATAW3SL,
    #[doc = "0x2c8 - Cache Data Storage (upper word)"]
    pub dataw3s1u: DATAW3SU,
    #[doc = "0x2cc - Cache Data Storage (lower word)"]
    pub dataw3s1l: DATAW3SL,
    #[doc = "0x2d0 - Cache Data Storage (upper word)"]
    pub dataw3s2u: DATAW3SU,
    #[doc = "0x2d4 - Cache Data Storage (lower word)"]
    pub dataw3s2l: DATAW3SL,
    #[doc = "0x2d8 - Cache Data Storage (upper word)"]
    pub dataw3s3u: DATAW3SU,
    #[doc = "0x2dc - Cache Data Storage (lower word)"]
    pub dataw3s3l: DATAW3SL,
    #[doc = "0x2e0 - Cache Data Storage (upper word)"]
    pub dataw3s4u: DATAW3SU,
    #[doc = "0x2e4 - Cache Data Storage (lower word)"]
    pub dataw3s4l: DATAW3SL,
    #[doc = "0x2e8 - Cache Data Storage (upper word)"]
    pub dataw3s5u: DATAW3SU,
    #[doc = "0x2ec - Cache Data Storage (lower word)"]
    pub dataw3s5l: DATAW3SL,
    #[doc = "0x2f0 - Cache Data Storage (upper word)"]
    pub dataw3s6u: DATAW3SU,
    #[doc = "0x2f4 - Cache Data Storage (lower word)"]
    pub dataw3s6l: DATAW3SL,
    #[doc = "0x2f8 - Cache Data Storage (upper word)"]
    pub dataw3s7u: DATAW3SU,
    #[doc = "0x2fc - Cache Data Storage (lower word)"]
    pub dataw3s7l: DATAW3SL,
}
#[doc = "Flash Access Protection Register"]
pub struct PFAPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Access Protection Register"]
pub mod pfapr;
#[doc = "Flash Bank 0 Control Register"]
pub struct PFB0CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 0 Control Register"]
pub mod pfb0cr;
#[doc = "Flash Bank 1 Control Register"]
pub struct PFB1CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 1 Control Register"]
pub mod pfb1cr;
#[doc = "Cache Tag Storage"]
pub struct TAGVDW0S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Tag Storage"]
pub mod tagvdw0s;
#[doc = "Cache Tag Storage"]
pub struct TAGVDW1S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Tag Storage"]
pub mod tagvdw1s;
#[doc = "Cache Tag Storage"]
pub struct TAGVDW2S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Tag Storage"]
pub mod tagvdw2s;
#[doc = "Cache Tag Storage"]
pub struct TAGVDW3S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Tag Storage"]
pub mod tagvdw3s;
#[doc = "Cache Data Storage (upper word)"]
pub struct DATAW0SU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (upper word)"]
pub mod dataw0su;
#[doc = "Cache Data Storage (lower word)"]
pub struct DATAW0SL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (lower word)"]
pub mod dataw0sl;
#[doc = "Cache Data Storage (upper word)"]
pub struct DATAW1SU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (upper word)"]
pub mod dataw1su;
#[doc = "Cache Data Storage (lower word)"]
pub struct DATAW1SL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (lower word)"]
pub mod dataw1sl;
#[doc = "Cache Data Storage (upper word)"]
pub struct DATAW2SU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (upper word)"]
pub mod dataw2su;
#[doc = "Cache Data Storage (lower word)"]
pub struct DATAW2SL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (lower word)"]
pub mod dataw2sl;
#[doc = "Cache Data Storage (upper word)"]
pub struct DATAW3SU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (upper word)"]
pub mod dataw3su;
#[doc = "Cache Data Storage (lower word)"]
pub struct DATAW3SL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (lower word)"]
pub mod dataw3sl;
