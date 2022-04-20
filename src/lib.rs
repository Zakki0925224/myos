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
        screen.write_char('H');
        screen.write_char('e');
        screen.write_char('l');
        screen.write_char('l');
        screen.write_char('o');
        screen.write_char('!');
    }

    loop {};
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {};
}