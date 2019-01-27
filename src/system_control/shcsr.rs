#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHCSR {
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
#[doc = "Possible values of the field `MEMFAULTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTACTR {
    #[doc = "exception is not active"]
    _0,
    #[doc = "exception is active"]
    _1,
}
impl MEMFAULTACTR {
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
            MEMFAULTACTR::_0 => false,
            MEMFAULTACTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEMFAULTACTR {
        match value {
            false => MEMFAULTACTR::_0,
            true => MEMFAULTACTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MEMFAULTACTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MEMFAULTACTR::_1
    }
}
#[doc = "Possible values of the field `BUSFAULTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTACTR {
    #[doc = "exception is not active"]
    _0,
    #[doc = "exception is active"]
    _1,
}
impl BUSFAULTACTR {
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
            BUSFAULTACTR::_0 => false,
            BUSFAULTACTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSFAULTACTR {
        match value {
            false => BUSFAULTACTR::_0,
            true => BUSFAULTACTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUSFAULTACTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUSFAULTACTR::_1
    }
}
#[doc = "Possible values of the field `USGFAULTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTACTR {
    #[doc = "exception is not active"]
    _0,
    #[doc = "exception is active"]
    _1,
}
impl USGFAULTACTR {
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
            USGFAULTACTR::_0 => false,
            USGFAULTACTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USGFAULTACTR {
        match value {
            false => USGFAULTACTR::_0,
            true => USGFAULTACTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USGFAULTACTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USGFAULTACTR::_1
    }
}
#[doc = "Possible values of the field `SVCALLACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLACTR {
    #[doc = "exception is not active"]
    _0,
    #[doc = "exception is active"]
    _1,
}
impl SVCALLACTR {
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
            SVCALLACTR::_0 => false,
            SVCALLACTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SVCALLACTR {
        match value {
            false => SVCALLACTR::_0,
            true => SVCALLACTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SVCALLACTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SVCALLACTR::_1
    }
}
#[doc = "Possible values of the field `MONITORACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONITORACTR {
    #[doc = "exception is not active"]
    _0,
    #[doc = "exception is active"]
    _1,
}
impl MONITORACTR {
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
            MONITORACTR::_0 => false,
            MONITORACTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONITORACTR {
        match value {
            false => MONITORACTR::_0,
            true => MONITORACTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MONITORACTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MONITORACTR::_1
    }
}
#[doc = "Possible values of the field `PENDSVACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVACTR {
    #[doc = "exception is not active"]
    _0,
    #[doc = "exception is active"]
    _1,
}
impl PENDSVACTR {
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
            PENDSVACTR::_0 => false,
            PENDSVACTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENDSVACTR {
        match value {
            false => PENDSVACTR::_0,
            true => PENDSVACTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PENDSVACTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PENDSVACTR::_1
    }
}
#[doc = "Possible values of the field `SYSTICKACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTICKACTR {
    #[doc = "exception is not active"]
    _0,
    #[doc = "exception is active"]
    _1,
}
impl SYSTICKACTR {
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
            SYSTICKACTR::_0 => false,
            SYSTICKACTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSTICKACTR {
        match value {
            false => SYSTICKACTR::_0,
            true => SYSTICKACTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SYSTICKACTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SYSTICKACTR::_1
    }
}
#[doc = "Possible values of the field `USGFAULTPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTPENDEDR {
    #[doc = "exception is not pending"]
    _0,
    #[doc = "exception is pending"]
    _1,
}
impl USGFAULTPENDEDR {
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
            USGFAULTPENDEDR::_0 => false,
            USGFAULTPENDEDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USGFAULTPENDEDR {
        match value {
            false => USGFAULTPENDEDR::_0,
            true => USGFAULTPENDEDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USGFAULTPENDEDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USGFAULTPENDEDR::_1
    }
}
#[doc = "Possible values of the field `MEMFAULTPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTPENDEDR {
    #[doc = "exception is not pending"]
    _0,
    #[doc = "exception is pending"]
    _1,
}
impl MEMFAULTPENDEDR {
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
            MEMFAULTPENDEDR::_0 => false,
            MEMFAULTPENDEDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEMFAULTPENDEDR {
        match value {
            false => MEMFAULTPENDEDR::_0,
            true => MEMFAULTPENDEDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MEMFAULTPENDEDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MEMFAULTPENDEDR::_1
    }
}
#[doc = "Possible values of the field `BUSFAULTPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTPENDEDR {
    #[doc = "exception is not pending"]
    _0,
    #[doc = "exception is pending"]
    _1,
}
impl BUSFAULTPENDEDR {
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
            BUSFAULTPENDEDR::_0 => false,
            BUSFAULTPENDEDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSFAULTPENDEDR {
        match value {
            false => BUSFAULTPENDEDR::_0,
            true => BUSFAULTPENDEDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUSFAULTPENDEDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUSFAULTPENDEDR::_1
    }
}
#[doc = "Possible values of the field `SVCALLPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLPENDEDR {
    #[doc = "exception is not pending"]
    _0,
    #[doc = "exception is pending"]
    _1,
}
impl SVCALLPENDEDR {
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
            SVCALLPENDEDR::_0 => false,
            SVCALLPENDEDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SVCALLPENDEDR {
        match value {
            false => SVCALLPENDEDR::_0,
            true => SVCALLPENDEDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SVCALLPENDEDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SVCALLPENDEDR::_1
    }
}
#[doc = "Possible values of the field `MEMFAULTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTENAR {
    #[doc = "disable the exception"]
    _0,
    #[doc = "enable the exception"]
    _1,
}
impl MEMFAULTENAR {
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
            MEMFAULTENAR::_0 => false,
            MEMFAULTENAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEMFAULTENAR {
        match value {
            false => MEMFAULTENAR::_0,
            true => MEMFAULTENAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MEMFAULTENAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MEMFAULTENAR::_1
    }
}
#[doc = "Possible values of the field `BUSFAULTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTENAR {
    #[doc = "disable the exception"]
    _0,
    #[doc = "enable the exception"]
    _1,
}
impl BUSFAULTENAR {
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
            BUSFAULTENAR::_0 => false,
            BUSFAULTENAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSFAULTENAR {
        match value {
            false => BUSFAULTENAR::_0,
            true => BUSFAULTENAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUSFAULTENAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUSFAULTENAR::_1
    }
}
#[doc = "Possible values of the field `USGFAULTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTENAR {
    #[doc = "disable the exception"]
    _0,
    #[doc = "enable the exception"]
    _1,
}
impl USGFAULTENAR {
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
            USGFAULTENAR::_0 => false,
            USGFAULTENAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USGFAULTENAR {
        match value {
            false => USGFAULTENAR::_0,
            true => USGFAULTENAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USGFAULTENAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USGFAULTENAR::_1
    }
}
#[doc = "Values that can be written to the field `MEMFAULTACT`"]
pub enum MEMFAULTACTW {
    #[doc = "exception is not active"]
    _0,
    #[doc = "exception is active"]
    _1,
}
impl MEMFAULTACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEMFAULTACTW::_0 => false,
            MEMFAULTACTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEMFAULTACTW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMFAULTACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEMFAULTACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not active"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MEMFAULTACTW::_0)
    }
    #[doc = "exception is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MEMFAULTACTW::_1)
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
#[doc = "Values that can be written to the field `BUSFAULTACT`"]
pub enum BUSFAULTACTW {
    #[doc = "exception is not active"]
    _0,
    #[doc = "exception is active"]
    _1,
}
impl BUSFAULTACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUSFAULTACTW::_0 => false,
            BUSFAULTACTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUSFAULTACTW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSFAULTACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUSFAULTACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not active"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSFAULTACTW::_0)
    }
    #[doc = "exception is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSFAULTACTW::_1)
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
#[doc = "Values that can be written to the field `USGFAULTACT`"]
pub enum USGFAULTACTW {
    #[doc = "exception is not active"]
    _0,
    #[doc = "exception is active"]
    _1,
}
impl USGFAULTACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USGFAULTACTW::_0 => false,
            USGFAULTACTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USGFAULTACTW<'a> {
    w: &'a mut W,
}
impl<'a> _USGFAULTACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USGFAULTACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not active"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USGFAULTACTW::_0)
    }
    #[doc = "exception is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USGFAULTACTW::_1)
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
#[doc = "Values that can be written to the field `SVCALLACT`"]
pub enum SVCALLACTW {
    #[doc = "exception is not active"]
    _0,
    #[doc = "exception is active"]
    _1,
}
impl SVCALLACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SVCALLACTW::_0 => false,
            SVCALLACTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SVCALLACTW<'a> {
    w: &'a mut W,
}
impl<'a> _SVCALLACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SVCALLACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not active"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SVCALLACTW::_0)
    }
    #[doc = "exception is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SVCALLACTW::_1)
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
#[doc = "Values that can be written to the field `MONITORACT`"]
pub enum MONITORACTW {
    #[doc = "exception is not active"]
    _0,
    #[doc = "exception is active"]
    _1,
}
impl MONITORACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONITORACTW::_0 => false,
            MONITORACTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONITORACTW<'a> {
    w: &'a mut W,
}
impl<'a> _MONITORACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONITORACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not active"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MONITORACTW::_0)
    }
    #[doc = "exception is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MONITORACTW::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSVACT`"]
pub enum PENDSVACTW {
    #[doc = "exception is not active"]
    _0,
    #[doc = "exception is active"]
    _1,
}
impl PENDSVACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSVACTW::_0 => false,
            PENDSVACTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSVACTW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSVACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSVACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not active"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSVACTW::_0)
    }
    #[doc = "exception is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSVACTW::_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYSTICKACT`"]
pub enum SYSTICKACTW {
    #[doc = "exception is not active"]
    _0,
    #[doc = "exception is active"]
    _1,
}
impl SYSTICKACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSTICKACTW::_0 => false,
            SYSTICKACTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSTICKACTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSTICKACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSTICKACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not active"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYSTICKACTW::_0)
    }
    #[doc = "exception is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYSTICKACTW::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USGFAULTPENDED`"]
pub enum USGFAULTPENDEDW {
    #[doc = "exception is not pending"]
    _0,
    #[doc = "exception is pending"]
    _1,
}
impl USGFAULTPENDEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USGFAULTPENDEDW::_0 => false,
            USGFAULTPENDEDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USGFAULTPENDEDW<'a> {
    w: &'a mut W,
}
impl<'a> _USGFAULTPENDEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USGFAULTPENDEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not pending"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USGFAULTPENDEDW::_0)
    }
    #[doc = "exception is pending"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USGFAULTPENDEDW::_1)
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
#[doc = "Values that can be written to the field `MEMFAULTPENDED`"]
pub enum MEMFAULTPENDEDW {
    #[doc = "exception is not pending"]
    _0,
    #[doc = "exception is pending"]
    _1,
}
impl MEMFAULTPENDEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEMFAULTPENDEDW::_0 => false,
            MEMFAULTPENDEDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEMFAULTPENDEDW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMFAULTPENDEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEMFAULTPENDEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not pending"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MEMFAULTPENDEDW::_0)
    }
    #[doc = "exception is pending"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MEMFAULTPENDEDW::_1)
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
#[doc = "Values that can be written to the field `BUSFAULTPENDED`"]
pub enum BUSFAULTPENDEDW {
    #[doc = "exception is not pending"]
    _0,
    #[doc = "exception is pending"]
    _1,
}
impl BUSFAULTPENDEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUSFAULTPENDEDW::_0 => false,
            BUSFAULTPENDEDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUSFAULTPENDEDW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSFAULTPENDEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUSFAULTPENDEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not pending"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSFAULTPENDEDW::_0)
    }
    #[doc = "exception is pending"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSFAULTPENDEDW::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SVCALLPENDED`"]
pub enum SVCALLPENDEDW {
    #[doc = "exception is not pending"]
    _0,
    #[doc = "exception is pending"]
    _1,
}
impl SVCALLPENDEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SVCALLPENDEDW::_0 => false,
            SVCALLPENDEDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SVCALLPENDEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SVCALLPENDEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SVCALLPENDEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "exception is not pending"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SVCALLPENDEDW::_0)
    }
    #[doc = "exception is pending"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SVCALLPENDEDW::_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MEMFAULTENA`"]
pub enum MEMFAULTENAW {
    #[doc = "disable the exception"]
    _0,
    #[doc = "enable the exception"]
    _1,
}
impl MEMFAULTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEMFAULTENAW::_0 => false,
            MEMFAULTENAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEMFAULTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMFAULTENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEMFAULTENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable the exception"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MEMFAULTENAW::_0)
    }
    #[doc = "enable the exception"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MEMFAULTENAW::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUSFAULTENA`"]
pub enum BUSFAULTENAW {
    #[doc = "disable the exception"]
    _0,
    #[doc = "enable the exception"]
    _1,
}
impl BUSFAULTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUSFAULTENAW::_0 => false,
            BUSFAULTENAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUSFAULTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSFAULTENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUSFAULTENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable the exception"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSFAULTENAW::_0)
    }
    #[doc = "enable the exception"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSFAULTENAW::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USGFAULTENA`"]
pub enum USGFAULTENAW {
    #[doc = "disable the exception"]
    _0,
    #[doc = "enable the exception"]
    _1,
}
impl USGFAULTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USGFAULTENAW::_0 => false,
            USGFAULTENAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USGFAULTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _USGFAULTENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USGFAULTENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable the exception"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USGFAULTENAW::_0)
    }
    #[doc = "enable the exception"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USGFAULTENAW::_1)
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
        const OFFSET: u8 = 18;
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
    pub fn memfaultact(&self) -> MEMFAULTACTR {
        MEMFAULTACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn busfaultact(&self) -> BUSFAULTACTR {
        BUSFAULTACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - no description available"]
    #[inline]
    pub fn usgfaultact(&self) -> USGFAULTACTR {
        USGFAULTACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - no description available"]
    #[inline]
    pub fn svcallact(&self) -> SVCALLACTR {
        SVCALLACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - no description available"]
    #[inline]
    pub fn monitoract(&self) -> MONITORACTR {
        MONITORACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - no description available"]
    #[inline]
    pub fn pendsvact(&self) -> PENDSVACTR {
        PENDSVACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - no description available"]
    #[inline]
    pub fn systickact(&self) -> SYSTICKACTR {
        SYSTICKACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - no description available"]
    #[inline]
    pub fn usgfaultpended(&self) -> USGFAULTPENDEDR {
        USGFAULTPENDEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - no description available"]
    #[inline]
    pub fn memfaultpended(&self) -> MEMFAULTPENDEDR {
        MEMFAULTPENDEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - no description available"]
    #[inline]
    pub fn busfaultpended(&self) -> BUSFAULTPENDEDR {
        BUSFAULTPENDEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - no description available"]
    #[inline]
    pub fn svcallpended(&self) -> SVCALLPENDEDR {
        SVCALLPENDEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - no description available"]
    #[inline]
    pub fn memfaultena(&self) -> MEMFAULTENAR {
        MEMFAULTENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - no description available"]
    #[inline]
    pub fn busfaultena(&self) -> BUSFAULTENAR {
        BUSFAULTENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - no description available"]
    #[inline]
    pub fn usgfaultena(&self) -> USGFAULTENAR {
        USGFAULTENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
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
    pub fn memfaultact(&mut self) -> _MEMFAULTACTW {
        _MEMFAULTACTW { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn busfaultact(&mut self) -> _BUSFAULTACTW {
        _BUSFAULTACTW { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline]
    pub fn usgfaultact(&mut self) -> _USGFAULTACTW {
        _USGFAULTACTW { w: self }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline]
    pub fn svcallact(&mut self) -> _SVCALLACTW {
        _SVCALLACTW { w: self }
    }
    #[doc = "Bit 8 - no description available"]
    #[inline]
    pub fn monitoract(&mut self) -> _MONITORACTW {
        _MONITORACTW { w: self }
    }
    #[doc = "Bit 10 - no description available"]
    #[inline]
    pub fn pendsvact(&mut self) -> _PENDSVACTW {
        _PENDSVACTW { w: self }
    }
    #[doc = "Bit 11 - no description available"]
    #[inline]
    pub fn systickact(&mut self) -> _SYSTICKACTW {
        _SYSTICKACTW { w: self }
    }
    #[doc = "Bit 12 - no description available"]
    #[inline]
    pub fn usgfaultpended(&mut self) -> _USGFAULTPENDEDW {
        _USGFAULTPENDEDW { w: self }
    }
    #[doc = "Bit 13 - no description available"]
    #[inline]
    pub fn memfaultpended(&mut self) -> _MEMFAULTPENDEDW {
        _MEMFAULTPENDEDW { w: self }
    }
    #[doc = "Bit 14 - no description available"]
    #[inline]
    pub fn busfaultpended(&mut self) -> _BUSFAULTPENDEDW {
        _BUSFAULTPENDEDW { w: self }
    }
    #[doc = "Bit 15 - no description available"]
    #[inline]
    pub fn svcallpended(&mut self) -> _SVCALLPENDEDW {
        _SVCALLPENDEDW { w: self }
    }
    #[doc = "Bit 16 - no description available"]
    #[inline]
    pub fn memfaultena(&mut self) -> _MEMFAULTENAW {
        _MEMFAULTENAW { w: self }
    }
    #[doc = "Bit 17 - no description available"]
    #[inline]
    pub fn busfaultena(&mut self) -> _BUSFAULTENAW {
        _BUSFAULTENAW { w: self }
    }
    #[doc = "Bit 18 - no description available"]
    #[inline]
    pub fn usgfaultena(&mut self) -> _USGFAULTENAW {
        _USGFAULTENAW { w: self }
    }
}
