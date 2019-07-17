#![no_std]
#![no_main]

extern crate panic_halt;

use embedded_hal::digital::v1_compat;
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

    let mut spi = arduino_uno::spi::Spi::new(
        dp.SPI,
        pins.d11.into_output(&mut pins.ddr),
        pins.d12.into_pull_up_input(&mut pins.ddr),
        arduino_uno::spi::Settings::default(),
    );

    let mut cs_w5500 = v1_compat::OldOutputPin::new(pins.d13.into_output(&mut pins.ddr));

    let mut w5500 = W5500::with_initialisation(
        &mut cs_w5500,
        &mut spi,
        OnWakeOnLan::Ignore,
        OnPingRequest::Respond,
        ConnectionType::Ethernet,
        ArpResponses::Cache
    ).ok();

    if let Some(ref mut w5500) = w5500 {
        let mut w5500 = w5500.activate(&mut spi).unwrap_or_else(|_| panic!());
        w5500.set_mac(MacAddress::new(0x02, 0x01, 0x02, 0x03, 0x04, 0x05)).unwrap_or_else(|_| panic!());
        w5500.set_ip(IpAddress::new(192, 168, 0, 100)).unwrap_or_else(|_| panic!());
        w5500.set_subnet(IpAddress::new(255, 255, 255, 0)).unwrap_or_else(|_| panic!());
        w5500.set_gateway(IpAddress::new(192, 168, 0, 1)).unwrap_or_else(|_| panic!());

        let socket0 = w5500.take_socket(Socket::Socket0).unwrap_or_else(|| panic!());
        let mut udp_server_socket = (&mut w5500, socket0).try_into_udp_server_socket(1234).ok();

        let mut buffer = [0u8; 256];

        if let Some(ref socket) = udp_server_socket {
            let mut udp = (&mut w5500, socket);
            udp.receive(&mut buffer[..]).unwrap_or_else(|_| panic!());
            // if let Ok(Some((ip, port, length))) = (&mut w5500, socket).receive(&mut buffer[..]) {
            //     let (_request_buffer, response_buffer) = buffer.split_at_mut(length);
            //     response_buffer[0] = 115;
            //     response_buffer[1] = 117;
            //     response_buffer[2] = 112;
            //     let response_length = 3;
            //     (&mut w5500, socket).blocking_send(&ip, port, &response_buffer[..response_length]).unwrap_or_else(|_| panic!());
            // }
        }
    }
}
