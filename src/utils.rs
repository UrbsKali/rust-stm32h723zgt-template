use core::panic::PanicInfo;
use rtt_target::rprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rprintln!("\n#--------- Panic! ---------#\n");
    rprintln!("{}", _info);
    loop {}
}
