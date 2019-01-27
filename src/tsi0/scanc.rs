#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCANC {
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
#[doc = "Possible values of the field `AMPSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMPSCR {
    #[doc = "Input Clock Source divided by 1."]
    _000,
    #[doc = "Input Clock Source divided by 2."]
    _001,
    #[doc = "Input Clock Source divided by 4."]
    _010,
    #[doc = "Input Clock Source divided by 8."]
    _011,
    #[doc = "Input Clock Source divided by 16."]
    _100,
    #[doc = "Input Clock Source divided by 32."]
    _101,
    #[doc = "Input Clock Source divided by 64."]
    _110,
    #[doc = "Input Clock Source divided by 128."]
    _111,
}
impl AMPSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AMPSCR::_000 => 0,
            AMPSCR::_001 => 1,
            AMPSCR::_010 => 2,
            AMPSCR::_011 => 3,
            AMPSCR::_100 => 4,
            AMPSCR::_101 => 5,
            AMPSCR::_110 => 6,
            AMPSCR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AMPSCR {
        match value {
            0 => AMPSCR::_000,
            1 => AMPSCR::_001,
            2 => AMPSCR::_010,
            3 => AMPSCR::_011,
            4 => AMPSCR::_100,
            5 => AMPSCR::_101,
            6 => AMPSCR::_110,
            7 => AMPSCR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == AMPSCR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == AMPSCR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == AMPSCR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == AMPSCR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == AMPSCR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == AMPSCR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == AMPSCR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == AMPSCR::_111
    }
}
#[doc = "Possible values of the field `AMCLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMCLKSR {
    #[doc = "LPOSCCLK"]
    _00,
    #[doc = "MCGIRCLK."]
    _01,
    #[doc = "OSCERCLK."]
    _10,
    #[doc = "Not valid."]
    _11,
}
impl AMCLKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AMCLKSR::_00 => 0,
            AMCLKSR::_01 => 1,
            AMCLKSR::_10 => 2,
            AMCLKSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AMCLKSR {
        match value {
            0 => AMCLKSR::_00,
            1 => AMCLKSR::_01,
            2 => AMCLKSR::_10,
            3 => AMCLKSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == AMCLKSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == AMCLKSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == AMCLKSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == AMCLKSR::_11
    }
}
#[doc = "Possible values of the field `SMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMODR {
    #[doc = "Continue Scan."]
    _00000000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMODR::_00000000 => 0,
            SMODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMODR {
        match value {
            0 => SMODR::_00000000,
            i => SMODR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00000000`"]
    #[inline]
    pub fn is_00000000(&self) -> bool {
        *self == SMODR::_00000000
    }
}
#[doc = "Possible values of the field `EXTCHRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTCHRGR {
    #[doc = "2 uA charge current."]
    _0000,
    #[doc = "4 uA charge current."]
    _0001,
    #[doc = "6 uA charge current."]
    _0010,
    #[doc = "8 uA charge current."]
    _0011,
    #[doc = "10 uA charge current."]
    _0100,
    #[doc = "12 uA charge current."]
    _0101,
    #[doc = "14 uA charge current."]
    _0110,
    #[doc = "16 uA charge current."]
    _0111,
    #[doc = "18 uA charge current."]
    _1000,
    #[doc = "20 uA charge current."]
    _1001,
    #[doc = "22 uA charge current."]
    _1010,
    #[doc = "24 uA charge current."]
    _1011,
    #[doc = "26 uA charge current."]
    _1100,
    #[doc = "28 uA charge current."]
    _1101,
    #[doc = "30 uA charge current."]
    _1110,
    #[doc = "32 uA charge current."]
    _1111,
}
impl EXTCHRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTCHRGR::_0000 => 0,
            EXTCHRGR::_0001 => 1,
            EXTCHRGR::_0010 => 2,
            EXTCHRGR::_0011 => 3,
            EXTCHRGR::_0100 => 4,
            EXTCHRGR::_0101 => 5,
            EXTCHRGR::_0110 => 6,
            EXTCHRGR::_0111 => 7,
            EXTCHRGR::_1000 => 8,
            EXTCHRGR::_1001 => 9,
            EXTCHRGR::_1010 => 10,
            EXTCHRGR::_1011 => 11,
            EXTCHRGR::_1100 => 12,
            EXTCHRGR::_1101 => 13,
            EXTCHRGR::_1110 => 14,
            EXTCHRGR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTCHRGR {
        match value {
            0 => EXTCHRGR::_0000,
            1 => EXTCHRGR::_0001,
            2 => EXTCHRGR::_0010,
            3 => EXTCHRGR::_0011,
            4 => EXTCHRGR::_0100,
            5 => EXTCHRGR::_0101,
            6 => EXTCHRGR::_0110,
            7 => EXTCHRGR::_0111,
            8 => EXTCHRGR::_1000,
            9 => EXTCHRGR::_1001,
            10 => EXTCHRGR::_1010,
            11 => EXTCHRGR::_1011,
            12 => EXTCHRGR::_1100,
            13 => EXTCHRGR::_1101,
            14 => EXTCHRGR::_1110,
            15 => EXTCHRGR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == EXTCHRGR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == EXTCHRGR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == EXTCHRGR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == EXTCHRGR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == EXTCHRGR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == EXTCHRGR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == EXTCHRGR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == EXTCHRGR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == EXTCHRGR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == EXTCHRGR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == EXTCHRGR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == EXTCHRGR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == EXTCHRGR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == EXTCHRGR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == EXTCHRGR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == EXTCHRGR::_1111
    }
}
#[doc = "Possible values of the field `REFCHRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFCHRGR {
    #[doc = "2 uA charge current."]
    _0000,
    #[doc = "4 uA charge current."]
    _0001,
    #[doc = "6 uA charge current."]
    _0010,
    #[doc = "8 uA charge current."]
    _0011,
    #[doc = "10 uA charge current."]
    _0100,
    #[doc = "12 uA charge current."]
    _0101,
    #[doc = "14 uA charge current."]
    _0110,
    #[doc = "16 uA charge current."]
    _0111,
    #[doc = "18 uA charge current."]
    _1000,
    #[doc = "20 uA charge current."]
    _1001,
    #[doc = "22 uA charge current."]
    _1010,
    #[doc = "24 uA charge current."]
    _1011,
    #[doc = "26 uA charge current."]
    _1100,
    #[doc = "28 uA charge current."]
    _1101,
    #[doc = "30 uA charge current."]
    _1110,
    #[doc = "32 uA charge current."]
    _1111,
}
impl REFCHRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFCHRGR::_0000 => 0,
            REFCHRGR::_0001 => 1,
            REFCHRGR::_0010 => 2,
            REFCHRGR::_0011 => 3,
            REFCHRGR::_0100 => 4,
            REFCHRGR::_0101 => 5,
            REFCHRGR::_0110 => 6,
            REFCHRGR::_0111 => 7,
            REFCHRGR::_1000 => 8,
            REFCHRGR::_1001 => 9,
            REFCHRGR::_1010 => 10,
            REFCHRGR::_1011 => 11,
            REFCHRGR::_1100 => 12,
            REFCHRGR::_1101 => 13,
            REFCHRGR::_1110 => 14,
            REFCHRGR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFCHRGR {
        match value {
            0 => REFCHRGR::_0000,
            1 => REFCHRGR::_0001,
            2 => REFCHRGR::_0010,
            3 => REFCHRGR::_0011,
            4 => REFCHRGR::_0100,
            5 => REFCHRGR::_0101,
            6 => REFCHRGR::_0110,
            7 => REFCHRGR::_0111,
            8 => REFCHRGR::_1000,
            9 => REFCHRGR::_1001,
            10 => REFCHRGR::_1010,
            11 => REFCHRGR::_1011,
            12 => REFCHRGR::_1100,
            13 => REFCHRGR::_1101,
            14 => REFCHRGR::_1110,
            15 => REFCHRGR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == REFCHRGR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == REFCHRGR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == REFCHRGR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == REFCHRGR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == REFCHRGR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == REFCHRGR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == REFCHRGR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == REFCHRGR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == REFCHRGR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == REFCHRGR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == REFCHRGR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == REFCHRGR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == REFCHRGR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == REFCHRGR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == REFCHRGR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == REFCHRGR::_1111
    }
}
#[doc = "Values that can be written to the field `AMPSC`"]
pub enum AMPSCW {
    #[doc = "Input Clock Source divided by 1."]
    _000,
    #[doc = "Input Clock Source divided by 2."]
    _001,
    #[doc = "Input Clock Source divided by 4."]
    _010,
    #[doc = "Input Clock Source divided by 8."]
    _011,
    #[doc = "Input Clock Source divided by 16."]
    _100,
    #[doc = "Input Clock Source divided by 32."]
    _101,
    #[doc = "Input Clock Source divided by 64."]
    _110,
    #[doc = "Input Clock Source divided by 128."]
    _111,
}
impl AMPSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AMPSCW::_000 => 0,
            AMPSCW::_001 => 1,
            AMPSCW::_010 => 2,
            AMPSCW::_011 => 3,
            AMPSCW::_100 => 4,
            AMPSCW::_101 => 5,
            AMPSCW::_110 => 6,
            AMPSCW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AMPSCW<'a> {
    w: &'a mut W,
}
impl<'a> _AMPSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AMPSCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input Clock Source divided by 1."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(AMPSCW::_000)
    }
    #[doc = "Input Clock Source divided by 2."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(AMPSCW::_001)
    }
    #[doc = "Input Clock Source divided by 4."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(AMPSCW::_010)
    }
    #[doc = "Input Clock Source divided by 8."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(AMPSCW::_011)
    }
    #[doc = "Input Clock Source divided by 16."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(AMPSCW::_100)
    }
    #[doc = "Input Clock Source divided by 32."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(AMPSCW::_101)
    }
    #[doc = "Input Clock Source divided by 64."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(AMPSCW::_110)
    }
    #[doc = "Input Clock Source divided by 128."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(AMPSCW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AMCLKS`"]
pub enum AMCLKSW {
    #[doc = "LPOSCCLK"]
    _00,
    #[doc = "MCGIRCLK."]
    _01,
    #[doc = "OSCERCLK."]
    _10,
    #[doc = "Not valid."]
    _11,
}
impl AMCLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AMCLKSW::_00 => 0,
            AMCLKSW::_01 => 1,
            AMCLKSW::_10 => 2,
            AMCLKSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AMCLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _AMCLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AMCLKSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LPOSCCLK"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(AMCLKSW::_00)
    }
    #[doc = "MCGIRCLK."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(AMCLKSW::_01)
    }
    #[doc = "OSCERCLK."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(AMCLKSW::_10)
    }
    #[doc = "Not valid."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(AMCLKSW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMOD`"]
pub enum SMODW {
    #[doc = "Continue Scan."]
    _00000000,
}
impl SMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMODW::_00000000 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMODW<'a> {
    w: &'a mut W,
}
impl<'a> _SMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Continue Scan."]
    #[inline]
    pub fn _00000000(self) -> &'a mut W {
        self.variant(SMODW::_00000000)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTCHRG`"]
pub enum EXTCHRGW {
    #[doc = "2 uA charge current."]
    _0000,
    #[doc = "4 uA charge current."]
    _0001,
    #[doc = "6 uA charge current."]
    _0010,
    #[doc = "8 uA charge current."]
    _0011,
    #[doc = "10 uA charge current."]
    _0100,
    #[doc = "12 uA charge current."]
    _0101,
    #[doc = "14 uA charge current."]
    _0110,
    #[doc = "16 uA charge current."]
    _0111,
    #[doc = "18 uA charge current."]
    _1000,
    #[doc = "20 uA charge current."]
    _1001,
    #[doc = "22 uA charge current."]
    _1010,
    #[doc = "24 uA charge current."]
    _1011,
    #[doc = "26 uA charge current."]
    _1100,
    #[doc = "28 uA charge current."]
    _1101,
    #[doc = "30 uA charge current."]
    _1110,
    #[doc = "32 uA charge current."]
    _1111,
}
impl EXTCHRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTCHRGW::_0000 => 0,
            EXTCHRGW::_0001 => 1,
            EXTCHRGW::_0010 => 2,
            EXTCHRGW::_0011 => 3,
            EXTCHRGW::_0100 => 4,
            EXTCHRGW::_0101 => 5,
            EXTCHRGW::_0110 => 6,
            EXTCHRGW::_0111 => 7,
            EXTCHRGW::_1000 => 8,
            EXTCHRGW::_1001 => 9,
            EXTCHRGW::_1010 => 10,
            EXTCHRGW::_1011 => 11,
            EXTCHRGW::_1100 => 12,
            EXTCHRGW::_1101 => 13,
            EXTCHRGW::_1110 => 14,
            EXTCHRGW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTCHRGW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTCHRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTCHRGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "2 uA charge current."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(EXTCHRGW::_0000)
    }
    #[doc = "4 uA charge current."]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(EXTCHRGW::_0001)
    }
    #[doc = "6 uA charge current."]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(EXTCHRGW::_0010)
    }
    #[doc = "8 uA charge current."]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(EXTCHRGW::_0011)
    }
    #[doc = "10 uA charge current."]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(EXTCHRGW::_0100)
    }
    #[doc = "12 uA charge current."]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(EXTCHRGW::_0101)
    }
    #[doc = "14 uA charge current."]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(EXTCHRGW::_0110)
    }
    #[doc = "16 uA charge current."]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(EXTCHRGW::_0111)
    }
    #[doc = "18 uA charge current."]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(EXTCHRGW::_1000)
    }
    #[doc = "20 uA charge current."]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(EXTCHRGW::_1001)
    }
    #[doc = "22 uA charge current."]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(EXTCHRGW::_1010)
    }
    #[doc = "24 uA charge current."]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(EXTCHRGW::_1011)
    }
    #[doc = "26 uA charge current."]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(EXTCHRGW::_1100)
    }
    #[doc = "28 uA charge current."]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(EXTCHRGW::_1101)
    }
    #[doc = "30 uA charge current."]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(EXTCHRGW::_1110)
    }
    #[doc = "32 uA charge current."]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(EXTCHRGW::_1111)
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
#[doc = "Values that can be written to the field `REFCHRG`"]
pub enum REFCHRGW {
    #[doc = "2 uA charge current."]
    _0000,
    #[doc = "4 uA charge current."]
    _0001,
    #[doc = "6 uA charge current."]
    _0010,
    #[doc = "8 uA charge current."]
    _0011,
    #[doc = "10 uA charge current."]
    _0100,
    #[doc = "12 uA charge current."]
    _0101,
    #[doc = "14 uA charge current."]
    _0110,
    #[doc = "16 uA charge current."]
    _0111,
    #[doc = "18 uA charge current."]
    _1000,
    #[doc = "20 uA charge current."]
    _1001,
    #[doc = "22 uA charge current."]
    _1010,
    #[doc = "24 uA charge current."]
    _1011,
    #[doc = "26 uA charge current."]
    _1100,
    #[doc = "28 uA charge current."]
    _1101,
    #[doc = "30 uA charge current."]
    _1110,
    #[doc = "32 uA charge current."]
    _1111,
}
impl REFCHRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFCHRGW::_0000 => 0,
            REFCHRGW::_0001 => 1,
            REFCHRGW::_0010 => 2,
            REFCHRGW::_0011 => 3,
            REFCHRGW::_0100 => 4,
            REFCHRGW::_0101 => 5,
            REFCHRGW::_0110 => 6,
            REFCHRGW::_0111 => 7,
            REFCHRGW::_1000 => 8,
            REFCHRGW::_1001 => 9,
            REFCHRGW::_1010 => 10,
            REFCHRGW::_1011 => 11,
            REFCHRGW::_1100 => 12,
            REFCHRGW::_1101 => 13,
            REFCHRGW::_1110 => 14,
            REFCHRGW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFCHRGW<'a> {
    w: &'a mut W,
}
impl<'a> _REFCHRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFCHRGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "2 uA charge current."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(REFCHRGW::_0000)
    }
    #[doc = "4 uA charge current."]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(REFCHRGW::_0001)
    }
    #[doc = "6 uA charge current."]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(REFCHRGW::_0010)
    }
    #[doc = "8 uA charge current."]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(REFCHRGW::_0011)
    }
    #[doc = "10 uA charge current."]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(REFCHRGW::_0100)
    }
    #[doc = "12 uA charge current."]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(REFCHRGW::_0101)
    }
    #[doc = "14 uA charge current."]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(REFCHRGW::_0110)
    }
    #[doc = "16 uA charge current."]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(REFCHRGW::_0111)
    }
    #[doc = "18 uA charge current."]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(REFCHRGW::_1000)
    }
    #[doc = "20 uA charge current."]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(REFCHRGW::_1001)
    }
    #[doc = "22 uA charge current."]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(REFCHRGW::_1010)
    }
    #[doc = "24 uA charge current."]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(REFCHRGW::_1011)
    }
    #[doc = "26 uA charge current."]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(REFCHRGW::_1100)
    }
    #[doc = "28 uA charge current."]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(REFCHRGW::_1101)
    }
    #[doc = "30 uA charge current."]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(REFCHRGW::_1110)
    }
    #[doc = "32 uA charge current."]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(REFCHRGW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:2 - Active Mode Prescaler"]
    #[inline]
    pub fn ampsc(&self) -> AMPSCR {
        AMPSCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:4 - Active Mode Clock Source"]
    #[inline]
    pub fn amclks(&self) -> AMCLKSR {
        AMCLKSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Scan Module"]
    #[inline]
    pub fn smod(&self) -> SMODR {
        SMODR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - External OSC Charge Current select"]
    #[inline]
    pub fn extchrg(&self) -> EXTCHRGR {
        EXTCHRGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Ref OSC Charge Current select"]
    #[inline]
    pub fn refchrg(&self) -> REFCHRGR {
        REFCHRGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:2 - Active Mode Prescaler"]
    #[inline]
    pub fn ampsc(&mut self) -> _AMPSCW {
        _AMPSCW { w: self }
    }
    #[doc = "Bits 3:4 - Active Mode Clock Source"]
    #[inline]
    pub fn amclks(&mut self) -> _AMCLKSW {
        _AMCLKSW { w: self }
    }
    #[doc = "Bits 8:15 - Scan Module"]
    #[inline]
    pub fn smod(&mut self) -> _SMODW {
        _SMODW { w: self }
    }
    #[doc = "Bits 16:19 - External OSC Charge Current select"]
    #[inline]
    pub fn extchrg(&mut self) -> _EXTCHRGW {
        _EXTCHRGW { w: self }
    }
    #[doc = "Bits 24:27 - Ref OSC Charge Current select"]
    #[inline]
    pub fn refchrg(&mut self) -> _REFCHRGW {
        _REFCHRGW { w: self }
    }
}
