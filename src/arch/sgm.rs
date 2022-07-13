use core::ptr::{read_volatile, write_volatile};

use crate::{handler, util::logger::*};

use super::asm;

const GDT_ADDR: u32 = 0x270000;
const GDT_LIMIT: u32 = 0xffff;
const IDT_ADDR: u32 = 0x26f800;
const IDT_LIMIT: u32 = 0x7ff;
const IDT_INT_SELECTOR: u32 = 0x8;

const INTGATE: u8 = 0x8e;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct SegmentDescriptor
{
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    flags: u16,
    base_high: u8
}

impl SegmentDescriptor
{
    fn new(mut limit: u32, base: u32, mut flags: u16) -> SegmentDescriptor
    {
        if limit > 0xfffff
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
struct GateDescriptor
{
    base_low: u16,
    selector: u16,
    reserved: u8,
    flags: u8,
    base_high: u16
}

impl GateDescriptor
{
    fn new(base: u32, selector: u32, flags: u8) -> GateDescriptor
    {
        return GateDescriptor
        {
            base_low: base as u16,
            selector: selector as u16,
            reserved: 0x0,
            flags,
            base_high: (base >> 16) as u16
        }
    }
}

pub fn init()
{
    use crate::arch::int::*;
    use crate::arch::ex_int::*;
    use core::arch::asm;

    // init GDT
    for i in 0..=(GDT_LIMIT / 8)
    {
        let gdt = SegmentDescriptor::new(0, 0, 0);
        write_gdt(i, gdt);
    }

    // null descriptor
    let gdt = SegmentDescriptor::new(0, 0, 0);
    write_gdt(0, gdt);

    // code descriptor
    let gdt = SegmentDescriptor::new(0xffff, 0, 0xcf9a);
    write_gdt(1, gdt);

    // data descriptor
    let gdt = SegmentDescriptor::new(0xffff, 0, 0xcf92);
    write_gdt(2, gdt);

    // temp descriptor
    // task code descriptor
    // task data descriptor
    // ktss descriptor

    asm::load_gdtr(GDT_LIMIT as i32, GDT_ADDR as i32);
    log_info("GDT initialized");

    // init IDT
    for i in 0..=(IDT_LIMIT / 8)
    {
        let idt = GateDescriptor::new(0, 0, 0);
        write_idt(i, idt);
    }

    // set exceptions
    // divided by zero
    let idt = GateDescriptor::new(handler!(ex_divided_by_zero) as u32, IDT_INT_SELECTOR, INTGATE);
    write_idt(EX_INT_DIVIDED_BY_ZERO, idt);

    // double fault
    let idt = GateDescriptor::new(handler!(ex_double_fault) as u32, IDT_INT_SELECTOR, INTGATE);
    write_idt(EX_INT_DOUBLE_FAULT, idt);

    // general protection fault
    let idt = GateDescriptor::new(handler!(ex_general_protection_fault) as u32, IDT_INT_SELECTOR, INTGATE);
    write_idt(EX_INT_GENERAL_PROTECTION_FAULT, idt);

    // breakpoint
    let idt = GateDescriptor::new(handler!(ex_breakpoint) as u32, IDT_INT_SELECTOR, INTGATE);
    write_idt(EX_INT_BREAKPOINT, idt);

    // set interrupts
    // PS/2 keyboard
    let idt = GateDescriptor::new(handler!(keyboard_int) as u32, IDT_INT_SELECTOR, INTGATE);
    write_idt(INT_VECTOR_IRQ1, idt);

    // PS/2 mouse
    let idt = GateDescriptor::new(handler!(mouse_int) as u32, IDT_INT_SELECTOR, INTGATE);
    write_idt(INT_VECTOR_IRQ12, idt);

    asm::load_idtr(IDT_LIMIT as i32, IDT_ADDR as i32);
    log_info("IDT initialized");
}

pub fn enable_page_fault_handler()
{
    use crate::arch::ex_int::*;
    use core::arch::asm;

    let idt = GateDescriptor::new(handler!(ex_page_fault) as u32, IDT_INT_SELECTOR, INTGATE);
    write_idt(EX_INT_PAGE_FAULT, idt);
}

fn read_gdt(index: u32) -> Option<SegmentDescriptor>
{
    if index > (GDT_LIMIT / 8)
    {
        return None;
    }

    unsafe
    {
        let ptr = (GDT_ADDR + index * 8) as *const SegmentDescriptor;
        return Some(read_volatile(ptr));
    }
}

fn write_gdt(index: u32, gdt: SegmentDescriptor)
{
    if index > (GDT_LIMIT / 8)
    {
        return;
    }

    unsafe
    {
        let ptr = (GDT_ADDR + index * 8) as *mut SegmentDescriptor;
        write_volatile(ptr, gdt);
    }
}

fn read_idt(index: u32) -> Option<GateDescriptor>
{
    if index > (IDT_LIMIT / 8)
    {
        return None;
    }

    unsafe
    {
        let ptr = (IDT_ADDR + index * 8) as *const GateDescriptor;
        return Some(read_volatile(ptr));
    }
}

fn write_idt(index: u32, idt: GateDescriptor)
{
    if index > (IDT_LIMIT / 8)
    {
        return;
    }

    unsafe
    {
        let ptr = (IDT_ADDR + index * 8) as *mut GateDescriptor;
        write_volatile(ptr, idt);
    }
}