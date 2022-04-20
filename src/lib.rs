#![no_std]
#![feature(asm)]
#![feature(start)]
#![feature(core_intrinsics)]

pub mod arch;
pub mod meta;

use core::{panic::PanicInfo, fmt::Write};

use arch::{vga::{VgaScreen, Color}, asm};

#[no_mangle]
#[start]
pub extern "C" fn kernel_main() -> !
{
    let mut screen = VgaScreen::new(Color::White, Color::Black);
    screen.cls();
    //screen.write_string("Welcome to my os!");
    write!(screen, "Welcome to {}!\n", meta::OS_NAME).unwrap();
    write!(screen, "Description: {}\n", meta::OS_DESCRIPTION).unwrap();
    write!(screen, "Version: {}\n", meta::OS_VERSION).unwrap();
    write!(screen, "Author: {}\n", meta::OS_AUTHORS).unwrap();

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