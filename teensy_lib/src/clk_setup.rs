use mk20d7::Peripherals;

pub const MHZ_96: u32 = 96_000_000;
pub const MHZ_72: u32 = 72_000_000;
pub const MHZ_48: u32 = 48_000_000;
pub const MHZ_36: u32 = 36_000_000;
pub const MHZ_24: u32 = 24_000_000;


pub const F_CPU: u32 = clk_inner::F_CPU;
pub const F_PLL: u32 = clk_inner::F_PLL;
pub const F_BUS: u32 = clk_inner::F_BUS;
pub const F_MEM: u32 = clk_inner::F_MEM;
pub const F_TIMER: u32 = clk_inner::F_TIMER;
pub const DEFAULT_FTM_MOD: u32 = clk_inner::DEFAULT_FTM_MOD;
pub const DEFAULT_FTM_PRESCALE: u8 = clk_inner::DEFAULT_FTM_PRESCALE;


// "Error, F_CPU must be 192, 180, 168, 144, 120, 96, 72, 48, 24, 16, 8, 4, or 2 MHz"

pub(crate) unsafe fn setup() {
    let periphs = Peripherals::steal();
    periphs.OSC.cr.write(|b| b.bits(0).sc8p().set_bit().sc2p().set_bit().erclken().set_bit());
    // enable osc, 8-32 MHz range, low power mode
    periphs.MCG.c2.write(|b| b.bits(0).range0().bits(2).erefs0().set_bit());
    // switch to crystal as clock source, FLL input = 16 MHz / 512
    periphs.MCG.c1.write(|b| b.bits(0).clks()._10().frdiv()._100());
    // wait for crystal oscillator to begin
    while periphs.MCG.s.read().oscinit0().bit_is_clear() {}
    // wait for FLL to use oscillator
    while periphs.MCG.s.read().irefst().bit_is_set() {}
    // wait for MCGOUT to use oscillator
    while !periphs.MCG.s.read().clkst().is_10() {}

    #[cfg(feature = "clk_72MHz")]
    {
        periphs.MCG.c5.write(|b| b.prdiv0().bits(5));
        periphs.MCG.c6.write(|b| b.plls().set_bit().vdiv0().bits(3))
    }

    #[cfg(any(feature = "clk_96MHz", feature = "clk_48MHz", feature = "clk_24MHz"))]
    {
        periphs.MCG.c5.write(|b| b.prdiv0().bits(3));
        periphs.MCG.c6.write(|b| b.plls().set_bit().vdiv0().bits(0))
    }

    // wait for PLL to start using xtal as its input
    while periphs.MCG.s.read().pllst().bit_is_clear() {}
    // wait for PLL to lock
    while periphs.MCG.s.read().lock0().bit_is_clear() {}
    // now we're in PBE mode
    clk_inner::clkdiv_setup();

    // switch to PLL as clock source, FLL input = 16 MHz / 512
    periphs.MCG.c1.write(|b| b.clks()._00().frdiv()._100());
    // wait for PLL clock to be used
    while periphs.MCG.s.read().clkst().bits() != 3 {}
    // now we're in PEE mode
    // USB uses PLL clock, trace is CPU clock, CLKOUT=OSCERCLK0
    periphs.SIM.sopt2.write(|b| b.usbsrc().set_bit()
        .pllfllsel().set_bit()
        .traceclksel().set_bit()
        .clkoutsel()._110());
}

#[cfg(feature = "clk_96MHz")]
mod clk_inner {
    use super::*;
    pub const F_CPU: u32 = MHZ_96;
    pub const F_PLL: u32 = MHZ_96;
    pub const F_BUS: u32 = MHZ_48;
    pub const F_MEM: u32 = MHZ_24;
    pub const F_TIMER: u32 = F_BUS;
    pub const DEFAULT_FTM_MOD: u32 = (49152 - 1);
    pub const DEFAULT_FTM_PRESCALE: u8 = 1;

    pub unsafe fn clkdiv_setup() {
        let periphs = Peripherals::steal();
        if F_BUS == MHZ_96 {panic!("This F_CPU and F_BUS combo is not supported (yet)")}
        if F_BUS != MHZ_48 {panic!("This F_CPU and F_BUS combo is not supported")}
        periphs.SIM.clkdiv1.write(|b| b.outdiv1()._0000().outdiv2()._0001().outdiv4()._0011());
        periphs.SIM.clkdiv2.write(|b| b.usbdiv().bits(1).usbfrac().set_bit());
    }
}

#[cfg(feature = "clk_72MHz")]
mod clk_inner {
    use super::*;
    pub const F_CPU: u32 = MHZ_72;
    pub const F_PLL: u32 = MHZ_72;
    pub const F_BUS: u32 = MHZ_36;
    pub const F_MEM: u32 = MHZ_24;
    pub const F_TIMER: u32 = F_BUS;
    pub const DEFAULT_FTM_MOD: u32 = (36864 - 1);
    pub const DEFAULT_FTM_PRESCALE: u8 = 1;

    pub unsafe fn clkdiv_setup() {
        let periphs = Peripherals::steal();
        if F_BUS == MHZ_72 {panic!("This F_CPU and F_BUS combo is not supported (yet)");}
        if F_BUS != MHZ_36 {panic!("This F_CPU and F_BUS combo is not supported");}
        periphs.SIM.clkdiv1.write(|b| b.outdiv1()._0000().outdiv2()._0001().outdiv4()._0010());
        periphs.SIM.clkdiv2.write(|b| b.usbdiv().bits(1));
    }
}

#[cfg(feature = "clk_48MHz")]
mod clk_inner {
    use super::*;
    pub const F_CPU: u32 = MHZ_48;
    pub const F_PLL: u32 = MHZ_96;
    pub const F_BUS: u32 = MHZ_48;
    pub const F_MEM: u32 = MHZ_24;
    pub const F_TIMER: u32 = F_BUS;
    pub const DEFAULT_FTM_MOD: u32 = (49152 - 1);
    pub const DEFAULT_FTM_PRESCALE: u8 = 1;

    pub unsafe fn clkdiv_setup() {
        let periphs = Peripherals::steal();
        periphs.SIM.clkdiv1.write(|b| b.outdiv1()._0001().outdiv2()._0001().outdiv3()._0001().outdiv4()._0011());
        periphs.SIM.clkdiv2.write(|b| b.usbdiv().bits(1));
    }
}

#[cfg(feature = "clk_24MHz")]
mod clk_inner {
    use super::*;
    pub const F_CPU: u32 = MHZ_24;
    pub const F_PLL: u32 = MHZ_96;
    pub const F_BUS: u32 = MHZ_24;
    pub const F_MEM: u32 = MHZ_24;
    pub const F_TIMER: u32 = F_BUS;
    pub const DEFAULT_FTM_MOD: u32 = (49152 - 1);
    pub const DEFAULT_FTM_PRESCALE: u8 = 0;

    pub unsafe fn clkdiv_setup() {
        let periphs = Peripherals::steal();
        periphs.SIM.clkdiv1.write(|b| b.outdiv1()._0011().outdiv2()._0011().outdiv3()._0011().outdiv4()._0011());
        periphs.SIM.clkdiv2.write(|b| b.usbdiv().bits(2));

    }
}
