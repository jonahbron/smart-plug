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

    // WRITE TO MAC
    cs_w5500.set_low().unwrap_or_else(|_| panic!());
    spi
        // 16-bit offset
        .send(0)
        .and_then(|_| spi.send(0x9))
        // control byte: Common register, write mode, variable-length mode
        .and_then(|_| spi.send(0b00000100))
        // push read byte from chip
        .and_then(|_| spi.send(10))
        .and_then(|_| spi.send(15))
        .and_then(|_| spi.send(20))
        .and_then(|_| spi.send(25))
        .and_then(|_| spi.send(30))
        .and_then(|_| spi.send(35))
        .unwrap_or_else(|_| panic!());
    cs_w5500.set_high().unwrap_or_else(|_| panic!());

    // TODO READ FROM MAC
    cs_w5500.set_low().unwrap_or_else(|_| panic!());
    spi
        // 16-bit offset
        .send(0)
        .and_then(|_| spi.send(0x9))
        // control byte: Common register, write mode, variable-length mode
        .and_then(|_| spi.send(0b00000000))
        .unwrap_or_else(|_| panic!());
    let a = spi.send(0).and_then(|_| spi.read()).unwrap_or_else(|_| panic!());
    let b = spi.send(0).and_then(|_| spi.read()).unwrap_or_else(|_| panic!());
    let c = spi.send(0).and_then(|_| spi.read()).unwrap_or_else(|_| panic!());
    let d = spi.send(0).and_then(|_| spi.read()).unwrap_or_else(|_| panic!());
    let e = spi.send(0).and_then(|_| spi.read()).unwrap_or_else(|_| panic!());
    let f = spi.send(0).and_then(|_| spi.read()).unwrap_or_else(|_| panic!());

    cs_w5500.set_high().unwrap_or_else(|_| panic!());

    ufmt::uwriteln!(&mut serial, "MAC: {}:{}:{}:{}:{}:{}\r", a, b, c, d, e, f).unwrap_or_else(|_| panic!());
}
