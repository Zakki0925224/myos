use crate::{println, util::logger::*, mem::paging::Paging};
use multiboot2::BootInformation;
use lazy_static::lazy_static;
use spin::Mutex;

use self::virt_mem::VirtualAddress;
use self::phys_mem::PhysicalMemoryManager;

pub mod phys_mem;
pub mod virt_mem;
pub mod paging;

lazy_static!
{
    pub static ref PHYS_MEM_MANAGER: Mutex<PhysicalMemoryManager> = Mutex::new(PhysicalMemoryManager::new());
    pub static ref PAGING: Mutex<Paging> = Mutex::new(Paging::new());
}

pub fn init(boot_info: &BootInformation)
{
    PHYS_MEM_MANAGER.lock().init(boot_info);

    PAGING.lock().init();
    //PAGING.lock().enable(); // disabled until the problem is resolved

    if PAGING.lock().is_enabled()
    {
        log_info("Paging enabled");
    }
    else
    {
        log_warn("Failed to enable paging");
    }
}

pub fn free()
{
    println!("Total: {}B", PHYS_MEM_MANAGER.lock().get_total_mem_size());
    println!("Used: {}B", PHYS_MEM_MANAGER.lock().get_used_mem_size());
    println!("Free: {}B", PHYS_MEM_MANAGER.lock().get_free_mem_size());
}