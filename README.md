# rust-os
Doing this tutorial: https://os.phil-opp.com/

## Requirements
- rust
- rustup
- qemu

## Install the first used target

It is not necessary, but is used in one of the first steps.

```bash
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf
```

## Install cargo-xbuild

Necessary to build and run (already with qemu) the image.

```bash
rustup component add rust-src
cargo install cargo-xbuild
```

## Build project
```bash
cargo xbuild
```

## Being able to create the image

```bash
cargo install bootimage
rustup component add llvm-tools-preview.
```

## Create the image

Creates the image to futher use:

```bash
cargo bootimage
```

Now it should be somewhere in `target/x86_64-target_system/debug/bootimage-rust-os.bin`

## Run the image in qemu

Make a bootable disk

```bash
dd if=target/x86_64-target_system/debug/bootimage-rust-os.bin of=/dev/sdX && sync
```

Run in qemu
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-target_system/debug/bootimage-rust-os.bin
```

Automaticaly build and run in qemu
```bash
cargo xrun
```
