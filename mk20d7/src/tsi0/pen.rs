#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PEN {
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
#[doc = "Possible values of the field `PEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN0R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN0R {
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
            PEN0R::_0 => false,
            PEN0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN0R {
        match value {
            false => PEN0R::_0,
            true => PEN0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN0R::_1
    }
}
#[doc = "Possible values of the field `PEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN1R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN1R {
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
            PEN1R::_0 => false,
            PEN1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN1R {
        match value {
            false => PEN1R::_0,
            true => PEN1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN1R::_1
    }
}
#[doc = "Possible values of the field `PEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN2R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN2R {
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
            PEN2R::_0 => false,
            PEN2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN2R {
        match value {
            false => PEN2R::_0,
            true => PEN2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN2R::_1
    }
}
#[doc = "Possible values of the field `PEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN3R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN3R {
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
            PEN3R::_0 => false,
            PEN3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN3R {
        match value {
            false => PEN3R::_0,
            true => PEN3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN3R::_1
    }
}
#[doc = "Possible values of the field `PEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN4R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN4R {
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
            PEN4R::_0 => false,
            PEN4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN4R {
        match value {
            false => PEN4R::_0,
            true => PEN4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN4R::_1
    }
}
#[doc = "Possible values of the field `PEN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN5R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN5R {
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
            PEN5R::_0 => false,
            PEN5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN5R {
        match value {
            false => PEN5R::_0,
            true => PEN5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN5R::_1
    }
}
#[doc = "Possible values of the field `PEN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN6R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN6R {
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
            PEN6R::_0 => false,
            PEN6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN6R {
        match value {
            false => PEN6R::_0,
            true => PEN6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN6R::_1
    }
}
#[doc = "Possible values of the field `PEN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN7R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN7R {
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
            PEN7R::_0 => false,
            PEN7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN7R {
        match value {
            false => PEN7R::_0,
            true => PEN7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN7R::_1
    }
}
#[doc = "Possible values of the field `PEN8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN8R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN8R {
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
            PEN8R::_0 => false,
            PEN8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN8R {
        match value {
            false => PEN8R::_0,
            true => PEN8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN8R::_1
    }
}
#[doc = "Possible values of the field `PEN9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN9R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN9R {
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
            PEN9R::_0 => false,
            PEN9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN9R {
        match value {
            false => PEN9R::_0,
            true => PEN9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN9R::_1
    }
}
#[doc = "Possible values of the field `PEN10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN10R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN10R {
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
            PEN10R::_0 => false,
            PEN10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN10R {
        match value {
            false => PEN10R::_0,
            true => PEN10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN10R::_1
    }
}
#[doc = "Possible values of the field `PEN11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN11R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN11R {
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
            PEN11R::_0 => false,
            PEN11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN11R {
        match value {
            false => PEN11R::_0,
            true => PEN11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN11R::_1
    }
}
#[doc = "Possible values of the field `PEN12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN12R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN12R {
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
            PEN12R::_0 => false,
            PEN12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN12R {
        match value {
            false => PEN12R::_0,
            true => PEN12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN12R::_1
    }
}
#[doc = "Possible values of the field `PEN13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN13R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN13R {
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
            PEN13R::_0 => false,
            PEN13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN13R {
        match value {
            false => PEN13R::_0,
            true => PEN13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN13R::_1
    }
}
#[doc = "Possible values of the field `PEN14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN14R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN14R {
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
            PEN14R::_0 => false,
            PEN14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN14R {
        match value {
            false => PEN14R::_0,
            true => PEN14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN14R::_1
    }
}
#[doc = "Possible values of the field `PEN15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN15R {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN15R {
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
            PEN15R::_0 => false,
            PEN15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN15R {
        match value {
            false => PEN15R::_0,
            true => PEN15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEN15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEN15R::_1
    }
}
#[doc = "Possible values of the field `LPSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPR {
    #[doc = "TSI_IN\\[0\\] is active in low power mode."]
    _0000,
    #[doc = "TSI_IN\\[1\\] is active in low power mode."]
    _0001,
    #[doc = "TSI_IN\\[2\\] is active in low power mode."]
    _0010,
    #[doc = "TSI_IN\\[3\\] is active in low power mode."]
    _0011,
    #[doc = "TSI_IN\\[4\\] is active in low power mode."]
    _0100,
    #[doc = "TSI_IN\\[5\\] is active in low power mode."]
    _0101,
    #[doc = "TSI_IN\\[6\\] is active in low power mode."]
    _0110,
    #[doc = "TSI_IN\\[7\\] is active in low power mode."]
    _0111,
    #[doc = "TSI_IN\\[8\\] is active in low power mode."]
    _1000,
    #[doc = "TSI_IN\\[9\\] is active in low power mode."]
    _1001,
    #[doc = "TSI_IN\\[10\\] is active in low power mode."]
    _1010,
    #[doc = "TSI_IN\\[11\\] is active in low power mode."]
    _1011,
    #[doc = "TSI_IN\\[12\\] is active in low power mode."]
    _1100,
    #[doc = "TSI_IN\\[13\\] is active in low power mode."]
    _1101,
    #[doc = "TSI_IN\\[14\\] is active in low power mode."]
    _1110,
    #[doc = "TSI_IN\\[15\\] is active in low power mode."]
    _1111,
}
impl LPSPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPSPR::_0000 => 0,
            LPSPR::_0001 => 1,
            LPSPR::_0010 => 2,
            LPSPR::_0011 => 3,
            LPSPR::_0100 => 4,
            LPSPR::_0101 => 5,
            LPSPR::_0110 => 6,
            LPSPR::_0111 => 7,
            LPSPR::_1000 => 8,
            LPSPR::_1001 => 9,
            LPSPR::_1010 => 10,
            LPSPR::_1011 => 11,
            LPSPR::_1100 => 12,
            LPSPR::_1101 => 13,
            LPSPR::_1110 => 14,
            LPSPR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPSPR {
        match value {
            0 => LPSPR::_0000,
            1 => LPSPR::_0001,
            2 => LPSPR::_0010,
            3 => LPSPR::_0011,
            4 => LPSPR::_0100,
            5 => LPSPR::_0101,
            6 => LPSPR::_0110,
            7 => LPSPR::_0111,
            8 => LPSPR::_1000,
            9 => LPSPR::_1001,
            10 => LPSPR::_1010,
            11 => LPSPR::_1011,
            12 => LPSPR::_1100,
            13 => LPSPR::_1101,
            14 => LPSPR::_1110,
            15 => LPSPR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == LPSPR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == LPSPR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == LPSPR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == LPSPR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == LPSPR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == LPSPR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == LPSPR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == LPSPR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == LPSPR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == LPSPR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == LPSPR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == LPSPR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == LPSPR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == LPSPR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == LPSPR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == LPSPR::_1111
    }
}
#[doc = "Values that can be written to the field `PEN0`"]
pub enum PEN0W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN0W::_0 => false,
            PEN0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN0W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN0W::_1)
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
#[doc = "Values that can be written to the field `PEN1`"]
pub enum PEN1W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN1W::_0 => false,
            PEN1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN1W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN1W::_1)
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
#[doc = "Values that can be written to the field `PEN2`"]
pub enum PEN2W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN2W::_0 => false,
            PEN2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN2W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN2W::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PEN3`"]
pub enum PEN3W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN3W::_0 => false,
            PEN3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN3W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN3W::_1)
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
#[doc = "Values that can be written to the field `PEN4`"]
pub enum PEN4W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN4W::_0 => false,
            PEN4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN4W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN4W::_1)
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
#[doc = "Values that can be written to the field `PEN5`"]
pub enum PEN5W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN5W::_0 => false,
            PEN5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN5W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN5W::_1)
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
#[doc = "Values that can be written to the field `PEN6`"]
pub enum PEN6W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN6W::_0 => false,
            PEN6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN6W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN6W::_1)
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
#[doc = "Values that can be written to the field `PEN7`"]
pub enum PEN7W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN7W::_0 => false,
            PEN7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN7W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN7W::_1)
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
#[doc = "Values that can be written to the field `PEN8`"]
pub enum PEN8W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN8W::_0 => false,
            PEN8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN8W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN8W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN8W::_1)
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
#[doc = "Values that can be written to the field `PEN9`"]
pub enum PEN9W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN9W::_0 => false,
            PEN9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN9W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN9W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN9W::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PEN10`"]
pub enum PEN10W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN10W::_0 => false,
            PEN10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN10W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN10W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN10W::_1)
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
#[doc = "Values that can be written to the field `PEN11`"]
pub enum PEN11W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN11W::_0 => false,
            PEN11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN11W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN11W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN11W::_1)
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
#[doc = "Values that can be written to the field `PEN12`"]
pub enum PEN12W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN12W::_0 => false,
            PEN12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN12W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN12W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN12W::_1)
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
#[doc = "Values that can be written to the field `PEN13`"]
pub enum PEN13W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN13W::_0 => false,
            PEN13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN13W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN13W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN13W::_1)
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
#[doc = "Values that can be written to the field `PEN14`"]
pub enum PEN14W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN14W::_0 => false,
            PEN14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN14W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN14W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN14W::_1)
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
#[doc = "Values that can be written to the field `PEN15`"]
pub enum PEN15W {
    #[doc = "The corresponding pin is not used by TSI."]
    _0,
    #[doc = "The corresponding pin is used by TSI."]
    _1,
}
impl PEN15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN15W::_0 => false,
            PEN15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN15W<'a> {
    w: &'a mut W,
}
impl<'a> _PEN15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN15W::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN15W::_1)
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
#[doc = "Values that can be written to the field `LPSP`"]
pub enum LPSPW {
    #[doc = "TSI_IN\\[0\\] is active in low power mode."]
    _0000,
    #[doc = "TSI_IN\\[1\\] is active in low power mode."]
    _0001,
    #[doc = "TSI_IN\\[2\\] is active in low power mode."]
    _0010,
    #[doc = "TSI_IN\\[3\\] is active in low power mode."]
    _0011,
    #[doc = "TSI_IN\\[4\\] is active in low power mode."]
    _0100,
    #[doc = "TSI_IN\\[5\\] is active in low power mode."]
    _0101,
    #[doc = "TSI_IN\\[6\\] is active in low power mode."]
    _0110,
    #[doc = "TSI_IN\\[7\\] is active in low power mode."]
    _0111,
    #[doc = "TSI_IN\\[8\\] is active in low power mode."]
    _1000,
    #[doc = "TSI_IN\\[9\\] is active in low power mode."]
    _1001,
    #[doc = "TSI_IN\\[10\\] is active in low power mode."]
    _1010,
    #[doc = "TSI_IN\\[11\\] is active in low power mode."]
    _1011,
    #[doc = "TSI_IN\\[12\\] is active in low power mode."]
    _1100,
    #[doc = "TSI_IN\\[13\\] is active in low power mode."]
    _1101,
    #[doc = "TSI_IN\\[14\\] is active in low power mode."]
    _1110,
    #[doc = "TSI_IN\\[15\\] is active in low power mode."]
    _1111,
}
impl LPSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPSPW::_0000 => 0,
            LPSPW::_0001 => 1,
            LPSPW::_0010 => 2,
            LPSPW::_0011 => 3,
            LPSPW::_0100 => 4,
            LPSPW::_0101 => 5,
            LPSPW::_0110 => 6,
            LPSPW::_0111 => 7,
            LPSPW::_1000 => 8,
            LPSPW::_1001 => 9,
            LPSPW::_1010 => 10,
            LPSPW::_1011 => 11,
            LPSPW::_1100 => 12,
            LPSPW::_1101 => 13,
            LPSPW::_1110 => 14,
            LPSPW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPSPW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPSPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "TSI_IN\\[0\\] is active in low power mode."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(LPSPW::_0000)
    }
    #[doc = "TSI_IN\\[1\\] is active in low power mode."]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(LPSPW::_0001)
    }
    #[doc = "TSI_IN\\[2\\] is active in low power mode."]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(LPSPW::_0010)
    }
    #[doc = "TSI_IN\\[3\\] is active in low power mode."]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(LPSPW::_0011)
    }
    #[doc = "TSI_IN\\[4\\] is active in low power mode."]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(LPSPW::_0100)
    }
    #[doc = "TSI_IN\\[5\\] is active in low power mode."]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(LPSPW::_0101)
    }
    #[doc = "TSI_IN\\[6\\] is active in low power mode."]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(LPSPW::_0110)
    }
    #[doc = "TSI_IN\\[7\\] is active in low power mode."]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(LPSPW::_0111)
    }
    #[doc = "TSI_IN\\[8\\] is active in low power mode."]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(LPSPW::_1000)
    }
    #[doc = "TSI_IN\\[9\\] is active in low power mode."]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(LPSPW::_1001)
    }
    #[doc = "TSI_IN\\[10\\] is active in low power mode."]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(LPSPW::_1010)
    }
    #[doc = "TSI_IN\\[11\\] is active in low power mode."]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(LPSPW::_1011)
    }
    #[doc = "TSI_IN\\[12\\] is active in low power mode."]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(LPSPW::_1100)
    }
    #[doc = "TSI_IN\\[13\\] is active in low power mode."]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(LPSPW::_1101)
    }
    #[doc = "TSI_IN\\[14\\] is active in low power mode."]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(LPSPW::_1110)
    }
    #[doc = "TSI_IN\\[15\\] is active in low power mode."]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(LPSPW::_1111)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Touch Sensing Input Pin Enable Register 0"]
    #[inline]
    pub fn pen0(&self) -> PEN0R {
        PEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Touch Sensing Input Pin Enable Register 1"]
    #[inline]
    pub fn pen1(&self) -> PEN1R {
        PEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Touch Sensing Input Pin Enable Register 2"]
    #[inline]
    pub fn pen2(&self) -> PEN2R {
        PEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Touch Sensing Input Pin Enable Register 3"]
    #[inline]
    pub fn pen3(&self) -> PEN3R {
        PEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Touch Sensing Input Pin Enable Register 4"]
    #[inline]
    pub fn pen4(&self) -> PEN4R {
        PEN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Touch Sensing Input Pin Enable Register 5"]
    #[inline]
    pub fn pen5(&self) -> PEN5R {
        PEN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Touch Sensing Input Pin Enable Register 6"]
    #[inline]
    pub fn pen6(&self) -> PEN6R {
        PEN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Touch Sensing Input Pin Enable Register 7"]
    #[inline]
    pub fn pen7(&self) -> PEN7R {
        PEN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Touch Sensing Input Pin Enable Register 8"]
    #[inline]
    pub fn pen8(&self) -> PEN8R {
        PEN8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Touch Sensing Input Pin Enable Register 9"]
    #[inline]
    pub fn pen9(&self) -> PEN9R {
        PEN9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Touch Sensing Input Pin Enable Register 10"]
    #[inline]
    pub fn pen10(&self) -> PEN10R {
        PEN10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Touch Sensing Input Pin Enable Register 11"]
    #[inline]
    pub fn pen11(&self) -> PEN11R {
        PEN11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Touch Sensing Input Pin Enable Register 12"]
    #[inline]
    pub fn pen12(&self) -> PEN12R {
        PEN12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Touch Sensing Input Pin Enable Register 13"]
    #[inline]
    pub fn pen13(&self) -> PEN13R {
        PEN13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Touch Sensing Input Pin Enable Register 14"]
    #[inline]
    pub fn pen14(&self) -> PEN14R {
        PEN14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Touch Sensing Input Pin Enable Register 15"]
    #[inline]
    pub fn pen15(&self) -> PEN15R {
        PEN15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - Low Power Scan Pin"]
    #[inline]
    pub fn lpsp(&self) -> LPSPR {
        LPSPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Touch Sensing Input Pin Enable Register 0"]
    #[inline]
    pub fn pen0(&mut self) -> _PEN0W {
        _PEN0W { w: self }
    }
    #[doc = "Bit 1 - Touch Sensing Input Pin Enable Register 1"]
    #[inline]
    pub fn pen1(&mut self) -> _PEN1W {
        _PEN1W { w: self }
    }
    #[doc = "Bit 2 - Touch Sensing Input Pin Enable Register 2"]
    #[inline]
    pub fn pen2(&mut self) -> _PEN2W {
        _PEN2W { w: self }
    }
    #[doc = "Bit 3 - Touch Sensing Input Pin Enable Register 3"]
    #[inline]
    pub fn pen3(&mut self) -> _PEN3W {
        _PEN3W { w: self }
    }
    #[doc = "Bit 4 - Touch Sensing Input Pin Enable Register 4"]
    #[inline]
    pub fn pen4(&mut self) -> _PEN4W {
        _PEN4W { w: self }
    }
    #[doc = "Bit 5 - Touch Sensing Input Pin Enable Register 5"]
    #[inline]
    pub fn pen5(&mut self) -> _PEN5W {
        _PEN5W { w: self }
    }
    #[doc = "Bit 6 - Touch Sensing Input Pin Enable Register 6"]
    #[inline]
    pub fn pen6(&mut self) -> _PEN6W {
        _PEN6W { w: self }
    }
    #[doc = "Bit 7 - Touch Sensing Input Pin Enable Register 7"]
    #[inline]
    pub fn pen7(&mut self) -> _PEN7W {
        _PEN7W { w: self }
    }
    #[doc = "Bit 8 - Touch Sensing Input Pin Enable Register 8"]
    #[inline]
    pub fn pen8(&mut self) -> _PEN8W {
        _PEN8W { w: self }
    }
    #[doc = "Bit 9 - Touch Sensing Input Pin Enable Register 9"]
    #[inline]
    pub fn pen9(&mut self) -> _PEN9W {
        _PEN9W { w: self }
    }
    #[doc = "Bit 10 - Touch Sensing Input Pin Enable Register 10"]
    #[inline]
    pub fn pen10(&mut self) -> _PEN10W {
        _PEN10W { w: self }
    }
    #[doc = "Bit 11 - Touch Sensing Input Pin Enable Register 11"]
    #[inline]
    pub fn pen11(&mut self) -> _PEN11W {
        _PEN11W { w: self }
    }
    #[doc = "Bit 12 - Touch Sensing Input Pin Enable Register 12"]
    #[inline]
    pub fn pen12(&mut self) -> _PEN12W {
        _PEN12W { w: self }
    }
    #[doc = "Bit 13 - Touch Sensing Input Pin Enable Register 13"]
    #[inline]
    pub fn pen13(&mut self) -> _PEN13W {
        _PEN13W { w: self }
    }
    #[doc = "Bit 14 - Touch Sensing Input Pin Enable Register 14"]
    #[inline]
    pub fn pen14(&mut self) -> _PEN14W {
        _PEN14W { w: self }
    }
    #[doc = "Bit 15 - Touch Sensing Input Pin Enable Register 15"]
    #[inline]
    pub fn pen15(&mut self) -> _PEN15W {
        _PEN15W { w: self }
    }
    #[doc = "Bits 16:19 - Low Power Scan Pin"]
    #[inline]
    pub fn lpsp(&mut self) -> _LPSPW {
        _LPSPW { w: self }
    }
}
