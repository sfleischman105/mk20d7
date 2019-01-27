#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PGA {
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
#[doc = "Possible values of the field `PGAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGAGR {
    #[doc = "1"]
    _0000,
    #[doc = "2"]
    _0001,
    #[doc = "4"]
    _0010,
    #[doc = "8"]
    _0011,
    #[doc = "16"]
    _0100,
    #[doc = "32"]
    _0101,
    #[doc = "64"]
    _0110,
    #[doc = "Reserved"]
    _0111,
    #[doc = "Reserved"]
    _1000,
    #[doc = "Reserved"]
    _1001,
    #[doc = "Reserved"]
    _1010,
    #[doc = "Reserved"]
    _1011,
    #[doc = "Reserved"]
    _1100,
    #[doc = "Reserved"]
    _1101,
    #[doc = "Reserved"]
    _1110,
    #[doc = "Reserved"]
    _1111,
}
impl PGAGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PGAGR::_0000 => 0,
            PGAGR::_0001 => 1,
            PGAGR::_0010 => 2,
            PGAGR::_0011 => 3,
            PGAGR::_0100 => 4,
            PGAGR::_0101 => 5,
            PGAGR::_0110 => 6,
            PGAGR::_0111 => 7,
            PGAGR::_1000 => 8,
            PGAGR::_1001 => 9,
            PGAGR::_1010 => 10,
            PGAGR::_1011 => 11,
            PGAGR::_1100 => 12,
            PGAGR::_1101 => 13,
            PGAGR::_1110 => 14,
            PGAGR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PGAGR {
        match value {
            0 => PGAGR::_0000,
            1 => PGAGR::_0001,
            2 => PGAGR::_0010,
            3 => PGAGR::_0011,
            4 => PGAGR::_0100,
            5 => PGAGR::_0101,
            6 => PGAGR::_0110,
            7 => PGAGR::_0111,
            8 => PGAGR::_1000,
            9 => PGAGR::_1001,
            10 => PGAGR::_1010,
            11 => PGAGR::_1011,
            12 => PGAGR::_1100,
            13 => PGAGR::_1101,
            14 => PGAGR::_1110,
            15 => PGAGR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == PGAGR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == PGAGR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == PGAGR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == PGAGR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == PGAGR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == PGAGR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == PGAGR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == PGAGR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == PGAGR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == PGAGR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == PGAGR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == PGAGR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == PGAGR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == PGAGR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == PGAGR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == PGAGR::_1111
    }
}
#[doc = "Possible values of the field `PGALPb`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGALPBR {
    #[doc = "PGA runs in low power mode."]
    _0,
    #[doc = "PGA runs in normal power mode."]
    _1,
}
impl PGALPBR {
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
            PGALPBR::_0 => false,
            PGALPBR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PGALPBR {
        match value {
            false => PGALPBR::_0,
            true => PGALPBR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PGALPBR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PGALPBR::_1
    }
}
#[doc = "Possible values of the field `PGAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGAENR {
    #[doc = "PGA disabled."]
    _0,
    #[doc = "PGA enabled."]
    _1,
}
impl PGAENR {
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
            PGAENR::_0 => false,
            PGAENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PGAENR {
        match value {
            false => PGAENR::_0,
            true => PGAENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PGAENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PGAENR::_1
    }
}
#[doc = "Values that can be written to the field `PGAG`"]
pub enum PGAGW {
    #[doc = "1"]
    _0000,
    #[doc = "2"]
    _0001,
    #[doc = "4"]
    _0010,
    #[doc = "8"]
    _0011,
    #[doc = "16"]
    _0100,
    #[doc = "32"]
    _0101,
    #[doc = "64"]
    _0110,
    #[doc = "Reserved"]
    _0111,
    #[doc = "Reserved"]
    _1000,
    #[doc = "Reserved"]
    _1001,
    #[doc = "Reserved"]
    _1010,
    #[doc = "Reserved"]
    _1011,
    #[doc = "Reserved"]
    _1100,
    #[doc = "Reserved"]
    _1101,
    #[doc = "Reserved"]
    _1110,
    #[doc = "Reserved"]
    _1111,
}
impl PGAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PGAGW::_0000 => 0,
            PGAGW::_0001 => 1,
            PGAGW::_0010 => 2,
            PGAGW::_0011 => 3,
            PGAGW::_0100 => 4,
            PGAGW::_0101 => 5,
            PGAGW::_0110 => 6,
            PGAGW::_0111 => 7,
            PGAGW::_1000 => 8,
            PGAGW::_1001 => 9,
            PGAGW::_1010 => 10,
            PGAGW::_1011 => 11,
            PGAGW::_1100 => 12,
            PGAGW::_1101 => 13,
            PGAGW::_1110 => 14,
            PGAGW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PGAGW<'a> {
    w: &'a mut W,
}
impl<'a> _PGAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PGAGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(PGAGW::_0000)
    }
    #[doc = "2"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(PGAGW::_0001)
    }
    #[doc = "4"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(PGAGW::_0010)
    }
    #[doc = "8"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(PGAGW::_0011)
    }
    #[doc = "16"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(PGAGW::_0100)
    }
    #[doc = "32"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(PGAGW::_0101)
    }
    #[doc = "64"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(PGAGW::_0110)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(PGAGW::_0111)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(PGAGW::_1000)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(PGAGW::_1001)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(PGAGW::_1010)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(PGAGW::_1011)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(PGAGW::_1100)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(PGAGW::_1101)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(PGAGW::_1110)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(PGAGW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PGALPb`"]
pub enum PGALPBW {
    #[doc = "PGA runs in low power mode."]
    _0,
    #[doc = "PGA runs in normal power mode."]
    _1,
}
impl PGALPBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PGALPBW::_0 => false,
            PGALPBW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PGALPBW<'a> {
    w: &'a mut W,
}
impl<'a> _PGALPBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PGALPBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PGA runs in low power mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGALPBW::_0)
    }
    #[doc = "PGA runs in normal power mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGALPBW::_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PGAEN`"]
pub enum PGAENW {
    #[doc = "PGA disabled."]
    _0,
    #[doc = "PGA enabled."]
    _1,
}
impl PGAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PGAENW::_0 => false,
            PGAENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PGAENW<'a> {
    w: &'a mut W,
}
impl<'a> _PGAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PGAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PGA disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGAENW::_0)
    }
    #[doc = "PGA enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGAENW::_1)
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bits 16:19 - PGA gain setting"]
    #[inline]
    pub fn pgag(&self) -> PGAGR {
        PGAGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - PGA low-power mode control"]
    #[inline]
    pub fn pgalpb(&self) -> PGALPBR {
        PGALPBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - PGA enable"]
    #[inline]
    pub fn pgaen(&self) -> PGAENR {
        PGAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 16:19 - PGA gain setting"]
    #[inline]
    pub fn pgag(&mut self) -> _PGAGW {
        _PGAGW { w: self }
    }
    #[doc = "Bit 20 - PGA low-power mode control"]
    #[inline]
    pub fn pgalpb(&mut self) -> _PGALPBW {
        _PGALPBW { w: self }
    }
    #[doc = "Bit 23 - PGA enable"]
    #[inline]
    pub fn pgaen(&mut self) -> _PGAENW {
        _PGAENW { w: self }
    }
}
