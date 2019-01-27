#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT2 {
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
#[doc = "Possible values of the field `RTCCLKOUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCLKOUTSELR {
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0,
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    _1,
}
impl RTCCLKOUTSELR {
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
            RTCCLKOUTSELR::_0 => false,
            RTCCLKOUTSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCCLKOUTSELR {
        match value {
            false => RTCCLKOUTSELR::_0,
            true => RTCCLKOUTSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RTCCLKOUTSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RTCCLKOUTSELR::_1
    }
}
#[doc = "Possible values of the field `CLKOUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSELR {
    #[doc = "FlexBus CLKOUT"]
    _000,
    #[doc = "Reserved"]
    _001,
    #[doc = "Flash clock"]
    _010,
    #[doc = "LPO clock (1 kHz)"]
    _011,
    #[doc = "IRCLK"]
    _100,
    #[doc = "RTC 32.768kHz"]
    _101,
    #[doc = "ERCLK0"]
    _110,
    #[doc = "Reserved"]
    _111,
}
impl CLKOUTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKOUTSELR::_000 => 0,
            CLKOUTSELR::_001 => 1,
            CLKOUTSELR::_010 => 2,
            CLKOUTSELR::_011 => 3,
            CLKOUTSELR::_100 => 4,
            CLKOUTSELR::_101 => 5,
            CLKOUTSELR::_110 => 6,
            CLKOUTSELR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKOUTSELR {
        match value {
            0 => CLKOUTSELR::_000,
            1 => CLKOUTSELR::_001,
            2 => CLKOUTSELR::_010,
            3 => CLKOUTSELR::_011,
            4 => CLKOUTSELR::_100,
            5 => CLKOUTSELR::_101,
            6 => CLKOUTSELR::_110,
            7 => CLKOUTSELR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == CLKOUTSELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == CLKOUTSELR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == CLKOUTSELR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == CLKOUTSELR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == CLKOUTSELR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == CLKOUTSELR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == CLKOUTSELR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == CLKOUTSELR::_111
    }
}
#[doc = "Possible values of the field `FBSL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBSLR {
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus are disallowed."]
    _00,
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus are disallowed."]
    _01,
    #[doc = "Off-chip instruction accesses are disallowed. Data accesses are allowed."]
    _10,
    #[doc = "Off-chip instruction accesses and data accesses are allowed."]
    _11,
}
impl FBSLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FBSLR::_00 => 0,
            FBSLR::_01 => 1,
            FBSLR::_10 => 2,
            FBSLR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FBSLR {
        match value {
            0 => FBSLR::_00,
            1 => FBSLR::_01,
            2 => FBSLR::_10,
            3 => FBSLR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FBSLR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FBSLR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FBSLR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FBSLR::_11
    }
}
#[doc = "Possible values of the field `PTD7PAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTD7PADR {
    #[doc = "Single-pad drive strength for PTD7."]
    _0,
    #[doc = "Double pad drive strength for PTD7."]
    _1,
}
impl PTD7PADR {
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
            PTD7PADR::_0 => false,
            PTD7PADR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PTD7PADR {
        match value {
            false => PTD7PADR::_0,
            true => PTD7PADR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PTD7PADR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PTD7PADR::_1
    }
}
#[doc = "Possible values of the field `TRACECLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACECLKSELR {
    #[doc = "MCGOUTCLK"]
    _0,
    #[doc = "Core/system clock"]
    _1,
}
impl TRACECLKSELR {
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
            TRACECLKSELR::_0 => false,
            TRACECLKSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRACECLKSELR {
        match value {
            false => TRACECLKSELR::_0,
            true => TRACECLKSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRACECLKSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRACECLKSELR::_1
    }
}
#[doc = "Possible values of the field `PLLFLLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLFLLSELR {
    #[doc = "MCGFLLCLK clock"]
    _0,
    #[doc = "MCGPLLCLK clock"]
    _1,
}
impl PLLFLLSELR {
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
            PLLFLLSELR::_0 => false,
            PLLFLLSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLFLLSELR {
        match value {
            false => PLLFLLSELR::_0,
            true => PLLFLLSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLLFLLSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLLFLLSELR::_1
    }
}
#[doc = "Possible values of the field `USBSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSRCR {
    #[doc = "External bypass clock (USB_CLKIN)."]
    _0,
    #[doc = "MCGPLLCLK/MCGFLLCLK clock divided by the USB fractional divider. See the SIM_CLKDIV2\\[USBFRAC, USBDIV\\] descriptions."]
    _1,
}
impl USBSRCR {
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
            USBSRCR::_0 => false,
            USBSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBSRCR {
        match value {
            false => USBSRCR::_0,
            true => USBSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USBSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USBSRCR::_1
    }
}
#[doc = "Values that can be written to the field `RTCCLKOUTSEL`"]
pub enum RTCCLKOUTSELW {
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0,
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    _1,
}
impl RTCCLKOUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCCLKOUTSELW::_0 => false,
            RTCCLKOUTSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCCLKOUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCCLKOUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCCLKOUTSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCCLKOUTSELW::_0)
    }
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCCLKOUTSELW::_1)
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
#[doc = "Values that can be written to the field `CLKOUTSEL`"]
pub enum CLKOUTSELW {
    #[doc = "FlexBus CLKOUT"]
    _000,
    #[doc = "Reserved"]
    _001,
    #[doc = "Flash clock"]
    _010,
    #[doc = "LPO clock (1 kHz)"]
    _011,
    #[doc = "IRCLK"]
    _100,
    #[doc = "RTC 32.768kHz"]
    _101,
    #[doc = "ERCLK0"]
    _110,
    #[doc = "Reserved"]
    _111,
}
impl CLKOUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSELW::_000 => 0,
            CLKOUTSELW::_001 => 1,
            CLKOUTSELW::_010 => 2,
            CLKOUTSELW::_011 => 3,
            CLKOUTSELW::_100 => 4,
            CLKOUTSELW::_101 => 5,
            CLKOUTSELW::_110 => 6,
            CLKOUTSELW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOUTSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FlexBus CLKOUT"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_000)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_001)
    }
    #[doc = "Flash clock"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_010)
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_011)
    }
    #[doc = "IRCLK"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_100)
    }
    #[doc = "RTC 32.768kHz"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_101)
    }
    #[doc = "ERCLK0"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_110)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FBSL`"]
pub enum FBSLW {
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus are disallowed."]
    _00,
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus are disallowed."]
    _01,
    #[doc = "Off-chip instruction accesses are disallowed. Data accesses are allowed."]
    _10,
    #[doc = "Off-chip instruction accesses and data accesses are allowed."]
    _11,
}
impl FBSLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FBSLW::_00 => 0,
            FBSLW::_01 => 1,
            FBSLW::_10 => 2,
            FBSLW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FBSLW<'a> {
    w: &'a mut W,
}
impl<'a> _FBSLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FBSLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus are disallowed."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FBSLW::_00)
    }
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus are disallowed."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FBSLW::_01)
    }
    #[doc = "Off-chip instruction accesses are disallowed. Data accesses are allowed."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FBSLW::_10)
    }
    #[doc = "Off-chip instruction accesses and data accesses are allowed."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FBSLW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PTD7PAD`"]
pub enum PTD7PADW {
    #[doc = "Single-pad drive strength for PTD7."]
    _0,
    #[doc = "Double pad drive strength for PTD7."]
    _1,
}
impl PTD7PADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTD7PADW::_0 => false,
            PTD7PADW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTD7PADW<'a> {
    w: &'a mut W,
}
impl<'a> _PTD7PADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTD7PADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single-pad drive strength for PTD7."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTD7PADW::_0)
    }
    #[doc = "Double pad drive strength for PTD7."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTD7PADW::_1)
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
#[doc = "Values that can be written to the field `TRACECLKSEL`"]
pub enum TRACECLKSELW {
    #[doc = "MCGOUTCLK"]
    _0,
    #[doc = "Core/system clock"]
    _1,
}
impl TRACECLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRACECLKSELW::_0 => false,
            TRACECLKSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRACECLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACECLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRACECLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCGOUTCLK"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRACECLKSELW::_0)
    }
    #[doc = "Core/system clock"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRACECLKSELW::_1)
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
#[doc = "Values that can be written to the field `PLLFLLSEL`"]
pub enum PLLFLLSELW {
    #[doc = "MCGFLLCLK clock"]
    _0,
    #[doc = "MCGPLLCLK clock"]
    _1,
}
impl PLLFLLSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLFLLSELW::_0 => false,
            PLLFLLSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLFLLSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLFLLSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLFLLSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCGFLLCLK clock"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLFLLSELW::_0)
    }
    #[doc = "MCGPLLCLK clock"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLFLLSELW::_1)
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
#[doc = "Values that can be written to the field `USBSRC`"]
pub enum USBSRCW {
    #[doc = "External bypass clock (USB_CLKIN)."]
    _0,
    #[doc = "MCGPLLCLK/MCGFLLCLK clock divided by the USB fractional divider. See the SIM_CLKDIV2\\[USBFRAC, USBDIV\\] descriptions."]
    _1,
}
impl USBSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBSRCW::_0 => false,
            USBSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _USBSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External bypass clock (USB_CLKIN)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBSRCW::_0)
    }
    #[doc = "MCGPLLCLK/MCGFLLCLK clock divided by the USB fractional divider. See the SIM_CLKDIV2\\[USBFRAC, USBDIV\\] descriptions."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBSRCW::_1)
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
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline]
    pub fn rtcclkoutsel(&self) -> RTCCLKOUTSELR {
        RTCCLKOUTSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline]
    pub fn clkoutsel(&self) -> CLKOUTSELR {
        CLKOUTSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - FlexBus security level"]
    #[inline]
    pub fn fbsl(&self) -> FBSLR {
        FBSLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - PTD7 pad drive strength"]
    #[inline]
    pub fn ptd7pad(&self) -> PTD7PADR {
        PTD7PADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline]
    pub fn traceclksel(&self) -> TRACECLKSELR {
        TRACECLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - PLL/FLL clock select"]
    #[inline]
    pub fn pllfllsel(&self) -> PLLFLLSELR {
        PLLFLLSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline]
    pub fn usbsrc(&self) -> USBSRCR {
        USBSRCR::_from({
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
        W { bits: 4096 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline]
    pub fn rtcclkoutsel(&mut self) -> _RTCCLKOUTSELW {
        _RTCCLKOUTSELW { w: self }
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline]
    pub fn clkoutsel(&mut self) -> _CLKOUTSELW {
        _CLKOUTSELW { w: self }
    }
    #[doc = "Bits 8:9 - FlexBus security level"]
    #[inline]
    pub fn fbsl(&mut self) -> _FBSLW {
        _FBSLW { w: self }
    }
    #[doc = "Bit 11 - PTD7 pad drive strength"]
    #[inline]
    pub fn ptd7pad(&mut self) -> _PTD7PADW {
        _PTD7PADW { w: self }
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline]
    pub fn traceclksel(&mut self) -> _TRACECLKSELW {
        _TRACECLKSELW { w: self }
    }
    #[doc = "Bit 16 - PLL/FLL clock select"]
    #[inline]
    pub fn pllfllsel(&mut self) -> _PLLFLLSELW {
        _PLLFLLSELW { w: self }
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline]
    pub fn usbsrc(&mut self) -> _USBSRCW {
        _USBSRCW { w: self }
    }
}
