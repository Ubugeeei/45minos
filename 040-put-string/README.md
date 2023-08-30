[Prev](https://github.com/Ubugeeei/45minos/tree/master/030-init-stack-pointer)

# 文字列を表示する

## SBI コールの実装

```rs
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
```

## SBI を使って文字を出力

```rs
fn put_char(ch: u8) {
    sbi_call(0x01, 1, ch as usize, 0, 0);
}
```

## 簡易的な println マクロの実装

buffer に書き込んで flush で put_char する.

フォーマットは core に生えてるものを使う

```rs
use core::fmt;

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
```

kernel_main で使ってみる

```rs
fn kernel_main() {
    println!("Hello, world! {:#04x}", 1);
    #[allow(clippy::empty_loop)]
    loop {}
}
```

`Hello, world! 0x01` が表示されれば OK!

[Prev](https://github.com/Ubugeeei/45minos/tree/master/030-init-stack-pointer)
