use super::asm;

const GDT_ADDR: i32 = 0x00270000;
const GDT_LIMIT: i32 = 0x0000ffff;
const IDT_ADDR: i32 = 0x0026f800;
const IDT_LIMIT: i32 = 0x000007ff;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SegmentDescriptor
{
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access_right: u8,
    limit_high: u8,
    base_high: u8
}

impl SegmentDescriptor
{
    fn new(mut limit: u32, base: i32, mut ar: i32) -> SegmentDescriptor
    {
        if limit > 0xfffff
        {
            ar |= 0x8000;
            limit /= 0x1000;
        }

        return SegmentDescriptor
        {
            limit_low: limit as u16,
            base_low: base as u16,
            base_mid: (base >> 16) as u8,
            access_right: ar as u8,
            limit_high: ((limit >> 16) as u8 & 0x0f) | ((ar >> 8) as u8 & 0xf0),
            base_high: (base >> 24) as u8
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct GateDescriptor
{
    offset_low: u16,
    selector: u16,
    dw_count: u8,
    access_right: u8,
    offset_high: u16
}

impl GateDescriptor
{
    fn new(offset: u32, selector: i32, ar: i32) -> GateDescriptor
    {
        return GateDescriptor
        {
            offset_low: offset as u16,
            selector: selector as u16,
            dw_count: (ar >> 8) as u8,
            access_right: ar as u8,
            offset_high: (offset >> 16) as u16
        }
    }
}

pub fn init()
{
    // init GDT
    for i in 0..=(GDT_LIMIT / 8)
    {
        let gdt = unsafe { &mut *((GDT_ADDR + i * 8) as *mut SegmentDescriptor) };
        *gdt = SegmentDescriptor::new(0, 0, 0);
    }

    asm::load_gdtr(GDT_LIMIT, GDT_ADDR);

    // init IDT
    for i in 0..=(IDT_LIMIT / 8)
    {
        let idt = unsafe { &mut *((IDT_ADDR + i * 8) as *mut GateDescriptor) };
        *idt = GateDescriptor::new(0, 0, 0);
    }

    asm::load_idtr(IDT_LIMIT, IDT_ADDR);
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