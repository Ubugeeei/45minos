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

#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn sbi_call(
    ext_id: usize,
    fn_id: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> SBIRet {
    let error: usize;
    let value: usize;

    unsafe {
        asm!(
            "ecall",
            inout("a0") arg0 => value,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fn_id,
            inout("a7") ext_id => error
        );
    }

    SBIRet { error, value }
}

fn put_char(ch: u8) {
    sbi_call(0x01, 0x00, ch as usize, 0, 0, 0, 0, 0);
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

    println!("Hello, world!");
    println!("{:#04x}", 0x12345abc);

    unsafe {
        asm!("ebreak");
    }

    #[allow(clippy::empty_loop)]
    loop {}
}
