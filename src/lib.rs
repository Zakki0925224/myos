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

use core::panic::PanicInfo;
use arch::{vga::{VGA_SCREEN, Color}, asm, sgm};

use crate::{arch::int::{self, KEYBUF, MOUSEBUF}, device::keyboard};

#[no_mangle]
#[start]
pub extern "C" fn kernel_main() -> !
{
    println!("Welcome to {}!", meta::OS_NAME);
    println!("Description: {}", meta::OS_DESCRIPTION);
    println!("Version: {}", meta::OS_VERSION);
    println!("Author: {}", meta::OS_AUTHORS);

    sgm::init();
    int::init_pic();
    int::enable_mouse();
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
            let e = keyboard::get_key(key);
            println!("[K]: {:?}", e);
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