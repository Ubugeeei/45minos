[Next](https://github.com/Ubugeeei/45minos/tree/master/020-build-and-book-kernel)

# 1. 必要なもののインストール

- qemu (qemu-system-riscv32)
- rustup, cargo
- make

# 2. クレートの作成

```sh
$ cargo new 45minos
$ cd 45minos
```

# 3. run.sh で QEMU で起動できるように

```sh
$ touch run.sh
```

```sh
QEMU=qemu-system-riscv32

$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot
```

[Next](https://github.com/Ubugeeei/45minos/tree/master/020-build-and-book-kernel)
