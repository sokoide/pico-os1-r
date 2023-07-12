use cortex_m_rt::exception;

#[exception]
fn SysTick() {
    unsafe {
        if crate::COUNTER == 0xFFFF_FFFF {
            crate::COUNTER = 0;
        } else {
            crate::COUNTER += 1;
        }
    }
}

#[exception]
unsafe fn DefaultHandler(_irqn: i16) {}
