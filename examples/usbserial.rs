//! Transmit the word 'Rust' repeatedly via USB serial.

#![no_std]

extern crate particle_hal as hal;
#[macro_use]
extern crate photon;

app! {
    setup: setup,
    loop: loop_,
}

use hal::delay;
use hal::usb;

const PERIOD: u32 = 100; // ms
const BAUD_RATE: u32 = 115_200;

fn setup() {
    unsafe {
        usb::USB_USART_Init(BAUD_RATE);
    }
}

fn loop_() {
    unsafe {
        usb::USB_USART_Send_Data(b'R');
        delay::HAL_Delay_Milliseconds(PERIOD);
        usb::USB_USART_Send_Data(b'u');
        delay::HAL_Delay_Milliseconds(PERIOD);
        usb::USB_USART_Send_Data(b's');
        delay::HAL_Delay_Milliseconds(PERIOD);
        usb::USB_USART_Send_Data(b't');
        delay::HAL_Delay_Milliseconds(PERIOD);
        usb::USB_USART_Send_Data(b' ');
        delay::HAL_Delay_Milliseconds(PERIOD);
    }
}
