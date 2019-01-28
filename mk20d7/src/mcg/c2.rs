#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C2 {
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
#[doc = "Possible values of the field `IRCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCSR {
    #[doc = "Slow internal reference clock selected."]
    _0,
    #[doc = "Fast internal reference clock selected."]
    _1,
}
impl IRCSR {
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
            IRCSR::_0 => false,
            IRCSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRCSR {
        match value {
            false => IRCSR::_0,
            true => IRCSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IRCSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IRCSR::_1
    }
}
#[doc = "Possible values of the field `LP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPR {
    #[doc = "FLL (or PLL) is not disabled in bypass modes."]
    _0,
    #[doc = "FLL (or PLL) is disabled in bypass modes (lower power)"]
    _1,
}
impl LPR {
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
            LPR::_0 => false,
            LPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPR {
        match value {
            false => LPR::_0,
            true => LPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPR::_1
    }
}
#[doc = "Possible values of the field `EREFS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EREFS0R {
    #[doc = "External reference clock requested."]
    _0,
    #[doc = "Oscillator requested."]
    _1,
}
impl EREFS0R {
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
            EREFS0R::_0 => false,
            EREFS0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EREFS0R {
        match value {
            false => EREFS0R::_0,
            true => EREFS0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EREFS0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EREFS0R::_1
    }
}
#[doc = "Possible values of the field `HGO0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HGO0R {
    #[doc = "Configure crystal oscillator for low-power operation."]
    _0,
    #[doc = "Configure crystal oscillator for high-gain operation."]
    _1,
}
impl HGO0R {
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
            HGO0R::_0 => false,
            HGO0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HGO0R {
        match value {
            false => HGO0R::_0,
            true => HGO0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HGO0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HGO0R::_1
    }
}
#[doc = "Possible values of the field `RANGE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANGE0R {
    #[doc = "Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    _00,
    #[doc = "Encoding 1 - High frequency range selected for the crystal oscillator ."]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RANGE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RANGE0R::_00 => 0,
            RANGE0R::_01 => 1,
            RANGE0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RANGE0R {
        match value {
            0 => RANGE0R::_00,
            1 => RANGE0R::_01,
            i => RANGE0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RANGE0R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RANGE0R::_01
    }
}
#[doc = "Possible values of the field `LOCRE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCRE0R {
    #[doc = "Interrupt request is generated on a loss of OSC0 external reference clock."]
    _0,
    #[doc = "Generate a reset request on a loss of OSC0 external reference clock"]
    _1,
}
impl LOCRE0R {
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
            LOCRE0R::_0 => false,
            LOCRE0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCRE0R {
        match value {
            false => LOCRE0R::_0,
            true => LOCRE0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOCRE0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOCRE0R::_1
    }
}
#[doc = "Values that can be written to the field `IRCS`"]
pub enum IRCSW {
    #[doc = "Slow internal reference clock selected."]
    _0,
    #[doc = "Fast internal reference clock selected."]
    _1,
}
impl IRCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRCSW::_0 => false,
            IRCSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRCSW<'a> {
    w: &'a mut W,
}
impl<'a> _IRCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slow internal reference clock selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRCSW::_0)
    }
    #[doc = "Fast internal reference clock selected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRCSW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LP`"]
pub enum LPW {
    #[doc = "FLL (or PLL) is not disabled in bypass modes."]
    _0,
    #[doc = "FLL (or PLL) is disabled in bypass modes (lower power)"]
    _1,
}
impl LPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPW::_0 => false,
            LPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPW<'a> {
    w: &'a mut W,
}
impl<'a> _LPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FLL (or PLL) is not disabled in bypass modes."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPW::_0)
    }
    #[doc = "FLL (or PLL) is disabled in bypass modes (lower power)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EREFS0`"]
pub enum EREFS0W {
    #[doc = "External reference clock requested."]
    _0,
    #[doc = "Oscillator requested."]
    _1,
}
impl EREFS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EREFS0W::_0 => false,
            EREFS0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EREFS0W<'a> {
    w: &'a mut W,
}
impl<'a> _EREFS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EREFS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External reference clock requested."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EREFS0W::_0)
    }
    #[doc = "Oscillator requested."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EREFS0W::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HGO0`"]
pub enum HGO0W {
    #[doc = "Configure crystal oscillator for low-power operation."]
    _0,
    #[doc = "Configure crystal oscillator for high-gain operation."]
    _1,
}
impl HGO0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HGO0W::_0 => false,
            HGO0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HGO0W<'a> {
    w: &'a mut W,
}
impl<'a> _HGO0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HGO0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure crystal oscillator for low-power operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HGO0W::_0)
    }
    #[doc = "Configure crystal oscillator for high-gain operation."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HGO0W::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RANGE0`"]
pub enum RANGE0W {
    #[doc = "Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    _00,
    #[doc = "Encoding 1 - High frequency range selected for the crystal oscillator ."]
    _01,
}
impl RANGE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RANGE0W::_00 => 0,
            RANGE0W::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RANGE0W<'a> {
    w: &'a mut W,
}
impl<'a> _RANGE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RANGE0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RANGE0W::_00)
    }
    #[doc = "Encoding 1 - High frequency range selected for the crystal oscillator ."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RANGE0W::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOCRE0`"]
pub enum LOCRE0W {
    #[doc = "Interrupt request is generated on a loss of OSC0 external reference clock."]
    _0,
    #[doc = "Generate a reset request on a loss of OSC0 external reference clock"]
    _1,
}
impl LOCRE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCRE0W::_0 => false,
            LOCRE0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCRE0W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCRE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCRE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt request is generated on a loss of OSC0 external reference clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCRE0W::_0)
    }
    #[doc = "Generate a reset request on a loss of OSC0 external reference clock"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCRE0W::_1)
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
    #[doc = "Bit 0 - Internal Reference Clock Select"]
    #[inline]
    pub fn ircs(&self) -> IRCSR {
        IRCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Low Power Select"]
    #[inline]
    pub fn lp(&self) -> LPR {
        LPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline]
    pub fn erefs0(&self) -> EREFS0R {
        EREFS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline]
    pub fn hgo0(&self) -> HGO0R {
        HGO0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 4:5 - Frequency Range Select"]
    #[inline]
    pub fn range0(&self) -> RANGE0R {
        RANGE0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline]
    pub fn locre0(&self) -> LOCRE0R {
        LOCRE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 128 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Internal Reference Clock Select"]
    #[inline]
    pub fn ircs(&mut self) -> _IRCSW {
        _IRCSW { w: self }
    }
    #[doc = "Bit 1 - Low Power Select"]
    #[inline]
    pub fn lp(&mut self) -> _LPW {
        _LPW { w: self }
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline]
    pub fn erefs0(&mut self) -> _EREFS0W {
        _EREFS0W { w: self }
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline]
    pub fn hgo0(&mut self) -> _HGO0W {
        _HGO0W { w: self }
    }
    #[doc = "Bits 4:5 - Frequency Range Select"]
    #[inline]
    pub fn range0(&mut self) -> _RANGE0W {
        _RANGE0W { w: self }
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline]
    pub fn locre0(&mut self) -> _LOCRE0W {
        _LOCRE0W { w: self }
    }
}
