//! Blinks the LED on a Pico board
#![no_std]
#![no_main]

use bsp::entry;
use cortex_m_rt::exception;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

mod systimer;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;

static mut COUNTER: u32 = 0;

fn delay_ms(ms: u32) {
    let mut counter = ms / systimer::TIMER_PERIOD;
    let mut st = systimer::SystemTimer::new();
    while counter > 0 {
        if st.has_wrapped() {
            counter -= 1;
        }
    }
}

#[entry]
fn main() -> ! {
    info!("Program start");

    let mut st = systimer::SystemTimer::new();
    st.init();

    loop {
        info!("1");
        print_counter();
        delay_ms(500);

        info!("2");
        print_counter();
        delay_ms(500);
    }
}

fn print_counter() {
    unsafe {
        info!("unsafe counter: {}", COUNTER);
    }
}

#[exception]
fn SysTick() {
    unsafe {
        if COUNTER == 0xFFFF_FFFF {
            COUNTER = 0;
        } else {
            COUNTER += 1;
        }
    }
}

#[exception]
unsafe fn DefaultHandler(_irqn: i16) {}
