#![no_std]
#![feature(asm)]
#![feature(start)]
#![feature(core_intrinsics)]

pub mod arch;

use core::{panic::PanicInfo, fmt::Write};

use arch::vga::{VgaScreen, Color};

#[no_mangle]
#[start]
pub extern "C" fn kernel_main() -> !
{
    let mut screen = VgaScreen::new(Color::White, Color::Black);
    screen.cls();
    //screen.write_string("Welcome to my os!");
    write!(screen, "Hello, {}", 1 + 2 * 5);

    loop {};
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {};
}