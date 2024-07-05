#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

use stm32h7xx_hal as hal;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");

    loop {
        nop();
        // your code goes here
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rprintln!("\n#--------- Panic! ---------#\n");
    rprintln!("{}", _info);
    loop {}
}
