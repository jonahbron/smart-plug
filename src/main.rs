#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]

extern crate panic_halt;

// use embedded_hal::blocking::delay::DelayMs;
// use w5500::{W5500, OnWakeOnLan, OnPingRequest, ConnectionType, ArpResponses, MacAddress, IpAddress, Socket, IntoUdpSocket, Udp};
use arduino_uno::spi::{Settings, DataOrder, SerialClockRate, SerialClockPolarity, SerialClockPhase};

use w5500::uninitialized_w5500::{UninitializedW5500,InitializeError};
use w5500::bus::FourWire;
use w5500::MacAddress;
use w5500::IpAddress;
use w5500::Mode;

#[no_mangle]
pub extern fn main() -> () {
    let dp = arduino_uno::Peripherals::take().unwrap_or_else(|| panic!());
    // let mut delay = arduino_uno::Delay::new();
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

    let spi = arduino_uno::spi::Spi::new(
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

    let cs = pins.d10.into_output(&mut pins.ddr);


    ufmt::uwriteln!(&mut serial, "set up\r").unwrap();

    let uninitialized_w5500 = UninitializedW5500::new(FourWire::new(cs).activate(spi));
    let w5500 = uninitialized_w5500.initialize_manual(MacAddress::new(0, 1, 2, 3, 4, 5), IpAddress::new(192, 168, 86, 90), Mode::default());// handle error
    if let Ok((w5500, (socket, ..))) = w5500 {
        ufmt::uwriteln!(&mut serial, "initialized\r").unwrap();
        let udp_socket = w5500.open_udp_socket(8000, socket);
        if let Ok(udp_socket) = udp_socket {
            ufmt::uwriteln!(&mut serial, "socket opened\r").unwrap();
            let packet = udp_socket.send(IpAddress::new(192, 168, 86, 29), 4000);
            if let Ok(mut packet) = packet {
                ufmt::uwriteln!(&mut serial, "packet started\r").unwrap();
                let wrote = packet.write(&mut [104, 101, 108, 108, 111]);
                if let Ok(()) = wrote {
                    ufmt::uwriteln!(&mut serial, "packet wrote\r").unwrap();
                    // UNCOMMENTING THIS LINE BREAKS EVERYTHING
                    // SEND never returns, stuck in infinite loop
                    let sent = packet.send();
                    if let Ok(udp_socket) = sent {
                        ufmt::uwriteln!(&mut serial, "packet sent\r").unwrap();
                    } else {
                        ufmt::uwriteln!(&mut serial, "packet send failed\r").unwrap();
                    }
                } else {
                        ufmt::uwriteln!(&mut serial, "packet write failed\r").unwrap();
                }
            } else {
                ufmt::uwriteln!(&mut serial, "packet start failed\r").unwrap();
            }
        }
    } else if let Err(InitializeError::ChipNotConnected) = w5500 {
        ufmt::uwriteln!(&mut serial, "chip not connected\r").unwrap();
    } else {
        ufmt::uwriteln!(&mut serial, "not initialized\r").unwrap();
    }
    ufmt::uwriteln!(&mut serial, "DONE\r").unwrap();


    // let mut w5500 = W5500::with_initialisation(
    //     &mut cs_w5500,
    //     &mut spi,
    //     OnWakeOnLan::Ignore,
    //     OnPingRequest::Respond,
    //     ConnectionType::Ethernet,
    //     ArpResponses::Cache
    // ).ok();

    // if let Some(ref mut w5500) = w5500 {
    //     ufmt::uwriteln!(&mut serial, "initialized\r").unwrap();
    //     let mut w5500 = w5500.activate(&mut spi).unwrap_or_else(|_| panic!());
    //     ufmt::uwriteln!(&mut serial, "activated\r").unwrap();
    //     w5500.set_mac(MacAddress::new(0x02, 0x01, 0x02, 0x03, 0x04, 0x05)).unwrap_or_else(|_| panic!());
    //     w5500.set_ip(IpAddress::new(192, 168, 86, 30)).unwrap_or_else(|_| panic!());
    //     w5500.set_subnet(IpAddress::new(255, 255, 255, 0)).unwrap_or_else(|_| panic!());
    //     w5500.set_gateway(IpAddress::new(192, 168, 86, 1)).unwrap_or_else(|_| panic!());

    //     let socket0 = w5500.take_socket(Socket::Socket0).unwrap_or_else(|| panic!());
    //     let udp_server_socket = (&mut w5500, socket0).try_into_udp_server_socket(1234).ok();

    //     let mut buffer = [0u8; 16];
    //     ufmt::uwriteln!(&mut serial, "set up\r").unwrap();

    //     if let Some(ref socket) = udp_server_socket {
    //         ufmt::uwriteln!(&mut serial, "took socket\r").unwrap();
    //         let mut udp = (&mut w5500, socket);

    //         loop {
    //             if let Ok(Some((ip, port, length))) = udp.receive(&mut buffer[..]) {
    //                 ufmt::uwriteln!(&mut serial, "received: {} {} {} {} {} {}\r", buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5]).unwrap();
    //                 ufmt::uwriteln!(&mut serial, "port: {}\r", port).unwrap();
    //                 ufmt::uwriteln!(&mut serial, "length: {}\r", length).unwrap();
    //                 ufmt::uwriteln!(&mut serial, "ip: {}.{}.{}.{}\r", ip.address[0], ip.address[1], ip.address[2], ip.address[3]).unwrap();
    //             } else {
    //                 ufmt::uwriteln!(&mut serial, "could not receive\r").unwrap();
    //             }
    //             delay.delay_ms(1000);
    //         }
    //     } else {
    //         ufmt::uwriteln!(&mut serial, "could not take socket\r").unwrap();
    //     }
    // } else {
    //     ufmt::uwriteln!(&mut serial, "could not initialize\r").unwrap();
    // }
}
