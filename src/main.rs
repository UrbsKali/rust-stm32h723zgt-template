#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

use stm32h7xx_hal as hal;
mod utils;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");

    loop {
        nop();
        // your code goes here
    }
}
