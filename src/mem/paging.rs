use core::ptr::{write_volatile, read_volatile};

use crate::{arch::asm, println, print};

use super::phys_mem::PhysicalMemoryManager;

const PDE_PAGE_TABLE_ADDR_MASK: u32 = 0xfffff000;
const PDE_PAGE_TABLE_ADDR_MAX: u32 = 0xfffff;
const PDE_PAGE_TABLE_ADDR_SHIFT: u32 = 12;
const PDE_FLAGS_MASK: u32 = 0xff;
const PDE_FLAGS_MAX: u32 = 0xfff;
const PDE_FLAGS_PS: u32 = 0x80;
const PDE_FLAGS_AVL: u32 = 0x40;
const PDE_FLAGS_A: u32 = 0x20;
const PDE_FLAGS_PCD: u32 = 0x10;
const PDE_FLAGS_PWT: u32 = 0x8;
const PDE_FLAGS_U_S: u32 = 0x4;
const PDE_FLAGS_R_W: u32 = 0x2;
const PDE_FLAGS_P: u32 = 0x1;

const PTE_PAGE_FRAME_ADDR_MASK: u32 = 0xfffff000;
const PTE_PAGE_FRAME_ADDR_MAX: u32 = 0xfffff;
const PTE_PAGE_FRAME_ADDR_SHIFT: u32 = 12;
const PTE_FLAGS_MASK: u32 = 0x1ff;
const PTE_FLAGS_MAX: u32 = 0xfff;
const PTE_FLAGS_G: u32 = 0x100;
const PTE_FLAGS_PAT: u32 = 0x80;
const PTE_FLAGS_D: u32 = 0x40;
const PTE_FLAGS_A: u32 = 0x20;
const PTE_FLAGS_PCD: u32 = 0x10;
const PTE_FLAGS_PWT: u32 = 0x8;
const PTE_FLAGS_U_S: u32 = 0x4;
const PTE_FLAGS_R_W: u32 = 0x2;
const PTE_FLAGS_P: u32 = 0x1;

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

    pub fn set_flag(&mut self, flags: u32)
    {
        if flags > PTE_FLAGS_MAX
        {
            panic!("flags is out of range");
        }

        let mut tmp = unsafe { read_volatile(self.entry) };
        tmp |= flags;
        unsafe { write_volatile(self.entry, tmp) };
    }

    pub fn clear_flag(&mut self, flags: u32)
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

    pub fn set_page_frame_addr(&self, mut page_frame_addr: u32)
    {
        if page_frame_addr > PTE_PAGE_FRAME_ADDR_MAX
        {
            panic!("page_frame_addr is out of range");
        }

        page_frame_addr &= PTE_PAGE_FRAME_ADDR_MASK;
        let mut tmp = unsafe { read_volatile(self.entry) };
        tmp |= page_frame_addr;
        unsafe { write_volatile(self.entry, tmp) };
    }

    pub fn get_flags(&self) -> u32
    {
        return unsafe { read_volatile(self.entry) } & PTE_FLAGS_MASK;
    }

    pub fn get_flag_present(&self) -> bool
    {
        return self.get_flags() & PTE_FLAGS_P != 0;
    }

    pub fn get_flag_writable(&self) -> bool
    {
        return self.get_flags() & PTE_FLAGS_R_W != 0;
    }

    pub fn get_size() -> u32
    {
        return u32::BITS;
    }
}

#[derive(Debug, PartialEq, Eq)]
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

    pub fn set_flag(&mut self, flags: u32)
    {
        if flags > PDE_FLAGS_MAX
        {
            panic!("flags is out of range");
        }

        let mut tmp = unsafe { read_volatile(self.entry) };
        tmp |= flags;
        unsafe { write_volatile(self.entry, tmp) };
    }

    pub fn clear_flag(&mut self, flags: u32)
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
        return self.get_flags() & PDE_FLAGS_P != 0;
    }

    pub fn get_flag_writable(&self) -> bool
    {
        return self.get_flags() & PDE_FLAGS_R_W != 0;
    }

    pub fn get_size() -> u32
    {
        return u32::BITS;
    }
}

pub fn init(phys_mem_manager: &mut PhysicalMemoryManager)
{
    let pd_block = phys_mem_manager.alloc_single_mem_block(); // allocate block for page directory memory
    let pt_block = phys_mem_manager.alloc_single_mem_block(); // allocate block for page table memory

    for i in 0..1024
    {
        // init page directory
        let phys = unsafe { &mut *((pd_block.mem_block_start_addr + i * 4) as *mut u32) };
        let mut pde = PageDirectoryEntry::new(phys);
        pde.set_page_table_addr(i * 1024); // relative address of page table (page table index)
        pde.set_flag(PDE_FLAGS_R_W | PDE_FLAGS_R_W);

        for j in 0..1024
        {
            // init page table
            let phys = unsafe { &mut *((pt_block.mem_block_start_addr + i * j * 4) as *mut u32) };
            let mut pte = PageTableEntry::new(phys);
            pte.set_page_frame_addr(i * j); // relative address of page block (page block index)
            pte.set_flag(PTE_FLAGS_R_W | PTE_FLAGS_P);
        }
    }

    // enable paging
    asm::set_cr3(pd_block.mem_block_start_addr as i32);
    asm::enable_paging();

    println!("Paging enabled");
}