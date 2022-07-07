use core::ptr::{write_volatile, read_volatile};

use multiboot2::BootInformation;

use crate::{arch::asm, println};

use super::{phys_mem::{PhysicalMemoryManager, MemoryBlockInfo, MEM_BLOCK_SIZE}, virt_mem::VirtualAddress, PHYS_MEM_MANAGER};

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
    base_addr: u32
}

impl PageTableEntry
{
    pub fn new(base_addr: u32) -> PageTableEntry
    {
        return PageTableEntry { base_addr };
    }

    pub fn set(&mut self, mut page_frame_addr: u32, flags: u32)
    {
        if flags > PTE_FLAGS_MAX
        {
            panic!("flags is out of range");
        }

        page_frame_addr &= PTE_PAGE_FRAME_ADDR_MASK;
        self.set_inner(page_frame_addr | flags);
    }

    pub fn set_flag(&mut self, flags: u32)
    {
        if flags > PTE_FLAGS_MAX
        {
            panic!("flags is out of range");
        }

        let mut tmp = self.get_inner();
        tmp |= flags;
        self.set_inner(tmp);
    }

    pub fn clear_flag(&mut self, flags: u32)
    {
        if flags > PTE_FLAGS_MAX
        {
            panic!("flags is out of range");
        }

        let mut tmp = self.get_inner();
        tmp &= !flags;
        self.set_inner(tmp);
    }

    pub fn get_page_frame_addr(&self) -> u32
    {
        return self.get_inner() & PTE_PAGE_FRAME_ADDR_MASK;
    }

    pub fn set_page_frame_addr(&self, mut page_frame_addr: u32)
    {
        page_frame_addr &= PTE_PAGE_FRAME_ADDR_MASK;
        let mut tmp = self.get_inner();
        tmp |= page_frame_addr;
        self.set_inner(tmp);
    }

    pub fn get_flags(&self) -> u32
    {
        return self.get_inner() & PTE_FLAGS_MASK;
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

    fn get_inner(&self) -> u32
    {
        unsafe
        {
            let ptr = self.base_addr as *const u32;
            return read_volatile(ptr);
        }
    }

    fn set_inner(&self, inner: u32)
    {
        unsafe
        {
            let ptr = self.base_addr as *mut u32;
            write_volatile(ptr, inner);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PageDirectoryEntry
{
    base_addr: u32
}

impl PageDirectoryEntry
{
    pub fn new(base_addr: u32) -> PageDirectoryEntry
    {
        return PageDirectoryEntry { base_addr };
    }

    pub fn set(&mut self, mut page_table_addr: u32, flags: u32)
    {
        if flags > PDE_FLAGS_MAX
        {
            panic!("flags is out of range");
        }

        page_table_addr &= PDE_PAGE_TABLE_ADDR_MASK;
        self.set_inner(page_table_addr | flags);
    }

    pub fn set_flag(&mut self, flags: u32)
    {
        if flags > PDE_FLAGS_MAX
        {
            panic!("flags is out of range");
        }

        let mut tmp = self.get_inner();
        tmp |= flags;
        self.set_inner(tmp);
    }

    pub fn clear_flag(&mut self, flags: u32)
    {
        if flags > PDE_FLAGS_MAX
        {
            panic!("flags is out of range");
        }

        let mut tmp = self.get_inner();
        tmp &= !flags;
        self.set_inner(tmp);
    }

    pub fn get_page_table_addr(&self) -> u32
    {
        return self.get_inner() & PDE_PAGE_TABLE_ADDR_MASK;
    }

    pub fn set_page_table_addr(&self, mut page_table_addr: u32)
    {
        page_table_addr &= PDE_PAGE_TABLE_ADDR_MASK;
        let mut tmp = self.get_inner();
        tmp |= page_table_addr;
        self.set_inner(tmp);
    }

    pub fn get_flags(&self) -> u32
    {
        return self.get_inner() & PDE_FLAGS_MASK;
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

    fn get_inner(&self) -> u32
    {
        unsafe
        {
            let ptr = self.base_addr as *const u32;
            return read_volatile(ptr);
        }
    }

    fn set_inner(&self, inner: u32)
    {
        unsafe
        {
            let ptr = self.base_addr as *mut u32;
            write_volatile(ptr, inner);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Paging
{
    pd_block: MemoryBlockInfo,
    pt_blocks: [MemoryBlockInfo; 1024],
    page_directory_addr_backup: u32,
    is_init: bool,
    is_enabled: bool
}

impl Paging
{
    pub fn new() -> Paging
    {
        return Paging
        {
            pd_block: MemoryBlockInfo::new(),
            pt_blocks: [MemoryBlockInfo::new(); 1024],
            page_directory_addr_backup: 0,
            is_init: false,
            is_enabled: false
        };
    }

    pub fn init(&mut self)
    {
        if let Some(mb_info) = PHYS_MEM_MANAGER.lock().alloc_single_mem_block()
        {
            self.pd_block = mb_info;
        }
        else
        {
            return;
        }

        PHYS_MEM_MANAGER.lock().clear_mem_block(self.pd_block);

        // allocate block for page table memory
        for i in 0..self.pt_blocks.len()
        {
            let mb_info = PHYS_MEM_MANAGER.lock().alloc_single_mem_block();
            self.pt_blocks[i] = mb_info.unwrap();
            PHYS_MEM_MANAGER.lock().clear_mem_block(self.pt_blocks[i]);
        }

        // back up cr3 address
        self.page_directory_addr_backup = asm::get_cr3();

        // mapping physical and virtual address
        let mut i = 0;
        loop
        {
            let va = VirtualAddress::new(i);

            let pd_i = va.get_page_directory_index();
            let pt_i = va.get_page_table_index();
            let mut pte = self.get_page_table_entry(pd_i, pt_i);
            pte.set(i, PTE_FLAGS_P | PTE_FLAGS_R_W);

            if pt_i == 0
            {
                let mut pde = self.get_page_directory_entry(pd_i);
                let pt_block = self.pt_blocks.get(pd_i).unwrap();
                pde.set(pt_block.mem_block_start_addr, PDE_FLAGS_P | PDE_FLAGS_R_W);
            }

            if u32::MAX - i < MEM_BLOCK_SIZE || i + MEM_BLOCK_SIZE > PHYS_MEM_MANAGER.lock().get_total_mem_size()
            {
                break;
            }
            else
            {
                i += MEM_BLOCK_SIZE;
            }
        }

        self.is_init = true;
    }

    pub fn enable(&mut self)
    {
        if !self.is_init()
        {
            return;
        }

        asm::set_cr3(self.pd_block.mem_block_start_addr);
        asm::enable_paging();
        self.is_enabled = true;
    }

    pub fn is_enabled(&self) -> bool
    {
        return self.is_enabled;
    }

    pub fn is_init(&self) -> bool
    {
        return self.is_init;
    }

    pub fn alloc_single_page(&mut self) -> Option<MemoryBlockInfo>
    {
        if !self.is_enabled()
        {
            return None;
        }

        if let Some(mb_info) = PHYS_MEM_MANAGER.lock().alloc_single_mem_block()
        {
            PHYS_MEM_MANAGER.lock().clear_mem_block(mb_info);
            let pd_i = self.get_page_directory_index(mb_info.mem_block_index);
            let pt_i = self.get_page_table_index(mb_info.mem_block_index);
            let mut pte = self.get_page_table_entry(pd_i, pt_i);
            pte.set(mb_info.mem_block_start_addr, PTE_FLAGS_P);

            return Some(mb_info);
        }
        else
        {
            return None;
        }
    }

    pub fn dealloc_single_page(&mut self, mem_block: MemoryBlockInfo)
    {
        if !self.is_enabled()
        {
            return;
        }

        PHYS_MEM_MANAGER.lock().dealloc_single_mem_block(mem_block);
        PHYS_MEM_MANAGER.lock().clear_mem_block(mem_block);
        let pd_i = self.get_page_directory_index(mem_block.mem_block_index);
        let pt_i = self.get_page_table_index(mem_block.mem_block_index);
        let mut pte = self.get_page_table_entry(pd_i, pt_i);
        pte.clear_flag(PTE_FLAGS_P);
    }

    pub fn get_total_mem_size(&self) -> u32
    {
        return PHYS_MEM_MANAGER.lock().get_total_mem_size();
    }

    pub fn get_used_mem_size(&self) -> u32
    {
        return PHYS_MEM_MANAGER.lock().get_allocated_blocks() * MEM_BLOCK_SIZE;
    }

    pub fn get_free_mem_size(&self) -> u32
    {
        return self.get_total_mem_size() - self.get_used_mem_size();
    }

    fn get_page_directory_entry(&self, index: usize) -> PageDirectoryEntry
    {
        return PageDirectoryEntry::new(self.pd_block.mem_block_start_addr + index as u32 * 4);
    }

    fn get_page_table_entry(&self, page_directory_index: usize, page_table_index: usize) -> PageTableEntry
    {
        let pde = self.get_page_directory_entry(page_directory_index);
        let pt_addr = pde.get_page_table_addr();
        return PageTableEntry::new(pt_addr + page_table_index as u32 * 4);
    }

    fn get_page_directory_index(&self, mem_block_index: usize) -> usize
    {
        return mem_block_index / 1024 % 1024;
    }

    fn get_page_table_index(&self, mem_block_index: usize) -> usize
    {
        return mem_block_index % 1024;
    }
}