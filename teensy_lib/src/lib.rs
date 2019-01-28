#![no_std]
#![feature(alloc)]
#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(nll)]
#![allow(unused_variables)]
#![allow(dead_code)]

// https://docs.rs/cortex-m-rt/0.6.7/cortex_m_rt/
// https://docs.rs/cortex-m/0.5.8/cortex_m/
// https://github.com/rust-embedded/cortex-m-quickstart

extern crate alloc;
extern crate alloc_cortex_m;

extern crate cortex_m_rt as rt; // v0.5.x
extern crate panic_halt;
extern crate mk20d7;
//#[macro_use]
extern crate cfg_if;

mod clk_setup;
pub mod vectors_ram;


use mk20d7::{Peripherals};
use cortex_m::peripheral::Peripherals as CorePeripherals;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::asm;
use rt::{entry, pre_init};
use alloc_cortex_m::CortexMHeap;

extern "Rust" {
    fn teensy_main();
}

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[pre_init]
unsafe fn preinit() {
    let mk20_peripherals = Peripherals::take().unwrap();

    let watchdg = mk20_peripherals.WDOG;
    watchdg.unlock.write(| b| b.bits(0xC520));
    watchdg.unlock.write(| b| b.bits(0xD928));
    asm::nop();
    asm::nop();

    // enable clocks to always-used peripherals
    let sim =  mk20_peripherals.SIM;
    sim.scgc3.write(| b| b.adc1().set_bit().ftm2().set_bit() );
    sim.scgc5.write(| b| b.bits(0x00043F82)); // clocks active to all GPIO
    sim.scgc6.write(|b| b.rtc().set_bit()
                         .ftm0().set_bit()
                         .ftm1().set_bit()
                         .adc0().set_bit()
                         .ftfl().set_bit());

    // if the RTC oscillator isn't enabled, get it started early
    let rtc = mk20_peripherals.RTC;
    if rtc.cr.read().osce().is_0() {
        rtc.sr.write(|b| b.bits(0));
        rtc.cr.write(|b| b.sc16p().set_bit()
                          .sc4p().set_bit().osce().set_bit());
    }

    // release I/O pins hold, if we woke up from VLLS mode
    let pmc = mk20_peripherals.PMC;
    if pmc.regsc.read().ackiso().is_1() {
        pmc.regsc.write(|b| b.ackiso().set_bit());
    }

    // since this is a write once register, make it visible to all F_CPU's
    // so we can into other sleep modes in the future at any speed
    mk20_peripherals.SMC.pmprot.write(|b| b.avlp().set_bit()
                          .alls().set_bit().avlls().set_bit());
}


#[entry]
unsafe fn main() -> ! {
    asm::nop();
    let mut core_periphs: CorePeripherals = CorePeripherals::take().unwrap();
    let mk20_periphs: Peripherals = Peripherals::steal();

    vectors_ram::init_vectors_ram();
    clk_setup::setup();

    // Setup SysClk
    core_periphs.SYST.set_reload((clk_setup::F_CPU / 1000) - 1);
    core_periphs.SYST.clear_current();
    core_periphs.SYST.set_clock_source(SystClkSource::Core);
    core_periphs.SYST.enable_interrupt();
    core_periphs.SYST.enable_counter();
    vectors_ram::set_irq_priority(15, 32);  // Set SysClk Interrupt
    cortex_m::interrupt::enable();
    pins_init();
    // TOOD: Analog_init()

    teensy_main();
    loop {}
}

#[inline(never)]
unsafe fn pins_init() {
    let mut core_periphs: CorePeripherals = CorePeripherals::take().unwrap();
    let mk20_periphs: Peripherals = Peripherals::steal();
    core_periphs.NVIC.enable(mk20d7::Interrupt::INT_PORTA);
    core_periphs.NVIC.enable(mk20d7::Interrupt::INT_PORTB);
    core_periphs.NVIC.enable(mk20d7::Interrupt::INT_PORTC);
    core_periphs.NVIC.enable(mk20d7::Interrupt::INT_PORTD);
    core_periphs.NVIC.enable(mk20d7::Interrupt::INT_PORTE);
    mk20_periphs.FTM0.cnt.write(|b| b.bits(0));
    mk20_periphs.FTM0.mod_.write(|b| b.bits(clk_setup::DEFAULT_FTM_MOD));
    mk20_periphs.FTM0.c0sc.write(|b| b.bits(0x28)); // MSnB:MSnA = 10, ELSnB:ELSnA = 10
    mk20_periphs.FTM0.c1sc.write(|b| b.bits(0x28));
    mk20_periphs.FTM0.c2sc.write(|b| b.bits(0x28));
    mk20_periphs.FTM0.c3sc.write(|b| b.bits(0x28));
    mk20_periphs.FTM0.c4sc.write(|b| b.bits(0x28));
    mk20_periphs.FTM0.c5sc.write(|b| b.bits(0x28));
    mk20_periphs.FTM0.c6sc.write(|b| b.bits(0x28));
    mk20_periphs.FTM0.c7sc.write(|b| b.bits(0x28));
    mk20_periphs.FTM0.sc.write(|b| b.clks()._01().ps().bits(clk_setup::DEFAULT_FTM_PRESCALE));
    mk20_periphs.FTM1.cnt.write(|b| b.bits(0));
    mk20_periphs.FTM1.mod_.write(|b| b.bits(clk_setup::DEFAULT_FTM_MOD));
    mk20_periphs.FTM1.c0sc.write(|b| b.bits(0x28));
    mk20_periphs.FTM1.c1sc.write(|b| b.bits(0x28));
    mk20_periphs.FTM1.sc.write(|b| b.clks()._01().ps().bits(clk_setup::DEFAULT_FTM_PRESCALE));
    mk20_periphs.FTM2.cnt.write(|b| b.bits(0));
    mk20_periphs.FTM2.mod_.write(|b| b.bits(clk_setup::DEFAULT_FTM_MOD));
    mk20_periphs.FTM2.c0sc.write(|b| b.bits(0x28));
    mk20_periphs.FTM2.c1sc.write(|b| b.bits(0x28));
    mk20_periphs.FTM2.sc.write(|b| b.clks()._01().ps().bits(clk_setup::DEFAULT_FTM_PRESCALE));
}

#[lang = "oom"]
#[no_mangle]
pub fn rust_oom(_layout: core::alloc::Layout) -> ! {
    panic!();
}