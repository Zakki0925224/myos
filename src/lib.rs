#![no_std]
#![no_main]
#![feature(asm)]
#![feature(start)]
#![feature(core_intrinsics)]
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]
// #![reexport_test_harness_main = "test_main"]

pub mod arch;
pub mod data;
pub mod device;
pub mod meta;
pub mod util;

use core::panic::PanicInfo;
use arch::{vga::{VGA_SCREEN, Color}, asm, sgm};
use multiboot2;

use crate::{arch::int::{self, KEYBUF, MOUSEBUF}, device::keyboard::{self, Keyboard, KeyLayout}};

#[no_mangle]
#[start]
pub extern "C" fn kernel_main(magic: u32, boot_info_addr: u32) -> !
{
    let boot_info = unsafe { multiboot2::load(boot_info_addr as usize).expect("Failed to load boot info") };
    let memory_map_tag = boot_info.memory_map_tag().expect("No memory map tag");
    let elf_sections_tag = boot_info.elf_sections_tag().expect("No elf sections tag");
    let kernel_start = elf_sections_tag.sections().map(|s| s.start_address()).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.start_address() + s.size()).max().unwrap();
    let mboot_start = boot_info.start_address();
    let mboot_end = mboot_start + (boot_info.total_size() as usize);

    if magic != multiboot2::MULTIBOOT2_BOOTLOADER_MAGIC
    {
        panic!("Invalid magic number: 0x{:x}", magic);
    }

    println!("All available memory areas:");
    let mut total_size = 0;
    for area in memory_map_tag.memory_areas()
    {
        let (b, u) = util::convert_bytes_to_any(area.size());
        total_size += area.size();
        println!("  start: 0x{:x}, end: 0x{:x}, size: {}{}", area.start_address(), area.end_address(), b, u);
    }
    let (b, u) = util::convert_bytes_to_any(total_size);
    println!("  total: {}{}", b, u);

    // println!("Elf sections:");
    // let mut total = 0;
    // for section in elf_sections_tag.sections()
    // {
    //     println!("  addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}", section.start_address(), section.size(), section.flags());
    //     total += 1;
    // }
    // println!("  total: {}", total);

    let (b, u) = util::convert_bytes_to_any((kernel_end - kernel_start) as u64);
    println!("Kernel start: 0x{:x}, end: 0x{:x}, size: {}{}", kernel_start, kernel_end, b, u);
    let (b, u) = util::convert_bytes_to_any((mboot_end - mboot_start) as u64);
    println!("Multiboot start: 0x{:x}, end: 0x{:x}, size: {}{}", mboot_start, mboot_end, b, u);

    println!("\n===============================");
    println!("Welcome to {}!", meta::OS_NAME);
    println!("Description: {}", meta::OS_DESCRIPTION);
    println!("Version: {}", meta::OS_VERSION);
    println!("Author: {}", meta::OS_AUTHORS);

    sgm::init();
    int::init_pic();
    int::enable_mouse();
    asm::sti();

    let mut keyboard = Keyboard::new(KeyLayout::AnsiUs104);

    // #[cfg(test)]
    // test_main();

    loop
    {
        asm::cli();

        if KEYBUF.lock().status() != 0
        {
            let key = KEYBUF.lock().get().unwrap();
            asm::sti();
            keyboard.input(key);
        }
        else if MOUSEBUF.lock().status() != 0
        {
            let i = MOUSEBUF.lock().get().unwrap();
            asm::sti();
            println!("[M]0x{:x}", i);
        }
        else
        {
            asm::sti();
            asm::hlt();
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
    VGA_SCREEN.lock().set_color(Color::Red, Color::Black);
    println!("{}", info);
    loop {};
}

// #[cfg(test)]
// fn test_runner(tests: &[&dyn Fn()])
// {
//     println!("Running {} tests", tests.len());

//     for test in tests
//     {
//         test();
//     }
// }

// #[test_case]
// fn trivial_assertion()
// {
//     print!("trivial assertion... ");
//     assert_eq!(1, 1);
//     println!("[ok]");
// }