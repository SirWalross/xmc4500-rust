#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals: xmc4500::Peripherals = unsafe { xmc4500::Peripherals::steal() };

    // setup led: port1 pin 0
    peripherals.PORT1.hwsel().write(|w| w.hw0().value1());
    peripherals.PORT1.omr().write(|w| w.pr0().set_bit());
    peripherals.PORT1.pdr0().write(|w| w.pd0().sd_sle());
    peripherals
        .PORT1
        .iocr0()
        .write(|w| w.pc0().value9() );

    loop {
        // your code goes here
        for _ in 0..(1 << 20) {
            asm::nop();
        }
        peripherals.PORT1.omr().write(|w| w.pr0().set_bit().ps0().set_bit());
    }
}
