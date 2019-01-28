#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CNTR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CTN1R {
    bits: u16,
}
impl CTN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTNR {
    bits: u16,
}
impl CTNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - TouchSensing Channel n-1 16-bit counter value"]
    #[inline]
    pub fn ctn1(&self) -> CTN1R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CTN1R { bits }
    }
    #[doc = "Bits 16:31 - TouchSensing Channel n 16-bit counter value"]
    #[inline]
    pub fn ctn(&self) -> CTNR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CTNR { bits }
    }
}
