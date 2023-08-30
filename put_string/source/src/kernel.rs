#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[repr(C)]
struct SBIRet {
    error: usize,
    value: usize,
}

fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> SBIRet {
    let error: usize;
    let value: usize;

    unsafe {
        asm!(
            "mv a0, {arg0}",
            "mv a1, {arg1}",
            "mv a2, {arg2}",
            "mv a7, {eid}",
            "mv a6, {fid}",
            "ecall",

            arg0 = in(reg) arg0,
            arg1 = in(reg) arg1,
            arg2 = in(reg) arg2,
            eid = in(reg) eid,
            fid = in(reg) fid,

            out("a0") error,
            out("a1") value,
        );
    }

    SBIRet { error, value }
}

fn put_char(ch: u8) {
    sbi_call(0x01, 0, ch as usize, 0, 0);
}

fn put_string(s: &str) {
    for ch in s.bytes() {
        put_char(ch);
    }
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
    put_string("Hello, world!");
    #[allow(clippy::empty_loop)]
    loop {}
}
