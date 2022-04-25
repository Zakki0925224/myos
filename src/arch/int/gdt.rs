pub const GDT_ADDR: i32 = 0x00270000;
pub const GDT_LIMIT: i32 = 0x0000ffff;

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
    pub fn new(mut limit: u32, base: i32, mut ar: i32) -> SegmentDescriptor
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