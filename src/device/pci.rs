use pci_ids::Vendors;

use crate::{arch::asm, println};

const PCI_CS32_DEVICE_NOT_EXIST: u32 = 0xffffffff;

const PCI_CS32_DEVICE_ID_MASK: u32 = 0xffff0000;
const PCI_CS32_VENDOR_ID_MASK: u32 = 0x0000ffff;
const PCI_CS32_STATUS_MASK: u32 = 0xffff0000;
const PCI_CS32_COMMAND_MASK: u32 = 0x0000ffff;
const PCI_CS32_CLASS_CODE_MASK: u32 = 0xffffff0;
const PCI_CS32_REVISION_ID_MASK: u32 = 0x000000f;
const PCI_CS32_BIST_MASK: u32 = 0xff000000;
const PCI_CS32_HEADER_TYPE_MASK: u32 = 0xff0000;
const PCI_CS32_LATENCY_TIMER_MASK: u32 = 0xff00;
const PCI_CS32_CACHE_LINE_SIZE_MASK: u32 = 0x000000ff;
const PCI_CS32_BASE_ADDR_REGISTER_MASK: u32 = 0xffffffff;
const PCI_CS32_CARDBUS_CIS_POINTER_MASK: u32 = 0xffffffff;
const PCI_CS32_SUBSYSTEM_ID_MASK: u32 = 0xffff0000;
const PCI_CS32_SUBSYSTEM_VENDOR_ID_MASK: u32 = 0x0000ffff;
const PCI_CS32_EXPANSION_ROM_BASE_ADDR_MASK: u32 = 0xffffffff;
const PCI_CS32_CAPABILITIES_POINTER_MASK: u32 = 0x000000ff;
const PCI_CS32_MAX_LAT_MASK: u32 = 0xff000000;
const PCI_CS32_MIN_GNT_MASK: u32 = 0x00ff0000;
const PCI_CS32_INTERRUPT_PIN_MASK: u32 = 0x0000ff00;
const PCI_CS32_INTERRUPT_LINE_MASK: u32 = 0x000000ff;


pub fn init()
{
    let mut pcs = PciConfigSpace::new();
    pcs.get(0, 0, 0).expect("PCI device not found");

    for i in 0..255
    {
        for j in 0..32
        {
            for k in 0..8
            {
                let p = pcs.get(i, j, k);
                if p != None
                {
                    println!("Bus: {}, Device: {}, Func: {}", i, j, k);
                    println!("Vendor Name: {}", pcs.get_vendor_name());
                    println!("Device Name: {}", pcs.get_device_name());
                    println!("================================");
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct PciConfigSpace
{
    raw_data: [u32; 16],
    pub device_id: u16,
    pub vendor_id: u16,
    pub status: u16,
    pub command: u16,
    pub class_code_high: u8,
    pub class_code_middle: u8,
    pub class_code_low: u8,
    pub revision_id: u8,
    pub bist: u8,
    pub header_type: u8,
    pub latency_timer: u8,
    pub cache_line_size: u8,
    pub base_addr_registers: [u32; 6],
    pub cardbus_cis_pointer: u32,
    pub subsystem_id: u16,
    pub subsystem_vendor_id: u16,
    pub expansion_rom_base_addr: u32,
    pub capabilities_pointer: u8,
    pub max_lat: u8,
    pub min_gnt: u8,
    pub interrupt_pin: u8,
    pub interrupt_line: u8
}

impl PciConfigSpace
{
    pub fn new() -> PciConfigSpace
    {
        return PciConfigSpace
        {
            raw_data: [0; 16],
            device_id: 0,
            vendor_id: 0,
            status: 0,
            command: 0,
            class_code_high: 0,
            class_code_middle: 0,
            class_code_low: 0,
            revision_id: 0,
            bist: 0,
            header_type: 0,
            latency_timer: 0,
            cache_line_size: 0,
            base_addr_registers: [0; 6],
            cardbus_cis_pointer: 0,
            subsystem_id: 0,
            subsystem_vendor_id: 0,
            expansion_rom_base_addr: 0,
            capabilities_pointer: 0,
            max_lat: 0,
            min_gnt: 0,
            interrupt_pin: 0,
            interrupt_line: 0
        };
    }

    pub fn get(&mut self, bus: u8, device: u8, func: u8) -> Option<&PciConfigSpace>
    {
        self.get_all_config_space(bus, device, func);

        if self.is_exist()
        {
            self.set_config();
            return Some(self);
        }
        else
        {
            return None;
        }
    }

    pub fn get_vendor_name(&self) -> &str
    {
        let mut name = "Unknown";

        if !self.is_exist()
        {
            panic!("PCI device not found");
        }

        for vendor in Vendors::iter()
        {
            if vendor.id() == self.vendor_id
            {
                name = vendor.name();
                break;
            }
        }

        return name;
    }

    pub fn get_device_name(&self) -> &str
    {
        let mut name = "Unknown";

        if !self.is_exist()
        {
            panic!("PCI device not found");
        }

        for vendor in Vendors::iter()
        {
            if vendor.id() == self.vendor_id
            {
                for device in vendor.devices()
                {
                    if device.id() == self.device_id
                    {
                        name = device.name();
                        break;
                    }
                }

                break;
            }
        }

        return name;
    }

    fn set_config(&mut self)
    {
        self.device_id = ((self.raw_data[0] & PCI_CS32_DEVICE_ID_MASK) >> 16) as u16;
        self.vendor_id = (self.raw_data[0] & PCI_CS32_VENDOR_ID_MASK) as u16;
        self.status = ((self.raw_data[1] & PCI_CS32_STATUS_MASK) >> 16) as u16;
        self.command = (self.raw_data[1] & PCI_CS32_COMMAND_MASK) as u16;
        self.class_code_high = ((self.raw_data[2] & PCI_CS32_CLASS_CODE_MASK) >> 24) as u8;
        self.class_code_middle = ((self.raw_data[2] & PCI_CS32_CLASS_CODE_MASK) >> 16) as u8;
        self.class_code_low = ((self.raw_data[2] & PCI_CS32_CLASS_CODE_MASK) >> 8) as u8;
        self.revision_id = (self.raw_data[2] & PCI_CS32_REVISION_ID_MASK) as u8;
        self.bist = ((self.raw_data[3] & PCI_CS32_BIST_MASK) >> 24) as u8;
        self.header_type = ((self.raw_data[3] & PCI_CS32_HEADER_TYPE_MASK) >> 16) as u8;
        self.latency_timer = ((self.raw_data[3] & PCI_CS32_LATENCY_TIMER_MASK) >> 8) as u8;
        self.cache_line_size = (self.raw_data[3] & PCI_CS32_CACHE_LINE_SIZE_MASK) as u8;
        self.base_addr_registers[0] = self.raw_data[4];
        self.base_addr_registers[1] = self.raw_data[5];
        self.base_addr_registers[2] = self.raw_data[6];
        self.base_addr_registers[3] = self.raw_data[7];
        self.base_addr_registers[4] = self.raw_data[8];
        self.base_addr_registers[5] = self.raw_data[9];
        self.cardbus_cis_pointer = self.raw_data[10];
        self.subsystem_id = ((self.raw_data[11] & PCI_CS32_SUBSYSTEM_ID_MASK) >> 16) as u16;
        self.subsystem_vendor_id = (self.raw_data[11] & PCI_CS32_SUBSYSTEM_VENDOR_ID_MASK) as u16;
        self.expansion_rom_base_addr = self.raw_data[12];
        self.capabilities_pointer = (self.raw_data[13] & PCI_CS32_CAPABILITIES_POINTER_MASK) as u8;
        self.max_lat = ((self.raw_data[15] & PCI_CS32_MAX_LAT_MASK) >> 24) as u8;
        self.min_gnt = ((self.raw_data[15] & PCI_CS32_MIN_GNT_MASK) >> 16) as u8;
        self.interrupt_pin = ((self.raw_data[15] & PCI_CS32_INTERRUPT_PIN_MASK) >> 8) as u8;
        self.interrupt_line = (self.raw_data[15] & PCI_CS32_INTERRUPT_LINE_MASK) as u8;

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