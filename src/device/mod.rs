use crate::{device::usb::{Usb, UsbMode}, util::logger::{log_info, log_debug, log_warn}};
use self::pci::{Pci, PciHeaderType};
use lazy_static::lazy_static;
use spin::Mutex;

pub mod keyboard;
pub mod pci;
pub mod usb;

lazy_static!
{
    pub static ref PCI: Mutex<Pci> = Mutex::new(Pci::new());
    pub static ref USB: Mutex<Usb> = Mutex::new(Usb::new());
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
            log_debug(device.get_device_name(), device.get_standard_base_addr());
        }
    }

    // usb3.0
    USB.lock().init(UsbMode::Xhci);

    if USB.lock().is_init()
    {
        log_info("USB driver initialized");
    }
    else
    {
        log_warn("Failed to initialize USB driver");
    }
}