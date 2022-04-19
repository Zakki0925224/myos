#![no_std]
#![feature(asm)]
#![feature(start)]
#![feature(core_intrinsics)]

use core::panic::PanicInfo;

#[no_mangle]
#[start]
pub extern "C" fn kernel_main() -> !
{
    loop {};
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {};
}
