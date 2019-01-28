#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::VLLSCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `VLLSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLLSMR {
    #[doc = "Reserved"]
    _000,
    #[doc = "VLLS1"]
    _001,
    #[doc = "VLLS2"]
    _010,
    #[doc = "VLLS3"]
    _011,
    #[doc = "Reserved"]
    _100,
    #[doc = "Reserved"]
    _101,
    #[doc = "Reserved"]
    _110,
    #[doc = "Reserved"]
    _111,
}
impl VLLSMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VLLSMR::_000 => 0,
            VLLSMR::_001 => 1,
            VLLSMR::_010 => 2,
            VLLSMR::_011 => 3,
            VLLSMR::_100 => 4,
            VLLSMR::_101 => 5,
            VLLSMR::_110 => 6,
            VLLSMR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VLLSMR {
        match value {
            0 => VLLSMR::_000,
            1 => VLLSMR::_001,
            2 => VLLSMR::_010,
            3 => VLLSMR::_011,
            4 => VLLSMR::_100,
            5 => VLLSMR::_101,
            6 => VLLSMR::_110,
            7 => VLLSMR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == VLLSMR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == VLLSMR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == VLLSMR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == VLLSMR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == VLLSMR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == VLLSMR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == VLLSMR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == VLLSMR::_111
    }
}
#[doc = "Values that can be written to the field `VLLSM`"]
pub enum VLLSMW {
    #[doc = "Reserved"]
    _000,
    #[doc = "VLLS1"]
    _001,
    #[doc = "VLLS2"]
    _010,
    #[doc = "VLLS3"]
    _011,
    #[doc = "Reserved"]
    _100,
    #[doc = "Reserved"]
    _101,
    #[doc = "Reserved"]
    _110,
    #[doc = "Reserved"]
    _111,
}
impl VLLSMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VLLSMW::_000 => 0,
            VLLSMW::_001 => 1,
            VLLSMW::_010 => 2,
            VLLSMW::_011 => 3,
            VLLSMW::_100 => 4,
            VLLSMW::_101 => 5,
            VLLSMW::_110 => 6,
            VLLSMW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VLLSMW<'a> {
    w: &'a mut W,
}
impl<'a> _VLLSMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VLLSMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(VLLSMW::_000)
    }
    #[doc = "VLLS1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(VLLSMW::_001)
    }
    #[doc = "VLLS2"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(VLLSMW::_010)
    }
    #[doc = "VLLS3"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(VLLSMW::_011)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(VLLSMW::_100)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(VLLSMW::_101)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(VLLSMW::_110)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(VLLSMW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:2 - VLLS Mode Control"]
    #[inline]
    pub fn vllsm(&self) -> VLLSMR {
        VLLSMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - VLLS Mode Control"]
    #[inline]
    pub fn vllsm(&mut self) -> _VLLSMW {
        _VLLSMW { w: self }
    }
}
