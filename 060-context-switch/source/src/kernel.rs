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
    sbi_call(0x01, 1, ch as usize, 0, 0);
}

const BUFFER_SIZE: usize = 128;

struct Buffer {
    data: [u8; BUFFER_SIZE],
    pos: usize,
}

impl Buffer {
    fn new() -> Self {
        Buffer {
            data: [0; BUFFER_SIZE],
            pos: 0,
        }
    }

    fn write(&mut self, byte: u8) {
        if self.pos < BUFFER_SIZE {
            self.data[self.pos] = byte;
            self.pos += 1;
        }
    }

    fn flush(&mut self) {
        for i in 0..self.pos {
            put_char(self.data[i]);
        }
        self.pos = 0;
    }
}

impl core::fmt::Write for Buffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.write(byte);
        }
        Ok(())
    }
}

macro_rules! println {
    ($($arg:tt)*) => {
        {
            let mut buffer = Buffer::new();
            core::fmt::write(&mut buffer, format_args!($($arg)*)).unwrap();
            buffer.write(b'\n');
            buffer.flush();
        }
    };
}

extern "C" fn trap_handler() {
    let (mut _scause, mut _sepc) = (0, 0);
    unsafe {
        asm!(
            "csrr {scause}, scause",
            "csrr {sepc}, sepc",
            scause = out(reg) _scause,
            sepc = out(reg) _sepc,
        );
    }
    println!("Trap: scause={:#x} sepc={:#x}", _scause, _sepc);

    #[allow(clippy::empty_loop)]
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
    unsafe {
        asm!("csrw stvec, {0}", in(reg) trap_handler);
    }

    println!("Hello, world! {:#04x}", 1);

    unsafe {
        asm!("ebreak");
    }

    #[allow(clippy::empty_loop)]
    loop {}
}
