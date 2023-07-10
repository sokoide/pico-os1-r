#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

mod exception;
mod systimer;

use rp_pico as bsp;

#[entry]
fn main() -> ! {
    info!("Program start");

    let mut st = systimer::SystemTimer::new();
    st.init();

    loop {
        info!("1");
        exception::print_unsafe_counter();
        st.delay_ms(500);

        info!("2");
        exception::print_unsafe_counter();
        st.delay_ms(500);
    }
}
