use core::{ptr::{write_volatile, read_volatile}};

use multiboot2::{BootInformation, MemoryAreaType};

use crate::println;

use super::{get_total_mem_size, get_multiboot_addr, get_all_mem_areas};

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
        let total_mem_size = get_total_mem_size(boot_info) as u32;
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
            memmap_size: mem_blocks / u32::BITS // 32 bits (u32) per block
        }
    }

    pub fn init(&mut self, boot_info: &BootInformation)
    {
        let all_mem_areas = get_all_mem_areas(boot_info);
        let (_, e) = get_multiboot_addr(boot_info);

        // set all blocks to allocated
        for i in 0..self.mem_blocks
        {
            self.allocate_mem_block(i as usize);
        }

        // set blocks of available memory to free
        let mut mb_index = 0;
        let mut tmp = 0;

        for area in all_mem_areas
        {
            let mut i = area.start_address() as u32 + tmp;

            loop
            {
                if area.typ() != MemoryAreaType::Available
                {
                    mb_index = area.size() as u32 / BLOCK_SIZE;
                    tmp = area.size() as u32 % BLOCK_SIZE;
                    break;
                }

                if i > area.end_address() as u32
                {
                    tmp = i - area.end_address() as u32;
                    break;
                }

                self.unallocate_mem_block(mb_index as usize);
                self.allocated_blocks -= 1;
                self.free_blocks += 1;

                mb_index += 1;
                i += BLOCK_SIZE;
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
                i += u32::BITS as usize;
                continue;
            }

            if !self.is_allocated_mem_block(i)
            {
                mem_block = self.get_mem_block(i);
                break;
            }

            i += u32::BITS as usize;
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
        let buffer = self.memmap_ptr(mem_block_index / u32::BITS as usize);
        let mut tmp = unsafe { read_volatile(buffer) };
        tmp |= 1 << (mem_block_index % u32::BITS as usize);
        unsafe { write_volatile(buffer, tmp); }
    }

    fn unallocate_mem_block(&mut self, mem_block_index: usize)
    {
        let buffer = self.memmap_ptr(mem_block_index / u32::BITS as usize);
        let mut tmp = unsafe { read_volatile(buffer) };
        tmp &= !(1 << (mem_block_index % u32::BITS as usize));
        unsafe { write_volatile(buffer, tmp); }
    }

    fn is_allocated_mem_block(&mut self, mem_block_index: usize) -> bool
    {
        let buffer = self.memmap_ptr(mem_block_index / u32::BITS as usize as usize);
        let tmp = unsafe { read_volatile(buffer) };
        return tmp & (1 << (mem_block_index % u32::BITS as usize)) > 0;
    }

    fn memmap_ptr(&mut self, offset: usize) -> &mut u32
    {
        if offset > self.memmap_size as usize
        {
            panic!("Memory map offset out of range");
        }

        return unsafe { &mut *((self.memmap_addr as *mut u32)).offset(offset as isize) };
    }
}