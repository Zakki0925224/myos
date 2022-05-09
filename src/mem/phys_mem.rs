use multiboot2::BootInformation;

use super::{get_total_available_mem_size, get_multiboot_addr};

const BLOCK_SIZE: u32 = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryMap
{
    pub start_addr: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalMemoryManager
{
    system_mem_size: u32,
    system_mem_blocks: u32,
    allocated_blocks: u32,
    free_blocks: u32,
    memory_map: *const u32,
    memory_map_size: u32
}

impl PhysicalMemoryManager
{
    pub fn new(boot_info: &BootInformation) -> PhysicalMemoryManager
    {
        let total_mem_size = get_total_available_mem_size(boot_info) as u32;
        let blocks = total_mem_size / BLOCK_SIZE;
        let (_, e) = get_multiboot_addr(boot_info);

        return
        PhysicalMemoryManager
        {
            system_mem_size: total_mem_size,
            system_mem_blocks: blocks,
            allocated_blocks: blocks,
            free_blocks: 0,
            memory_map: ((e + 1) as u32) as *const u32,
            memory_map_size: total_mem_size - (e + 1) as u32
        }
    }
}