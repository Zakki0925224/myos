use core::ptr::{write_volatile, read_volatile};
use multiboot2::{BootInformation, MemoryAreaType};
use crate::{println, util::{boot_info::*, logger::*}, mem};

use super::allocator::{HEAP_AREA_BASE_ADDR, HEAP_SIZE};

pub const MEM_BLOCK_SIZE: u32 = 4096;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MemoryBlockInfo
{
    pub memmap_addr: u32,
    pub mem_block_start_addr: u32,
    pub mem_block_size: u32,
    pub mem_block_index: usize,
    pub is_used: bool
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
            is_used: false
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
    pub fn new() -> PhysicalMemoryManager
    {
        return PhysicalMemoryManager
        {
            total_mem_size: 0,
            mem_blocks: 0,
            allocated_blocks: 0,
            free_blocks: 0,
            memmap_addr: 0,
            memmap_size: 0
        }
    }

    pub fn init(&mut self, boot_info: &BootInformation)
    {
        let total = get_total_mem_size(boot_info);

        if total <= u32::MAX as u64
        {
            self.total_mem_size = total as u32;
        }
        else  // greetar than 4GB memory
        {
            self.total_mem_size = u32::MAX;
        }

        self.mem_blocks = self.total_mem_size / MEM_BLOCK_SIZE;
        self.allocated_blocks = self.mem_blocks;
        self.free_blocks = 0;
        self.memmap_size = self.mem_blocks / u32::BITS * 4; // memmap size (byte)

        for area in get_all_mem_areas(boot_info)
        {
            if area.typ() == MemoryAreaType::Available
            {
                self.memmap_addr = area.start_address() as u32;

                // allocate memory blocks
                for i in self.memmap_addr..self.memmap_addr + self.memmap_size
                {
                    if i % MEM_BLOCK_SIZE == 0
                    {
                        let mb_index = self.get_mem_block_index_from_phys_addr(i);
                        if !self.is_allocated_mem_block(mb_index)
                        {
                            self.allocate_mem_block(mb_index);
                            self.allocated_blocks += 1;
                        }
                    }
                }
            }
        }

        // set all blocks to allocated
        for i in 0..self.mem_blocks
        {
            self.allocate_mem_block(i as usize);
        }

        // set blocks of available memory to free
        let mut mb_index = 0;
        let mut tmp = 0;

        for area in get_all_mem_areas(boot_info)
        {
            let mut i = area.start_address() as u32 + tmp;

            loop
            {
                if self.free_blocks == self.mem_blocks
                {
                    break;
                }

                if area.typ() != MemoryAreaType::Available
                {
                    mb_index = area.size() as u32 / MEM_BLOCK_SIZE;
                    tmp = area.size() as u32 % MEM_BLOCK_SIZE;
                    break;
                }

                if i > (area.end_address() - 1) as u32
                {
                    tmp = i - (area.end_address() - 1) as u32;
                    break;
                }

                self.deallocate_mem_block(mb_index as usize);
                self.allocated_blocks -= 1;
                self.free_blocks += 1;

                mb_index += 1;
                i += MEM_BLOCK_SIZE;
            }
        }

        // set reallocate blocks
        let (_, e) = get_multiboot_addr(boot_info);
        for i in 0..e as u32
        {
            if i % MEM_BLOCK_SIZE == 0
            {
                let mb_index = self.get_mem_block_index_from_phys_addr(i);
                if !self.is_allocated_mem_block(mb_index)
                {
                    self.allocate_mem_block(mb_index);
                    self.allocated_blocks += 1;
                    self.free_blocks -= 1;
                }
            }
        }

        // set allocate heap area blocks
        for i in HEAP_AREA_BASE_ADDR..HEAP_AREA_BASE_ADDR + HEAP_SIZE
        {
            if i % MEM_BLOCK_SIZE == 0
            {
                let mb_index = self.get_mem_block_index_from_phys_addr(i);
                if !self.is_allocated_mem_block(mb_index)
                {
                    self.allocate_mem_block(mb_index);
                    self.allocated_blocks += 1;
                    self.free_blocks -= 1;
                }
            }
        }
    }

    pub fn get_mem_block(&mut self, index: usize) -> Option<MemoryBlockInfo>
    {
        if index > self.mem_blocks as usize
        {
            return None;
        }

        let memmap_addr = self.memmap_addr + index as u32 / 8;
        let mem_block_start_addr = index as u32 * MEM_BLOCK_SIZE;
        let mem_block_size = MEM_BLOCK_SIZE;
        let is_used = self.is_allocated_mem_block(index);

        return Some(MemoryBlockInfo
        {
            memmap_addr,
            mem_block_start_addr,
            mem_block_size,
            mem_block_index: index,
            is_used
        });
    }

    fn get_first_free_mem_block(&mut self) -> MemoryBlockInfo
    {
        let mut mem_block = MemoryBlockInfo::new();
        let mut i = 0;

        loop
        {
            if i > self.mem_blocks as usize
            {
                break;
            }

            if self.read_memmap(i as isize) == u32::MAX
            {
                i += u32::BITS as usize;
                continue;
            }

            if !self.is_allocated_mem_block(i)
            {
                if let Some(mb) = self.get_mem_block(i)
                {
                    mem_block = mb;
                }

                break;
            }

            i += u32::BITS as usize;
        }

        return mem_block;
    }

    pub fn alloc_single_mem_block(&mut self) -> Option<MemoryBlockInfo>
    {
        let mut result = None;

        if self.free_blocks <= 0
        {
            result = None;
        }

        let free_mb = self.get_first_free_mem_block();

        self.allocate_mem_block(free_mb.mem_block_index);
        self.free_blocks -= 1;
        self.allocated_blocks += 1;

        match self.get_mem_block(free_mb.mem_block_index)
        {
            Some(mb_info) => result = Some(mb_info),
            None => result = None
        }

        if result == None
        {
            log_error("Failed to allocate memory block");
        }

        return result;
    }

    pub fn dealloc_single_mem_block(&mut self, mem_block: MemoryBlockInfo)
    {
        if mem_block.is_used
        {
            self.deallocate_mem_block(mem_block.mem_block_index);
            self.free_blocks += 1;
            self.allocated_blocks -= 1;
        }
    }

    // FIXME: this function has no end (but, throw no exception)
    pub fn clear_mem_block(&self, mem_block: &MemoryBlockInfo)
    {
        //println!("Clearing memory block 0x{:x} - 0x{:x}...", mem_block.mem_block_start_addr, mem_block.mem_block_start_addr + mem_block.mem_block_size);

        let mut i = mem_block.mem_block_start_addr;

        while i < mem_block.mem_block_start_addr + mem_block.mem_block_size
        {
            unsafe
            {
                let ptr = i as *mut u32;
                write_volatile(ptr, 0);
            }

            i += 4;
        }
    }

    pub fn memset(&self, base_addr: u32, size: u32, data: u8)
    {
        for i in base_addr..base_addr + size
        {
            unsafe
            {
                let ptr = i as *mut u8;
                write_volatile(ptr, data);
            }
        }
    }

    pub fn get_total_mem_size(&self) -> u32
    {
        return self.total_mem_size;
    }

    pub fn get_free_mem_size(&self) -> u32
    {
        return self.total_mem_size - self.allocated_blocks * MEM_BLOCK_SIZE;
    }

    pub fn get_used_mem_size(&self) -> u32
    {
        return self.allocated_blocks * MEM_BLOCK_SIZE;
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

    pub fn get_mem_block_index_from_phys_addr(&self, phys_addr: u32) -> usize
    {
        return (phys_addr / MEM_BLOCK_SIZE) as usize;
    }

    pub fn get_memmap_start_addr(&self) -> u32
    {
        return self.memmap_addr;
    }

    pub fn get_memmap_end_addr(&self) -> u32
    {
        return self.memmap_addr + self.memmap_size;
    }

    fn allocate_mem_block(&mut self, mem_block_index: usize)
    {
        let offset = (mem_block_index / u32::BITS as usize) as isize;
        let mut map = self.read_memmap(offset);
        map |= 1 << (mem_block_index % u32::BITS as usize);
        self.write_memmap(offset, map);
    }

    fn deallocate_mem_block(&self, mem_block_index: usize)
    {
        let offset = (mem_block_index / u32::BITS as usize) as isize;
        let mut map = self.read_memmap(offset);
        map &= !(1 << (mem_block_index % u32::BITS as usize));
        self.write_memmap(offset, map);
    }

    fn is_allocated_mem_block(&self, mem_block_index: usize) -> bool
    {
        let tmp = self.read_memmap((mem_block_index / u32::BITS as usize) as isize);
        return tmp & (1 << (mem_block_index % u32::BITS as usize)) > 0;
    }

    fn read_memmap(&self, offset: isize) -> u32
    {
        unsafe
        {
            let ptr = self.memmap_addr as *const u32;
            return read_volatile(ptr.offset(offset));
        }
    }

    fn write_memmap(&self, offset: isize, map: u32)
    {
        unsafe
        {
            let ptr = self.memmap_addr as *mut u32;
            write_volatile(ptr.offset(offset), map);
        }
    }
}