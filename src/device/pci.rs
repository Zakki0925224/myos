use pci_ids::{Vendors, Classes};

use crate::{arch::asm, println};

const PCI_CS32_DEVICE_NOT_EXIST: u32 = 0xffffffff;

pub fn init()
{
    let pci = Pci::new();

    for device in pci.get_devices()
    {
        if device.is_exist()
        {
            println!("==Bus: {}, Device: {}, Function: {}==", device.get_bus_num(), device.get_device_num(), device.get_func_num());
            println!("Device name: {}, Rev: {}", device.get_device_name(), device.get_revision_id());
            println!("Vendor name: {}", device.get_vendor_name());
            println!("Base class: {}, Sub class: {}, Pif: {}", device.get_class_name(), device.get_subclass_name(), device.get_program_interface_class_name());
            println!("Type: {:?}", device.get_header_type());
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Pci
{
    devices: [PciDevice; 256]
}

impl Pci
{
    pub fn new() -> Pci
    {
        let mut devices = [PciDevice::new(); 256];
        let mut device_cnt = 0;

        for i in 0..=255
        {
            for j in 0..32
            {
                for k in 0..8
                {
                    devices[device_cnt].set(i, j, k);
                    if devices[device_cnt].is_exist()
                    {
                        device_cnt += 1;
                    }
                }
            }
        }

        return Pci { devices };
    }

    pub fn get_devices(&self) -> [PciDevice; 256]
    {
        return self.devices;
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PciHeaderType
{
    StandardPci,
    PciToPciBridge,
    CardBusBridge
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PciDevice
{
    config_space: PciConfigSpace,
    bus: u8,
    device: u8,
    func: u8
}

impl PciDevice
{
    pub fn new() -> PciDevice
    {
        let config_space = PciConfigSpace::new();
        return PciDevice
        {
            config_space,
            bus: 0,
            device: 0,
            func: 0
        }
    }

    pub fn set(&mut self, bus: u8, device: u8, func: u8)
    {
        self.config_space.get(bus, device, func);
        self.bus = bus;
        self.device = device;
        self.func = func;
    }

    pub fn get_bus_num(&self) -> u8
    {
        return self.bus;
    }

    pub fn get_device_num(&self) -> u8
    {
        return self.device;
    }

    pub fn get_func_num(&self) -> u8
    {
        return self.func;
    }

    pub fn is_exist(&self) -> bool
    {
        return self.config_space.is_exist();
    }

    pub fn get_device_name(&self) -> &str
    {
        let mut name = "Unknown";

        let vendor = Vendors::iter().find(|v| v.id() == self.get_vendor_id());

        if vendor == None
        {
            return name;
        }

        let device = vendor.unwrap().devices().find(|d| d.id() == self.get_device_id());

        if device == None
        {
            return name;
        }

        return device.unwrap().name();
    }

    pub fn get_vendor_name(&self) -> &str
    {
        let mut name = "Unknown";

        let vendor = Vendors::iter().find(|v| v.id() == self.get_vendor_id());

        if vendor == None
        {
            return name;
        }

        return vendor.unwrap().name();
    }

    pub fn get_device_id(&self) -> u16
    {
        return (self.config_space.raw_data[0] >> 16) as u16;
    }

    pub fn get_vendor_id(&self) -> u16
    {
        return self.config_space.raw_data[0] as u16;
    }

    pub fn get_revision_id(&self) -> u8
    {
        return self.config_space.raw_data[2] as u8;
    }

    pub fn get_program_interface_class_code(&self) -> u8
    {
        return (self.config_space.raw_data[2] >> 8) as u8
    }

    pub fn get_sub_class_code(&self) -> u8
    {
        return (self.config_space.raw_data[2] >> 16) as u8;
    }

    pub fn get_base_class_code(&self) -> u8
    {
        return (self.config_space.raw_data[2] >> 24) as u8;
    }

    pub fn get_class_name(&self) -> &str
    {
        let mut name = "Unknown";

        let class = Classes::iter().find(|c| c.id() == self.get_base_class_code());

        if class == None
        {
            return name;
        }

        return class.unwrap().name();
    }

    pub fn get_subclass_name(&self) -> &str
    {
        let mut name = "Unknown";

        let class = Classes::iter().find(|c| c.id() == self.get_base_class_code());

        if class == None
        {
            return name;
        }

        let subclass = class.unwrap().subclasses().find(|sc| sc.id() == self.get_sub_class_code());

        if subclass == None
        {
            return name;
        }

        return subclass.unwrap().name();
    }

    pub fn get_program_interface_class_name(&self) -> &str
    {
        let mut name = "Unknown";

        let class = Classes::iter().find(|c| c.id() == self.get_base_class_code());

        if class == None
        {
            return name;
        }

        let subclass = class.unwrap().subclasses().find(|sc| sc.id() == self.get_sub_class_code());

        if subclass == None
        {
            return name;
        }

        let prog_if = subclass.unwrap().prog_ifs().find(|pif| pif.id() == self.get_program_interface_class_code());

        if prog_if == None
        {
            return name;
        }

        return prog_if.unwrap().name();
    }

    pub fn get_chache_line_size(&self) -> u8
    {
        return self.config_space.raw_data[3] as u8;
    }

    pub fn get_latency_timer(&self) -> u8
    {
        return (self.config_space.raw_data[3] >> 8) as u8;
    }

    pub fn get_header_type(&self) -> PciHeaderType
    {
        let tp = (self.config_space.raw_data[3] >> 16) as u8;

        match tp
        {
            1 => return PciHeaderType::PciToPciBridge,
            2 => return PciHeaderType::CardBusBridge,
            _ => return PciHeaderType::StandardPci
        }
    }

    pub fn get_bist_register(&self) -> u8
    {
        return (self.config_space.raw_data[3] >> 24) as u8;
    }

    pub fn is_multi_function_device(&self) -> bool
    {
        return ((self.config_space.raw_data[3] >> 16) as u8 & 0x80) != 0;
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct PciConfigSpace
{
    raw_data: [u32; 16]
}

impl PciConfigSpace
{
    pub fn new() -> PciConfigSpace
    {
        return PciConfigSpace
        {
            raw_data: [0; 16]
        };
    }

    pub fn get(&mut self, bus: u8, device: u8, func: u8) -> Option<&PciConfigSpace>
    {
        self.get_all_config_space(bus, device, func);

        if self.is_exist()
        {
            return Some(self);
        }
        else
        {
            return None;
        }
    }

    fn read_pci_config(&self, bus: u8, device: u8, func: u8, offset: u32) -> u32
    {
        // offset is a multiple of 4
        let addr = 0x80000000 | (bus as u32) << 16 | (device as u32) << 11 | (func as u32) << 8 | offset;
        asm::out32(0xcf8, addr);

        return asm::in32(0xcfc);
    }

    fn get_all_config_space(&mut self, bus: u8, device: u8, func: u8)
    {
        let raw_data =
        [
            self.read_pci_config(bus, device, func, 0),
            self.read_pci_config(bus, device, func, 4),
            self.read_pci_config(bus, device, func, 8),
            self.read_pci_config(bus, device, func, 16),
            self.read_pci_config(bus, device, func, 32),
            self.read_pci_config(bus, device, func, 64),
            self.read_pci_config(bus, device, func, 128),
            self.read_pci_config(bus, device, func, 256),
            self.read_pci_config(bus, device, func, 512),
            self.read_pci_config(bus, device, func, 1024),
            self.read_pci_config(bus, device, func, 2048),
            self.read_pci_config(bus, device, func, 4096),
            self.read_pci_config(bus, device, func, 8192),
            self.read_pci_config(bus, device, func, 16384),
            self.read_pci_config(bus, device, func, 32768),
            self.read_pci_config(bus, device, func, 65536)
        ];

        self.raw_data = raw_data;
    }

    fn is_exist(&self) -> bool
    {
        return self.raw_data[0] != 0 && self.raw_data[0] != PCI_CS32_DEVICE_NOT_EXIST
    }
}