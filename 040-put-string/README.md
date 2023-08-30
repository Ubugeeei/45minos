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
    sbi_call(0x01, 0, ch as usize, 0, 0);
}


// 文字列も対応できるように
fn put_string(s: &str) {
    for ch in s.bytes() {
        put_char(ch);
    }
}
```

kernel_main で使ってみる

```rs
fn kernel_main() {
    put_string("Hello, world!");
    #[allow(clippy::empty_loop)]
    loop {}
}
```

`Hello, world!` が表示されれば OK!

[Prev](https://github.com/Ubugeeei/45minos/tree/master/030-init-stack-pointer)
