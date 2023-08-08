#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::cell::RefCell;

use arduino_hal::{
    clock::MHz16,
    delay_ms,
    hal::{
        port::{PD0, PD1},
        Usart,
    },
    port::{
        mode::{Input, Output},
        Pin,
    },
    Peripherals,
};
use arduino_hal::{prelude::*, Pins};
use avr_device::{
    atmega328p::USART0,
    interrupt::{self, Mutex},
};
use once_cell::sync::OnceCell;
use panic_halt as _;
use ufmt::uwriteln;

type Serial = Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>, MHz16>;

static SERIAL: Mutex<OnceCell<RefCell<Serial>>> = Mutex::new(OnceCell::new());

#[arduino_hal::entry]
fn main() -> ! {
    let d7 = avr_device::interrupt::free(|cs| {
        let dp = Peripherals::take().unwrap();
        let pins: Pins = arduino_hal::pins!(dp);
        let serial = arduino_hal::default_serial!(dp, pins, 57600);

        SERIAL.borrow(cs).get_or_init(|| RefCell::new(serial));

        pins.d2.into_pull_up_input();
        dp.EXINT.eicra.modify(|_, w| w.isc0().bits(0x02));
        dp.EXINT.eimsk.modify(|_, w| w.int0().set_bit());

        pins.d7.into_pull_up_input()
    });

    unsafe { avr_device::interrupt::enable() };

    loop {
        interrupt::free(|cs| {
            let mut s = SERIAL.borrow(cs).get().unwrap().borrow_mut();
            let low = d7.is_low();
            uwriteln!(&mut s, "Low: {}", low).void_unwrap();
        });

        delay_ms(1000);
    }
}

#[avr_device::interrupt(atmega328p)]
fn INT0() {
    interrupt::free(|cs| {
        let mut s = SERIAL.borrow(cs).get().unwrap().borrow_mut();
        uwriteln!(&mut s, "Interrupt").void_unwrap();
    });
}
