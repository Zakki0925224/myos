use core::ptr::{write_volatile, read_volatile};

use crate::{arch::asm, println};

use super::phys_mem::{PhysicalMemoryManager, MemoryBlockInfo};

const PDE_PAGE_TABLE_ADDR_MASK: u32 = 0xfffff000;
const PDE_PAGE_TABLE_ADDR_MAX: u32 = 0xfffff;
const PDE_PAGE_TABLE_ADDR_SHIFT: u32 = 12;
const PDE_FLAGS_MASK: u32 = 0xff;
const PDE_FLAGS_MAX: u32 = 0xfff;
const PDE_FLAGS_PS_MASK: u32 = 0x80;
const PDE_FLAGS_AVL_MASK: u32 = 0x40;
const PDE_FLAGS_A_MASK: u32 = 0x20;
const PDE_FLAGS_PCD_MASK: u32 = 0x10;
const PDE_FLAGS_PWT_MASK: u32 = 0x8;
const PDE_FLAGS_U_S_MASK: u32 = 0x4;
const PDE_FLAGS_R_W_MASK: u32 = 0x2;
const PDE_FLAGS_P_MASK: u32 = 0x1;
const PDE_SIZE: usize = 1024;

const PTE_PAGE_FRAME_ADDR_MASK: u32 = 0xfffff000;
const PTE_PAGE_FRAME_ADDR_MAX: u32 = 0xfffff;
const PTE_PAGE_FRAME_ADDR_SHIFT: u32 = 12;
const PTE_FLAGS_MASK: u32 = 0x1ff;
const PTE_FLAGS_MAX: u32 = 0xfff;
const PTE_FLAGS_G_MASK: u32 = 0x100;
const PTE_FLAGS_PAT_MASK: u32 = 0x80;
const PTE_FLAGS_D_MASK: u32 = 0x40;
const PTE_FLAGS_A_MASK: u32 = 0x20;
const PTE_FLAGS_PCD_MASK: u32 = 0x10;
const PTE_FLAGS_PWT_MASK: u32 = 0x8;
const PTE_FLAGS_U_S_MASK: u32 = 0x4;
const PTE_FLAGS_R_W_MASK: u32 = 0x2;
const PTE_FLAGS_P_MASK: u32 = 0x1;
const PTE_SIZE: usize = 1024;

const VA_PDE_INDEX_MASK: u32 = 0xffc00000;
const VA_PDE_INDEX_MAX: u32 = 0x3ff;
const VA_PDE_INDEX_SHIFT: u32 = 22;
const VA_PTE_INDEX_MASK: u32 = 0x3ff000;
const VA_PTE_INDEX_MAX: u32 = 0x3ff;
const VA_PTE_INDEX_SHIFT: u32 = 12;
const VA_PAGE_OFFSET_MASK: u32 = 0xfff;
const VA_PAGE_OFFSET_MAX: u32 = 0xfff;
const VA_PAGE_OFFSET_SHIFT: u32 = 0;

const PDT_PHYS_ADDR: u32 = 0x14f61;

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

    pub fn set(page_directory_entry_index: u32,
               page_table_entry_index: u32,
               page_offset: u32) -> VirtualAddress
    {
        if page_directory_entry_index > VA_PDE_INDEX_MAX
        {
            panic!("page_directory_entry_addr is out of range");
        }

        if page_table_entry_index > VA_PTE_INDEX_MAX
        {
            panic!("page_table_entry_addr is out of range");
        }

        if page_offset > VA_PAGE_OFFSET_MAX
        {
            panic!("page_addr is out of range");
        }

        let inner = page_directory_entry_index << VA_PDE_INDEX_SHIFT |
                        page_table_entry_index << VA_PTE_INDEX_SHIFT |
                        page_offset << VA_PAGE_OFFSET_SHIFT;

        return VirtualAddress { inner };
    }

    pub fn get_page_directory_entry_index(&self) -> u32
    {
        return (self.inner & VA_PDE_INDEX_MASK) >> VA_PDE_INDEX_SHIFT;
    }

    pub fn get_page_table_entry_index(&self) -> u32
    {
        return (self.inner & VA_PTE_INDEX_MASK) >> VA_PTE_INDEX_SHIFT;
    }

    pub fn get_page_offset(&self) -> u32
    {
        return (self.inner & VA_PAGE_OFFSET_MASK) >> VA_PAGE_OFFSET_SHIFT;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct VirtualMemoryManager;

impl VirtualMemoryManager
{
    pub fn init()
    {
        // asm::set_cr3(PDT_PHYS_ADDR as i32);
        // asm::enable_paging();
    }

    // TODO: http://softwaretechnique.web.fc2.com/OS_Development/kernel_development08.html, 仮想メモリ管理の実装

    pub fn alloc_page(page_table_entry: &mut PageTableEntry, phys_mem_manager: &mut PhysicalMemoryManager)
    {
        let mb_info = phys_mem_manager.alloc_single_mem_block();
        page_table_entry.set(mb_info.mem_block_start_addr, PTE_FLAGS_P_MASK);
    }

    pub fn unalloc_page(page_table_entry: &mut PageTableEntry, phys_mem_manager: &mut PhysicalMemoryManager)
    {

    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PageTableEntry
{
    entry: *mut u32
}

impl PageTableEntry
{
    pub fn new(entry: *mut u32) -> PageTableEntry
    {
        return PageTableEntry { entry };
    }

    pub fn set(&mut self, page_frame_addr: u32, flags: u32)
    {
        if page_frame_addr > PTE_PAGE_FRAME_ADDR_MAX
        {
            panic!("page_frame_addr is out of range");
        }

        if flags > PTE_FLAGS_MAX
        {
            panic!("flags is out of range");
        }

        unsafe { write_volatile(self.entry, page_frame_addr << PTE_PAGE_FRAME_ADDR_SHIFT | flags) };
    }

    pub fn set_flags(&mut self, flags: u32)
    {
        if flags > PTE_FLAGS_MAX
        {
            panic!("flags is out of range");
        }

        let mut tmp = unsafe { read_volatile(self.entry) };
        tmp |= flags;
        unsafe { write_volatile(self.entry, tmp) };
    }

    pub fn clear_flags(&mut self, flags: u32)
    {
        if flags > PTE_FLAGS_MAX
        {
            panic!("flags is out of range");
        }

        let mut tmp = unsafe { read_volatile(self.entry) };
        tmp &= !flags;
        unsafe { write_volatile(self.entry, tmp) };
    }

    pub fn get_page_frame_addr(&self) -> u32
    {
        return unsafe { read_volatile(self.entry) } >> PTE_PAGE_FRAME_ADDR_SHIFT;
    }

    pub fn get_flags(&self) -> u32
    {
        return unsafe { read_volatile(self.entry) } & PTE_FLAGS_MASK;
    }

    pub fn get_flag_present(&self) -> bool
    {
        return self.get_flags() & PTE_FLAGS_P_MASK != 0;
    }

    pub fn get_flag_writable(&self) -> bool
    {
        return self.get_flags() & PTE_FLAGS_R_W_MASK != 0;
    }
}

pub struct PageDirectoryEntry
{
    entry: *mut u32
}

impl PageDirectoryEntry
{
    pub fn new(entry: *mut u32) -> PageDirectoryEntry
    {
        return PageDirectoryEntry { entry };
    }

    pub fn set(&mut self, page_table_addr: u32, flags: u32)
    {
        if page_table_addr > PDE_PAGE_TABLE_ADDR_MAX
        {
            panic!("page_frame_addr is out of range");
        }

        if flags > PDE_FLAGS_MAX
        {
            panic!("flags is out of range");
        }

        unsafe { write_volatile(self.entry, page_table_addr << PDE_PAGE_TABLE_ADDR_SHIFT | flags) };
    }

    pub fn set_flags(&mut self, flags: u32)
    {
        if flags > PDE_FLAGS_MAX
        {
            panic!("flags is out of range");
        }

        let mut tmp = unsafe { read_volatile(self.entry) };
        tmp |= flags;
        unsafe { write_volatile(self.entry, tmp) };
    }

    pub fn clear_flags(&mut self, flags: u32)
    {
        if flags > PDE_FLAGS_MAX
        {
            panic!("flags is out of range");
        }

        let mut tmp = unsafe { read_volatile(self.entry) };
        tmp &= !flags;
        unsafe { write_volatile(self.entry, tmp) };
    }

    pub fn get_page_table_addr(&self) -> u32
    {
        return unsafe { read_volatile(self.entry) } >> PDE_PAGE_TABLE_ADDR_SHIFT;
    }

    pub fn set_page_table_addr(&self, mut page_table_addr: u32)
    {
        if page_table_addr > PDE_PAGE_TABLE_ADDR_MAX
        {
            panic!("page_table_addr is out of range");
        }

        page_table_addr &= PDE_PAGE_TABLE_ADDR_MASK;
        let mut tmp = unsafe { read_volatile(self.entry) };
        tmp |= page_table_addr;
        unsafe { write_volatile(self.entry, tmp) };
    }

    pub fn get_flags(&self) -> u32
    {
        return unsafe { read_volatile(self.entry) } & PDE_FLAGS_MASK;
    }

    pub fn get_flag_present(&self) -> bool
    {
        return self.get_flags() & PDE_FLAGS_P_MASK != 0;
    }

    pub fn get_flag_writable(&self) -> bool
    {
        return self.get_flags() & PDE_FLAGS_R_W_MASK != 0;
    }
}