#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RAR {
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
#[doc = "Possible values of the field `TSRR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSRRR {
    #[doc = "Reads to the time seconds register are ignored."]
    _0,
    #[doc = "Reads to the time seconds register complete as normal."]
    _1,
}
impl TSRRR {
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
            TSRRR::_0 => false,
            TSRRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSRRR {
        match value {
            false => TSRRR::_0,
            true => TSRRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSRRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSRRR::_1
    }
}
#[doc = "Possible values of the field `TPRR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPRRR {
    #[doc = "Reads to the time prescaler register are ignored."]
    _0,
    #[doc = "Reads to the time prescaler register complete as normal."]
    _1,
}
impl TPRRR {
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
            TPRRR::_0 => false,
            TPRRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPRRR {
        match value {
            false => TPRRR::_0,
            true => TPRRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TPRRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TPRRR::_1
    }
}
#[doc = "Possible values of the field `TARR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TARRR {
    #[doc = "Reads to the time alarm register are ignored."]
    _0,
    #[doc = "Reads to the time alarm register complete as normal."]
    _1,
}
impl TARRR {
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
            TARRR::_0 => false,
            TARRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TARRR {
        match value {
            false => TARRR::_0,
            true => TARRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TARRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TARRR::_1
    }
}
#[doc = "Possible values of the field `TCRR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCRRR {
    #[doc = "Reads to the time compensation register are ignored."]
    _0,
    #[doc = "Reads to the time compensation register complete as normal."]
    _1,
}
impl TCRRR {
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
            TCRRR::_0 => false,
            TCRRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCRRR {
        match value {
            false => TCRRR::_0,
            true => TCRRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCRRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCRRR::_1
    }
}
#[doc = "Possible values of the field `CRR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRRR {
    #[doc = "Reads to the control register are ignored."]
    _0,
    #[doc = "Reads to the control register complete as normal."]
    _1,
}
impl CRRR {
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
            CRRR::_0 => false,
            CRRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRRR {
        match value {
            false => CRRR::_0,
            true => CRRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRRR::_1
    }
}
#[doc = "Possible values of the field `SRR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRRR {
    #[doc = "Reads to the status register are ignored."]
    _0,
    #[doc = "Reads to the status register complete as normal."]
    _1,
}
impl SRRR {
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
            SRRR::_0 => false,
            SRRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRRR {
        match value {
            false => SRRR::_0,
            true => SRRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SRRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SRRR::_1
    }
}
#[doc = "Possible values of the field `LRR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRRR {
    #[doc = "Reads to the lock register are ignored."]
    _0,
    #[doc = "Reads to the lock register complete as normal."]
    _1,
}
impl LRRR {
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
            LRRR::_0 => false,
            LRRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LRRR {
        match value {
            false => LRRR::_0,
            true => LRRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LRRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LRRR::_1
    }
}
#[doc = "Possible values of the field `IERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IERRR {
    #[doc = "Reads to the interrupt enable register are ignored."]
    _0,
    #[doc = "Reads to the interrupt enable register complete as normal."]
    _1,
}
impl IERRR {
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
            IERRR::_0 => false,
            IERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IERRR {
        match value {
            false => IERRR::_0,
            true => IERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IERRR::_1
    }
}
#[doc = "Values that can be written to the field `TSRR`"]
pub enum TSRRW {
    #[doc = "Reads to the time seconds register are ignored."]
    _0,
    #[doc = "Reads to the time seconds register complete as normal."]
    _1,
}
impl TSRRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSRRW::_0 => false,
            TSRRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSRRW<'a> {
    w: &'a mut W,
}
impl<'a> _TSRRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSRRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reads to the time seconds register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSRRW::_0)
    }
    #[doc = "Reads to the time seconds register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSRRW::_1)
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
#[doc = "Values that can be written to the field `TPRR`"]
pub enum TPRRW {
    #[doc = "Reads to the time prescaler register are ignored."]
    _0,
    #[doc = "Reads to the time prescaler register complete as normal."]
    _1,
}
impl TPRRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPRRW::_0 => false,
            TPRRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPRRW<'a> {
    w: &'a mut W,
}
impl<'a> _TPRRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPRRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reads to the time prescaler register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPRRW::_0)
    }
    #[doc = "Reads to the time prescaler register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPRRW::_1)
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
#[doc = "Values that can be written to the field `TARR`"]
pub enum TARRW {
    #[doc = "Reads to the time alarm register are ignored."]
    _0,
    #[doc = "Reads to the time alarm register complete as normal."]
    _1,
}
impl TARRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TARRW::_0 => false,
            TARRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TARRW<'a> {
    w: &'a mut W,
}
impl<'a> _TARRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TARRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reads to the time alarm register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TARRW::_0)
    }
    #[doc = "Reads to the time alarm register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TARRW::_1)
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
#[doc = "Values that can be written to the field `TCRR`"]
pub enum TCRRW {
    #[doc = "Reads to the time compensation register are ignored."]
    _0,
    #[doc = "Reads to the time compensation register complete as normal."]
    _1,
}
impl TCRRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCRRW::_0 => false,
            TCRRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCRRW<'a> {
    w: &'a mut W,
}
impl<'a> _TCRRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCRRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reads to the time compensation register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCRRW::_0)
    }
    #[doc = "Reads to the time compensation register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCRRW::_1)
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
#[doc = "Values that can be written to the field `CRR`"]
pub enum CRRW {
    #[doc = "Reads to the control register are ignored."]
    _0,
    #[doc = "Reads to the control register complete as normal."]
    _1,
}
impl CRRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRRW::_0 => false,
            CRRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRRW<'a> {
    w: &'a mut W,
}
impl<'a> _CRRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reads to the control register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRRW::_0)
    }
    #[doc = "Reads to the control register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRRW::_1)
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
#[doc = "Values that can be written to the field `SRR`"]
pub enum SRRW {
    #[doc = "Reads to the status register are ignored."]
    _0,
    #[doc = "Reads to the status register complete as normal."]
    _1,
}
impl SRRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRRW::_0 => false,
            SRRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reads to the status register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRRW::_0)
    }
    #[doc = "Reads to the status register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRRW::_1)
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
#[doc = "Values that can be written to the field `LRR`"]
pub enum LRRW {
    #[doc = "Reads to the lock register are ignored."]
    _0,
    #[doc = "Reads to the lock register complete as normal."]
    _1,
}
impl LRRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LRRW::_0 => false,
            LRRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LRRW<'a> {
    w: &'a mut W,
}
impl<'a> _LRRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LRRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reads to the lock register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LRRW::_0)
    }
    #[doc = "Reads to the lock register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LRRW::_1)
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
#[doc = "Values that can be written to the field `IERR`"]
pub enum IERRW {
    #[doc = "Reads to the interrupt enable register are ignored."]
    _0,
    #[doc = "Reads to the interrupt enable register complete as normal."]
    _1,
}
impl IERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IERRW::_0 => false,
            IERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IERRW<'a> {
    w: &'a mut W,
}
impl<'a> _IERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reads to the interrupt enable register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IERRW::_0)
    }
    #[doc = "Reads to the interrupt enable register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IERRW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Time Seconds Register Read"]
    #[inline]
    pub fn tsrr(&self) -> TSRRR {
        TSRRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Time Prescaler Register Read"]
    #[inline]
    pub fn tprr(&self) -> TPRRR {
        TPRRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Time Alarm Register Read"]
    #[inline]
    pub fn tarr(&self) -> TARRR {
        TARRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Time Compensation Register Read"]
    #[inline]
    pub fn tcrr(&self) -> TCRRR {
        TCRRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Control Register Read"]
    #[inline]
    pub fn crr(&self) -> CRRR {
        CRRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Status Register Read"]
    #[inline]
    pub fn srr(&self) -> SRRR {
        SRRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Lock Register Read"]
    #[inline]
    pub fn lrr(&self) -> LRRR {
        LRRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Interrupt Enable Register Read"]
    #[inline]
    pub fn ierr(&self) -> IERRR {
        IERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 255 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Time Seconds Register Read"]
    #[inline]
    pub fn tsrr(&mut self) -> _TSRRW {
        _TSRRW { w: self }
    }
    #[doc = "Bit 1 - Time Prescaler Register Read"]
    #[inline]
    pub fn tprr(&mut self) -> _TPRRW {
        _TPRRW { w: self }
    }
    #[doc = "Bit 2 - Time Alarm Register Read"]
    #[inline]
    pub fn tarr(&mut self) -> _TARRW {
        _TARRW { w: self }
    }
    #[doc = "Bit 3 - Time Compensation Register Read"]
    #[inline]
    pub fn tcrr(&mut self) -> _TCRRW {
        _TCRRW { w: self }
    }
    #[doc = "Bit 4 - Control Register Read"]
    #[inline]
    pub fn crr(&mut self) -> _CRRW {
        _CRRW { w: self }
    }
    #[doc = "Bit 5 - Status Register Read"]
    #[inline]
    pub fn srr(&mut self) -> _SRRW {
        _SRRW { w: self }
    }
    #[doc = "Bit 6 - Lock Register Read"]
    #[inline]
    pub fn lrr(&mut self) -> _LRRW {
        _LRRW { w: self }
    }
    #[doc = "Bit 7 - Interrupt Enable Register Read"]
    #[inline]
    pub fn ierr(&mut self) -> _IERRW {
        _IERRW { w: self }
    }
}
