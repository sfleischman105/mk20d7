#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::FSEC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SECR {
    bits: u8,
}
impl SECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FSLACCR {
    bits: u8,
}
impl FSLACCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MEENR {
    bits: u8,
}
impl MEENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct KEYENR {
    bits: u8,
}
impl KEYENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Flash Security"]
    #[inline]
    pub fn sec(&self) -> SECR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        SECR { bits }
    }
    #[doc = "Bits 2:3 - Freescale Failure Analysis Access Code"]
    #[inline]
    pub fn fslacc(&self) -> FSLACCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        FSLACCR { bits }
    }
    #[doc = "Bits 4:5 - no description available"]
    #[inline]
    pub fn meen(&self) -> MEENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        MEENR { bits }
    }
    #[doc = "Bits 6:7 - Backdoor Key Security Enable"]
    #[inline]
    pub fn keyen(&self) -> KEYENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        KEYENR { bits }
    }
}
