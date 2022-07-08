use crate::{device::usb::{Usb, UsbMode}, util::logger::*, println};
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

    // TODO: AHCI.read_hba_regs()
    // ahci
    // AHCI.lock().init();

    // if AHCI.lock().is_init()
    // {
    //     AHCI.lock().read(0, 10);
    //     log_info("AHCI controller initialized");
    // }
    // else
    // {
    //     log_warn("Failed to initialize AHCI controller");
    // }
}