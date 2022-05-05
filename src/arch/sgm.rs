use crate::{handler, println};

use super::asm;

const GDT_ADDR: i32 = 0x00270000;
const GDT_LIMIT: i32 = 0x0000ffff;
const IDT_ADDR: i32 = 0x0026f800;
const IDT_LIMIT: i32 = 0x000007ff;
const IDT_INT_SELECTOR: i32 = 0x00000008;

const INTGATE: i32 = 0x008e;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SegmentDescriptor
{
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    flags: u16,
    base_high: u8
}

impl SegmentDescriptor
{
    fn new(mut limit: u32, base: i32, mut flags: i32) -> SegmentDescriptor
    {
        if limit > 0xffff
        {
            limit /= 0x1000;
            flags |= 0x8000;
        }

        return SegmentDescriptor
        {
            limit_low: limit as u16,
            base_low: base as u16,
            base_middle: (base >> 16) as u8,
            flags: flags as u16,
            base_high: (base >> 24) as u8
        };
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct GateDescriptor
{
    base_low: u16,
    selector: u16,
    reserved: u8,
    flags: u8,
    base_high: u16
}

impl GateDescriptor
{
    fn new(base: u32, selector: i32, flags: i32) -> GateDescriptor
    {
        return GateDescriptor
        {
            base_low: base as u16,
            selector: selector as u16,
            reserved: 0x0,
            flags: flags as u8,
            base_high: (base >> 16) as u16
        }
    }
}

pub fn init()
{
    use crate::int::{keyboard_int, INT_VECTOR_IRQ1};
    use core::arch::asm;

    // init GDT
    for i in 0..=(GDT_LIMIT / 8)
    {
        let gdt = unsafe { &mut *((GDT_ADDR + i * 8) as *mut SegmentDescriptor) };
        *gdt = SegmentDescriptor::new(0, 0, 0);
    }

    // null descriptor
    let gdt = unsafe { &mut *((GDT_ADDR + 0 * 8) as *mut SegmentDescriptor) };
    *gdt = SegmentDescriptor::new(0, 0, 0);

    // code descriptor
    let gdt = unsafe { &mut *((GDT_ADDR + 1 * 8) as *mut SegmentDescriptor) };
    *gdt = SegmentDescriptor::new(0xffff, 0, 0xcf9a);

    // data descriptor
    let gdt = unsafe { &mut *((GDT_ADDR + 2 * 8) as *mut SegmentDescriptor) };
    *gdt = SegmentDescriptor::new(0xffff, 0, 0xcf92);

    // temp descriptor
    // task code descriptor
    // task data descriptor
    // ktss descriptor

    asm::load_gdtr(GDT_LIMIT, GDT_ADDR);
    println!("GDT initialized");

    // init IDT
    for i in 0..=(IDT_LIMIT / 8)
    {
        let idt = unsafe { &mut *((IDT_ADDR + i * 8) as *mut GateDescriptor) };
        *idt = GateDescriptor::new(0, 0, 0);
    }

    // set interrupts
    let idt = unsafe { &mut *((IDT_ADDR + INT_VECTOR_IRQ1 * 8) as *mut GateDescriptor) };
    *idt = GateDescriptor::new(handler!(keyboard_int) as u32, IDT_INT_SELECTOR, INTGATE);

    asm::load_idtr(IDT_LIMIT, IDT_ADDR);
    println!("IDT initialized");
}

pub fn get_gdt(index: i32) -> SegmentDescriptor
{
    if (index < 0) || (index > (GDT_LIMIT / 8))
    {
        panic!("GDT index out of range");
    }

    let gdt = unsafe { &mut *((GDT_ADDR + index * 8) as *mut SegmentDescriptor) };
    return *gdt;
}

pub fn get_idt(index: i32) -> GateDescriptor
{
    if (index < 0) || (index > (IDT_LIMIT / 8))
    {
        panic!("IDT index out of range");
    }

    let idt = unsafe { &mut *((IDT_ADDR + index * 8) as *mut GateDescriptor) };
    return *idt;
}