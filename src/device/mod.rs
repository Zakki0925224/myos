use crate::{device::usb::{Usb, UsbMode}, util::logger::{log_info, log_debug, log_warn}};

use self::pci::{Pci, PciHeaderType};

pub mod keyboard;
pub mod pci;
pub mod usb;

pub fn init()
{
    // pci
    let pci = Pci::new();
    log_info("PCI initialized");

    for device in pci.get_devices()
    {
        if device.is_exist() && device.get_header_type() == PciHeaderType::StandardPci
        {
            log_debug(device.get_device_name(), device.get_standard_base_addr());
        }
    }

    // usb3.0
    let usb = Usb::new(&pci, UsbMode::Xhci);

    if usb.eq(&None)
    {
        log_warn("USB driver wasn't initialized because USB controller not found");
    }
    else
    {
        usb.unwrap().init(&pci);
        log_info("USB driver initialized");
    }
}