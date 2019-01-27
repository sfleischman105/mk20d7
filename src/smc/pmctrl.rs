#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PMCTRL {
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
#[doc = "Possible values of the field `STOPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPMR {
    #[doc = "Normal stop (STOP)"]
    _000,
    #[doc = "Reserved"]
    _001,
    #[doc = "Very low power stop (VLPS)"]
    _010,
    #[doc = "Low leakage stop (LLS)"]
    _011,
    #[doc = "Very low leakage stop (VLLSx)"]
    _100,
    #[doc = "Reserved"]
    _101,
    #[doc = "Reseved"]
    _110,
    #[doc = "Reserved"]
    _111,
}
impl STOPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STOPMR::_000 => 0,
            STOPMR::_001 => 1,
            STOPMR::_010 => 2,
            STOPMR::_011 => 3,
            STOPMR::_100 => 4,
            STOPMR::_101 => 5,
            STOPMR::_110 => 6,
            STOPMR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STOPMR {
        match value {
            0 => STOPMR::_000,
            1 => STOPMR::_001,
            2 => STOPMR::_010,
            3 => STOPMR::_011,
            4 => STOPMR::_100,
            5 => STOPMR::_101,
            6 => STOPMR::_110,
            7 => STOPMR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == STOPMR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == STOPMR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == STOPMR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == STOPMR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == STOPMR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == STOPMR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == STOPMR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == STOPMR::_111
    }
}
#[doc = "Possible values of the field `STOPA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPAR {
    #[doc = "The previous stop mode entry was successsful."]
    _0,
    #[doc = "The previous stop mode entry was aborted."]
    _1,
}
impl STOPAR {
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
            STOPAR::_0 => false,
            STOPAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPAR {
        match value {
            false => STOPAR::_0,
            true => STOPAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STOPAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STOPAR::_1
    }
}
#[doc = "Possible values of the field `RUNM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNMR {
    #[doc = "Normal run mode (RUN)"]
    _00,
    #[doc = "Reserved"]
    _01,
    #[doc = "Very low power run mode (VLPR)"]
    _10,
    #[doc = "Reserved"]
    _11,
}
impl RUNMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RUNMR::_00 => 0,
            RUNMR::_01 => 1,
            RUNMR::_10 => 2,
            RUNMR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RUNMR {
        match value {
            0 => RUNMR::_00,
            1 => RUNMR::_01,
            2 => RUNMR::_10,
            3 => RUNMR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RUNMR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RUNMR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == RUNMR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == RUNMR::_11
    }
}
#[doc = "Values that can be written to the field `STOPM`"]
pub enum STOPMW {
    #[doc = "Normal stop (STOP)"]
    _000,
    #[doc = "Reserved"]
    _001,
    #[doc = "Very low power stop (VLPS)"]
    _010,
    #[doc = "Low leakage stop (LLS)"]
    _011,
    #[doc = "Very low leakage stop (VLLSx)"]
    _100,
    #[doc = "Reserved"]
    _101,
    #[doc = "Reseved"]
    _110,
    #[doc = "Reserved"]
    _111,
}
impl STOPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STOPMW::_000 => 0,
            STOPMW::_001 => 1,
            STOPMW::_010 => 2,
            STOPMW::_011 => 3,
            STOPMW::_100 => 4,
            STOPMW::_101 => 5,
            STOPMW::_110 => 6,
            STOPMW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPMW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal stop (STOP)"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(STOPMW::_000)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(STOPMW::_001)
    }
    #[doc = "Very low power stop (VLPS)"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(STOPMW::_010)
    }
    #[doc = "Low leakage stop (LLS)"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(STOPMW::_011)
    }
    #[doc = "Very low leakage stop (VLLSx)"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(STOPMW::_100)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(STOPMW::_101)
    }
    #[doc = "Reseved"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(STOPMW::_110)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(STOPMW::_111)
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
#[doc = "Values that can be written to the field `RUNM`"]
pub enum RUNMW {
    #[doc = "Normal run mode (RUN)"]
    _00,
    #[doc = "Reserved"]
    _01,
    #[doc = "Very low power run mode (VLPR)"]
    _10,
    #[doc = "Reserved"]
    _11,
}
impl RUNMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RUNMW::_00 => 0,
            RUNMW::_01 => 1,
            RUNMW::_10 => 2,
            RUNMW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RUNMW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RUNMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal run mode (RUN)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RUNMW::_00)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RUNMW::_01)
    }
    #[doc = "Very low power run mode (VLPR)"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RUNMW::_10)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(RUNMW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:2 - Stop Mode Control"]
    #[inline]
    pub fn stopm(&self) -> STOPMR {
        STOPMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 3 - Stop Aborted"]
    #[inline]
    pub fn stopa(&self) -> STOPAR {
        STOPAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 5:6 - Run Mode Control"]
    #[inline]
    pub fn runm(&self) -> RUNMR {
        RUNMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    #[doc = "Bits 0:2 - Stop Mode Control"]
    #[inline]
    pub fn stopm(&mut self) -> _STOPMW {
        _STOPMW { w: self }
    }
    #[doc = "Bits 5:6 - Run Mode Control"]
    #[inline]
    pub fn runm(&mut self) -> _RUNMW {
        _RUNMW { w: self }
    }
}
