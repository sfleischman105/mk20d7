#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GENCS {
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
#[doc = "Possible values of the field `STPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPER {
    #[doc = "Disable TSI when MCU goes into low power modes."]
    _0,
    #[doc = "Allows TSI to continue running in all low power modes."]
    _1,
}
impl STPER {
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
            STPER::_0 => false,
            STPER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STPER {
        match value {
            false => STPER::_0,
            true => STPER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STPER::_1
    }
}
#[doc = "Possible values of the field `STM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STMR {
    #[doc = "Software trigger scan."]
    _0,
    #[doc = "Periodical Scan."]
    _1,
}
impl STMR {
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
            STMR::_0 => false,
            STMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STMR {
        match value {
            false => STMR::_0,
            true => STMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STMR::_1
    }
}
#[doc = "Possible values of the field `ESOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESORR {
    #[doc = "Out-of-Range interrupt is allowed."]
    _0,
    #[doc = "End-of-Scan interrupt is allowed."]
    _1,
}
impl ESORR {
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
            ESORR::_0 => false,
            ESORR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESORR {
        match value {
            false => ESORR::_0,
            true => ESORR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ESORR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ESORR::_1
    }
}
#[doc = "Possible values of the field `ERIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERIER {
    #[doc = "Interrupt disabled for error."]
    _0,
    #[doc = "Interrupt enabled for error."]
    _1,
}
impl ERIER {
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
            ERIER::_0 => false,
            ERIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERIER {
        match value {
            false => ERIER::_0,
            true => ERIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERIER::_1
    }
}
#[doc = "Possible values of the field `TSIIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIIER {
    #[doc = "Interrupt from TSI is disabled"]
    _0,
    #[doc = "Interrupt from TSI is enabled"]
    _1,
}
impl TSIIER {
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
            TSIIER::_0 => false,
            TSIIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSIIER {
        match value {
            false => TSIIER::_0,
            true => TSIIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSIIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSIIER::_1
    }
}
#[doc = "Possible values of the field `TSIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIENR {
    #[doc = "TSI module is disabled"]
    _0,
    #[doc = "TSI module is enabled"]
    _1,
}
impl TSIENR {
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
            TSIENR::_0 => false,
            TSIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSIENR {
        match value {
            false => TSIENR::_0,
            true => TSIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSIENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SCNIPR {
    bits: bool,
}
impl SCNIPR {
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
#[doc = "Possible values of the field `OVRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRFR {
    #[doc = "No over run."]
    _0,
    #[doc = "Over Run occurred."]
    _1,
}
impl OVRFR {
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
            OVRFR::_0 => false,
            OVRFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVRFR {
        match value {
            false => OVRFR::_0,
            true => OVRFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OVRFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OVRFR::_1
    }
}
#[doc = "Possible values of the field `EXTERF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTERFR {
    #[doc = "No fault happend on TSI electrodes"]
    _0,
    #[doc = "Short to VDD or VSS was detected on one or more electrodes."]
    _1,
}
impl EXTERFR {
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
            EXTERFR::_0 => false,
            EXTERFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTERFR {
        match value {
            false => EXTERFR::_0,
            true => EXTERFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EXTERFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EXTERFR::_1
    }
}
#[doc = r" Value of the field"]
pub struct OUTRGFR {
    bits: bool,
}
impl OUTRGFR {
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
#[doc = r" Value of the field"]
pub struct EOSFR {
    bits: bool,
}
impl EOSFR {
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
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "Electrode Oscillator Frequency divided by 1"]
    _000,
    #[doc = "Electrode Oscillator Frequency divided by 2"]
    _001,
    #[doc = "Electrode Oscillator Frequency divided by 4"]
    _010,
    #[doc = "Electrode Oscillator Frequency divided by 8"]
    _011,
    #[doc = "Electrode Oscillator Frequency divided by 16"]
    _100,
    #[doc = "Electrode Oscillator Frequency divided by 32"]
    _101,
    #[doc = "Electrode Oscillator Frequency divided by 64"]
    _110,
    #[doc = "Electrode Oscillator Frequency divided by 128"]
    _111,
}
impl PSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSR::_000 => 0,
            PSR::_001 => 1,
            PSR::_010 => 2,
            PSR::_011 => 3,
            PSR::_100 => 4,
            PSR::_101 => 5,
            PSR::_110 => 6,
            PSR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSR {
        match value {
            0 => PSR::_000,
            1 => PSR::_001,
            2 => PSR::_010,
            3 => PSR::_011,
            4 => PSR::_100,
            5 => PSR::_101,
            6 => PSR::_110,
            7 => PSR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PSR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PSR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PSR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PSR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PSR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PSR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PSR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == PSR::_111
    }
}
#[doc = "Possible values of the field `NSCN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSCNR {
    #[doc = "Once per electrode"]
    _00000,
    #[doc = "Twice per electrode"]
    _00001,
    #[doc = "3 times per electrode"]
    _00010,
    #[doc = "4 times per electrode"]
    _00011,
    #[doc = "5 times per electrode"]
    _00100,
    #[doc = "6 times per electrode"]
    _00101,
    #[doc = "7 times per electrode"]
    _00110,
    #[doc = "8 times per electrode"]
    _00111,
    #[doc = "9 times per electrode"]
    _01000,
    #[doc = "10 times per electrode"]
    _01001,
    #[doc = "11 times per electrode"]
    _01010,
    #[doc = "12 times per electrode"]
    _01011,
    #[doc = "13 times per electrode"]
    _01100,
    #[doc = "14 times per electrode"]
    _01101,
    #[doc = "15 times per electrode"]
    _01110,
    #[doc = "16 times per electrode"]
    _01111,
    #[doc = "17 times per electrode"]
    _10000,
    #[doc = "18 times per electrode"]
    _10001,
    #[doc = "19 times per electrode"]
    _10010,
    #[doc = "20 times per electrode"]
    _10011,
    #[doc = "21 times per electrode"]
    _10100,
    #[doc = "22 times per electrode"]
    _10101,
    #[doc = "23 times per electrode"]
    _10110,
    #[doc = "24 times per electrode"]
    _10111,
    #[doc = "25 times per electrode"]
    _11000,
    #[doc = "26 times per electrode"]
    _11001,
    #[doc = "27 times per electrode"]
    _11010,
    #[doc = "28 times per electrode"]
    _11011,
    #[doc = "29 times per electrode"]
    _11100,
    #[doc = "30 times per electrode"]
    _11101,
    #[doc = "31 times per electrode"]
    _11110,
    #[doc = "32 times per electrode"]
    _11111,
}
impl NSCNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NSCNR::_00000 => 0,
            NSCNR::_00001 => 1,
            NSCNR::_00010 => 2,
            NSCNR::_00011 => 3,
            NSCNR::_00100 => 4,
            NSCNR::_00101 => 5,
            NSCNR::_00110 => 6,
            NSCNR::_00111 => 7,
            NSCNR::_01000 => 8,
            NSCNR::_01001 => 9,
            NSCNR::_01010 => 10,
            NSCNR::_01011 => 11,
            NSCNR::_01100 => 12,
            NSCNR::_01101 => 13,
            NSCNR::_01110 => 14,
            NSCNR::_01111 => 15,
            NSCNR::_10000 => 16,
            NSCNR::_10001 => 17,
            NSCNR::_10010 => 18,
            NSCNR::_10011 => 19,
            NSCNR::_10100 => 20,
            NSCNR::_10101 => 21,
            NSCNR::_10110 => 22,
            NSCNR::_10111 => 23,
            NSCNR::_11000 => 24,
            NSCNR::_11001 => 25,
            NSCNR::_11010 => 26,
            NSCNR::_11011 => 27,
            NSCNR::_11100 => 28,
            NSCNR::_11101 => 29,
            NSCNR::_11110 => 30,
            NSCNR::_11111 => 31,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NSCNR {
        match value {
            0 => NSCNR::_00000,
            1 => NSCNR::_00001,
            2 => NSCNR::_00010,
            3 => NSCNR::_00011,
            4 => NSCNR::_00100,
            5 => NSCNR::_00101,
            6 => NSCNR::_00110,
            7 => NSCNR::_00111,
            8 => NSCNR::_01000,
            9 => NSCNR::_01001,
            10 => NSCNR::_01010,
            11 => NSCNR::_01011,
            12 => NSCNR::_01100,
            13 => NSCNR::_01101,
            14 => NSCNR::_01110,
            15 => NSCNR::_01111,
            16 => NSCNR::_10000,
            17 => NSCNR::_10001,
            18 => NSCNR::_10010,
            19 => NSCNR::_10011,
            20 => NSCNR::_10100,
            21 => NSCNR::_10101,
            22 => NSCNR::_10110,
            23 => NSCNR::_10111,
            24 => NSCNR::_11000,
            25 => NSCNR::_11001,
            26 => NSCNR::_11010,
            27 => NSCNR::_11011,
            28 => NSCNR::_11100,
            29 => NSCNR::_11101,
            30 => NSCNR::_11110,
            31 => NSCNR::_11111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline]
    pub fn is_00000(&self) -> bool {
        *self == NSCNR::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline]
    pub fn is_00001(&self) -> bool {
        *self == NSCNR::_00001
    }
    #[doc = "Checks if the value of the field is `_00010`"]
    #[inline]
    pub fn is_00010(&self) -> bool {
        *self == NSCNR::_00010
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline]
    pub fn is_00011(&self) -> bool {
        *self == NSCNR::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline]
    pub fn is_00100(&self) -> bool {
        *self == NSCNR::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline]
    pub fn is_00101(&self) -> bool {
        *self == NSCNR::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline]
    pub fn is_00110(&self) -> bool {
        *self == NSCNR::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline]
    pub fn is_00111(&self) -> bool {
        *self == NSCNR::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline]
    pub fn is_01000(&self) -> bool {
        *self == NSCNR::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline]
    pub fn is_01001(&self) -> bool {
        *self == NSCNR::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline]
    pub fn is_01010(&self) -> bool {
        *self == NSCNR::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline]
    pub fn is_01011(&self) -> bool {
        *self == NSCNR::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline]
    pub fn is_01100(&self) -> bool {
        *self == NSCNR::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline]
    pub fn is_01101(&self) -> bool {
        *self == NSCNR::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline]
    pub fn is_01110(&self) -> bool {
        *self == NSCNR::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline]
    pub fn is_01111(&self) -> bool {
        *self == NSCNR::_01111
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline]
    pub fn is_10000(&self) -> bool {
        *self == NSCNR::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline]
    pub fn is_10001(&self) -> bool {
        *self == NSCNR::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline]
    pub fn is_10010(&self) -> bool {
        *self == NSCNR::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline]
    pub fn is_10011(&self) -> bool {
        *self == NSCNR::_10011
    }
    #[doc = "Checks if the value of the field is `_10100`"]
    #[inline]
    pub fn is_10100(&self) -> bool {
        *self == NSCNR::_10100
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline]
    pub fn is_10101(&self) -> bool {
        *self == NSCNR::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline]
    pub fn is_10110(&self) -> bool {
        *self == NSCNR::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline]
    pub fn is_10111(&self) -> bool {
        *self == NSCNR::_10111
    }
    #[doc = "Checks if the value of the field is `_11000`"]
    #[inline]
    pub fn is_11000(&self) -> bool {
        *self == NSCNR::_11000
    }
    #[doc = "Checks if the value of the field is `_11001`"]
    #[inline]
    pub fn is_11001(&self) -> bool {
        *self == NSCNR::_11001
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline]
    pub fn is_11010(&self) -> bool {
        *self == NSCNR::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline]
    pub fn is_11011(&self) -> bool {
        *self == NSCNR::_11011
    }
    #[doc = "Checks if the value of the field is `_11100`"]
    #[inline]
    pub fn is_11100(&self) -> bool {
        *self == NSCNR::_11100
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline]
    pub fn is_11101(&self) -> bool {
        *self == NSCNR::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline]
    pub fn is_11110(&self) -> bool {
        *self == NSCNR::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline]
    pub fn is_11111(&self) -> bool {
        *self == NSCNR::_11111
    }
}
#[doc = "Possible values of the field `LPSCNITV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSCNITVR {
    #[doc = "1 ms scan interval"]
    _0000,
    #[doc = "5 ms scan interval"]
    _0001,
    #[doc = "10 ms scan interval"]
    _0010,
    #[doc = "15 ms scan interval"]
    _0011,
    #[doc = "20 ms scan interval"]
    _0100,
    #[doc = "30 ms scan interval"]
    _0101,
    #[doc = "40 ms scan interval"]
    _0110,
    #[doc = "50 ms scan interval"]
    _0111,
    #[doc = "75 ms scan interval"]
    _1000,
    #[doc = "100 ms scan interval"]
    _1001,
    #[doc = "125 ms scan interval"]
    _1010,
    #[doc = "150 ms scan interval"]
    _1011,
    #[doc = "200 ms scan interval"]
    _1100,
    #[doc = "300 ms scan interval"]
    _1101,
    #[doc = "400 ms scan interval"]
    _1110,
    #[doc = "500 ms scan interval"]
    _1111,
}
impl LPSCNITVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPSCNITVR::_0000 => 0,
            LPSCNITVR::_0001 => 1,
            LPSCNITVR::_0010 => 2,
            LPSCNITVR::_0011 => 3,
            LPSCNITVR::_0100 => 4,
            LPSCNITVR::_0101 => 5,
            LPSCNITVR::_0110 => 6,
            LPSCNITVR::_0111 => 7,
            LPSCNITVR::_1000 => 8,
            LPSCNITVR::_1001 => 9,
            LPSCNITVR::_1010 => 10,
            LPSCNITVR::_1011 => 11,
            LPSCNITVR::_1100 => 12,
            LPSCNITVR::_1101 => 13,
            LPSCNITVR::_1110 => 14,
            LPSCNITVR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPSCNITVR {
        match value {
            0 => LPSCNITVR::_0000,
            1 => LPSCNITVR::_0001,
            2 => LPSCNITVR::_0010,
            3 => LPSCNITVR::_0011,
            4 => LPSCNITVR::_0100,
            5 => LPSCNITVR::_0101,
            6 => LPSCNITVR::_0110,
            7 => LPSCNITVR::_0111,
            8 => LPSCNITVR::_1000,
            9 => LPSCNITVR::_1001,
            10 => LPSCNITVR::_1010,
            11 => LPSCNITVR::_1011,
            12 => LPSCNITVR::_1100,
            13 => LPSCNITVR::_1101,
            14 => LPSCNITVR::_1110,
            15 => LPSCNITVR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == LPSCNITVR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == LPSCNITVR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == LPSCNITVR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == LPSCNITVR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == LPSCNITVR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == LPSCNITVR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == LPSCNITVR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == LPSCNITVR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == LPSCNITVR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == LPSCNITVR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == LPSCNITVR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == LPSCNITVR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == LPSCNITVR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == LPSCNITVR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == LPSCNITVR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == LPSCNITVR::_1111
    }
}
#[doc = "Possible values of the field `LPCLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCLKSR {
    #[doc = "LPOCLK is selected to determine the scan period in low power mode"]
    _0,
    #[doc = "VLPOSCCLK is selected to determine the scan period in low power mode"]
    _1,
}
impl LPCLKSR {
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
            LPCLKSR::_0 => false,
            LPCLKSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPCLKSR {
        match value {
            false => LPCLKSR::_0,
            true => LPCLKSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPCLKSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPCLKSR::_1
    }
}
#[doc = "Values that can be written to the field `STPE`"]
pub enum STPEW {
    #[doc = "Disable TSI when MCU goes into low power modes."]
    _0,
    #[doc = "Allows TSI to continue running in all low power modes."]
    _1,
}
impl STPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STPEW::_0 => false,
            STPEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STPEW<'a> {
    w: &'a mut W,
}
impl<'a> _STPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable TSI when MCU goes into low power modes."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STPEW::_0)
    }
    #[doc = "Allows TSI to continue running in all low power modes."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STPEW::_1)
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
#[doc = "Values that can be written to the field `STM`"]
pub enum STMW {
    #[doc = "Software trigger scan."]
    _0,
    #[doc = "Periodical Scan."]
    _1,
}
impl STMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STMW::_0 => false,
            STMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STMW<'a> {
    w: &'a mut W,
}
impl<'a> _STMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software trigger scan."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STMW::_0)
    }
    #[doc = "Periodical Scan."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STMW::_1)
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
#[doc = "Values that can be written to the field `ESOR`"]
pub enum ESORW {
    #[doc = "Out-of-Range interrupt is allowed."]
    _0,
    #[doc = "End-of-Scan interrupt is allowed."]
    _1,
}
impl ESORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESORW::_0 => false,
            ESORW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESORW<'a> {
    w: &'a mut W,
}
impl<'a> _ESORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Out-of-Range interrupt is allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESORW::_0)
    }
    #[doc = "End-of-Scan interrupt is allowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESORW::_1)
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
#[doc = "Values that can be written to the field `ERIE`"]
pub enum ERIEW {
    #[doc = "Interrupt disabled for error."]
    _0,
    #[doc = "Interrupt enabled for error."]
    _1,
}
impl ERIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERIEW::_0 => false,
            ERIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ERIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled for error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERIEW::_0)
    }
    #[doc = "Interrupt enabled for error."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERIEW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSIIE`"]
pub enum TSIIEW {
    #[doc = "Interrupt from TSI is disabled"]
    _0,
    #[doc = "Interrupt from TSI is enabled"]
    _1,
}
impl TSIIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSIIEW::_0 => false,
            TSIIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSIIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSIIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt from TSI is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSIIEW::_0)
    }
    #[doc = "Interrupt from TSI is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSIIEW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSIEN`"]
pub enum TSIENW {
    #[doc = "TSI module is disabled"]
    _0,
    #[doc = "TSI module is enabled"]
    _1,
}
impl TSIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSIENW::_0 => false,
            TSIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSIENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TSI module is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSIENW::_0)
    }
    #[doc = "TSI module is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSIENW::_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWTSW<'a> {
    w: &'a mut W,
}
impl<'a> _SWTSW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OVRF`"]
pub enum OVRFW {
    #[doc = "No over run."]
    _0,
    #[doc = "Over Run occurred."]
    _1,
}
impl OVRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVRFW::_0 => false,
            OVRFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVRFW<'a> {
    w: &'a mut W,
}
impl<'a> _OVRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVRFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No over run."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVRFW::_0)
    }
    #[doc = "Over Run occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVRFW::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTERF`"]
pub enum EXTERFW {
    #[doc = "No fault happend on TSI electrodes"]
    _0,
    #[doc = "Short to VDD or VSS was detected on one or more electrodes."]
    _1,
}
impl EXTERFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTERFW::_0 => false,
            EXTERFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTERFW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTERFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTERFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No fault happend on TSI electrodes"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTERFW::_0)
    }
    #[doc = "Short to VDD or VSS was detected on one or more electrodes."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTERFW::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUTRGFW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTRGFW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EOSFW<'a> {
    w: &'a mut W,
}
impl<'a> _EOSFW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PS`"]
pub enum PSW {
    #[doc = "Electrode Oscillator Frequency divided by 1"]
    _000,
    #[doc = "Electrode Oscillator Frequency divided by 2"]
    _001,
    #[doc = "Electrode Oscillator Frequency divided by 4"]
    _010,
    #[doc = "Electrode Oscillator Frequency divided by 8"]
    _011,
    #[doc = "Electrode Oscillator Frequency divided by 16"]
    _100,
    #[doc = "Electrode Oscillator Frequency divided by 32"]
    _101,
    #[doc = "Electrode Oscillator Frequency divided by 64"]
    _110,
    #[doc = "Electrode Oscillator Frequency divided by 128"]
    _111,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSW::_000 => 0,
            PSW::_001 => 1,
            PSW::_010 => 2,
            PSW::_011 => 3,
            PSW::_100 => 4,
            PSW::_101 => 5,
            PSW::_110 => 6,
            PSW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Electrode Oscillator Frequency divided by 1"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(PSW::_000)
    }
    #[doc = "Electrode Oscillator Frequency divided by 2"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PSW::_001)
    }
    #[doc = "Electrode Oscillator Frequency divided by 4"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PSW::_010)
    }
    #[doc = "Electrode Oscillator Frequency divided by 8"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PSW::_011)
    }
    #[doc = "Electrode Oscillator Frequency divided by 16"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PSW::_100)
    }
    #[doc = "Electrode Oscillator Frequency divided by 32"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PSW::_101)
    }
    #[doc = "Electrode Oscillator Frequency divided by 64"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PSW::_110)
    }
    #[doc = "Electrode Oscillator Frequency divided by 128"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(PSW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NSCN`"]
pub enum NSCNW {
    #[doc = "Once per electrode"]
    _00000,
    #[doc = "Twice per electrode"]
    _00001,
    #[doc = "3 times per electrode"]
    _00010,
    #[doc = "4 times per electrode"]
    _00011,
    #[doc = "5 times per electrode"]
    _00100,
    #[doc = "6 times per electrode"]
    _00101,
    #[doc = "7 times per electrode"]
    _00110,
    #[doc = "8 times per electrode"]
    _00111,
    #[doc = "9 times per electrode"]
    _01000,
    #[doc = "10 times per electrode"]
    _01001,
    #[doc = "11 times per electrode"]
    _01010,
    #[doc = "12 times per electrode"]
    _01011,
    #[doc = "13 times per electrode"]
    _01100,
    #[doc = "14 times per electrode"]
    _01101,
    #[doc = "15 times per electrode"]
    _01110,
    #[doc = "16 times per electrode"]
    _01111,
    #[doc = "17 times per electrode"]
    _10000,
    #[doc = "18 times per electrode"]
    _10001,
    #[doc = "19 times per electrode"]
    _10010,
    #[doc = "20 times per electrode"]
    _10011,
    #[doc = "21 times per electrode"]
    _10100,
    #[doc = "22 times per electrode"]
    _10101,
    #[doc = "23 times per electrode"]
    _10110,
    #[doc = "24 times per electrode"]
    _10111,
    #[doc = "25 times per electrode"]
    _11000,
    #[doc = "26 times per electrode"]
    _11001,
    #[doc = "27 times per electrode"]
    _11010,
    #[doc = "28 times per electrode"]
    _11011,
    #[doc = "29 times per electrode"]
    _11100,
    #[doc = "30 times per electrode"]
    _11101,
    #[doc = "31 times per electrode"]
    _11110,
    #[doc = "32 times per electrode"]
    _11111,
}
impl NSCNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NSCNW::_00000 => 0,
            NSCNW::_00001 => 1,
            NSCNW::_00010 => 2,
            NSCNW::_00011 => 3,
            NSCNW::_00100 => 4,
            NSCNW::_00101 => 5,
            NSCNW::_00110 => 6,
            NSCNW::_00111 => 7,
            NSCNW::_01000 => 8,
            NSCNW::_01001 => 9,
            NSCNW::_01010 => 10,
            NSCNW::_01011 => 11,
            NSCNW::_01100 => 12,
            NSCNW::_01101 => 13,
            NSCNW::_01110 => 14,
            NSCNW::_01111 => 15,
            NSCNW::_10000 => 16,
            NSCNW::_10001 => 17,
            NSCNW::_10010 => 18,
            NSCNW::_10011 => 19,
            NSCNW::_10100 => 20,
            NSCNW::_10101 => 21,
            NSCNW::_10110 => 22,
            NSCNW::_10111 => 23,
            NSCNW::_11000 => 24,
            NSCNW::_11001 => 25,
            NSCNW::_11010 => 26,
            NSCNW::_11011 => 27,
            NSCNW::_11100 => 28,
            NSCNW::_11101 => 29,
            NSCNW::_11110 => 30,
            NSCNW::_11111 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSCNW<'a> {
    w: &'a mut W,
}
impl<'a> _NSCNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSCNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Once per electrode"]
    #[inline]
    pub fn _00000(self) -> &'a mut W {
        self.variant(NSCNW::_00000)
    }
    #[doc = "Twice per electrode"]
    #[inline]
    pub fn _00001(self) -> &'a mut W {
        self.variant(NSCNW::_00001)
    }
    #[doc = "3 times per electrode"]
    #[inline]
    pub fn _00010(self) -> &'a mut W {
        self.variant(NSCNW::_00010)
    }
    #[doc = "4 times per electrode"]
    #[inline]
    pub fn _00011(self) -> &'a mut W {
        self.variant(NSCNW::_00011)
    }
    #[doc = "5 times per electrode"]
    #[inline]
    pub fn _00100(self) -> &'a mut W {
        self.variant(NSCNW::_00100)
    }
    #[doc = "6 times per electrode"]
    #[inline]
    pub fn _00101(self) -> &'a mut W {
        self.variant(NSCNW::_00101)
    }
    #[doc = "7 times per electrode"]
    #[inline]
    pub fn _00110(self) -> &'a mut W {
        self.variant(NSCNW::_00110)
    }
    #[doc = "8 times per electrode"]
    #[inline]
    pub fn _00111(self) -> &'a mut W {
        self.variant(NSCNW::_00111)
    }
    #[doc = "9 times per electrode"]
    #[inline]
    pub fn _01000(self) -> &'a mut W {
        self.variant(NSCNW::_01000)
    }
    #[doc = "10 times per electrode"]
    #[inline]
    pub fn _01001(self) -> &'a mut W {
        self.variant(NSCNW::_01001)
    }
    #[doc = "11 times per electrode"]
    #[inline]
    pub fn _01010(self) -> &'a mut W {
        self.variant(NSCNW::_01010)
    }
    #[doc = "12 times per electrode"]
    #[inline]
    pub fn _01011(self) -> &'a mut W {
        self.variant(NSCNW::_01011)
    }
    #[doc = "13 times per electrode"]
    #[inline]
    pub fn _01100(self) -> &'a mut W {
        self.variant(NSCNW::_01100)
    }
    #[doc = "14 times per electrode"]
    #[inline]
    pub fn _01101(self) -> &'a mut W {
        self.variant(NSCNW::_01101)
    }
    #[doc = "15 times per electrode"]
    #[inline]
    pub fn _01110(self) -> &'a mut W {
        self.variant(NSCNW::_01110)
    }
    #[doc = "16 times per electrode"]
    #[inline]
    pub fn _01111(self) -> &'a mut W {
        self.variant(NSCNW::_01111)
    }
    #[doc = "17 times per electrode"]
    #[inline]
    pub fn _10000(self) -> &'a mut W {
        self.variant(NSCNW::_10000)
    }
    #[doc = "18 times per electrode"]
    #[inline]
    pub fn _10001(self) -> &'a mut W {
        self.variant(NSCNW::_10001)
    }
    #[doc = "19 times per electrode"]
    #[inline]
    pub fn _10010(self) -> &'a mut W {
        self.variant(NSCNW::_10010)
    }
    #[doc = "20 times per electrode"]
    #[inline]
    pub fn _10011(self) -> &'a mut W {
        self.variant(NSCNW::_10011)
    }
    #[doc = "21 times per electrode"]
    #[inline]
    pub fn _10100(self) -> &'a mut W {
        self.variant(NSCNW::_10100)
    }
    #[doc = "22 times per electrode"]
    #[inline]
    pub fn _10101(self) -> &'a mut W {
        self.variant(NSCNW::_10101)
    }
    #[doc = "23 times per electrode"]
    #[inline]
    pub fn _10110(self) -> &'a mut W {
        self.variant(NSCNW::_10110)
    }
    #[doc = "24 times per electrode"]
    #[inline]
    pub fn _10111(self) -> &'a mut W {
        self.variant(NSCNW::_10111)
    }
    #[doc = "25 times per electrode"]
    #[inline]
    pub fn _11000(self) -> &'a mut W {
        self.variant(NSCNW::_11000)
    }
    #[doc = "26 times per electrode"]
    #[inline]
    pub fn _11001(self) -> &'a mut W {
        self.variant(NSCNW::_11001)
    }
    #[doc = "27 times per electrode"]
    #[inline]
    pub fn _11010(self) -> &'a mut W {
        self.variant(NSCNW::_11010)
    }
    #[doc = "28 times per electrode"]
    #[inline]
    pub fn _11011(self) -> &'a mut W {
        self.variant(NSCNW::_11011)
    }
    #[doc = "29 times per electrode"]
    #[inline]
    pub fn _11100(self) -> &'a mut W {
        self.variant(NSCNW::_11100)
    }
    #[doc = "30 times per electrode"]
    #[inline]
    pub fn _11101(self) -> &'a mut W {
        self.variant(NSCNW::_11101)
    }
    #[doc = "31 times per electrode"]
    #[inline]
    pub fn _11110(self) -> &'a mut W {
        self.variant(NSCNW::_11110)
    }
    #[doc = "32 times per electrode"]
    #[inline]
    pub fn _11111(self) -> &'a mut W {
        self.variant(NSCNW::_11111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPSCNITV`"]
pub enum LPSCNITVW {
    #[doc = "1 ms scan interval"]
    _0000,
    #[doc = "5 ms scan interval"]
    _0001,
    #[doc = "10 ms scan interval"]
    _0010,
    #[doc = "15 ms scan interval"]
    _0011,
    #[doc = "20 ms scan interval"]
    _0100,
    #[doc = "30 ms scan interval"]
    _0101,
    #[doc = "40 ms scan interval"]
    _0110,
    #[doc = "50 ms scan interval"]
    _0111,
    #[doc = "75 ms scan interval"]
    _1000,
    #[doc = "100 ms scan interval"]
    _1001,
    #[doc = "125 ms scan interval"]
    _1010,
    #[doc = "150 ms scan interval"]
    _1011,
    #[doc = "200 ms scan interval"]
    _1100,
    #[doc = "300 ms scan interval"]
    _1101,
    #[doc = "400 ms scan interval"]
    _1110,
    #[doc = "500 ms scan interval"]
    _1111,
}
impl LPSCNITVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPSCNITVW::_0000 => 0,
            LPSCNITVW::_0001 => 1,
            LPSCNITVW::_0010 => 2,
            LPSCNITVW::_0011 => 3,
            LPSCNITVW::_0100 => 4,
            LPSCNITVW::_0101 => 5,
            LPSCNITVW::_0110 => 6,
            LPSCNITVW::_0111 => 7,
            LPSCNITVW::_1000 => 8,
            LPSCNITVW::_1001 => 9,
            LPSCNITVW::_1010 => 10,
            LPSCNITVW::_1011 => 11,
            LPSCNITVW::_1100 => 12,
            LPSCNITVW::_1101 => 13,
            LPSCNITVW::_1110 => 14,
            LPSCNITVW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSCNITVW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSCNITVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSCNITVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 ms scan interval"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(LPSCNITVW::_0000)
    }
    #[doc = "5 ms scan interval"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(LPSCNITVW::_0001)
    }
    #[doc = "10 ms scan interval"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(LPSCNITVW::_0010)
    }
    #[doc = "15 ms scan interval"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(LPSCNITVW::_0011)
    }
    #[doc = "20 ms scan interval"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(LPSCNITVW::_0100)
    }
    #[doc = "30 ms scan interval"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(LPSCNITVW::_0101)
    }
    #[doc = "40 ms scan interval"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(LPSCNITVW::_0110)
    }
    #[doc = "50 ms scan interval"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(LPSCNITVW::_0111)
    }
    #[doc = "75 ms scan interval"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(LPSCNITVW::_1000)
    }
    #[doc = "100 ms scan interval"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(LPSCNITVW::_1001)
    }
    #[doc = "125 ms scan interval"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(LPSCNITVW::_1010)
    }
    #[doc = "150 ms scan interval"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(LPSCNITVW::_1011)
    }
    #[doc = "200 ms scan interval"]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(LPSCNITVW::_1100)
    }
    #[doc = "300 ms scan interval"]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(LPSCNITVW::_1101)
    }
    #[doc = "400 ms scan interval"]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(LPSCNITVW::_1110)
    }
    #[doc = "500 ms scan interval"]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(LPSCNITVW::_1111)
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
#[doc = "Values that can be written to the field `LPCLKS`"]
pub enum LPCLKSW {
    #[doc = "LPOCLK is selected to determine the scan period in low power mode"]
    _0,
    #[doc = "VLPOSCCLK is selected to determine the scan period in low power mode"]
    _1,
}
impl LPCLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPCLKSW::_0 => false,
            LPCLKSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPCLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _LPCLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPCLKSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LPOCLK is selected to determine the scan period in low power mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPCLKSW::_0)
    }
    #[doc = "VLPOSCCLK is selected to determine the scan period in low power mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPCLKSW::_1)
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn stpe(&self) -> STPER {
        STPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Scan Trigger Mode. This bit-field can only be changed if the TSI module is disabled (TSIEN bit = 0)."]
    #[inline]
    pub fn stm(&self) -> STMR {
        STMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - End-of-Scan or Out-of-Range Interrupt select"]
    #[inline]
    pub fn esor(&self) -> ESORR {
        ESORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Error Interrupt Enable"]
    #[inline]
    pub fn erie(&self) -> ERIER {
        ERIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Touch Sensing Input Interrupt Module Enable"]
    #[inline]
    pub fn tsiie(&self) -> TSIIER {
        TSIIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Touch Sensing Input Module Enable"]
    #[inline]
    pub fn tsien(&self) -> TSIENR {
        TSIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Scan In Progress status"]
    #[inline]
    pub fn scnip(&self) -> SCNIPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCNIPR { bits }
    }
    #[doc = "Bit 12 - Overrun error Flag. This flag is set when a scan trigger occurs while a scan is still in progress. Write \"1\", when this flag is set, to clear it."]
    #[inline]
    pub fn ovrf(&self) -> OVRFR {
        OVRFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - External Electrode error occurred"]
    #[inline]
    pub fn exterf(&self) -> EXTERFR {
        EXTERFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Out of Range Flag."]
    #[inline]
    pub fn outrgf(&self) -> OUTRGFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OUTRGFR { bits }
    }
    #[doc = "Bit 15 - End of Scan Flag."]
    #[inline]
    pub fn eosf(&self) -> EOSFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EOSFR { bits }
    }
    #[doc = "Bits 16:18 - Electrode Oscillator prescaler. ."]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:23 - Number of Consecutive Scans per Electrode electrode."]
    #[inline]
    pub fn nscn(&self) -> NSCNR {
        NSCNR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - TSI Low Power Mode Scan Interval."]
    #[inline]
    pub fn lpscnitv(&self) -> LPSCNITVR {
        LPSCNITVR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Low Power Mode Clock Source Selection."]
    #[inline]
    pub fn lpclks(&self) -> LPCLKSR {
        LPCLKSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn stpe(&mut self) -> _STPEW {
        _STPEW { w: self }
    }
    #[doc = "Bit 1 - Scan Trigger Mode. This bit-field can only be changed if the TSI module is disabled (TSIEN bit = 0)."]
    #[inline]
    pub fn stm(&mut self) -> _STMW {
        _STMW { w: self }
    }
    #[doc = "Bit 4 - End-of-Scan or Out-of-Range Interrupt select"]
    #[inline]
    pub fn esor(&mut self) -> _ESORW {
        _ESORW { w: self }
    }
    #[doc = "Bit 5 - Error Interrupt Enable"]
    #[inline]
    pub fn erie(&mut self) -> _ERIEW {
        _ERIEW { w: self }
    }
    #[doc = "Bit 6 - Touch Sensing Input Interrupt Module Enable"]
    #[inline]
    pub fn tsiie(&mut self) -> _TSIIEW {
        _TSIIEW { w: self }
    }
    #[doc = "Bit 7 - Touch Sensing Input Module Enable"]
    #[inline]
    pub fn tsien(&mut self) -> _TSIENW {
        _TSIENW { w: self }
    }
    #[doc = "Bit 8 - Software Trigger Start"]
    #[inline]
    pub fn swts(&mut self) -> _SWTSW {
        _SWTSW { w: self }
    }
    #[doc = "Bit 12 - Overrun error Flag. This flag is set when a scan trigger occurs while a scan is still in progress. Write \"1\", when this flag is set, to clear it."]
    #[inline]
    pub fn ovrf(&mut self) -> _OVRFW {
        _OVRFW { w: self }
    }
    #[doc = "Bit 13 - External Electrode error occurred"]
    #[inline]
    pub fn exterf(&mut self) -> _EXTERFW {
        _EXTERFW { w: self }
    }
    #[doc = "Bit 14 - Out of Range Flag."]
    #[inline]
    pub fn outrgf(&mut self) -> _OUTRGFW {
        _OUTRGFW { w: self }
    }
    #[doc = "Bit 15 - End of Scan Flag."]
    #[inline]
    pub fn eosf(&mut self) -> _EOSFW {
        _EOSFW { w: self }
    }
    #[doc = "Bits 16:18 - Electrode Oscillator prescaler. ."]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bits 19:23 - Number of Consecutive Scans per Electrode electrode."]
    #[inline]
    pub fn nscn(&mut self) -> _NSCNW {
        _NSCNW { w: self }
    }
    #[doc = "Bits 24:27 - TSI Low Power Mode Scan Interval."]
    #[inline]
    pub fn lpscnitv(&mut self) -> _LPSCNITVW {
        _LPSCNITVW { w: self }
    }
    #[doc = "Bit 28 - Low Power Mode Clock Source Selection."]
    #[inline]
    pub fn lpclks(&mut self) -> _LPCLKSW {
        _LPCLKSW { w: self }
    }
}
