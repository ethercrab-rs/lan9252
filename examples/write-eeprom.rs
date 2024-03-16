//! Read/write EEPROM attached to LAN9252.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

const ADDRESS: u8 = 0x50;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

/// Original EEPROM data dumped from an Aliexpress dev board.
const ORIGINAL: [u8; 12] = [0x89, 14, 128, 204, 136, 19, 0, 0, 0, 0, 0, 128];

/// 12.14.24 PDI CONTROL REGISTER
const COMM_MODE_SPI: u8 = 0x80;
/// Original control mode of AliExpress firmware.
const COMM_MODE_HBI_1PH_16BIT: u8 = 0x89; // 137 decimal
/// Simple digital IO.
const COMM_MODE_DIG_IO: u8 = 0x04;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Reading EEPROM");

    let mut i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        NoDma,
        NoDma,
        Hertz(100_000),
        Default::default(),
    );

    // // Uncomment to write comm mode config to address 0
    // i2c.blocking_write(ADDRESS, &[0x00, 0x00, COMM_MODE_DIG_IO])
    //     .expect("Write byte");

    Timer::after_millis(100).await;

    // The LAN9252 starts with a two byte 0 write. I think this sets the start address.
    let start_addr = [0x00u8, 0x00];

    // 6 words or 12 bytes
    // TABLE 12-3: ETHERCAT CORE EEPROM CONFIGURABLE REGISTERS
    let mut read_buffer = [0u8; 12];

    match i2c.blocking_write_read(ADDRESS, &start_addr, &mut read_buffer) {
        Ok(()) => {
            info!("Data {=[u8]}", read_buffer);
        }
        Err(e) => error!("I2C Error during read: {:?}", e),
    }
}
