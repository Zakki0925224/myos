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
    write!(screen, "Welcome to {}: v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).unwrap();

    loop {};
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {};
}