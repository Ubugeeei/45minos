[Prev](https://github.com/Ubugeeei/45minos/tree/master/030-init-stack-pointer) | [Next](https://github.com/Ubugeeei/45minos/tree/master/050-trap)

# 文字列を表示する

## SBI コールの実装

```rs
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
```

## SBI を使って文字を出力

```rs
fn put_char(ch: u8) {
    sbi_call(0x01, 0x00, ch as usize, 0, 0, 0, 0, 0);
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

[Prev](https://github.com/Ubugeeei/45minos/tree/master/030-init-stack-pointer) | [Next](https://github.com/Ubugeeei/45minos/tree/master/050-trap)
