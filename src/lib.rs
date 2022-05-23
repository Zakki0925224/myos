#![no_std]
#![no_main]
#![feature(asm)]
#![feature(start)]
#![feature(core_intrinsics)]
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]
// #![reexport_test_harness_main = "test_main"]

mod arch;
mod data;
mod device;
mod meta;
mod mem;
mod util;

use core::panic::PanicInfo;
use arch::{vga::{VGA_SCREEN, Color}, asm, sgm};
use multiboot2::{self, BootInformation};

use crate::{arch::int::{self, KEYBUF, MOUSEBUF}, device::keyboard::{Keyboard, KeyLayout}, util::boot_info::{get_kernel_addr, get_multiboot_addr, get_total_mem_size, get_all_mem_areas}};

#[no_mangle]
#[start]
pub extern "C" fn kernel_main(magic: u32, boot_info_addr: u32) -> !
{
    let boot_info = unsafe { multiboot2::load(boot_info_addr as usize).expect("Failed to load
    boot info") };

    if magic != multiboot2::MULTIBOOT2_BOOTLOADER_MAGIC
    {
        panic!("Invalid magic number: 0x{:x}", magic);
    }

    debug(&boot_info);

    println!("\n===============================");
    println!("Welcome to {}!", meta::OS_NAME);
    println!("Description: {}", meta::OS_DESCRIPTION);
    println!("Version: {}", meta::OS_VERSION);
    println!("Author: {}", meta::OS_AUTHORS);

    sgm::init();
    int::init_pic();
    int::enable_mouse();
    mem::init(&boot_info);

    let mut keyboard = Keyboard::new(KeyLayout::AnsiUs104);
    asm::sti();

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

fn debug(boot_info: &BootInformation)
{
    let (kernel_start, kernel_end) = get_kernel_addr(&boot_info);
    let (multiboot_start, multiboot_end) = get_multiboot_addr(&boot_info);

    println!("All memory areas:");
    for area in get_all_mem_areas(&boot_info)
    {
        println!("\t0x{:x} -> 0x{:x}, Size: {}B, Type: {:?}", area.start_address(), area.end_address() - 1, area.size() - 1, area.typ());
    }

    println!("\ttotal: {}B", get_total_mem_size(&boot_info));

    println!("Kernel 0x{:x} -> 0x{:x}, Size: {}B", kernel_start, kernel_end, kernel_start + kernel_end);
    println!("Multiboot 0x{:x} -> 0x{:x}, Size: {}B", multiboot_start, multiboot_end, multiboot_start + multiboot_end);
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