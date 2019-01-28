#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT5 {
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
#[doc = "Possible values of the field `UART0TXSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0TXSRCR {
    #[doc = "UART0_TX pin"]
    _00,
    #[doc = "UART0_TX pin modulated with FTM1 channel 0 output"]
    _01,
    #[doc = "UART0_TX pin modulated with FTM2 channel 0 output"]
    _10,
    #[doc = "Reserved"]
    _11,
}
impl UART0TXSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UART0TXSRCR::_00 => 0,
            UART0TXSRCR::_01 => 1,
            UART0TXSRCR::_10 => 2,
            UART0TXSRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UART0TXSRCR {
        match value {
            0 => UART0TXSRCR::_00,
            1 => UART0TXSRCR::_01,
            2 => UART0TXSRCR::_10,
            3 => UART0TXSRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == UART0TXSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == UART0TXSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == UART0TXSRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == UART0TXSRCR::_11
    }
}
#[doc = "Possible values of the field `UART0RXSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0RXSRCR {
    #[doc = "UART0_RX pin"]
    _00,
    #[doc = "CMP0"]
    _01,
    #[doc = "CMP1"]
    _10,
    #[doc = "Reserved"]
    _11,
}
impl UART0RXSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UART0RXSRCR::_00 => 0,
            UART0RXSRCR::_01 => 1,
            UART0RXSRCR::_10 => 2,
            UART0RXSRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UART0RXSRCR {
        match value {
            0 => UART0RXSRCR::_00,
            1 => UART0RXSRCR::_01,
            2 => UART0RXSRCR::_10,
            3 => UART0RXSRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == UART0RXSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == UART0RXSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == UART0RXSRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == UART0RXSRCR::_11
    }
}
#[doc = "Possible values of the field `UART1TXSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1TXSRCR {
    #[doc = "UART1_TX pin"]
    _00,
    #[doc = "UART1_TX pin modulated with FTM1 channel 0 output"]
    _01,
    #[doc = "UART1_TX pin modulated with FTM2 channel 0 output"]
    _10,
    #[doc = "Reserved"]
    _11,
}
impl UART1TXSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UART1TXSRCR::_00 => 0,
            UART1TXSRCR::_01 => 1,
            UART1TXSRCR::_10 => 2,
            UART1TXSRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UART1TXSRCR {
        match value {
            0 => UART1TXSRCR::_00,
            1 => UART1TXSRCR::_01,
            2 => UART1TXSRCR::_10,
            3 => UART1TXSRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == UART1TXSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == UART1TXSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == UART1TXSRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == UART1TXSRCR::_11
    }
}
#[doc = "Possible values of the field `UART1RXSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1RXSRCR {
    #[doc = "UART1_RX pin"]
    _00,
    #[doc = "CMP0"]
    _01,
    #[doc = "CMP1"]
    _10,
    #[doc = "Reserved"]
    _11,
}
impl UART1RXSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UART1RXSRCR::_00 => 0,
            UART1RXSRCR::_01 => 1,
            UART1RXSRCR::_10 => 2,
            UART1RXSRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UART1RXSRCR {
        match value {
            0 => UART1RXSRCR::_00,
            1 => UART1RXSRCR::_01,
            2 => UART1RXSRCR::_10,
            3 => UART1RXSRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == UART1RXSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == UART1RXSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == UART1RXSRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == UART1RXSRCR::_11
    }
}
#[doc = "Values that can be written to the field `UART0TXSRC`"]
pub enum UART0TXSRCW {
    #[doc = "UART0_TX pin"]
    _00,
    #[doc = "UART0_TX pin modulated with FTM1 channel 0 output"]
    _01,
    #[doc = "UART0_TX pin modulated with FTM2 channel 0 output"]
    _10,
    #[doc = "Reserved"]
    _11,
}
impl UART0TXSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UART0TXSRCW::_00 => 0,
            UART0TXSRCW::_01 => 1,
            UART0TXSRCW::_10 => 2,
            UART0TXSRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART0TXSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _UART0TXSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART0TXSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "UART0_TX pin"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART0TXSRCW::_00)
    }
    #[doc = "UART0_TX pin modulated with FTM1 channel 0 output"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART0TXSRCW::_01)
    }
    #[doc = "UART0_TX pin modulated with FTM2 channel 0 output"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART0TXSRCW::_10)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(UART0TXSRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART0RXSRC`"]
pub enum UART0RXSRCW {
    #[doc = "UART0_RX pin"]
    _00,
    #[doc = "CMP0"]
    _01,
    #[doc = "CMP1"]
    _10,
    #[doc = "Reserved"]
    _11,
}
impl UART0RXSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UART0RXSRCW::_00 => 0,
            UART0RXSRCW::_01 => 1,
            UART0RXSRCW::_10 => 2,
            UART0RXSRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART0RXSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _UART0RXSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART0RXSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "UART0_RX pin"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART0RXSRCW::_00)
    }
    #[doc = "CMP0"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART0RXSRCW::_01)
    }
    #[doc = "CMP1"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART0RXSRCW::_10)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(UART0RXSRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART1TXSRC`"]
pub enum UART1TXSRCW {
    #[doc = "UART1_TX pin"]
    _00,
    #[doc = "UART1_TX pin modulated with FTM1 channel 0 output"]
    _01,
    #[doc = "UART1_TX pin modulated with FTM2 channel 0 output"]
    _10,
    #[doc = "Reserved"]
    _11,
}
impl UART1TXSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UART1TXSRCW::_00 => 0,
            UART1TXSRCW::_01 => 1,
            UART1TXSRCW::_10 => 2,
            UART1TXSRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART1TXSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _UART1TXSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART1TXSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "UART1_TX pin"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART1TXSRCW::_00)
    }
    #[doc = "UART1_TX pin modulated with FTM1 channel 0 output"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART1TXSRCW::_01)
    }
    #[doc = "UART1_TX pin modulated with FTM2 channel 0 output"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART1TXSRCW::_10)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(UART1TXSRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART1RXSRC`"]
pub enum UART1RXSRCW {
    #[doc = "UART1_RX pin"]
    _00,
    #[doc = "CMP0"]
    _01,
    #[doc = "CMP1"]
    _10,
    #[doc = "Reserved"]
    _11,
}
impl UART1RXSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UART1RXSRCW::_00 => 0,
            UART1RXSRCW::_01 => 1,
            UART1RXSRCW::_10 => 2,
            UART1RXSRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART1RXSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _UART1RXSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART1RXSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "UART1_RX pin"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART1RXSRCW::_00)
    }
    #[doc = "CMP0"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART1RXSRCW::_01)
    }
    #[doc = "CMP1"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART1RXSRCW::_10)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(UART1RXSRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:1 - UART 0 transmit data source select"]
    #[inline]
    pub fn uart0txsrc(&self) -> UART0TXSRCR {
        UART0TXSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - UART 0 receive data source select"]
    #[inline]
    pub fn uart0rxsrc(&self) -> UART0RXSRCR {
        UART0RXSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - UART 1 transmit data source select"]
    #[inline]
    pub fn uart1txsrc(&self) -> UART1TXSRCR {
        UART1TXSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - UART 1 receive data source select"]
    #[inline]
    pub fn uart1rxsrc(&self) -> UART1RXSRCR {
        UART1RXSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:1 - UART 0 transmit data source select"]
    #[inline]
    pub fn uart0txsrc(&mut self) -> _UART0TXSRCW {
        _UART0TXSRCW { w: self }
    }
    #[doc = "Bits 2:3 - UART 0 receive data source select"]
    #[inline]
    pub fn uart0rxsrc(&mut self) -> _UART0RXSRCW {
        _UART0RXSRCW { w: self }
    }
    #[doc = "Bits 4:5 - UART 1 transmit data source select"]
    #[inline]
    pub fn uart1txsrc(&mut self) -> _UART1TXSRCW {
        _UART1TXSRCW { w: self }
    }
    #[doc = "Bits 6:7 - UART 1 receive data source select"]
    #[inline]
    pub fn uart1rxsrc(&mut self) -> _UART1RXSRCW {
        _UART1RXSRCW { w: self }
    }
}
