use crate::{device::usb::{Usb, UsbMode}, util::logger::{log_info, log_debug, log_warn}};
use self::{pci::{Pci, PciHeaderType}, storage::ahci::Ahci};
use lazy_static::lazy_static;
use spin::Mutex;

pub mod storage;
pub mod keyboard;
pub mod pci;
pub mod usb;

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

    for device in PCI.lock().get_devices()
    {
        if device.is_exist() && device.get_header_type() == PciHeaderType::StandardPci
        {
            log_debug(device.get_device_name(), (device.get_base_class_code(), device.get_sub_class_code()));

            for i in 0..device.get_base_addr_len()
            {
                log_debug("BAR", device.get_base_addr(i));
            }
        }
    }

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
        log_info("AHCI controller initialized");
    }
    else
    {
        log_warn("Failed to initialize AHCI controller");
    }
}