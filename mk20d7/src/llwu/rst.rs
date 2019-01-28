#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::RST {
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
#[doc = "Possible values of the field `RSTFILT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFILTR {
    #[doc = "Filter not enabled"]
    _0,
    #[doc = "Filter enabled"]
    _1,
}
impl RSTFILTR {
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
            RSTFILTR::_0 => false,
            RSTFILTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTFILTR {
        match value {
            false => RSTFILTR::_0,
            true => RSTFILTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RSTFILTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RSTFILTR::_1
    }
}
#[doc = "Possible values of the field `LLRSTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLRSTER {
    #[doc = "RESET pin not enabled as a leakage mode exit source"]
    _0,
    #[doc = "RESET pin enabled as a low leakage mode exit source"]
    _1,
}
impl LLRSTER {
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
            LLRSTER::_0 => false,
            LLRSTER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LLRSTER {
        match value {
            false => LLRSTER::_0,
            true => LLRSTER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LLRSTER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LLRSTER::_1
    }
}
#[doc = "Values that can be written to the field `RSTFILT`"]
pub enum RSTFILTW {
    #[doc = "Filter not enabled"]
    _0,
    #[doc = "Filter enabled"]
    _1,
}
impl RSTFILTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTFILTW::_0 => false,
            RSTFILTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTFILTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTFILTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTFILTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Filter not enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTFILTW::_0)
    }
    #[doc = "Filter enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTFILTW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LLRSTE`"]
pub enum LLRSTEW {
    #[doc = "RESET pin not enabled as a leakage mode exit source"]
    _0,
    #[doc = "RESET pin enabled as a low leakage mode exit source"]
    _1,
}
impl LLRSTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LLRSTEW::_0 => false,
            LLRSTEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LLRSTEW<'a> {
    w: &'a mut W,
}
impl<'a> _LLRSTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LLRSTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RESET pin not enabled as a leakage mode exit source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LLRSTEW::_0)
    }
    #[doc = "RESET pin enabled as a low leakage mode exit source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LLRSTEW::_1)
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
    #[doc = "Bit 0 - Digital Filter on RESET Pin"]
    #[inline]
    pub fn rstfilt(&self) -> RSTFILTR {
        RSTFILTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Low Leakage mode RESET enable"]
    #[inline]
    pub fn llrste(&self) -> LLRSTER {
        LLRSTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Digital Filter on RESET Pin"]
    #[inline]
    pub fn rstfilt(&mut self) -> _RSTFILTW {
        _RSTFILTW { w: self }
    }
    #[doc = "Bit 1 - Low Leakage mode RESET enable"]
    #[inline]
    pub fn llrste(&mut self) -> _LLRSTEW {
        _LLRSTEW { w: self }
    }
}
