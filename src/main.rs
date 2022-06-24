#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![feature(llvm_asm)]

mod sleep;
mod led;
mod board;

extern crate avr_device;

use core::panic::PanicInfo;
use avr_device::attiny85::Peripherals;

use sleep::{wdt_sleep, WDTSleepDur};
use led::{led_01_setup, led_01_set};
use board::board_setup;

#[avr_device::entry]
fn main() -> ! {
    board_setup();
    let peripherals = unsafe { Peripherals::steal() };
    let portb = peripherals.PORTB;
    // set port 1 (LED) as output
    led_01_setup();
    // set port 1 (LED) on
    portb.portb.write(|w| w.pb1().set_bit());

    loop {
        wdt_sleep(WDTSleepDur::Sleep8S);
        led_01_set(false);
        wdt_sleep(WDTSleepDur::Sleep8S);
        led_01_set(true);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}