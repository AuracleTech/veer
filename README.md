# veer

#### dependencies

```rust,ignore
rustup toolchain install nightly

rustup component add rust-src --toolchain nightly-x86_64-pc-windows-msvc

rustup component add llvm-tools-preview

cargo install bootimage
```

#### bootimage tasks

```rust,ignore
➡️ Compiles our kernel to an ELF file
➡️ Compiles the bootloader dependency as a standalone executable
➡️ Links the bytes of the kernel ELF file to the bootloader
```
