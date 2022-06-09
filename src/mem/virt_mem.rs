use crate::{arch::asm, println};

use super::paging::Paging;

const VA_PD_INDEX_MASK: u32 = 0xffc00000;
const VA_PD_INDEX_MAX: u32 = 0x3ff;
const VA_PD_INDEX_SHIFT: u32 = 22;
const VA_PT_INDEX_MASK: u32 = 0x3ff000;
const VA_PT_INDEX_MAX: u32 = 0x3ff;
const VA_PT_INDEX_SHIFT: u32 = 12;
const VA_PAGE_OFFSET_MASK: u32 = 0xfff;
const VA_PAGE_OFFSET_MAX: u32 = 0xfff;
const VA_PAGE_OFFSET_SHIFT: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub struct VirtualAddress
{
    inner: u32
}

impl VirtualAddress
{
    pub fn new(inner: u32) -> VirtualAddress
    {
        return VirtualAddress { inner };
    }

    pub fn set(mut page_directory_index: u32,
               mut page_table_index: u32,
               mut page_offset: u32) -> VirtualAddress
    {
        if page_directory_index > VA_PD_INDEX_MAX
        {
            panic!("page_directory_entry_addr is out of range");
        }

        if page_table_index > VA_PT_INDEX_MAX
        {
            panic!("page_table_entry_addr is out of range");
        }

        if page_offset > VA_PAGE_OFFSET_MAX
        {
            panic!("page_addr is out of range");
        }

        page_directory_index <<= VA_PD_INDEX_SHIFT;
        page_table_index <<= VA_PT_INDEX_SHIFT;
        page_offset <<= VA_PAGE_OFFSET_SHIFT;

        let inner = page_directory_index | page_table_index | page_offset;

        return VirtualAddress { inner };
    }

    pub fn flash_tlb(&self)
    {
        asm::invlpg(self.inner);
    }

    pub fn get_page_directory_index(&self) -> usize
    {
        return ((self.inner & VA_PD_INDEX_MASK) >> VA_PD_INDEX_SHIFT) as usize;
    }

    pub fn get_page_table_index(&self) -> usize
    {
        return ((self.inner & VA_PT_INDEX_MASK) >> VA_PT_INDEX_SHIFT) as usize;
    }

    pub fn get_page_offset(&self) -> usize
    {
        return ((self.inner & VA_PAGE_OFFSET_MASK) >> VA_PAGE_OFFSET_SHIFT) as usize;
    }

    pub fn get_addr(&self) -> u32
    {
        return self.inner;
    }
}