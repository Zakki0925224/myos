#![no_std]
#![no_main]
#![feature(asm)]
#![feature(start)]
#![feature(core_intrinsics)]
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]
// #![reexport_test_harness_main = "test_main"]

mod arch;
mod console;
mod data;
mod device;
mod meta;
mod mem;
mod util;

use core::panic::PanicInfo;
use arch::{vga::{VGA_SCREEN, Color}, asm, sgm};
use multiboot2::{self, BootInformation};

use crate::{arch::int::{self, KEYBUF, MOUSEBUF}, device::keyboard::{Keyboard, KeyLayout}, util::{boot_info::*, logger::*}, console::{SystemConsole, ascii}, mem::PAGING};

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

    println!("Welcome to {}!", meta::OS_NAME);
    println!("Description: {}", meta::OS_DESCRIPTION);
    println!("Version: {}", meta::OS_VERSION);
    println!("Author: {}", meta::OS_AUTHORS);

    sgm::init();
    int::init_pic();
    int::enable_mouse();
    mem::init(&boot_info);
    device::init();

    let mut keyboard = Keyboard::new(KeyLayout::AnsiUs104);

    let mut console = SystemConsole::new();
    console.start();
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
            let e = keyboard.input(key);

            if !e.eq(&None)
            {
                let asc = ascii::key_event_to_ascii_code(e.unwrap().0, e.unwrap().1);

                if !asc.eq(&None) && console.is_waiting_input()
                {
                    console.input_char(asc.unwrap());
                }
            }
        }
        else if MOUSEBUF.lock().status() != 0
        {
            let i = MOUSEBUF.lock().get().unwrap();
            asm::sti();
            //log_debug("mouse", i);
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
    VGA_SCREEN.lock().set_fore_color(Color::Red);
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