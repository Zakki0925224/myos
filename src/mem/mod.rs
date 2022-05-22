use multiboot2::BootInformation;

use crate::{println, mem::paging::Paging};

use self::virt_mem::VirtualAddress;

pub mod phys_mem;
pub mod virt_mem;
pub mod paging;

pub fn init(boot_info: &BootInformation)
{
    let mut paging = Paging::new(boot_info);
    paging.init();
    paging.enable();
}