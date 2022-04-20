#![no_std]
#![feature(asm)]
#![feature(start)]
#![feature(core_intrinsics)]

pub mod arch;

use core::panic::PanicInfo;

use arch::vga::{VgaScreen, Color};

#[no_mangle]
#[start]
pub extern "C" fn kernel_main() -> !
{
    let mut screen = VgaScreen::new(Color::White, Color::Black);
    screen.cls();

    for i in 0..500
    {
        screen.write_string("HELLO\t");
    }

    loop {};
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {};
}