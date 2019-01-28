#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SDID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PINID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINIDR {
    #[doc = "Reserved"]
    _0000,
    #[doc = "Reserved"]
    _0001,
    #[doc = "Reserved"]
    _0010,
    #[doc = "Reserved"]
    _0011,
    #[doc = "Reserved"]
    _0100,
    #[doc = "64-pin"]
    _0101,
    #[doc = "80-pin"]
    _0110,
    #[doc = "81-pin"]
    _0111,
    #[doc = "100-pin"]
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
impl PINIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINIDR::_0000 => 0,
            PINIDR::_0001 => 1,
            PINIDR::_0010 => 2,
            PINIDR::_0011 => 3,
            PINIDR::_0100 => 4,
            PINIDR::_0101 => 5,
            PINIDR::_0110 => 6,
            PINIDR::_0111 => 7,
            PINIDR::_1000 => 8,
            PINIDR::_1001 => 9,
            PINIDR::_1010 => 10,
            PINIDR::_1011 => 11,
            PINIDR::_1100 => 12,
            PINIDR::_1101 => 13,
            PINIDR::_1110 => 14,
            PINIDR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINIDR {
        match value {
            0 => PINIDR::_0000,
            1 => PINIDR::_0001,
            2 => PINIDR::_0010,
            3 => PINIDR::_0011,
            4 => PINIDR::_0100,
            5 => PINIDR::_0101,
            6 => PINIDR::_0110,
            7 => PINIDR::_0111,
            8 => PINIDR::_1000,
            9 => PINIDR::_1001,
            10 => PINIDR::_1010,
            11 => PINIDR::_1011,
            12 => PINIDR::_1100,
            13 => PINIDR::_1101,
            14 => PINIDR::_1110,
            15 => PINIDR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == PINIDR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == PINIDR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == PINIDR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == PINIDR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == PINIDR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == PINIDR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == PINIDR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == PINIDR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == PINIDR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == PINIDR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == PINIDR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == PINIDR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == PINIDR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == PINIDR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == PINIDR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == PINIDR::_1111
    }
}
#[doc = "Possible values of the field `FAMID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAMIDR {
    #[doc = "K10"]
    _000,
    #[doc = "K20"]
    _001,
    #[doc = "K30"]
    _010,
    #[doc = "K40"]
    _011,
    #[doc = "Reserved"]
    _100,
    #[doc = "Reserved"]
    _101,
    #[doc = "K50"]
    _110,
    #[doc = "K51"]
    _111,
}
impl FAMIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FAMIDR::_000 => 0,
            FAMIDR::_001 => 1,
            FAMIDR::_010 => 2,
            FAMIDR::_011 => 3,
            FAMIDR::_100 => 4,
            FAMIDR::_101 => 5,
            FAMIDR::_110 => 6,
            FAMIDR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FAMIDR {
        match value {
            0 => FAMIDR::_000,
            1 => FAMIDR::_001,
            2 => FAMIDR::_010,
            3 => FAMIDR::_011,
            4 => FAMIDR::_100,
            5 => FAMIDR::_101,
            6 => FAMIDR::_110,
            7 => FAMIDR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == FAMIDR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == FAMIDR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == FAMIDR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == FAMIDR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == FAMIDR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == FAMIDR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == FAMIDR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == FAMIDR::_111
    }
}
#[doc = r" Value of the field"]
pub struct REVIDR {
    bits: u8,
}
impl REVIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Pincount identification"]
    #[inline]
    pub fn pinid(&self) -> PINIDR {
        PINIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Kinetis family identification"]
    #[inline]
    pub fn famid(&self) -> FAMIDR {
        FAMIDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Device revision number"]
    #[inline]
    pub fn revid(&self) -> REVIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REVIDR { bits }
    }
}
