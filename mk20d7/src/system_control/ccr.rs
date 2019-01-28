#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `NONBASETHRDENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NONBASETHRDENAR {
    #[doc = "processor can enter Thread mode only when no exception is active"]
    _0,
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    _1,
}
impl NONBASETHRDENAR {
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
            NONBASETHRDENAR::_0 => false,
            NONBASETHRDENAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NONBASETHRDENAR {
        match value {
            false => NONBASETHRDENAR::_0,
            true => NONBASETHRDENAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NONBASETHRDENAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NONBASETHRDENAR::_1
    }
}
#[doc = "Possible values of the field `USERSETMPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USERSETMPENDR {
    #[doc = "disable"]
    _0,
    #[doc = "enable"]
    _1,
}
impl USERSETMPENDR {
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
            USERSETMPENDR::_0 => false,
            USERSETMPENDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USERSETMPENDR {
        match value {
            false => USERSETMPENDR::_0,
            true => USERSETMPENDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USERSETMPENDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USERSETMPENDR::_1
    }
}
#[doc = "Possible values of the field `UNALIGN_TRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGN_TRPR {
    #[doc = "do not trap unaligned halfword and word accesses"]
    _0,
    #[doc = "trap unaligned halfword and word accesses"]
    _1,
}
impl UNALIGN_TRPR {
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
            UNALIGN_TRPR::_0 => false,
            UNALIGN_TRPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNALIGN_TRPR {
        match value {
            false => UNALIGN_TRPR::_0,
            true => UNALIGN_TRPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UNALIGN_TRPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UNALIGN_TRPR::_1
    }
}
#[doc = "Possible values of the field `DIV_0_TRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV_0_TRPR {
    #[doc = "do not trap divide by 0"]
    _0,
    #[doc = "trap divide by 0"]
    _1,
}
impl DIV_0_TRPR {
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
            DIV_0_TRPR::_0 => false,
            DIV_0_TRPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIV_0_TRPR {
        match value {
            false => DIV_0_TRPR::_0,
            true => DIV_0_TRPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DIV_0_TRPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DIV_0_TRPR::_1
    }
}
#[doc = "Possible values of the field `BFHFNMIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFHFNMIGNR {
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    _0,
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    _1,
}
impl BFHFNMIGNR {
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
            BFHFNMIGNR::_0 => false,
            BFHFNMIGNR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFHFNMIGNR {
        match value {
            false => BFHFNMIGNR::_0,
            true => BFHFNMIGNR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BFHFNMIGNR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BFHFNMIGNR::_1
    }
}
#[doc = "Possible values of the field `STKALIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKALIGNR {
    #[doc = "4-byte aligned"]
    _0,
    #[doc = "8-byte aligned"]
    _1,
}
impl STKALIGNR {
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
            STKALIGNR::_0 => false,
            STKALIGNR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STKALIGNR {
        match value {
            false => STKALIGNR::_0,
            true => STKALIGNR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STKALIGNR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STKALIGNR::_1
    }
}
#[doc = "Values that can be written to the field `NONBASETHRDENA`"]
pub enum NONBASETHRDENAW {
    #[doc = "processor can enter Thread mode only when no exception is active"]
    _0,
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    _1,
}
impl NONBASETHRDENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NONBASETHRDENAW::_0 => false,
            NONBASETHRDENAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NONBASETHRDENAW<'a> {
    w: &'a mut W,
}
impl<'a> _NONBASETHRDENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NONBASETHRDENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "processor can enter Thread mode only when no exception is active"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONBASETHRDENAW::_0)
    }
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONBASETHRDENAW::_1)
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
#[doc = "Values that can be written to the field `USERSETMPEND`"]
pub enum USERSETMPENDW {
    #[doc = "disable"]
    _0,
    #[doc = "enable"]
    _1,
}
impl USERSETMPENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USERSETMPENDW::_0 => false,
            USERSETMPENDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USERSETMPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _USERSETMPENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USERSETMPENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USERSETMPENDW::_0)
    }
    #[doc = "enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USERSETMPENDW::_1)
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
#[doc = "Values that can be written to the field `UNALIGN_TRP`"]
pub enum UNALIGN_TRPW {
    #[doc = "do not trap unaligned halfword and word accesses"]
    _0,
    #[doc = "trap unaligned halfword and word accesses"]
    _1,
}
impl UNALIGN_TRPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNALIGN_TRPW::_0 => false,
            UNALIGN_TRPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNALIGN_TRPW<'a> {
    w: &'a mut W,
}
impl<'a> _UNALIGN_TRPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNALIGN_TRPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not trap unaligned halfword and word accesses"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNALIGN_TRPW::_0)
    }
    #[doc = "trap unaligned halfword and word accesses"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNALIGN_TRPW::_1)
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
#[doc = "Values that can be written to the field `DIV_0_TRP`"]
pub enum DIV_0_TRPW {
    #[doc = "do not trap divide by 0"]
    _0,
    #[doc = "trap divide by 0"]
    _1,
}
impl DIV_0_TRPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIV_0_TRPW::_0 => false,
            DIV_0_TRPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIV_0_TRPW<'a> {
    w: &'a mut W,
}
impl<'a> _DIV_0_TRPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIV_0_TRPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not trap divide by 0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIV_0_TRPW::_0)
    }
    #[doc = "trap divide by 0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIV_0_TRPW::_1)
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
#[doc = "Values that can be written to the field `BFHFNMIGN`"]
pub enum BFHFNMIGNW {
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    _0,
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    _1,
}
impl BFHFNMIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFHFNMIGNW::_0 => false,
            BFHFNMIGNW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFHFNMIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _BFHFNMIGNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFHFNMIGNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFHFNMIGNW::_0)
    }
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFHFNMIGNW::_1)
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
#[doc = "Values that can be written to the field `STKALIGN`"]
pub enum STKALIGNW {
    #[doc = "4-byte aligned"]
    _0,
    #[doc = "8-byte aligned"]
    _1,
}
impl STKALIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STKALIGNW::_0 => false,
            STKALIGNW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STKALIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _STKALIGNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STKALIGNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "4-byte aligned"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STKALIGNW::_0)
    }
    #[doc = "8-byte aligned"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STKALIGNW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn nonbasethrdena(&self) -> NONBASETHRDENAR {
        NONBASETHRDENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enables unprivileged software access to the STIR"]
    #[inline]
    pub fn usersetmpend(&self) -> USERSETMPENDR {
        USERSETMPENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline]
    pub fn unalign_trp(&self) -> UNALIGN_TRPR {
        UNALIGN_TRPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    #[inline]
    pub fn div_0_trp(&self) -> DIV_0_TRPR {
        DIV_0_TRPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    #[inline]
    pub fn bfhfnmign(&self) -> BFHFNMIGNR {
        BFHFNMIGNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline]
    pub fn stkalign(&self) -> STKALIGNR {
        STKALIGNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
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
    pub fn nonbasethrdena(&mut self) -> _NONBASETHRDENAW {
        _NONBASETHRDENAW { w: self }
    }
    #[doc = "Bit 1 - Enables unprivileged software access to the STIR"]
    #[inline]
    pub fn usersetmpend(&mut self) -> _USERSETMPENDW {
        _USERSETMPENDW { w: self }
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline]
    pub fn unalign_trp(&mut self) -> _UNALIGN_TRPW {
        _UNALIGN_TRPW { w: self }
    }
    #[doc = "Bit 4 - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    #[inline]
    pub fn div_0_trp(&mut self) -> _DIV_0_TRPW {
        _DIV_0_TRPW { w: self }
    }
    #[doc = "Bit 8 - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    #[inline]
    pub fn bfhfnmign(&mut self) -> _BFHFNMIGNW {
        _BFHFNMIGNW { w: self }
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline]
    pub fn stkalign(&mut self) -> _STKALIGNW {
        _STKALIGNW { w: self }
    }
}
