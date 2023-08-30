.PHONY: default

QEMU=qemu-system-riscv32

default: ./target/riscv32i-unknown-none-elf/release/45minos_kernel
	- $(QEMU) -machine virt -bios default -nographic -serial mon:stdio --no-reboot \
      -kernel ./target/riscv32i-unknown-none-elf/release/45minos_kernel

./target/riscv32i-unknown-none-elf/release/45minos_kernel:
	- cargo build --release