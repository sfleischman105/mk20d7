use core::mem::transmute;
use mk20d7::__INTERRUPTS;
use cortex_m::peripheral::SCB;
use cortex_m::peripheral::scb::RegisterBlock;
use cortex_m::peripheral::Peripherals as CorePeripherals;

static mut _VECTORS_RAM: [u32; 108] = [0; 108];

struct DummyInterrupt(u8);

pub(crate) fn set_irq_priority(irq_num: u8, priority: u8) {
    unsafe {
        CorePeripherals::steal().NVIC.ipr[usize::from(irq_num)].write(priority);
    }
}

pub(crate) fn init_vectors_ram() {
    unsafe {
        for (idx, irq) in __INTERRUPTS.iter().enumerate() {
            _VECTORS_RAM[idx] = transmute(irq);
        }

        // Set all interrupts to 128
        let core_periphs: CorePeripherals = CorePeripherals::steal();
        for inum in 16u8..108 {
            core_periphs.NVIC.ipr[usize::from(inum)].write(128u8);
        }

        // TODO: Set priority
        let scb_regblk: *const RegisterBlock = SCB::ptr();
        (*scb_regblk).vtor.write(transmute(_VECTORS_RAM.as_ptr()))
    }
}