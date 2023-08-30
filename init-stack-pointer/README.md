[Prev](https://github.com/Ubugeeei/45minos/tree/master/build-and-book-kernel)

# スタックポインタの初期化

初期化するだけ (ポインタを与えるだけ)

```rs
extern "C" {
    static __stack_top: u8;
}


// .
// .
// .
pub unsafe extern "C" fn boot() -> ! {
    asm!(
        "mv sp, {stack_top}", // これ
        "j {kernel_main}",
        stack_top = in(reg) &__stack_top, // これ
        kernel_main = sym kernel_main,
    );
    #[allow(clippy::empty_loop)]
    loop {}
}
```

[Prev](https://github.com/Ubugeeei/45minos/tree/master/build-and-book-kernel)
