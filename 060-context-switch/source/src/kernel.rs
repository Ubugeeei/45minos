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

#[inline(always)]
unsafe extern "C" fn switch_context(old_sp: usize, new_sp: usize) {
    asm!(
        "addi sp, sp, -13 * 4",
        // store
        "sw ra, 0 * 4(sp)",
        "sw s0, 1 * 4(sp)",
        "sw s1, 2 * 4(sp)",
        "sw s2, 3 * 4(sp)",
        "sw s3, 4 * 4(sp)",
        "sw s4, 5 * 4(sp)",
        "sw s5, 6 * 4(sp)",
        "sw s6, 7 * 4(sp)",
        "sw s7, 8 * 4(sp)",
        "sw s8, 9 * 4(sp)",
        "sw s9, 10 * 4(sp)",
        "sw s10, 11 * 4(sp)",
        "sw s11, 12 * 4(sp)",
        "sw sp, ({old_sp})",
        // load
        "lw sp, ({new_sp})",
        "lw ra, 0 * 4(sp)",
        "lw s0, 1 * 4(sp)",
        "lw s1, 2 * 4(sp)",
        "lw s2, 3 * 4(sp)",
        "lw s3, 4 * 4(sp)",
        "lw s4, 5 * 4(sp)",
        "lw s5, 6 * 4(sp)",
        "lw s6, 7 * 4(sp)",
        "lw s7, 8 * 4(sp)",
        "lw s8, 9 * 4(sp)",
        "lw s9, 10 * 4(sp)",
        "lw s10, 11 * 4(sp)",
        "lw s11, 12 * 4(sp)",
        "addi sp, sp, 13 * 4",
        "ret",
        old_sp = in(reg) old_sp,
        new_sp = in(reg) new_sp,
    );
}

const STACK_SIZE: usize = 8192;

#[repr(C)]
struct Thread {
    sp: usize,
    stack: [usize; STACK_SIZE],
}

impl Thread {
    fn init(&mut self, entry: extern "C" fn(*mut ())) {
        let sp = self.stack.as_mut_ptr() as *mut usize;
        let mut sp = unsafe { sp.add(STACK_SIZE / core::mem::size_of::<usize>()) };

        for i in 0..12 {
            unsafe {
                sp = sp.offset(-1);
                *sp = 0;
                println!("sp: {:#x} (s{})", sp as usize, 11 - i);
            }
        }

        unsafe {
            sp = sp.offset(-1);
            *sp = entry as usize;
            println!("sp: {:#x} (ra)", sp as usize);
        } // ra
        self.sp = sp as usize;
        println!("sp: {:#x} (sp)", self.sp);
    }
}

static mut THREAD_A: Thread = Thread {
    sp: 0,
    stack: [0; STACK_SIZE],
};
static mut THREAD_B: Thread = Thread {
    sp: 0,
    stack: [0; STACK_SIZE],
};

extern "C" fn thread_a_entry(_arg: *mut ()) {
    loop {
        unsafe {
            println!("A");
            switch_context(THREAD_A.sp, THREAD_B.sp);
        }
    }
}

extern "C" fn thread_b_entry(_arg: *mut ()) {
    loop {
        unsafe {
            println!("B");
            switch_context(THREAD_B.sp, THREAD_A.sp);
        }
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
    unsafe { asm!("csrw stvec, {0}", in(reg) trap_handler) }

    unsafe { THREAD_A.init(thread_a_entry) };
    unsafe { println!("THREAD_A.sp: {:#x}", THREAD_A.sp) }
    unsafe { THREAD_B.init(thread_b_entry) };
    unsafe { println!("THREAD_B.sp: {:#x}", THREAD_B.sp) }
    let tmp = &0 as *const _ as usize;
    println!("tmp: {:#x}", tmp);
    unsafe { switch_context(tmp, THREAD_A.sp) };

    println!("unreachable");

    #[allow(clippy::empty_loop)]
    loop {}
}
