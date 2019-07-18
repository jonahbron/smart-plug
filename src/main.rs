#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]

extern crate panic_halt;

use embedded_hal::digital::v1_compat;
use embedded_hal::blocking::delay::DelayMs;
use w5500::{W5500, OnWakeOnLan, OnPingRequest, ConnectionType, ArpResponses, MacAddress, IpAddress, Socket, IntoUdpSocket, Udp};

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
        w5500.set_mac(MacAddress::new(0x02, 0x01, 0x02, 0x03, 0x04, 0x05)).unwrap_or_else(|_| panic!());
        w5500.set_ip(IpAddress::new(192, 168, 86, 30)).unwrap_or_else(|_| panic!());
        w5500.set_subnet(IpAddress::new(255, 255, 255, 0)).unwrap_or_else(|_| panic!());
        w5500.set_gateway(IpAddress::new(192, 168, 86, 1)).unwrap_or_else(|_| panic!());

        let socket0 = w5500.take_socket(Socket::Socket0).unwrap_or_else(|| panic!());
        let udp_server_socket = (&mut w5500, socket0).try_into_udp_server_socket(1234).ok();

        let mut buffer = [0u8; 256];
        ufmt::uwriteln!(&mut serial, "set up\r").unwrap();

        if let Some(ref socket) = udp_server_socket {
            ufmt::uwriteln!(&mut serial, "took socket\r").unwrap();
            let mut udp = (&mut w5500, socket);

            loop {
                if let Ok(Some((ip, port, length))) = udp.receive(&mut buffer[..]) {
                    ufmt::uwriteln!(&mut serial, "received\r").unwrap();
                    let (_request_buffer, response_buffer) = buffer.split_at_mut(length);
                    response_buffer[0] = 115;
                    response_buffer[1] = 117;
                    response_buffer[2] = 112;
                    let response_length = 3;
                    udp.blocking_send(&ip, port, &response_buffer[..response_length]).unwrap_or_else(|_| panic!());
                } else {
                    // ufmt::uwriteln!(&mut serial, "could not receive\r").unwrap();
                }
                // delay.delay_ms(1000);
            }
        } else {
            ufmt::uwriteln!(&mut serial, "could not take socket\r").unwrap();
        }
    } else {
        ufmt::uwriteln!(&mut serial, "could not initialize\r").unwrap();
    }
}
