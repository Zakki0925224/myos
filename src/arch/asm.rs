use core::arch::asm;

use crate::println;

// extfunc
extern
{
    fn init_sgm_reg();
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct Dtr
{
    limit: i16,
    base: i32,
}

pub fn hlt()
{
    unsafe { asm!("hlt"); }
}

pub fn cli()
{
    unsafe { asm!("cli"); }
}

pub fn sti()
{
    unsafe { asm!("sti"); }
}

pub fn test()
{
    //unsafe { asm!("int 0x16"); }
    load_gdtr(0xdead, 0xbeaf);
}

pub fn load_idtr(limit: i32, addr: i32)
{
    unsafe
    {
        asm!("lidt [{}]", in(reg) &Dtr { limit: limit as i16, base: addr });
    }
}

pub fn load_gdtr(limit: i32, addr: i32)
{
    unsafe
    {
        asm!("lgdt [{}]", in(reg) &Dtr { limit: limit as i16, base: addr });
        init_sgm_reg();
    }
}

pub fn out8(port: u32, data: u8)
{
    unsafe { asm!("out dx, al", in("edx") port, in("al") data); }
}

pub fn in8(port: u32) -> u8
{
    let mut data: u8;
    unsafe { asm!("in al, dx", out("al") data, in("edx") port); }
    return data;
}

#[macro_export]
macro_rules! handler
{
    ($name: ident) =>
    {{
        pub extern "C" fn wrapper() -> !
        {
            unsafe
            {
                asm!("push es");
                asm!("push ds");
                asm!("pushad");
                asm!("mov eax, esp");
                asm!("push eax");
                asm!("mov ax, ss");
                asm!("mov ds, ax");
                asm!("mov es, ax");
                asm!("call {}", in(reg) $name as extern "C" fn());
                asm!("pop eax");
                asm!("popad");
                asm!("pop ds");
                asm!("pop es");
                asm!("iret");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}