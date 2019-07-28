#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]

extern crate panic_halt;

use embedded_hal::digital::v1_compat;
use embedded_hal::blocking::delay::DelayMs;
use w5500::{W5500, OnWakeOnLan, OnPingRequest, ConnectionType, ArpResponses, MacAddress, IpAddress, Socket, IntoUdpSocket, Udp, Register};

#[no_mangle]
pub extern fn main() -> () {
    let dp = arduino_uno::Peripherals::take().unwrap_or_else(|| panic!());
    let mut delay = arduino_uno::Delay::new();
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

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap();


    let mut spi = arduino_uno::spi::Spi::new(
        dp.SPI,
        pins.d13.into_output(&mut pins.ddr),
        pins.d11.into_output(&mut pins.ddr),
        pins.d12.into_pull_up_input(&mut pins.ddr),
        arduino_uno::spi::Settings::default(),
    );

    let mut cs_w5500 = v1_compat::OldOutputPin::new(pins.d10.into_output(&mut pins.ddr));

    let mut w5500 = W5500::with_initialisation(
        &mut cs_w5500,
        &mut spi,
        OnWakeOnLan::Ignore,
        OnPingRequest::Respond,
        ConnectionType::Ethernet,
        ArpResponses::Cache
    ).ok();

    if let Some(ref mut w5500) = w5500 {
        ufmt::uwriteln!(&mut serial, "initialized\r").unwrap();
        let mut w5500 = w5500.activate(&mut spi).unwrap_or_else(|_| panic!());
        ufmt::uwriteln!(&mut serial, "activated\r").unwrap();
        w5500.set_ip(IpAddress::new(192, 168, 86, 30)).unwrap_or_else(|_| panic!());
        let IpAddress { address: ip } = w5500.read_ip(Register::CommonRegister(0x00_0F_u16)).unwrap_or_else(|_| panic!());
        ufmt::uwriteln!(&mut serial, "IP: {}.{}.{}.{}\r", ip[0], ip[1], ip[2], ip[3]).unwrap();
    } else {
        ufmt::uwriteln!(&mut serial, "could not initialize\r").unwrap();
    }
}
