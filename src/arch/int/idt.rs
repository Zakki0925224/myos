pub const IDT_ADDR: i32 = 0x0026f800;
pub const IDT_LIMIT: i32 = 0x000007ff;

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
    pub fn new(offset: u32, selector: i32, ar: i32) -> GateDescriptor
    {
        return GateDescriptor
        {
            offset_low: offset as u16,
            selector: selector as u16,
            dw_count: (ar >> 8) as u8,
            access_right: ar as u8,
            offset_high: (offset >> 16) as u16
        };
    }
}