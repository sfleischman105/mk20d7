#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DSPI Module Configuration Register"]
    pub mcr: MCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - DSPI Transfer Count Register"]
    pub tcr: TCR,
    #[doc = "0x0c - DSPI Clock and Transfer Attributes Register (In Slave Mode)"]
    pub ctar_slave: CTAR_SLAVE,
    _reserved1: [u8; 28usize],
    #[doc = "0x2c - DSPI Status Register"]
    pub sr: SR,
    #[doc = "0x30 - DSPI DMA/Interrupt Request Select and Enable Register"]
    pub rser: RSER,
    #[doc = "0x34 - DSPI PUSH TX FIFO Register In Master Mode"]
    pub pushr: PUSHR,
    #[doc = "0x38 - DSPI POP RX FIFO Register"]
    pub popr: POPR,
    #[doc = "0x3c - DSPI Transmit FIFO Registers"]
    pub txfr: [TXFR; 4],
    _reserved2: [u8; 48usize],
    #[doc = "0x7c - DSPI Receive FIFO Registers"]
    pub rxfr: [RXFR; 4],
}
#[doc = "DSPI Module Configuration Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSPI Module Configuration Register"]
pub mod mcr;
#[doc = "DSPI Transfer Count Register"]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSPI Transfer Count Register"]
pub mod tcr;
#[doc = "DSPI Clock and Transfer Attributes Register (In Slave Mode)"]
pub struct CTAR_SLAVE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSPI Clock and Transfer Attributes Register (In Slave Mode)"]
pub mod ctar_slave;
#[doc = "DSPI Clock and Transfer Attributes Register (In Master Mode)"]
pub struct CTAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSPI Clock and Transfer Attributes Register (In Master Mode)"]
pub mod ctar;
#[doc = "DSPI Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSPI Status Register"]
pub mod sr;
#[doc = "DSPI DMA/Interrupt Request Select and Enable Register"]
pub struct RSER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSPI DMA/Interrupt Request Select and Enable Register"]
pub mod rser;
#[doc = "DSPI PUSH TX FIFO Register In Master Mode"]
pub struct PUSHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSPI PUSH TX FIFO Register In Master Mode"]
pub mod pushr;
#[doc = "DSPI PUSH TX FIFO Register In Slave Mode"]
pub struct PUSHR_SLAVE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSPI PUSH TX FIFO Register In Slave Mode"]
pub mod pushr_slave;
#[doc = "DSPI POP RX FIFO Register"]
pub struct POPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSPI POP RX FIFO Register"]
pub mod popr;
#[doc = "DSPI Transmit FIFO Registers"]
pub struct TXFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSPI Transmit FIFO Registers"]
pub mod txfr;
#[doc = "DSPI Receive FIFO Registers"]
pub struct RXFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSPI Receive FIFO Registers"]
pub mod rxfr;
