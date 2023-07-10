use cortex_m_rt::exception;
use defmt::*;

pub static mut COUNTER: u32 = 0;

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

pub fn print_unsafe_counter() {
    unsafe {
        info!("unsafe counter: {}", COUNTER);
    }
}
