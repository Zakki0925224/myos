# myos
GRUBを使ったrust製自作OS
## Used tools
* cargo
* cargo-xbuild
* dd
* find
* go-task
* grub
* ld
* mtools
* nasm
* qemu
* qemu-arch-extra
* rust (nightly)
* xargs
* xorriso

## Features
- [x] x86 support
- [x] BIOS support
- [x] Written in Rust and Assembly
- [x] Boot loader: GRUB2
- Screen rendering
  - [x] VGA Text mode
  - [ ] VESA BIOS Extensions
- Device support
  - [x] PS/2 Keyboard (Only ANSI US 104 Keyboard)
  - [x] PS/2 Mouse
  - [x] Serial: UART 16650
  - [ ] PCI connection
    - [ ] SATA drive support (AHCI)
  - [ ] USB connection
- File system support
  - [ ] FAT32
- [x] Memory management: Segmentation -> Paging
- [x] Interrupts controller (PIC): Intel 8259A
- [x] Built-In Shell