#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HRS {
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
#[doc = "Possible values of the field `HRS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS0R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS0R {
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
            HRS0R::_0 => false,
            HRS0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS0R {
        match value {
            false => HRS0R::_0,
            true => HRS0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS0R::_1
    }
}
#[doc = "Possible values of the field `HRS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS1R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS1R {
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
            HRS1R::_0 => false,
            HRS1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS1R {
        match value {
            false => HRS1R::_0,
            true => HRS1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS1R::_1
    }
}
#[doc = "Possible values of the field `HRS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS2R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS2R {
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
            HRS2R::_0 => false,
            HRS2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS2R {
        match value {
            false => HRS2R::_0,
            true => HRS2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS2R::_1
    }
}
#[doc = "Possible values of the field `HRS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS3R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS3R {
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
            HRS3R::_0 => false,
            HRS3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS3R {
        match value {
            false => HRS3R::_0,
            true => HRS3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS3R::_1
    }
}
#[doc = "Possible values of the field `HRS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS4R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS4R {
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
            HRS4R::_0 => false,
            HRS4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS4R {
        match value {
            false => HRS4R::_0,
            true => HRS4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS4R::_1
    }
}
#[doc = "Possible values of the field `HRS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS5R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS5R {
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
            HRS5R::_0 => false,
            HRS5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS5R {
        match value {
            false => HRS5R::_0,
            true => HRS5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS5R::_1
    }
}
#[doc = "Possible values of the field `HRS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS6R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS6R {
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
            HRS6R::_0 => false,
            HRS6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS6R {
        match value {
            false => HRS6R::_0,
            true => HRS6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS6R::_1
    }
}
#[doc = "Possible values of the field `HRS7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS7R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS7R {
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
            HRS7R::_0 => false,
            HRS7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS7R {
        match value {
            false => HRS7R::_0,
            true => HRS7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS7R::_1
    }
}
#[doc = "Possible values of the field `HRS8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS8R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS8R {
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
            HRS8R::_0 => false,
            HRS8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS8R {
        match value {
            false => HRS8R::_0,
            true => HRS8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS8R::_1
    }
}
#[doc = "Possible values of the field `HRS9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS9R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS9R {
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
            HRS9R::_0 => false,
            HRS9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS9R {
        match value {
            false => HRS9R::_0,
            true => HRS9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS9R::_1
    }
}
#[doc = "Possible values of the field `HRS10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS10R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS10R {
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
            HRS10R::_0 => false,
            HRS10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS10R {
        match value {
            false => HRS10R::_0,
            true => HRS10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS10R::_1
    }
}
#[doc = "Possible values of the field `HRS11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS11R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS11R {
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
            HRS11R::_0 => false,
            HRS11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS11R {
        match value {
            false => HRS11R::_0,
            true => HRS11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS11R::_1
    }
}
#[doc = "Possible values of the field `HRS12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS12R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS12R {
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
            HRS12R::_0 => false,
            HRS12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS12R {
        match value {
            false => HRS12R::_0,
            true => HRS12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS12R::_1
    }
}
#[doc = "Possible values of the field `HRS13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS13R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS13R {
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
            HRS13R::_0 => false,
            HRS13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS13R {
        match value {
            false => HRS13R::_0,
            true => HRS13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS13R::_1
    }
}
#[doc = "Possible values of the field `HRS14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS14R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS14R {
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
            HRS14R::_0 => false,
            HRS14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS14R {
        match value {
            false => HRS14R::_0,
            true => HRS14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS14R::_1
    }
}
#[doc = "Possible values of the field `HRS15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS15R {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS15R {
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
            HRS15R::_0 => false,
            HRS15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS15R {
        match value {
            false => HRS15R::_0,
            true => HRS15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS15R::_1
    }
}
#[doc = "Values that can be written to the field `HRS0`"]
pub enum HRS0W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS0W::_0 => false,
            HRS0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS0W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS0W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS0W::_1)
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
#[doc = "Values that can be written to the field `HRS1`"]
pub enum HRS1W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS1W::_0 => false,
            HRS1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS1W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS1W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS1W::_1)
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
#[doc = "Values that can be written to the field `HRS2`"]
pub enum HRS2W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS2W::_0 => false,
            HRS2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS2W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS2W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS2W::_1)
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
#[doc = "Values that can be written to the field `HRS3`"]
pub enum HRS3W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS3W::_0 => false,
            HRS3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS3W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS3W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS3W::_1)
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
#[doc = "Values that can be written to the field `HRS4`"]
pub enum HRS4W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS4W::_0 => false,
            HRS4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS4W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS4W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS4W::_1)
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
#[doc = "Values that can be written to the field `HRS5`"]
pub enum HRS5W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS5W::_0 => false,
            HRS5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS5W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS5W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS5W::_1)
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
#[doc = "Values that can be written to the field `HRS6`"]
pub enum HRS6W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS6W::_0 => false,
            HRS6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS6W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS6W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS6W::_1)
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
#[doc = "Values that can be written to the field `HRS7`"]
pub enum HRS7W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS7W::_0 => false,
            HRS7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS7W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS7W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS7W::_1)
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
#[doc = "Values that can be written to the field `HRS8`"]
pub enum HRS8W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS8W::_0 => false,
            HRS8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS8W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS8W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS8W::_1)
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
#[doc = "Values that can be written to the field `HRS9`"]
pub enum HRS9W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS9W::_0 => false,
            HRS9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS9W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS9W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS9W::_1)
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
#[doc = "Values that can be written to the field `HRS10`"]
pub enum HRS10W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS10W::_0 => false,
            HRS10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS10W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS10W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS10W::_1)
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
#[doc = "Values that can be written to the field `HRS11`"]
pub enum HRS11W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS11W::_0 => false,
            HRS11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS11W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS11W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS11W::_1)
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
#[doc = "Values that can be written to the field `HRS12`"]
pub enum HRS12W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS12W::_0 => false,
            HRS12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS12W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS12W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS12W::_1)
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
#[doc = "Values that can be written to the field `HRS13`"]
pub enum HRS13W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS13W::_0 => false,
            HRS13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS13W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS13W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS13W::_1)
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
#[doc = "Values that can be written to the field `HRS14`"]
pub enum HRS14W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS14W::_0 => false,
            HRS14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS14W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS14W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS14W::_1)
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
#[doc = "Values that can be written to the field `HRS15`"]
pub enum HRS15W {
    #[doc = "A hardware service request for the corresponding channel is not present"]
    _0,
    #[doc = "A hardware service request for the corresponding channel is present"]
    _1,
}
impl HRS15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRS15W::_0 => false,
            HRS15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRS15W<'a> {
    w: &'a mut W,
}
impl<'a> _HRS15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRS15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS15W::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS15W::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline]
    pub fn hrs0(&self) -> HRS0R {
        HRS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline]
    pub fn hrs1(&self) -> HRS1R {
        HRS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline]
    pub fn hrs2(&self) -> HRS2R {
        HRS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline]
    pub fn hrs3(&self) -> HRS3R {
        HRS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Hardware Request Status Channel 4"]
    #[inline]
    pub fn hrs4(&self) -> HRS4R {
        HRS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Hardware Request Status Channel 5"]
    #[inline]
    pub fn hrs5(&self) -> HRS5R {
        HRS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Hardware Request Status Channel 6"]
    #[inline]
    pub fn hrs6(&self) -> HRS6R {
        HRS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Hardware Request Status Channel 7"]
    #[inline]
    pub fn hrs7(&self) -> HRS7R {
        HRS7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Hardware Request Status Channel 8"]
    #[inline]
    pub fn hrs8(&self) -> HRS8R {
        HRS8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Hardware Request Status Channel 9"]
    #[inline]
    pub fn hrs9(&self) -> HRS9R {
        HRS9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Hardware Request Status Channel 10"]
    #[inline]
    pub fn hrs10(&self) -> HRS10R {
        HRS10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Hardware Request Status Channel 11"]
    #[inline]
    pub fn hrs11(&self) -> HRS11R {
        HRS11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Hardware Request Status Channel 12"]
    #[inline]
    pub fn hrs12(&self) -> HRS12R {
        HRS12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Hardware Request Status Channel 13"]
    #[inline]
    pub fn hrs13(&self) -> HRS13R {
        HRS13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Hardware Request Status Channel 14"]
    #[inline]
    pub fn hrs14(&self) -> HRS14R {
        HRS14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Hardware Request Status Channel 15"]
    #[inline]
    pub fn hrs15(&self) -> HRS15R {
        HRS15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline]
    pub fn hrs0(&mut self) -> _HRS0W {
        _HRS0W { w: self }
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline]
    pub fn hrs1(&mut self) -> _HRS1W {
        _HRS1W { w: self }
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline]
    pub fn hrs2(&mut self) -> _HRS2W {
        _HRS2W { w: self }
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline]
    pub fn hrs3(&mut self) -> _HRS3W {
        _HRS3W { w: self }
    }
    #[doc = "Bit 4 - Hardware Request Status Channel 4"]
    #[inline]
    pub fn hrs4(&mut self) -> _HRS4W {
        _HRS4W { w: self }
    }
    #[doc = "Bit 5 - Hardware Request Status Channel 5"]
    #[inline]
    pub fn hrs5(&mut self) -> _HRS5W {
        _HRS5W { w: self }
    }
    #[doc = "Bit 6 - Hardware Request Status Channel 6"]
    #[inline]
    pub fn hrs6(&mut self) -> _HRS6W {
        _HRS6W { w: self }
    }
    #[doc = "Bit 7 - Hardware Request Status Channel 7"]
    #[inline]
    pub fn hrs7(&mut self) -> _HRS7W {
        _HRS7W { w: self }
    }
    #[doc = "Bit 8 - Hardware Request Status Channel 8"]
    #[inline]
    pub fn hrs8(&mut self) -> _HRS8W {
        _HRS8W { w: self }
    }
    #[doc = "Bit 9 - Hardware Request Status Channel 9"]
    #[inline]
    pub fn hrs9(&mut self) -> _HRS9W {
        _HRS9W { w: self }
    }
    #[doc = "Bit 10 - Hardware Request Status Channel 10"]
    #[inline]
    pub fn hrs10(&mut self) -> _HRS10W {
        _HRS10W { w: self }
    }
    #[doc = "Bit 11 - Hardware Request Status Channel 11"]
    #[inline]
    pub fn hrs11(&mut self) -> _HRS11W {
        _HRS11W { w: self }
    }
    #[doc = "Bit 12 - Hardware Request Status Channel 12"]
    #[inline]
    pub fn hrs12(&mut self) -> _HRS12W {
        _HRS12W { w: self }
    }
    #[doc = "Bit 13 - Hardware Request Status Channel 13"]
    #[inline]
    pub fn hrs13(&mut self) -> _HRS13W {
        _HRS13W { w: self }
    }
    #[doc = "Bit 14 - Hardware Request Status Channel 14"]
    #[inline]
    pub fn hrs14(&mut self) -> _HRS14W {
        _HRS14W { w: self }
    }
    #[doc = "Bit 15 - Hardware Request Status Channel 15"]
    #[inline]
    pub fn hrs15(&mut self) -> _HRS15W {
        _HRS15W { w: self }
    }
}
