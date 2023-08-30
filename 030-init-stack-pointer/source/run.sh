QEMU=qemu-system-riscv32

cargo build --release

$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot \
  -kernel ./target/riscv32i-unknown-none-elf/release/45minos_kernel
