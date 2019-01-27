#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C5 {
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
#[doc = r" Value of the field"]
pub struct PRDIV0R {
    bits: u8,
}
impl PRDIV0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PLLSTEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSTEN0R {
    #[doc = "MCGPLLCLK is disabled in any of the Stop modes."]
    _0,
    #[doc = "MCGPLLCLK is enabled if system is in Normal Stop mode."]
    _1,
}
impl PLLSTEN0R {
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
            PLLSTEN0R::_0 => false,
            PLLSTEN0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLSTEN0R {
        match value {
            false => PLLSTEN0R::_0,
            true => PLLSTEN0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLLSTEN0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLLSTEN0R::_1
    }
}
#[doc = "Possible values of the field `PLLCLKEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLCLKEN0R {
    #[doc = "MCGPLLCLK is inactive."]
    _0,
    #[doc = "MCGPLLCLK is active."]
    _1,
}
impl PLLCLKEN0R {
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
            PLLCLKEN0R::_0 => false,
            PLLCLKEN0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLCLKEN0R {
        match value {
            false => PLLCLKEN0R::_0,
            true => PLLCLKEN0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLLCLKEN0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLLCLKEN0R::_1
    }
}
#[doc = r" Proxy"]
pub struct _PRDIV0W<'a> {
    w: &'a mut W,
}
impl<'a> _PRDIV0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLSTEN0`"]
pub enum PLLSTEN0W {
    #[doc = "MCGPLLCLK is disabled in any of the Stop modes."]
    _0,
    #[doc = "MCGPLLCLK is enabled if system is in Normal Stop mode."]
    _1,
}
impl PLLSTEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLSTEN0W::_0 => false,
            PLLSTEN0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLSTEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _PLLSTEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLSTEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCGPLLCLK is disabled in any of the Stop modes."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLSTEN0W::_0)
    }
    #[doc = "MCGPLLCLK is enabled if system is in Normal Stop mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLSTEN0W::_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLCLKEN0`"]
pub enum PLLCLKEN0W {
    #[doc = "MCGPLLCLK is inactive."]
    _0,
    #[doc = "MCGPLLCLK is active."]
    _1,
}
impl PLLCLKEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLCLKEN0W::_0 => false,
            PLLCLKEN0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLCLKEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _PLLCLKEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLCLKEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCGPLLCLK is inactive."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLCLKEN0W::_0)
    }
    #[doc = "MCGPLLCLK is active."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLCLKEN0W::_1)
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
        const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:4 - PLL External Reference Divider"]
    #[inline]
    pub fn prdiv0(&self) -> PRDIV0R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        PRDIV0R { bits }
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline]
    pub fn pllsten0(&self) -> PLLSTEN0R {
        PLLSTEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline]
    pub fn pllclken0(&self) -> PLLCLKEN0R {
        PLLCLKEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - PLL External Reference Divider"]
    #[inline]
    pub fn prdiv0(&mut self) -> _PRDIV0W {
        _PRDIV0W { w: self }
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline]
    pub fn pllsten0(&mut self) -> _PLLSTEN0W {
        _PLLSTEN0W { w: self }
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline]
    pub fn pllclken0(&mut self) -> _PLLCLKEN0W {
        _PLLCLKEN0W { w: self }
    }
}
