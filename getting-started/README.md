# 1. 必要なもののインストール

- qemu (qemu-system-riscv32)
- rustup, cargo
- make

# 2. クレートの作成

```sh
$ cargo new 45minos
$ cd 45minos
```

# 3. ターゲットの追加

```sh
$ rustup target add riscv32i-unknown-none-elf
```

# 4. カーネルのソースコードをビルドできるように

main.rs を削除し、kernel.rs を作成

```sh
$ rm src/main.rs
$ touch src/kernel.rs
```

Cargo.toml を編集

```toml
[[bin]]
name = "45minos_kernel"
path = "src/kernel.rs"
```

リンカスクリプトを作成

```sh
$ touch kernel.ld
```

```
ENTRY(boot)

SECTIONS {
    . = 0x80200000;

    .text :{
        KEEP(*(.text.boot));
        *(.text .text.*);
    }

    .rodata : ALIGN(4) {
        *(.rodata .rodata.*);
    }

    .data : ALIGN(4) {
        *(.data .data.*);
    }

    .bss : ALIGN(4) {
        __bss = .;
        *(.bss .bss.* .sbss .sbss.*);
        __bss_end = .;
    }

    . = ALIGN(4);
    . += 128 * 1024;
    __stack_top = .;
}
```

リンカスクリプトの設定とターゲット指定

```sh
$ mkdir .cargo
$ touch .cargo/config
```

```toml
[target.riscv32i-unknown-none-elf]
rustflags = ["-Clink-arg=-Tkernel.ld", "-Clink-arg=-Map=target/kernel.map"]

[build]
target = "riscv32i-unknown-none-elf"
```

本体のコードを書く

no_std, no_main でビルド。

extern C で boot 関数を作成し、
インラインアセンブリで kernel_main にジャンプする。

```rs
#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[link_section = ".text.boot"]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn boot() -> ! {
    asm!(
        "j {kernel_main}",
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
```

# 5. Makefile を作っておく

```sh
$ touch Makefile
```

```makefile
.PHONY: default

QEMU=qemu-system-riscv32

default: ./target/riscv32i-unknown-none-elf/release/45minos_kernel
	- $(QEMU) -machine virt -bios default -nographic -serial mon:stdio --no-reboot \
      -kernel ./target/riscv32i-unknown-none-elf/release/45minos_kernel

./target/riscv32i-unknown-none-elf/release/45minos_kernel:
	- cargo build --release
```

あとは

```sh
$ make
```

で起動できれば OK
