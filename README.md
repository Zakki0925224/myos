# myos
GRUBを使ったrust製自作OS
## Used tools
* rust (nightly)
* cargo
* cargo-xbuild
* qemu
* qemu-arch-extra
* nasm
* mtools
* go-task
* xorriso
* grub
* ld
* dd

## Features
- [x] x86 support
- [x] BIOS support
- [x] Written in Rust and Assembly
- [x] Boot loader: GRUB2
- Screen rendering
  - [x] VGA Text mode
  - [ ] Own GUI
- Device support
  - [x] PS/2 Keyboard (Only ANSI US 104 Keyboard)
  - [x] PS/2 Mouse
  - [ ] Serial: UART 16650
  - [ ] PCI connection
    - [ ] SATA drive support (AHCI)
  - [ ] USB connection
- File system support
  - [ ] FAT32
- [x] Memory management: Segmentation -> Paging
- [x] Interrupts controller (PIC): Intel 8259A
- [x] Built-In Shell