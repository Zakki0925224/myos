#![no_std]
#![feature(asm)]
#![feature(start)]
#![feature(core_intrinsics)]

pub mod arch;
pub mod meta;

use core::{panic::PanicInfo, fmt::Write};

use arch::{vga::{VGA_SCREEN}, asm};

#[no_mangle]
#[start]
pub extern "C" fn kernel_main() -> !
{
    // VGA_SCREEN.lock().cls();

    // write!(VGA_SCREEN.lock(), "Welcome to {}!\n", meta::OS_NAME).unwrap();
    // write!(VGA_SCREEN.lock(), "Description: {}\n", meta::OS_DESCRIPTION).unwrap();
    // write!(VGA_SCREEN.lock(), "Version: {}\n", meta::OS_VERSION).unwrap();
    // write!(VGA_SCREEN.lock(), "Author: {}\n", meta::OS_AUTHORS).unwrap();

    println!("Welcome to {}!", meta::OS_NAME);
    println!("Description: {}", meta::OS_DESCRIPTION);
    println!("Version: {}", meta::OS_VERSION);
    println!("Author: {}", meta::OS_AUTHORS);

    loop
    {
        unsafe { asm::io_hlt(); }
    }

    loop
    {
        unsafe { asm::io_hlt(); }
    };
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {};
}