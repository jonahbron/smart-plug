#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]

extern crate panic_halt;

use embedded_hal::spi::FullDuplex;
use embedded_hal::digital::v2::OutputPin;
use arduino_uno::spi::{Settings, DataOrder, SerialClockRate, SerialClockPolarity, SerialClockPhase};

#[no_mangle]
pub extern fn main() -> () {
    let dp = arduino_uno::Peripherals::take().unwrap_or_else(|| panic!());
    let mut pins = arduino_uno::Pins::new(
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
    );

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    let mut spi = arduino_uno::spi::Spi::new(
        dp.SPI,
        pins.d13.into_output(&mut pins.ddr),
        pins.d11.into_output(&mut pins.ddr),
        pins.d12.into_pull_up_input(&mut pins.ddr),
        Settings {
            data_order: DataOrder::MostSignificantFirst,
            clock: SerialClockRate::OscfOver4,
            clock_polarity: SerialClockPolarity::IdleLow,
            clock_phase: SerialClockPhase::SampleLeading,
        },
    );

    let mut cs_w5500 = pins.d10.into_output(&mut pins.ddr);

    // begin frame
    cs_w5500.set_low().unwrap_or_else(|_| panic!());

    let version = spi
        // 16-bit offset
        .send(0)
        .and_then(|_| spi.send(0x39))
        // control byte: Common register, read mode, variable-length mode
        .and_then(|_| spi.send(0b00000000))
        // push read byte from chip
        .and_then(|_| spi.send(0))
        .and_then(|_| spi.read())
        .unwrap_or_else(|_| panic!());

    // end frame
    cs_w5500.set_high().unwrap_or_else(|_| panic!());

    ufmt::uwriteln!(&mut serial, "Version: {} (should be 4)\r", version).unwrap_or_else(|_| panic!());
}
