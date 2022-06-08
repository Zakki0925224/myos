use crate::{device::usb::{Usb, UsbMode}, util::logger::log_info};

use self::pci::Pci;

pub mod keyboard;
pub mod pci;
pub mod usb;

pub fn init()
{
    // pci
    let pci = Pci::new();
    log_info("PCI initialized");

    // usb3.0
    let usb = Usb::new(&pci, UsbMode::Xhci);
    log_info("USB driver initialized");
}