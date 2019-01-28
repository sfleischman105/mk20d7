#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFSR {
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
#[doc = "Possible values of the field `IACCVIOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IACCVIOLR {
    #[doc = "no instruction access violation fault"]
    _0,
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution"]
    _1,
}
impl IACCVIOLR {
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
            IACCVIOLR::_0 => false,
            IACCVIOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IACCVIOLR {
        match value {
            false => IACCVIOLR::_0,
            true => IACCVIOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IACCVIOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IACCVIOLR::_1
    }
}
#[doc = "Possible values of the field `DACCVIOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACCVIOLR {
    #[doc = "no data access violation fault"]
    _0,
    #[doc = "the processor attempted a load or store at a location that does not permit the operation"]
    _1,
}
impl DACCVIOLR {
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
            DACCVIOLR::_0 => false,
            DACCVIOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACCVIOLR {
        match value {
            false => DACCVIOLR::_0,
            true => DACCVIOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DACCVIOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DACCVIOLR::_1
    }
}
#[doc = "Possible values of the field `MUNSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUNSTKERRR {
    #[doc = "no unstacking fault"]
    _0,
    #[doc = "unstack for an exception return has caused one or more access violations"]
    _1,
}
impl MUNSTKERRR {
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
            MUNSTKERRR::_0 => false,
            MUNSTKERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MUNSTKERRR {
        match value {
            false => MUNSTKERRR::_0,
            true => MUNSTKERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MUNSTKERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MUNSTKERRR::_1
    }
}
#[doc = "Possible values of the field `MSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTKERRR {
    #[doc = "no stacking fault"]
    _0,
    #[doc = "stacking for an exception entry has caused one or more access violations"]
    _1,
}
impl MSTKERRR {
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
            MSTKERRR::_0 => false,
            MSTKERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTKERRR {
        match value {
            false => MSTKERRR::_0,
            true => MSTKERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MSTKERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MSTKERRR::_1
    }
}
#[doc = "Possible values of the field `MLSPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MLSPERRR {
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    _0,
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    _1,
}
impl MLSPERRR {
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
            MLSPERRR::_0 => false,
            MLSPERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MLSPERRR {
        match value {
            false => MLSPERRR::_0,
            true => MLSPERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MLSPERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MLSPERRR::_1
    }
}
#[doc = "Possible values of the field `MMARVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMARVALIDR {
    #[doc = "value in MMAR is not a valid fault address"]
    _0,
    #[doc = "MMAR holds a valid fault address"]
    _1,
}
impl MMARVALIDR {
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
            MMARVALIDR::_0 => false,
            MMARVALIDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMARVALIDR {
        match value {
            false => MMARVALIDR::_0,
            true => MMARVALIDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MMARVALIDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MMARVALIDR::_1
    }
}
#[doc = "Possible values of the field `IBUSERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBUSERRR {
    #[doc = "no instruction bus error"]
    _0,
    #[doc = "instruction bus error"]
    _1,
}
impl IBUSERRR {
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
            IBUSERRR::_0 => false,
            IBUSERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBUSERRR {
        match value {
            false => IBUSERRR::_0,
            true => IBUSERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IBUSERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IBUSERRR::_1
    }
}
#[doc = "Possible values of the field `PRECISERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRECISERRR {
    #[doc = "no precise data bus error"]
    _0,
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault"]
    _1,
}
impl PRECISERRR {
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
            PRECISERRR::_0 => false,
            PRECISERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRECISERRR {
        match value {
            false => PRECISERRR::_0,
            true => PRECISERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PRECISERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PRECISERRR::_1
    }
}
#[doc = "Possible values of the field `IMPRECISERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMPRECISERRR {
    #[doc = "no imprecise data bus error"]
    _0,
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error"]
    _1,
}
impl IMPRECISERRR {
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
            IMPRECISERRR::_0 => false,
            IMPRECISERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IMPRECISERRR {
        match value {
            false => IMPRECISERRR::_0,
            true => IMPRECISERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IMPRECISERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IMPRECISERRR::_1
    }
}
#[doc = "Possible values of the field `UNSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNSTKERRR {
    #[doc = "no unstacking fault"]
    _0,
    #[doc = "unstack for an exception return has caused one or more BusFaults"]
    _1,
}
impl UNSTKERRR {
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
            UNSTKERRR::_0 => false,
            UNSTKERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNSTKERRR {
        match value {
            false => UNSTKERRR::_0,
            true => UNSTKERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UNSTKERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UNSTKERRR::_1
    }
}
#[doc = "Possible values of the field `STKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKERRR {
    #[doc = "no stacking fault"]
    _0,
    #[doc = "stacking for an exception entry has caused one or more BusFaults"]
    _1,
}
impl STKERRR {
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
            STKERRR::_0 => false,
            STKERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STKERRR {
        match value {
            false => STKERRR::_0,
            true => STKERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STKERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STKERRR::_1
    }
}
#[doc = "Possible values of the field `LSPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPERRR {
    #[doc = "No bus fault occurred during floating-point lazy state preservation"]
    _0,
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    _1,
}
impl LSPERRR {
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
            LSPERRR::_0 => false,
            LSPERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSPERRR {
        match value {
            false => LSPERRR::_0,
            true => LSPERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LSPERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LSPERRR::_1
    }
}
#[doc = "Possible values of the field `BFARVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFARVALIDR {
    #[doc = "value in BFAR is not a valid fault address"]
    _0,
    #[doc = "BFAR holds a valid fault address"]
    _1,
}
impl BFARVALIDR {
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
            BFARVALIDR::_0 => false,
            BFARVALIDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFARVALIDR {
        match value {
            false => BFARVALIDR::_0,
            true => BFARVALIDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BFARVALIDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BFARVALIDR::_1
    }
}
#[doc = "Possible values of the field `UNDEFINSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDEFINSTRR {
    #[doc = "no undefined instruction UsageFault"]
    _0,
    #[doc = "the processor has attempted to execute an undefined instruction"]
    _1,
}
impl UNDEFINSTRR {
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
            UNDEFINSTRR::_0 => false,
            UNDEFINSTRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNDEFINSTRR {
        match value {
            false => UNDEFINSTRR::_0,
            true => UNDEFINSTRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UNDEFINSTRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UNDEFINSTRR::_1
    }
}
#[doc = "Possible values of the field `INVSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVSTATER {
    #[doc = "no invalid state UsageFault"]
    _0,
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR"]
    _1,
}
impl INVSTATER {
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
            INVSTATER::_0 => false,
            INVSTATER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVSTATER {
        match value {
            false => INVSTATER::_0,
            true => INVSTATER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INVSTATER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INVSTATER::_1
    }
}
#[doc = "Possible values of the field `INVPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVPCR {
    #[doc = "no invalid PC load UsageFault"]
    _0,
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC"]
    _1,
}
impl INVPCR {
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
            INVPCR::_0 => false,
            INVPCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVPCR {
        match value {
            false => INVPCR::_0,
            true => INVPCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INVPCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INVPCR::_1
    }
}
#[doc = "Possible values of the field `NOCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOCPR {
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    _0,
    #[doc = "the processor has attempted to access a coprocessor"]
    _1,
}
impl NOCPR {
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
            NOCPR::_0 => false,
            NOCPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NOCPR {
        match value {
            false => NOCPR::_0,
            true => NOCPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NOCPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NOCPR::_1
    }
}
#[doc = "Possible values of the field `UNALIGNED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGNEDR {
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    _0,
    #[doc = "the processor has made an unaligned memory access"]
    _1,
}
impl UNALIGNEDR {
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
            UNALIGNEDR::_0 => false,
            UNALIGNEDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNALIGNEDR {
        match value {
            false => UNALIGNEDR::_0,
            true => UNALIGNEDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UNALIGNEDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UNALIGNEDR::_1
    }
}
#[doc = "Possible values of the field `DIVBYZERO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVBYZEROR {
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    _0,
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    _1,
}
impl DIVBYZEROR {
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
            DIVBYZEROR::_0 => false,
            DIVBYZEROR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIVBYZEROR {
        match value {
            false => DIVBYZEROR::_0,
            true => DIVBYZEROR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DIVBYZEROR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DIVBYZEROR::_1
    }
}
#[doc = "Values that can be written to the field `IACCVIOL`"]
pub enum IACCVIOLW {
    #[doc = "no instruction access violation fault"]
    _0,
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution"]
    _1,
}
impl IACCVIOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IACCVIOLW::_0 => false,
            IACCVIOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IACCVIOLW<'a> {
    w: &'a mut W,
}
impl<'a> _IACCVIOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IACCVIOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no instruction access violation fault"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IACCVIOLW::_0)
    }
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IACCVIOLW::_1)
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
#[doc = "Values that can be written to the field `DACCVIOL`"]
pub enum DACCVIOLW {
    #[doc = "no data access violation fault"]
    _0,
    #[doc = "the processor attempted a load or store at a location that does not permit the operation"]
    _1,
}
impl DACCVIOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACCVIOLW::_0 => false,
            DACCVIOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACCVIOLW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCVIOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACCVIOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no data access violation fault"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACCVIOLW::_0)
    }
    #[doc = "the processor attempted a load or store at a location that does not permit the operation"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACCVIOLW::_1)
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
#[doc = "Values that can be written to the field `MUNSTKERR`"]
pub enum MUNSTKERRW {
    #[doc = "no unstacking fault"]
    _0,
    #[doc = "unstack for an exception return has caused one or more access violations"]
    _1,
}
impl MUNSTKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MUNSTKERRW::_0 => false,
            MUNSTKERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUNSTKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MUNSTKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUNSTKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no unstacking fault"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MUNSTKERRW::_0)
    }
    #[doc = "unstack for an exception return has caused one or more access violations"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MUNSTKERRW::_1)
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
#[doc = "Values that can be written to the field `MSTKERR`"]
pub enum MSTKERRW {
    #[doc = "no stacking fault"]
    _0,
    #[doc = "stacking for an exception entry has caused one or more access violations"]
    _1,
}
impl MSTKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTKERRW::_0 => false,
            MSTKERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no stacking fault"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTKERRW::_0)
    }
    #[doc = "stacking for an exception entry has caused one or more access violations"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTKERRW::_1)
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
#[doc = "Values that can be written to the field `MLSPERR`"]
pub enum MLSPERRW {
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    _0,
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    _1,
}
impl MLSPERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MLSPERRW::_0 => false,
            MLSPERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MLSPERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MLSPERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MLSPERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MLSPERRW::_0)
    }
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MLSPERRW::_1)
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
#[doc = "Values that can be written to the field `MMARVALID`"]
pub enum MMARVALIDW {
    #[doc = "value in MMAR is not a valid fault address"]
    _0,
    #[doc = "MMAR holds a valid fault address"]
    _1,
}
impl MMARVALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMARVALIDW::_0 => false,
            MMARVALIDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMARVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _MMARVALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMARVALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "value in MMAR is not a valid fault address"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MMARVALIDW::_0)
    }
    #[doc = "MMAR holds a valid fault address"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MMARVALIDW::_1)
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
#[doc = "Values that can be written to the field `IBUSERR`"]
pub enum IBUSERRW {
    #[doc = "no instruction bus error"]
    _0,
    #[doc = "instruction bus error"]
    _1,
}
impl IBUSERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBUSERRW::_0 => false,
            IBUSERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBUSERRW<'a> {
    w: &'a mut W,
}
impl<'a> _IBUSERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBUSERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no instruction bus error"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IBUSERRW::_0)
    }
    #[doc = "instruction bus error"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IBUSERRW::_1)
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
#[doc = "Values that can be written to the field `PRECISERR`"]
pub enum PRECISERRW {
    #[doc = "no precise data bus error"]
    _0,
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault"]
    _1,
}
impl PRECISERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRECISERRW::_0 => false,
            PRECISERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRECISERRW<'a> {
    w: &'a mut W,
}
impl<'a> _PRECISERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRECISERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no precise data bus error"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRECISERRW::_0)
    }
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRECISERRW::_1)
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
#[doc = "Values that can be written to the field `IMPRECISERR`"]
pub enum IMPRECISERRW {
    #[doc = "no imprecise data bus error"]
    _0,
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error"]
    _1,
}
impl IMPRECISERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IMPRECISERRW::_0 => false,
            IMPRECISERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IMPRECISERRW<'a> {
    w: &'a mut W,
}
impl<'a> _IMPRECISERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IMPRECISERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no imprecise data bus error"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IMPRECISERRW::_0)
    }
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IMPRECISERRW::_1)
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
#[doc = "Values that can be written to the field `UNSTKERR`"]
pub enum UNSTKERRW {
    #[doc = "no unstacking fault"]
    _0,
    #[doc = "unstack for an exception return has caused one or more BusFaults"]
    _1,
}
impl UNSTKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNSTKERRW::_0 => false,
            UNSTKERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNSTKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _UNSTKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNSTKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no unstacking fault"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNSTKERRW::_0)
    }
    #[doc = "unstack for an exception return has caused one or more BusFaults"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNSTKERRW::_1)
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
#[doc = "Values that can be written to the field `STKERR`"]
pub enum STKERRW {
    #[doc = "no stacking fault"]
    _0,
    #[doc = "stacking for an exception entry has caused one or more BusFaults"]
    _1,
}
impl STKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STKERRW::_0 => false,
            STKERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _STKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no stacking fault"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STKERRW::_0)
    }
    #[doc = "stacking for an exception entry has caused one or more BusFaults"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STKERRW::_1)
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
#[doc = "Values that can be written to the field `LSPERR`"]
pub enum LSPERRW {
    #[doc = "No bus fault occurred during floating-point lazy state preservation"]
    _0,
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    _1,
}
impl LSPERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSPERRW::_0 => false,
            LSPERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSPERRW<'a> {
    w: &'a mut W,
}
impl<'a> _LSPERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSPERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No bus fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSPERRW::_0)
    }
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSPERRW::_1)
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
#[doc = "Values that can be written to the field `BFARVALID`"]
pub enum BFARVALIDW {
    #[doc = "value in BFAR is not a valid fault address"]
    _0,
    #[doc = "BFAR holds a valid fault address"]
    _1,
}
impl BFARVALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFARVALIDW::_0 => false,
            BFARVALIDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFARVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _BFARVALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFARVALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "value in BFAR is not a valid fault address"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFARVALIDW::_0)
    }
    #[doc = "BFAR holds a valid fault address"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFARVALIDW::_1)
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
#[doc = "Values that can be written to the field `UNDEFINSTR`"]
pub enum UNDEFINSTRW {
    #[doc = "no undefined instruction UsageFault"]
    _0,
    #[doc = "the processor has attempted to execute an undefined instruction"]
    _1,
}
impl UNDEFINSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNDEFINSTRW::_0 => false,
            UNDEFINSTRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNDEFINSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _UNDEFINSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNDEFINSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no undefined instruction UsageFault"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNDEFINSTRW::_0)
    }
    #[doc = "the processor has attempted to execute an undefined instruction"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNDEFINSTRW::_1)
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
#[doc = "Values that can be written to the field `INVSTATE`"]
pub enum INVSTATEW {
    #[doc = "no invalid state UsageFault"]
    _0,
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR"]
    _1,
}
impl INVSTATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVSTATEW::_0 => false,
            INVSTATEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVSTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _INVSTATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVSTATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no invalid state UsageFault"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INVSTATEW::_0)
    }
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INVSTATEW::_1)
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
#[doc = "Values that can be written to the field `INVPC`"]
pub enum INVPCW {
    #[doc = "no invalid PC load UsageFault"]
    _0,
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC"]
    _1,
}
impl INVPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVPCW::_0 => false,
            INVPCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVPCW<'a> {
    w: &'a mut W,
}
impl<'a> _INVPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVPCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no invalid PC load UsageFault"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INVPCW::_0)
    }
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INVPCW::_1)
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
#[doc = "Values that can be written to the field `NOCP`"]
pub enum NOCPW {
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    _0,
    #[doc = "the processor has attempted to access a coprocessor"]
    _1,
}
impl NOCPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NOCPW::_0 => false,
            NOCPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NOCPW<'a> {
    w: &'a mut W,
}
impl<'a> _NOCPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NOCPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NOCPW::_0)
    }
    #[doc = "the processor has attempted to access a coprocessor"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NOCPW::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UNALIGNED`"]
pub enum UNALIGNEDW {
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    _0,
    #[doc = "the processor has made an unaligned memory access"]
    _1,
}
impl UNALIGNEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNALIGNEDW::_0 => false,
            UNALIGNEDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNALIGNEDW<'a> {
    w: &'a mut W,
}
impl<'a> _UNALIGNEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNALIGNEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNALIGNEDW::_0)
    }
    #[doc = "the processor has made an unaligned memory access"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNALIGNEDW::_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIVBYZERO`"]
pub enum DIVBYZEROW {
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    _0,
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    _1,
}
impl DIVBYZEROW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIVBYZEROW::_0 => false,
            DIVBYZEROW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVBYZEROW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVBYZEROW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVBYZEROW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIVBYZEROW::_0)
    }
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIVBYZEROW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn iaccviol(&self) -> IACCVIOLR {
        IACCVIOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn daccviol(&self) -> DACCVIOLR {
        DACCVIOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - no description available"]
    #[inline]
    pub fn munstkerr(&self) -> MUNSTKERRR {
        MUNSTKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - no description available"]
    #[inline]
    pub fn mstkerr(&self) -> MSTKERRR {
        MSTKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - no description available"]
    #[inline]
    pub fn mlsperr(&self) -> MLSPERRR {
        MLSPERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - no description available"]
    #[inline]
    pub fn mmarvalid(&self) -> MMARVALIDR {
        MMARVALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - no description available"]
    #[inline]
    pub fn ibuserr(&self) -> IBUSERRR {
        IBUSERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - no description available"]
    #[inline]
    pub fn preciserr(&self) -> PRECISERRR {
        PRECISERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - no description available"]
    #[inline]
    pub fn impreciserr(&self) -> IMPRECISERRR {
        IMPRECISERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - no description available"]
    #[inline]
    pub fn unstkerr(&self) -> UNSTKERRR {
        UNSTKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - no description available"]
    #[inline]
    pub fn stkerr(&self) -> STKERRR {
        STKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - no description available"]
    #[inline]
    pub fn lsperr(&self) -> LSPERRR {
        LSPERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - no description available"]
    #[inline]
    pub fn bfarvalid(&self) -> BFARVALIDR {
        BFARVALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - no description available"]
    #[inline]
    pub fn undefinstr(&self) -> UNDEFINSTRR {
        UNDEFINSTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - no description available"]
    #[inline]
    pub fn invstate(&self) -> INVSTATER {
        INVSTATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - no description available"]
    #[inline]
    pub fn invpc(&self) -> INVPCR {
        INVPCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - no description available"]
    #[inline]
    pub fn nocp(&self) -> NOCPR {
        NOCPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - no description available"]
    #[inline]
    pub fn unaligned(&self) -> UNALIGNEDR {
        UNALIGNEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - no description available"]
    #[inline]
    pub fn divbyzero(&self) -> DIVBYZEROR {
        DIVBYZEROR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
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
    pub fn iaccviol(&mut self) -> _IACCVIOLW {
        _IACCVIOLW { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn daccviol(&mut self) -> _DACCVIOLW {
        _DACCVIOLW { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline]
    pub fn munstkerr(&mut self) -> _MUNSTKERRW {
        _MUNSTKERRW { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline]
    pub fn mstkerr(&mut self) -> _MSTKERRW {
        _MSTKERRW { w: self }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline]
    pub fn mlsperr(&mut self) -> _MLSPERRW {
        _MLSPERRW { w: self }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline]
    pub fn mmarvalid(&mut self) -> _MMARVALIDW {
        _MMARVALIDW { w: self }
    }
    #[doc = "Bit 8 - no description available"]
    #[inline]
    pub fn ibuserr(&mut self) -> _IBUSERRW {
        _IBUSERRW { w: self }
    }
    #[doc = "Bit 9 - no description available"]
    #[inline]
    pub fn preciserr(&mut self) -> _PRECISERRW {
        _PRECISERRW { w: self }
    }
    #[doc = "Bit 10 - no description available"]
    #[inline]
    pub fn impreciserr(&mut self) -> _IMPRECISERRW {
        _IMPRECISERRW { w: self }
    }
    #[doc = "Bit 11 - no description available"]
    #[inline]
    pub fn unstkerr(&mut self) -> _UNSTKERRW {
        _UNSTKERRW { w: self }
    }
    #[doc = "Bit 12 - no description available"]
    #[inline]
    pub fn stkerr(&mut self) -> _STKERRW {
        _STKERRW { w: self }
    }
    #[doc = "Bit 13 - no description available"]
    #[inline]
    pub fn lsperr(&mut self) -> _LSPERRW {
        _LSPERRW { w: self }
    }
    #[doc = "Bit 15 - no description available"]
    #[inline]
    pub fn bfarvalid(&mut self) -> _BFARVALIDW {
        _BFARVALIDW { w: self }
    }
    #[doc = "Bit 16 - no description available"]
    #[inline]
    pub fn undefinstr(&mut self) -> _UNDEFINSTRW {
        _UNDEFINSTRW { w: self }
    }
    #[doc = "Bit 17 - no description available"]
    #[inline]
    pub fn invstate(&mut self) -> _INVSTATEW {
        _INVSTATEW { w: self }
    }
    #[doc = "Bit 18 - no description available"]
    #[inline]
    pub fn invpc(&mut self) -> _INVPCW {
        _INVPCW { w: self }
    }
    #[doc = "Bit 19 - no description available"]
    #[inline]
    pub fn nocp(&mut self) -> _NOCPW {
        _NOCPW { w: self }
    }
    #[doc = "Bit 24 - no description available"]
    #[inline]
    pub fn unaligned(&mut self) -> _UNALIGNEDW {
        _UNALIGNEDW { w: self }
    }
    #[doc = "Bit 25 - no description available"]
    #[inline]
    pub fn divbyzero(&mut self) -> _DIVBYZEROW {
        _DIVBYZEROW { w: self }
    }
}
