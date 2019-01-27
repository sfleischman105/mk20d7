#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICSR {
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
#[doc = r" Value of the field"]
pub struct VECTACTIVER {
    bits: u16,
}
impl VECTACTIVER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `RETTOBASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETTOBASER {
    #[doc = "there are preempted active exceptions to execute"]
    _0,
    #[doc = "there are no active exceptions, or the currently-executing exception is the only active exception"]
    _1,
}
impl RETTOBASER {
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
            RETTOBASER::_0 => false,
            RETTOBASER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RETTOBASER {
        match value {
            false => RETTOBASER::_0,
            true => RETTOBASER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RETTOBASER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RETTOBASER::_1
    }
}
#[doc = r" Value of the field"]
pub struct VECTPENDINGR {
    bits: u8,
}
impl VECTPENDINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ISRPENDINGR {
    bits: bool,
}
impl ISRPENDINGR {
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
#[doc = "Possible values of the field `ISRPREEMPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISRPREEMPTR {
    #[doc = "Will not service"]
    _0,
    #[doc = "Will service a pending exception"]
    _1,
}
impl ISRPREEMPTR {
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
            ISRPREEMPTR::_0 => false,
            ISRPREEMPTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISRPREEMPTR {
        match value {
            false => ISRPREEMPTR::_0,
            true => ISRPREEMPTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ISRPREEMPTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ISRPREEMPTR::_1
    }
}
#[doc = "Possible values of the field `PENDSTSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTSETR {
    #[doc = "write: no effect; read: SysTick exception is not pending"]
    _0,
    #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    _1,
}
impl PENDSTSETR {
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
            PENDSTSETR::_0 => false,
            PENDSTSETR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENDSTSETR {
        match value {
            false => PENDSTSETR::_0,
            true => PENDSTSETR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PENDSTSETR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PENDSTSETR::_1
    }
}
#[doc = "Possible values of the field `PENDSVSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVSETR {
    #[doc = "write: no effect; read: PendSV exception is not pending"]
    _0,
    #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    _1,
}
impl PENDSVSETR {
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
            PENDSVSETR::_0 => false,
            PENDSVSETR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENDSVSETR {
        match value {
            false => PENDSVSETR::_0,
            true => PENDSVSETR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PENDSVSETR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PENDSVSETR::_1
    }
}
#[doc = "Possible values of the field `NMIPENDSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIPENDSETR {
    #[doc = "write: no effect; read: NMI exception is not pending"]
    _0,
    #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
    _1,
}
impl NMIPENDSETR {
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
            NMIPENDSETR::_0 => false,
            NMIPENDSETR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NMIPENDSETR {
        match value {
            false => NMIPENDSETR::_0,
            true => NMIPENDSETR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NMIPENDSETR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NMIPENDSETR::_1
    }
}
#[doc = "Values that can be written to the field `PENDSTCLR`"]
pub enum PENDSTCLRW {
    #[doc = "no effect"]
    _0,
    #[doc = "removes the pending state from the SysTick exception"]
    _1,
}
impl PENDSTCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSTCLRW::_0 => false,
            PENDSTCLRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSTCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSTCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSTCLRW::_0)
    }
    #[doc = "removes the pending state from the SysTick exception"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSTCLRW::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSTSET`"]
pub enum PENDSTSETW {
    #[doc = "write: no effect; read: SysTick exception is not pending"]
    _0,
    #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    _1,
}
impl PENDSTSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSTSETW::_0 => false,
            PENDSTSETW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSTSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSTSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write: no effect; read: SysTick exception is not pending"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSTSETW::_0)
    }
    #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSTSETW::_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSVCLR`"]
pub enum PENDSVCLRW {
    #[doc = "no effect"]
    _0,
    #[doc = "removes the pending state from the PendSV exception"]
    _1,
}
impl PENDSVCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSVCLRW::_0 => false,
            PENDSVCLRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSVCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSVCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSVCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSVCLRW::_0)
    }
    #[doc = "removes the pending state from the PendSV exception"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSVCLRW::_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSVSET`"]
pub enum PENDSVSETW {
    #[doc = "write: no effect; read: PendSV exception is not pending"]
    _0,
    #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    _1,
}
impl PENDSVSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSVSETW::_0 => false,
            PENDSVSETW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENDSVSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSVSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENDSVSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write: no effect; read: PendSV exception is not pending"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSVSETW::_0)
    }
    #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSVSETW::_1)
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
#[doc = "Values that can be written to the field `NMIPENDSET`"]
pub enum NMIPENDSETW {
    #[doc = "write: no effect; read: NMI exception is not pending"]
    _0,
    #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
    _1,
}
impl NMIPENDSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NMIPENDSETW::_0 => false,
            NMIPENDSETW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NMIPENDSETW<'a> {
    w: &'a mut W,
}
impl<'a> _NMIPENDSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NMIPENDSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write: no effect; read: NMI exception is not pending"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NMIPENDSETW::_0)
    }
    #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NMIPENDSETW::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:8 - Active exception number"]
    #[inline]
    pub fn vectactive(&self) -> VECTACTIVER {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VECTACTIVER { bits }
    }
    #[doc = "Bit 11 - no description available"]
    #[inline]
    pub fn rettobase(&self) -> RETTOBASER {
        RETTOBASER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:17 - Exception number of the highest priority pending enabled exception"]
    #[inline]
    pub fn vectpending(&self) -> VECTPENDINGR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VECTPENDINGR { bits }
    }
    #[doc = "Bit 22 - no description available"]
    #[inline]
    pub fn isrpending(&self) -> ISRPENDINGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ISRPENDINGR { bits }
    }
    #[doc = "Bit 23 - no description available"]
    #[inline]
    pub fn isrpreempt(&self) -> ISRPREEMPTR {
        ISRPREEMPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - no description available"]
    #[inline]
    pub fn pendstset(&self) -> PENDSTSETR {
        PENDSTSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - no description available"]
    #[inline]
    pub fn pendsvset(&self) -> PENDSVSETR {
        PENDSVSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - no description available"]
    #[inline]
    pub fn nmipendset(&self) -> NMIPENDSETR {
        NMIPENDSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 25 - no description available"]
    #[inline]
    pub fn pendstclr(&mut self) -> _PENDSTCLRW {
        _PENDSTCLRW { w: self }
    }
    #[doc = "Bit 26 - no description available"]
    #[inline]
    pub fn pendstset(&mut self) -> _PENDSTSETW {
        _PENDSTSETW { w: self }
    }
    #[doc = "Bit 27 - no description available"]
    #[inline]
    pub fn pendsvclr(&mut self) -> _PENDSVCLRW {
        _PENDSVCLRW { w: self }
    }
    #[doc = "Bit 28 - no description available"]
    #[inline]
    pub fn pendsvset(&mut self) -> _PENDSVSETW {
        _PENDSVSETW { w: self }
    }
    #[doc = "Bit 31 - no description available"]
    #[inline]
    pub fn nmipendset(&mut self) -> _NMIPENDSETW {
        _NMIPENDSETW { w: self }
    }
}
