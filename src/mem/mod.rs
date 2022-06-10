use crate::{println, util::logger::{log_info, log_warn}, mem::paging::Paging};
use multiboot2::BootInformation;
use lazy_static::lazy_static;
use spin::Mutex;

use self::virt_mem::VirtualAddress;

pub mod phys_mem;
pub mod virt_mem;
pub mod paging;

lazy_static!
{
    pub static ref PAGING: Mutex<Paging> = Mutex::new(Paging::new());
}

pub fn init(boot_info: &BootInformation)
{
    PAGING.lock().init(boot_info);
    PAGING.lock().enable();

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
    if !PAGING.lock().is_enabled()
    {
        println!("Paging isn't enabled");
        return;
    }

    println!("Total: {}B", PAGING.lock().get_total_mem_size());
    println!("Used: {}B", PAGING.lock().get_used_mem_size());
    println!("Free: {}B", PAGING.lock().get_free_mem_size());
}