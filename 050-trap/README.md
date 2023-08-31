[Prev](https://github.com/Ubugeeei/45minos/tree/master/040-put-string)

# トラップ

システムコールや割り込みしょるのためにトラップハンドラを実装する

## ハンドラの実装

例外が起きた時に実際に実行される関数です

```rs

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
```

## ハンドラの登録

`csrw stvec` でハンドラのアドレスを登録します

```rs
fn kernel_main() {
    unsafe {
        asm!("csrw stvec, {0}", in(reg) trap_handler);
    }

    // .
    // .
    // .
}
```

## 例外を発生させてみる

```rs
fn kernel_main() {
    unsafe {
        asm!("ebreak");
    }
    #[allow(clippy::empty_loop)]
    loop {}
}

```

trap_handler が実行され、print されていれば Ok!

[Prev](https://github.com/Ubugeeei/45minos/tree/master/040-put-string)
