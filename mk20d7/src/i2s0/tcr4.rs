#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCR4 {
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
#[doc = "Possible values of the field `FSD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSDR {
    #[doc = "Frame Sync is generated externally (slave mode)."]
    _0,
    #[doc = "Frame Sync is generated internally (master mode)."]
    _1,
}
impl FSDR {
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
            FSDR::_0 => false,
            FSDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSDR {
        match value {
            false => FSDR::_0,
            true => FSDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FSDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FSDR::_1
    }
}
#[doc = "Possible values of the field `FSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSPR {
    #[doc = "Frame sync is active high."]
    _0,
    #[doc = "Frame sync is active low."]
    _1,
}
impl FSPR {
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
            FSPR::_0 => false,
            FSPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSPR {
        match value {
            false => FSPR::_0,
            true => FSPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FSPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FSPR::_1
    }
}
#[doc = "Possible values of the field `FSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSER {
    #[doc = "Frame sync asserts with the first bit of the frame."]
    _0,
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    _1,
}
impl FSER {
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
            FSER::_0 => false,
            FSER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSER {
        match value {
            false => FSER::_0,
            true => FSER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FSER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FSER::_1
    }
}
#[doc = "Possible values of the field `MF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MFR {
    #[doc = "LBS is transmitted/received first."]
    _0,
    #[doc = "MBS is transmitted/received first."]
    _1,
}
impl MFR {
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
            MFR::_0 => false,
            MFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MFR {
        match value {
            false => MFR::_0,
            true => MFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MFR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SYWDR {
    bits: u8,
}
impl SYWDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FRSZR {
    bits: u8,
}
impl FRSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `FSD`"]
pub enum FSDW {
    #[doc = "Frame Sync is generated externally (slave mode)."]
    _0,
    #[doc = "Frame Sync is generated internally (master mode)."]
    _1,
}
impl FSDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSDW::_0 => false,
            FSDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSDW<'a> {
    w: &'a mut W,
}
impl<'a> _FSDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frame Sync is generated externally (slave mode)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSDW::_0)
    }
    #[doc = "Frame Sync is generated internally (master mode)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSDW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FSP`"]
pub enum FSPW {
    #[doc = "Frame sync is active high."]
    _0,
    #[doc = "Frame sync is active low."]
    _1,
}
impl FSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSPW::_0 => false,
            FSPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSPW<'a> {
    w: &'a mut W,
}
impl<'a> _FSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frame sync is active high."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSPW::_0)
    }
    #[doc = "Frame sync is active low."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSPW::_1)
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
#[doc = "Values that can be written to the field `FSE`"]
pub enum FSEW {
    #[doc = "Frame sync asserts with the first bit of the frame."]
    _0,
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    _1,
}
impl FSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSEW::_0 => false,
            FSEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSEW<'a> {
    w: &'a mut W,
}
impl<'a> _FSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frame sync asserts with the first bit of the frame."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSEW::_0)
    }
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSEW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MF`"]
pub enum MFW {
    #[doc = "LBS is transmitted/received first."]
    _0,
    #[doc = "MBS is transmitted/received first."]
    _1,
}
impl MFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MFW::_0 => false,
            MFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MFW<'a> {
    w: &'a mut W,
}
impl<'a> _MFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LBS is transmitted/received first."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MFW::_0)
    }
    #[doc = "MBS is transmitted/received first."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MFW::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYWDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYWDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRSZW<'a> {
    w: &'a mut W,
}
impl<'a> _FRSZW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Frame sync direction"]
    #[inline]
    pub fn fsd(&self) -> FSDR {
        FSDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Frame sync polarity"]
    #[inline]
    pub fn fsp(&self) -> FSPR {
        FSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Frame sync early"]
    #[inline]
    pub fn fse(&self) -> FSER {
        FSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - MSB first"]
    #[inline]
    pub fn mf(&self) -> MFR {
        MFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:12 - Sync width"]
    #[inline]
    pub fn sywd(&self) -> SYWDR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYWDR { bits }
    }
    #[doc = "Bits 16:20 - Frame size"]
    #[inline]
    pub fn frsz(&self) -> FRSZR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FRSZR { bits }
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
    #[doc = "Bit 0 - Frame sync direction"]
    #[inline]
    pub fn fsd(&mut self) -> _FSDW {
        _FSDW { w: self }
    }
    #[doc = "Bit 1 - Frame sync polarity"]
    #[inline]
    pub fn fsp(&mut self) -> _FSPW {
        _FSPW { w: self }
    }
    #[doc = "Bit 3 - Frame sync early"]
    #[inline]
    pub fn fse(&mut self) -> _FSEW {
        _FSEW { w: self }
    }
    #[doc = "Bit 4 - MSB first"]
    #[inline]
    pub fn mf(&mut self) -> _MFW {
        _MFW { w: self }
    }
    #[doc = "Bits 8:12 - Sync width"]
    #[inline]
    pub fn sywd(&mut self) -> _SYWDW {
        _SYWDW { w: self }
    }
    #[doc = "Bits 16:20 - Frame size"]
    #[inline]
    pub fn frsz(&mut self) -> _FRSZW {
        _FRSZW { w: self }
    }
}
