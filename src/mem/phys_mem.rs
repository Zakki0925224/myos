use core::{ptr::{write_volatile, read_volatile}};

use multiboot2::BootInformation;

use super::{get_total_available_mem_size, get_multiboot_addr, get_all_available_mem_areas};

const BLOCK_SIZE: u32 = 4096;

#[derive(Debug, PartialEq, Eq)]
pub struct MemoryBlockInfo
{
    pub memmap_addr: u32,
    pub mem_block_start_addr: u32,
    pub mem_block_size: u32,
    pub mem_block_index: usize,
    pub is_available: bool
}

impl MemoryBlockInfo
{
    pub fn new() -> MemoryBlockInfo
    {
        return MemoryBlockInfo
        {
            memmap_addr: 0,
            mem_block_start_addr: 0,
            mem_block_size: 0,
            mem_block_index: 0,
            is_available: false
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PhysicalMemoryManager
{
    total_mem_size: u32,
    mem_blocks: u32,
    allocated_blocks: u32,
    free_blocks: u32,
    memmap_addr: u32,
    memmap_size: u32
}

impl PhysicalMemoryManager
{
    pub fn new(boot_info: &BootInformation) -> PhysicalMemoryManager
    {
        let total_mem_size = get_total_available_mem_size(boot_info) as u32;
        let mem_blocks = total_mem_size / BLOCK_SIZE;
        let (_, e) = get_multiboot_addr(boot_info);
        let memmap_addr = (e + 1) as u32;

        return PhysicalMemoryManager
        {
            total_mem_size,
            mem_blocks,
            allocated_blocks: mem_blocks,
            free_blocks: 0,
            memmap_addr,
            memmap_size: mem_blocks / 32 // 32 bits (u32) per block
        }
    }

    pub fn init(&mut self, boot_info: &BootInformation)
    {
        let all_available_mem_areas = get_all_available_mem_areas(boot_info);
        let (_, m_e) = get_multiboot_addr(boot_info);

        // set all blocks to allocated
        for i in 0..=self.mem_blocks
        {
            self.allocate_mem_block(i as usize);
        }

        // set blocks of available memory to free
        for area in all_available_mem_areas
        {
            let mut i = area.start_address() as u32;

            loop
            {
                if i > area.end_address() as u32
                {
                    break;
                }

                // skip to multiboot end addr
                if i < m_e as u32
                {
                    i += BLOCK_SIZE;
                    continue;
                }

                let block_addr = self.phys_mem_addr_to_mem_block_index(i as u32);
                self.unallocate_mem_block(block_addr);
                i += BLOCK_SIZE;
                self.free_blocks += 1;
                self.allocated_blocks -= 1;
            }
        }

    }

    pub fn get_mem_block(&mut self, index: usize) -> MemoryBlockInfo
    {
        if index > self.mem_blocks as usize
        {
            panic!("Memory block index out of range");
        }

        let memmap_addr = self.memmap_addr + index as u32;
        let mem_block_start_addr = index as u32 * BLOCK_SIZE;
        let mem_block_size = BLOCK_SIZE;
        let is_available = !self.is_allocated_mem_block(index);

        return MemoryBlockInfo
        {
            memmap_addr,
            mem_block_start_addr,
            mem_block_size,
            mem_block_index: index,
            is_available
        }
    }

    pub fn get_first_free_mem_block(&mut self) -> MemoryBlockInfo
    {
        let mut mem_block = MemoryBlockInfo::new();
        let mut i = 0;

        loop
        {
            if i > self.mem_blocks as usize
            {
                break;
            }

            let ptr = self.memmap_ptr(i);

            if unsafe { read_volatile(ptr) } == u32::MAX
            {
                i += 32;
                continue;
            }

            if !self.is_allocated_mem_block(i)
            {
                mem_block = self.get_mem_block(i);
                break;
            }

            i += 32;
        }

        return mem_block;
    }

    pub fn alloc_single_mem_block(&mut self) -> MemoryBlockInfo
    {
        if self.free_blocks <= 0
        {
            panic!("No free memory blocks");
        }

        let free_mb = self.get_first_free_mem_block();

        self.allocate_mem_block(free_mb.mem_block_index);
        self.free_blocks -= 1;
        self.allocated_blocks += 1;

        return self.get_mem_block(free_mb.mem_block_index);
    }

    pub fn unalloc_single_mem_block(&mut self, mem_block: MemoryBlockInfo)
    {
        if !mem_block.is_available
        {
            self.unallocate_mem_block(mem_block.mem_block_index);
            self.free_blocks += 1;
            self.allocated_blocks -= 1;
        }
    }

    pub fn get_total_mem_size(&self) -> u32
    {
        return self.total_mem_size;
    }

    pub fn get_mem_blocks(&self) -> u32
    {
        return self.mem_blocks;
    }

    pub fn get_allocated_blocks(&self) -> u32
    {
        return self.allocated_blocks;
    }

    pub fn get_free_blocks(&self) -> u32
    {
        return self.free_blocks;
    }

    fn allocate_mem_block(&mut self, mem_block_index: usize)
    {
        let buffer = self.memmap_ptr(mem_block_index / 32);
        let mut tmp = unsafe { read_volatile(buffer) };
        tmp |= 1 << (mem_block_index % 32);
        unsafe { write_volatile(buffer, tmp); }
    }

    fn unallocate_mem_block(&mut self, mem_block_index: usize)
    {
        let buffer = self.memmap_ptr(mem_block_index / 32);
        let mut tmp = unsafe { read_volatile(buffer) };
        tmp &= !(1 << (mem_block_index % 32));
        unsafe { write_volatile(buffer, tmp); }
    }

    fn is_allocated_mem_block(&mut self, mem_block_index: usize) -> bool
    {
        let buffer = self.memmap_ptr(mem_block_index / 32);
        let tmp = unsafe { read_volatile(buffer) };
        return tmp & (1 << (mem_block_index % 32)) > 0;
    }

    fn memmap_ptr(&mut self, offset: usize) -> &mut u32
    {
        if offset > self.memmap_size as usize
        {
            panic!("Memory map offset out of range");
        }

        return unsafe { &mut *((self.memmap_addr as *mut u32)).offset(offset as isize) };
    }

    fn phys_mem_addr_to_mem_block_index(&mut self, phys_mem_addr: u32) -> usize
    {
        return (phys_mem_addr / BLOCK_SIZE) as usize;
    }
}