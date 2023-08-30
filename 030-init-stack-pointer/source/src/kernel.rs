#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    static __stack_top: u8;
}

#[no_mangle]
#[link_section = ".text.boot"]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn boot() -> ! {
    asm!(
        "mv sp, {stack_top}",
        "j {kernel_main}",
        stack_top = in(reg) &__stack_top,
        kernel_main = sym kernel_main,
    );
    #[allow(clippy::empty_loop)]
    loop {}
}

#[allow(dead_code)]
fn kernel_main() {
    #[allow(clippy::empty_loop)]
    loop {}
}
