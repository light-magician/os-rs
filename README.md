# os-rs

operating system concepts in rust

### Building for Bare Metal Target

Rht rust compiler and linker assume that there is
an underlying OS that uses the C runtime by default.
This causes linker errors, so to avoid linker errors,
compile to a different environment with no underlying OS.

```bash
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf
```

By passing the `--target` argument, the executable
is cross compiled for a bare metal target system.

### Compile for Host Systems

```bash
# Linux
cargo rustc -- -C link-arg=-nostartfiles
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```
