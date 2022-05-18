use multiboot2::BootInformation;

use crate::{println, mem::{phys_mem::PhysicalMemoryManager}};

pub mod phys_mem;
pub mod virt_mem;
pub mod paging;

pub fn init(boot_info: &BootInformation)
{
    let mut physical_mem_manager = PhysicalMemoryManager::new(&boot_info);
    physical_mem_manager.init(&boot_info);

    println!("Memmap start: 0x{:x}, end: 0x{:x}", physical_mem_manager.get_memmap_start_addr(), physical_mem_manager.get_memmap_end_addr());
    println!("First memory block: {:?}", physical_mem_manager.get_mem_block(0));

    paging::init(&mut physical_mem_manager);
}