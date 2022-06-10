use crate::util::logger::{log_debug, log_info, log_warn};

use super::{pci::{PciDevice, Pci, PCI_VENDOR_ID_INTEL}, PCI};

const PCI_USB_CONTROLLER_BASE_CLASS_CODE: u8 = 0x0c;
const PCI_USB_CONTROLLER_SUB_CLASS_CODE: u8 = 0x03;
const PCI_OHCI_USB_CONTROLLER_PRGIF: u8 = 0x10;     // OHCI USB1.1
const PCI_UHCI_USB_CONTROLLER_PRGIF: u8 = 0x0;      // UHCI USB1.1
const PCI_EHCI_USB_CONTROLLER_PRGIF: u8 = 0x20;     // EHCI USB2.0
const PCI_XHCI_USB_CONTROLLER_PRGIF: u8 = 0x30;     // xHCI USB3.0

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum UsbMode
{
    Ohci,
    Uhci,
    Ehci,
    Xhci
}

#[derive(PartialEq)]
pub struct Usb
{
    is_init: bool,
    mode: UsbMode,
    pci_usb_device: PciDevice
}

impl Usb
{
    pub fn new() -> Usb
    {
        let pci_usb_device = PciDevice::new();
        let mode = UsbMode::Ohci; // default

        return Usb { is_init: false, mode, pci_usb_device };
    }

    pub fn init(&mut self, mode: UsbMode)
    {
        self.mode = mode;

        for device in PCI.lock().get_devices()
        {
            if device.get_base_class_code() == PCI_USB_CONTROLLER_BASE_CLASS_CODE &&
               device.get_sub_class_code() == PCI_USB_CONTROLLER_SUB_CLASS_CODE
            {
                let mut prog_if = 0;

                match self.mode
                {
                    UsbMode::Ohci => prog_if = PCI_OHCI_USB_CONTROLLER_PRGIF,
                    UsbMode::Uhci => prog_if = PCI_UHCI_USB_CONTROLLER_PRGIF,
                    UsbMode::Ehci => prog_if = PCI_EHCI_USB_CONTROLLER_PRGIF,
                    UsbMode::Xhci => prog_if = PCI_XHCI_USB_CONTROLLER_PRGIF,
                }

                if device.get_program_interface_class_code() == prog_if
                {
                    self.pci_usb_device = device;
                }

                if device.get_vendor_id() == PCI_VENDOR_ID_INTEL
                {
                    break;
                }
            }
        }

        if !self.pci_usb_device.is_exist()
        {
            log_warn("USB controller not found");
            return;
        }

        if self.mode == UsbMode::Xhci
        {
            self.switch_ehci_to_xhci_mode();
        }

        self.is_init = true;
    }

    pub fn is_init(&self) -> bool
    {
        return self.is_init;
    }

    fn switch_ehci_to_xhci_mode(&self)
    {
        let mut is_exist_intel_ehc = false;

        for device in PCI.lock().get_devices()
        {
            if device.get_vendor_id() == PCI_VENDOR_ID_INTEL &&
               device.get_base_class_code() == 0x0c &&
               device.get_sub_class_code() == 0x03 &&
               device.get_program_interface_class_code() == PCI_EHCI_USB_CONTROLLER_PRGIF
            {
                is_exist_intel_ehc = true;
                break;
            }
        }

        if !is_exist_intel_ehc
        {
            return;
        }

        let superspeed_ports = self.pci_usb_device.read_config(0xdc);
        self.pci_usb_device.write_config(0xd8, superspeed_ports);
        let ehci_to_xhci_ports = self.pci_usb_device.read_config(0xd4);
        self.pci_usb_device.write_config(0xd0, ehci_to_xhci_ports);

        log_info("Switched EHCI to xHCI");
    }
}