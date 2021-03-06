#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFSR {
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
#[doc = "Possible values of the field `VECTTBL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTTBLR {
    #[doc = "no BusFault on vector table read"]
    _0,
    #[doc = "BusFault on vector table read"]
    _1,
}
impl VECTTBLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VECTTBLR::_0 => false,
            VECTTBLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VECTTBLR {
        match value {
            false => VECTTBLR::_0,
            true => VECTTBLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VECTTBLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VECTTBLR::_1
    }
}
#[doc = "Possible values of the field `FORCED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEDR {
    #[doc = "no forced HardFault"]
    _0,
    #[doc = "forced HardFault"]
    _1,
}
impl FORCEDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FORCEDR::_0 => false,
            FORCEDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORCEDR {
        match value {
            false => FORCEDR::_0,
            true => FORCEDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FORCEDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FORCEDR::_1
    }
}
#[doc = r" Value of the field"]
pub struct DEBUGEVTR {
    bits: bool,
}
impl DEBUGEVTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `VECTTBL`"]
pub enum VECTTBLW {
    #[doc = "no BusFault on vector table read"]
    _0,
    #[doc = "BusFault on vector table read"]
    _1,
}
impl VECTTBLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VECTTBLW::_0 => false,
            VECTTBLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VECTTBLW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTTBLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VECTTBLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no BusFault on vector table read"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(VECTTBLW::_0)
    }
    #[doc = "BusFault on vector table read"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(VECTTBLW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FORCED`"]
pub enum FORCEDW {
    #[doc = "no forced HardFault"]
    _0,
    #[doc = "forced HardFault"]
    _1,
}
impl FORCEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FORCEDW::_0 => false,
            FORCEDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORCEDW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORCEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no forced HardFault"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FORCEDW::_0)
    }
    #[doc = "forced HardFault"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FORCEDW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEBUGEVTW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUGEVTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn vecttbl(&self) -> VECTTBLR {
        VECTTBLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - no description available"]
    #[inline]
    pub fn forced(&self) -> FORCEDR {
        FORCEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - no description available"]
    #[inline]
    pub fn debugevt(&self) -> DEBUGEVTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBUGEVTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn vecttbl(&mut self) -> _VECTTBLW {
        _VECTTBLW { w: self }
    }
    #[doc = "Bit 30 - no description available"]
    #[inline]
    pub fn forced(&mut self) -> _FORCEDW {
        _FORCEDW { w: self }
    }
    #[doc = "Bit 31 - no description available"]
    #[inline]
    pub fn debugevt(&mut self) -> _DEBUGEVTW {
        _DEBUGEVTW { w: self }
    }
}
