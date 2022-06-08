use super::pci::{PciDevice, Pci};

const PCI_USB_CONTROLLER_BASE_CLASS_CODE: u8 = 0x0c;
const PCI_USB_CONTROLLER_SUB_CLASS_CODE: u8 = 0x03;
const PCI_OHCI_USB_CONTROLLER_PRGIF: u8 = 0x10;     // OHCI USB1.1
const PCI_UHCI_USB_CONTROLLER_PRGIF: u8 = 0x0;      // UHCI USB1.1
const PCI_EHCI_USB_CONTROLLER_PRGIF: u8 = 0x20;     // EHCI USB2.0
const PCI_XHCI_USB_CONTROLLER_PRGIF: u8 = 0x30;     // xHCI USB3.0

pub enum UsbMode
{
    Ohci,
    Uhci,
    Ehci,
    Xhci
}

pub struct Usb
{
    mode: UsbMode,
    controller: PciDevice
}

impl Usb
{
    pub fn new(pci: &Pci, mode: UsbMode) -> Usb
    {
        let mut controller = PciDevice::new();

        for device in pci.get_devices()
        {
            if device.get_base_class_code() == PCI_USB_CONTROLLER_BASE_CLASS_CODE &&
               device.get_sub_class_code() == PCI_USB_CONTROLLER_SUB_CLASS_CODE
            {
                let mut prog_if = 0;

                match mode
                {
                    UsbMode::Ohci => prog_if = PCI_OHCI_USB_CONTROLLER_PRGIF,
                    UsbMode::Uhci => prog_if = PCI_UHCI_USB_CONTROLLER_PRGIF,
                    UsbMode::Ehci => prog_if = PCI_EHCI_USB_CONTROLLER_PRGIF,
                    UsbMode::Xhci => prog_if = PCI_XHCI_USB_CONTROLLER_PRGIF,
                }

                if device.get_program_interface_class_code() == prog_if
                {
                    controller = device;
                }

                break;
            }
        }

        if !controller.is_exist()
        {
            panic!("USB controller not found");
        }

        return Usb { mode, controller };
    }
}