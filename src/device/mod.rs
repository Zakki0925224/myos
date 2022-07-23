use core::ptr::read_volatile;

use crate::{device::usb::{Usb, UsbMode}, util::logger::*, println, mem::PHYS_MEM_MANAGER};
use self::{pci::{Pci, PciHeaderType}, storage::ahci::Ahci};
use lazy_static::lazy_static;
use spin::Mutex;

pub mod storage;
pub mod keyboard;
pub mod pci;
pub mod usb;
pub mod serial;

lazy_static!
{
    pub static ref PCI: Mutex<Pci> = Mutex::new(Pci::new());
    pub static ref USB: Mutex<Usb> = Mutex::new(Usb::new());
    pub static ref AHCI: Mutex<Ahci> = Mutex::new(Ahci::new());
}

pub fn init()
{
    // pci
    PCI.lock().init();
    log_info("PCI initialized");

    // usb3.0
    USB.lock().init(UsbMode::Xhci);

    if USB.lock().is_init()
    {
        log_info("USB controller initialized");
    }
    else
    {
        log_warn("Failed to initialize USB controller");
    }

    // ahci
    AHCI.lock().init();

    if AHCI.lock().is_init()
    {
        // let mb_info = PHYS_MEM_MANAGER.lock().alloc_single_mem_block().unwrap();

        // match AHCI.lock().read(0, 0, 0, mb_info.mem_block_start_addr, 8)
        // {
        //     Ok(_) =>
        //     {
        //         println!("OK!");

        //         for i in 0..512
        //         {
        //             unsafe
        //             {
        //                 let ptr = (mb_info.mem_block_start_addr + i) as *const u8;
        //                 println!("{:x}", read_volatile(ptr));
        //             }
        //         }
        //     }
        //     Err(msg) => log_error(msg)
        // }
        // AHCI.lock().test();

        log_info("AHCI controller initialized");
    }
    else
    {
        log_warn("Failed to initialize AHCI controller");
    }
}