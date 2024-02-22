//! Configure the LAN9252 to output a `SYNC0` pulse on the `SYNC0/LATCH0` pin.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::str::from_utf8;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::spi::{Config, Mode, Phase, Polarity, Spi};
use embassy_stm32::time::Hertz;
use embassy_time::Timer;
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

/// TABLE 10-1: SPI INSTRUCTIONS
#[repr(u8)]
enum Instruction {
    /// Read data from an address. Must be followed by two address bytes.
    Read = 0x03,
}

#[repr(u16)]
enum Register {
    /// Chip ID and revision (32 bits).
    ///
    /// TABLE 16-1: MISCELLANEOUS REGISTERS, 16.1.1 CHIP ID AND REVISION (ID_REV)
    ChipId = 0x0050,

    /// Byte order register, can be polled to check SPI readiness status.
    ///
    /// 16.1.2 BYTE ORDER TEST REGISTER (BYTE_TEST)
    ByteOrder = 0x0064,
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut spi_config = Config::default();
    spi_config.frequency = Hertz(1_000_000);
    spi_config.mode = Mode {
        polarity: Polarity::IdleLow,
        // LAN9252 samples on rising edge. TODO: is `CaptureOnFirstTransition` right for this?
        phase: Phase::CaptureOnFirstTransition,
    };

    // FIGURE 10-4: SPI READ: CS is active low.
    let mut cs = Output::new(p.PA4, Level::High, Speed::High);

    let mut spi = Spi::new(
        p.SPI1, p.PA5, p.PA7, p.PA6, p.DMA2_CH3, p.DMA2_CH2, spi_config,
    );

    let write = [Instruction::Read as u8, 0x00, 0x64, 0x00, 0x00, 0x00, 0x00];
    let mut read = [0u8; 7];

    for _ in 0..10 {
        cs.set_low();

        // spi.transfer(&mut read, &write).await.expect("xfer");

        spi.write(&write[0..3]).await.expect("Write");

        spi.read(&mut read[0..4]).await.expect("Read");

        cs.set_high();

        // spi.transfer(&mut read, &write_payload).await.expect("xfer");

        info!("Result {:?}", write);

        Timer::after_millis(100).await;
    }

    // for n in 0u32.. {
    //     let mut write: String<128> = String::new();
    //     let mut read = [0; 128];
    //     core::write!(&mut write, "Hello DMA World {}!\r\n", n).unwrap();
    //     spi.transfer(&mut read[0..write.len()], write.as_bytes())
    //         .await
    //         .ok();
    //     info!("read via spi+dma: {}", from_utf8(&read).unwrap());
    // }

    // loop {}
}
