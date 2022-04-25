use crate::{println, arch::{int::gdt::{GDT_ADDR, SegmentDescriptor, GDT_LIMIT}, asm::load_idtr}, handler, asm};

use self::idt::{IDT_LIMIT, IDT_ADDR, GateDescriptor};

pub mod idt;
pub mod gdt;
pub mod pic;

pub const BOTPACK_ADDR: i32 = 0x00280000;
pub const BOTPACK_LIMIT: u32 = 0x0007ffff;
pub const AR_INTGATE32: i32 = 0x008e;
pub const AR_DATA32_RW: i32 = 0x4092;
pub const AR_CODE32_ER: i32 = 0x409a;

pub fn init()
{
    use pic::{inthandler21, inthandler2c};
    use core::arch::asm;

    // init GDT
    for i in 0..=(IDT_LIMIT / 8)
    {
        let gdt = unsafe { &mut *((GDT_ADDR + i * 8) as *mut SegmentDescriptor)};
        *gdt = SegmentDescriptor::new(0, 0, 0);
    }

    let gdt = unsafe { &mut *((GDT_ADDR + 1 * 8) as *mut SegmentDescriptor)};
    *gdt = SegmentDescriptor::new(0xffffffff, 0x00000000, AR_DATA32_RW);

    let gdt = unsafe { &mut *((GDT_ADDR + 2 * 8) as *mut SegmentDescriptor)};
    *gdt = SegmentDescriptor::new(BOTPACK_LIMIT, BOTPACK_ADDR, AR_CODE32_ER);

    asm::load_gdtr(GDT_LIMIT, GDT_ADDR);

    // init IDT
    for i in 0..=(IDT_LIMIT / 8)
    {
        let idt = unsafe { &mut *((IDT_ADDR + i * 8) as *mut GateDescriptor)};
        *idt = GateDescriptor::new(0, 0, 0);
    }

    // set interrupt handler
    let idt = unsafe { &mut *((IDT_ADDR + 0x21 * 8) as *mut GateDescriptor)};
    *idt = GateDescriptor::new(handler!(inthandler21) as u32, 2 * 8, AR_INTGATE32);
    println!("{:?}", idt);

    let idt = unsafe { &mut *((IDT_ADDR + 0x2c * 8) as *mut GateDescriptor)};
    *idt = GateDescriptor::new(handler!(inthandler2c) as u32, 2 * 8, AR_INTGATE32);

    load_idtr(IDT_LIMIT, IDT_ADDR);

}