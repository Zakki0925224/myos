use crate::{println, device::usb::{Usb, UsbMode}};

use self::pci::Pci;

pub mod keyboard;
pub mod pci;
pub mod usb;

pub fn init()
{
    // pci
    let pci = Pci::new();
    println!("PCI initialized");

    // usb3.0
    let usb = Usb::new(&pci, UsbMode::Xhci);
}