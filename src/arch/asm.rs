use core::arch::asm;

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
    unsafe { asm!("int 8"); }
}

pub fn load_idtr(limit: i32, addr: i32)
{
    unsafe { asm!("lidt [{}]", in(reg) &Dtr { limit: limit as i16, base: addr }); }
}

pub fn load_gdtr(limit: i32, addr: i32)
{
    unsafe
    {
        asm!("lgdt [{}]", in(reg) &Dtr { limit: limit as i16, base: addr });
        init_sgm_reg();
    }
}

pub fn set_cr3(cr3: u32)
{
    unsafe { asm!("mov cr3, {}", in(reg) cr3); }
}

pub fn get_cr3() -> u32
{
    let mut cr3 = 0;
    unsafe { asm!("mov {}, cr3", out(reg) cr3); }
    return cr3;
}

pub fn invlpg(virt_addr: u32)
{
    cli();
    unsafe { asm!("invlpg {}", in(reg) virt_addr); }
    sti();
}

pub fn enable_paging()
{
    cli();
    unsafe
    {
        asm!("push eax");
        asm!("mov eax, cr0");
        asm!("or eax, 0x80000001");
        asm!("mov cr0, eax");
        asm!("pop eax");
    }
    sti();
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

pub fn out16(port: u32, data: u16)
{
    unsafe { asm!("out dx, ax", in("edx") port, in("ax") data); }
}

pub fn in16(port: u32) -> u16
{
    let mut data: u16;
    unsafe { asm!("in ax, dx", out("ax") data, in("edx") port); }
    return data;
}

pub fn out32(port: u32, data: u32)
{
    unsafe { asm!("out dx, eax", in("edx") port, in("eax") data); }
}

pub fn in32(port: u32) -> u32
{
    let mut data: u32;
    unsafe { asm!("in eax, dx", out("eax") data, in("edx") port); }
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
                asm!("iretd");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}