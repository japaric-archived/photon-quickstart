//! Transmit the device ID repeatedly via USB serial.

#![no_std]

#[macro_use]
extern crate photon;
extern crate photon_hal as hal;

use hal::UsbSerial;
use photon::App;

const PERIOD: u32 = 1_000; // ms
const BAUD_RATE: u32 = 115_200;

app! {
    setup: setup,
    loop: loop_,
}

fn setup(_: App) {
    UsbSerial.begin(BAUD_RATE);
}

fn loop_(ref mut app: App) {
    // Get Device ID
    let device_id = hal::device_id();

    // Print Device ID via USB serial
    for byte in b"Device ID is " {
        UsbSerial.write(*byte);
    }

    for byte in device_id.iter() {
        UsbSerial.write(*byte);
    }

    UsbSerial.write(b'\n');
    UsbSerial.write(b'\r');

    app.delay_ms(PERIOD);
}
