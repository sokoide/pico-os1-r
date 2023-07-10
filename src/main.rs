//! Blinks the LED on a Pico board
#![no_std]
#![no_main]

use bsp::entry;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::exception;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;

use bsp::hal::{clocks::init_clocks_and_plls, pac, watchdog::Watchdog};

use volatile_register::{RO, RW};

static mut COUNTER: u32 = 0;
const TIMER_MS: u32 = 10;
const SYST_CSR_COUNTFLAG: u32 = 1 << 16;

#[repr(C)]
struct SysTick {
    pub csr: RW<u32>,
    pub rvr: RW<u32>,
    pub cvr: RW<u32>,
    pub calib: RO<u32>,
}

fn get_systick() -> &'static mut SysTick {
    unsafe { &mut *(0xE000_E010 as *mut SysTick) }
}

fn get_csr() -> u32 {
    let systick = get_systick();
    systick.csr.read()
}

fn delay_ms(ms: u32) {
    let mut counter = ms / TIMER_MS;
    while counter > 0 {
        if (get_csr() & SYST_CSR_COUNTFLAG) != 0 {
            counter -= 1;
        }
    }
}

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut _pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(_pac.WATCHDOG);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    init_clocks_and_plls(
        external_xtal_freq_hz,
        _pac.XOSC,
        _pac.CLOCKS,
        _pac.PLL_SYS,
        _pac.PLL_USB,
        &mut _pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut syst = core.SYST;
    syst.set_clock_source(SystClkSource::Core);
    // 125MHz -> reload every 10 ms
    syst.set_reload(125_000_000u32 / 1000 * TIMER_MS);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

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
