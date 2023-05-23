#![no_std] // don't link the std lib
#![no_main] // don't use the usual entry points for rust programs

use core::panic::PanicInfo;

#[panic_handler] // called when there's a panic
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // use the name _start as it is
pub extern "C" fn _start() -> ! {
    loop {}
}
