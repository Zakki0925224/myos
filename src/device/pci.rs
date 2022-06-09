use pci_ids::{Vendors, Classes};

use crate::{arch::asm, println, print};

pub const PCI_VENDOR_ID_INTEL: u16 = 0x8086;
const PCI_CS32_DEVICE_NOT_EXIST: u32 = 0xffffffff;
const PCI_CS32_FILL: u32 = 0xffffffff;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Pci
{
    devices: [PciDevice; 256],
    device_cnt: usize
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

        return Pci { devices, device_cnt };
    }

    pub fn get_devices(&self) -> [PciDevice; 256]
    {
        return self.devices;
    }

    pub fn get_device_cnt(&self) -> usize
    {
        return self.device_cnt;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PciHeaderType
{
    StandardPci,
    PciToPciBridge,
    CardBusBridge
}

#[derive(Debug, PartialEq, Eq)]
pub enum BaseAddressRegisterType
{
    MemorySpace,
    IOSpace,
    NoSpace
}

#[derive(Debug, PartialEq, Eq)]
pub enum BaseAddressRegisterMemoryType
{
    Bit32Space,
    Bit64Space,
    Bit32SpaceUpTo1MB,
    Bit64SpaceUpTo1MB
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

    pub fn get_status(&self) -> u16
    {
        return (self.config_space.raw_data[1] >> 16) as u16;
    }

    pub fn get_commnad(&self) -> u16
    {
        return self.config_space.raw_data[1] as u16;
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

    pub fn get_sub_class_name(&self) -> &str
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

    pub fn get_lat_timer(&self) -> u8
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

    pub fn get_base_addr_reg_type(&self) -> Option<BaseAddressRegisterType>
    {
        if self.get_header_type() != PciHeaderType::StandardPci
        {
            return None
        }

        let config_space = self.config_space.raw_data;
        let mut bar_type = BaseAddressRegisterType::NoSpace;

        for i in 4..10
        {
            if config_space[i] == 0 || config_space[i] == PCI_CS32_DEVICE_NOT_EXIST
            {
                continue;
            }

            // bit 0
            if (config_space[i] & 0x1) != 0
            {
                bar_type = BaseAddressRegisterType::IOSpace;
            }
            else
            {
                bar_type = BaseAddressRegisterType::MemorySpace;
            }

            break;
        }

        return Some(bar_type);
    }

    pub fn get_base_addr_mem_type(&self) -> Option<BaseAddressRegisterMemoryType>
    {
        if self.get_base_addr_reg_type() != Some(BaseAddressRegisterType::MemorySpace)
        {
            return None;
        }

        let config_space = self.config_space.raw_data;
        let mut mem_type = BaseAddressRegisterMemoryType::Bit32Space;

        for i in 4..10
        {
            if config_space[i] == 0 || config_space[i] == PCI_CS32_DEVICE_NOT_EXIST
            {
                continue;
            }

            let data = (config_space[i] as u8) & 0x6;

            // bit 1~2
            if data == 0
            {
                mem_type = BaseAddressRegisterMemoryType::Bit32Space;
            }
            else if data == 0x2
            {
                mem_type = BaseAddressRegisterMemoryType::Bit32SpaceUpTo1MB;
            }
            else if data == 0x4
            {
                mem_type = BaseAddressRegisterMemoryType::Bit64Space;
            }
            else if data == 0x6
            {
                mem_type = BaseAddressRegisterMemoryType::Bit64SpaceUpTo1MB;
            }
            else
            {
                panic!("Invalid base address register memory type");
            }

            break;
        }

        return Some(mem_type);
    }

    pub fn get_standard_base_addr(&self) -> Option<u32>
    {

        if self.get_header_type() != PciHeaderType::StandardPci
        {
            return None;
        }

        if self.get_base_addr_mem_type() == Some(BaseAddressRegisterMemoryType::Bit64Space) ||
           self.get_base_addr_mem_type() == Some(BaseAddressRegisterMemoryType::Bit64SpaceUpTo1MB)
        {
            return None;
        }

        let config_space = self.config_space.raw_data;

        for i in 4..10
        {
            if config_space[i] == 0 || config_space[i] == PCI_CS32_DEVICE_NOT_EXIST
            {
                continue;
            }

            if self.get_base_addr_reg_type() == Some(BaseAddressRegisterType::MemorySpace)
            {
                return Some(config_space[i] & !0xf);
            }
            else if self.get_base_addr_reg_type() == Some(BaseAddressRegisterType::IOSpace)
            {
                return Some(config_space[i] & !0x3);
            }

        }

        return None;
    }

    pub fn get_bist_reg(&self) -> u8
    {
        return (self.config_space.raw_data[3] >> 24) as u8;
    }

    pub fn get_interrupt_pin(&self) -> u8
    {
        return (self.config_space.raw_data[15] >> 8) as u8;
    }

    pub fn get_interrupt_line(&self) -> u8
    {
        return self.config_space.raw_data[15] as u8;
    }

    pub fn get_cardbus_sis_pointer(&self) -> Option<u32>
    {
        if self.get_header_type() != PciHeaderType::StandardPci
        {
            return None;
        }

        return Some(self.config_space.raw_data[10]);
    }

    pub fn get_subsystem_device_id(&self) -> Option<u16>
    {
        match self.get_header_type()
        {
            PciHeaderType::StandardPci => return Some((self.config_space.raw_data[11] >> 16) as u16),
            PciHeaderType::PciToPciBridge => return None,
            PciHeaderType::CardBusBridge => return Some(self.config_space.raw_data[16] as u16)
        }
    }

    pub fn get_subsystem_vendor_id(&self) -> Option<u16>
    {
        match self.get_header_type()
        {
            PciHeaderType::StandardPci => return Some(self.config_space.raw_data[11] as u16),
            PciHeaderType::PciToPciBridge => return None,
            PciHeaderType::CardBusBridge => return Some((self.config_space.raw_data[16] >> 16) as u16)
        }
    }

    pub fn get_expansion_rom_base_addr(&self) -> Option<u32>
    {
        match self.get_header_type()
        {
            PciHeaderType::StandardPci => return Some(self.config_space.raw_data[12]),
            PciHeaderType::PciToPciBridge => return Some(self.config_space.raw_data[14]),
            PciHeaderType::CardBusBridge => return None
        }
    }

    pub fn get_cap_pointer(&self) -> Option<u8>
    {
        if self.get_header_type() != PciHeaderType::StandardPci
        {
            return None;
        }

        return Some(self.config_space.raw_data[13] as u8);
    }

    pub fn get_max_lat(&self) -> Option<u8>
    {
        if self.get_header_type() != PciHeaderType::StandardPci
        {
            return None;
        }

        return Some((self.config_space.raw_data[15] >> 24) as u8);
    }

    pub fn get_min_gnt(&self) -> Option<u8>
    {
        if self.get_header_type() != PciHeaderType::StandardPci
        {
            return None;
        }

        return Some((self.config_space.raw_data[15] >> 16) as u8);
    }

    pub fn get_secondary_lat_timer(&self) -> Option<u8>
    {
        if self.get_header_type() != PciHeaderType::PciToPciBridge
        {
            return None;
        }

        return Some((self.config_space.raw_data[6] >> 24) as u8);
    }

    pub fn get_subordinate_bus_num(&self) -> Option<u8>
    {
        if self.get_header_type() == PciHeaderType::StandardPci
        {
            return None;
        }

        return Some((self.config_space.raw_data[6] >> 16) as u8);
    }

    pub fn get_secondary_bus_num(&self) -> Option<u8>
    {
        if self.get_header_type() != PciHeaderType::PciToPciBridge
        {
            return None;
        }

        return Some((self.config_space.raw_data[6] >> 8) as u8);
    }

    pub fn get_primary_bus_num(&self) -> Option<u8>
    {
        if self.get_header_type() != PciHeaderType::PciToPciBridge
        {
            return None;
        }

        return Some(self.config_space.raw_data[6] as u8);
    }

    pub fn get_secondary_status(&self) -> Option<u16>
    {
        match self.get_header_type()
        {
            PciHeaderType::StandardPci => return None,
            PciHeaderType::PciToPciBridge => return Some((self.config_space.raw_data[7] >> 16) as u16),
            PciHeaderType::CardBusBridge => return Some((self.config_space.raw_data[5] >> 16) as u16)
        }
    }

    pub fn get_io_limit(&self) -> Option<u8>
    {
        if self.get_header_type() != PciHeaderType::PciToPciBridge
        {
            return None;
        }

        return Some((self.config_space.raw_data[7] >> 8) as u8);
    }

    pub fn get_io_base(&self) -> Option<u8>
    {
        if self.get_header_type() != PciHeaderType::PciToPciBridge
        {
            return None;
        }

        return Some(self.config_space.raw_data[7] as u8);
    }

    pub fn get_mem_limit(&self) -> Option<u16>
    {
        if self.get_header_type() != PciHeaderType::PciToPciBridge
        {
            return None;
        }

        return Some((self.config_space.raw_data[8] >> 16) as u16);
    }

    pub fn get_mem_base(&self) -> Option<u16>
    {
        if self.get_header_type() != PciHeaderType::PciToPciBridge
        {
            return None;
        }

        return Some(self.config_space.raw_data[8] as u16);
    }

    pub fn get_prefetchable_mem_limit(&self) -> Option<u16>
    {
        if self.get_header_type() != PciHeaderType::PciToPciBridge
        {
            return None;
        }

        return Some((self.config_space.raw_data[9] >> 16) as u16);
    }

    pub fn get_prefetchable_mem_base(&self) -> Option<u16>
    {
        if self.get_header_type() != PciHeaderType::PciToPciBridge
        {
            return None;
        }

        return Some(self.config_space.raw_data[9] as u16);
    }

    pub fn get_prefetchable_base_upper(&self) -> Option<u32>
    {
        if self.get_header_type() != PciHeaderType::PciToPciBridge
        {
            return None;
        }

        return Some(self.config_space.raw_data[10]);
    }

    pub fn get_prefetchable_limit_upper(&self) -> Option<u32>
    {
        if self.get_header_type() != PciHeaderType::PciToPciBridge
        {
            return None;
        }

        return Some(self.config_space.raw_data[11]);
    }

    pub fn get_io_limit_upper(&self) -> Option<u16>
    {
        if self.get_header_type() != PciHeaderType::PciToPciBridge
        {
            return None;
        }

        return Some((self.config_space.raw_data[12] >> 16) as u16);
    }

    pub fn get_io_base_upper(&self) -> Option<u16>
    {
        if self.get_header_type() != PciHeaderType::PciToPciBridge
        {
            return None;
        }

        return Some(self.config_space.raw_data[12] as u16);
    }

    pub fn get_bridge_control_reg(&self) -> Option<u16>
    {
        if self.get_header_type() == PciHeaderType::StandardPci
        {
            return None;
        }

        return Some((self.config_space.raw_data[15] >> 16) as u16);
    }

    pub fn get_cardbus_socket_exca_base_addr(&self) -> Option<u32>
    {
        if self.get_header_type() != PciHeaderType::CardBusBridge
        {
            return None;
        }

        return Some(self.config_space.raw_data[4]);
    }

    pub fn get_cap_list_offset(&self) -> Option<u8>
    {
        if self.get_header_type() != PciHeaderType::CardBusBridge
        {
            return None;
        }

        return Some(self.config_space.raw_data[5] as u8);
    }

    pub fn get_cardbus_lat_timer(&self) -> Option<u8>
    {
        if self.get_header_type() != PciHeaderType::CardBusBridge
        {
            return None;
        }

        return Some((self.config_space.raw_data[6] >> 24) as u8);
    }

    pub fn get_cardbus_num(&self) -> Option<u8>
    {
        if self.get_header_type() != PciHeaderType::CardBusBridge
        {
            return None;
        }

        return Some((self.config_space.raw_data[6] >> 8) as u8);
    }

    pub fn get_pci_bus_num(&self) -> Option<u8>
    {
        if self.get_header_type() != PciHeaderType::CardBusBridge
        {
            return None;
        }

        return Some(self.config_space.raw_data[6] as u8);
    }

    pub fn get_pc_card_legacy_mode_base_addr(&self) -> Option<u32>
    {
        if self.get_header_type() != PciHeaderType::CardBusBridge
        {
            return None;
        }

        return Some(self.config_space.raw_data[17]);
    }

    pub fn get_standard_base_addr_regs(&self) -> Option<[u32; 6]>
    {
        if self.get_header_type() != PciHeaderType::StandardPci
        {
            return None;
        }

        let regs =
        [
            self.config_space.raw_data[4],
            self.config_space.raw_data[5],
            self.config_space.raw_data[6],
            self.config_space.raw_data[7],
            self.config_space.raw_data[8],
            self.config_space.raw_data[9]
        ];

        return Some(regs);
    }

    pub fn get_pci_to_pci_bridge_base_addr_regs(&self) -> Option<[u32; 2]>
    {
        if self.get_header_type() != PciHeaderType::PciToPciBridge
        {
            return None;
        }

        let regs =
        [
            self.config_space.raw_data[4],
            self.config_space.raw_data[5],
        ];

        return Some(regs);
    }

    pub fn get_cardbus_bridge_mem_base_addr_regs(&self) -> Option<[u32; 2]>
    {
        if self.get_header_type() != PciHeaderType::CardBusBridge
        {
            return None;
        }

        let regs =
        [
            self.config_space.raw_data[7],
            self.config_space.raw_data[9],
        ];

        return Some(regs);
    }

    pub fn get_cardbus_bridge_mem_limit_regs(&self) -> Option<[u32; 2]>
    {
        if self.get_header_type() != PciHeaderType::CardBusBridge
        {
            return None;
        }

        let regs =
        [
            self.config_space.raw_data[8],
            self.config_space.raw_data[10],
        ];

        return Some(regs);
    }

    pub fn get_cardbus_bridge_io_base_addr_regs(&self) -> Option<[u32; 2]>
    {
        if self.get_header_type() != PciHeaderType::CardBusBridge
        {
            return None;
        }

        let regs =
        [
            self.config_space.raw_data[11],
            self.config_space.raw_data[13],
        ];

        return Some(regs);
    }

    pub fn get_cardbus_bridge_io_limit_regs(&self) -> Option<[u32; 2]>
    {
        if self.get_header_type() != PciHeaderType::CardBusBridge
        {
            return None;
        }

        let regs =
        [
            self.config_space.raw_data[12],
            self.config_space.raw_data[14],
        ];

        return Some(regs);
    }

    pub fn is_multi_function_device(&self) -> bool
    {
        return ((self.config_space.raw_data[3] >> 16) as u8 & 0x80) != 0;
    }

    pub fn read_config(&self, offset: u32) -> u32
    {
        return self.config_space.read_pci_config(self.bus, self.device, self.func, offset);
    }

    pub fn write_config(&self, offset: u32, data: u32)
    {
        self.config_space.write_pci_config(self.bus, self.device, self.func, offset, data);
    }

    pub fn dump_lspci(&self)
    {
        println!("{}:{}.{} {}: {} {} (rev {:02})", self.get_bus_num(), self.get_device_num(), self.get_func_num(), self.get_class_name(), self.get_vendor_name(), self.get_device_name(), self.get_revision_id());
    }

    // like lspci -x command
    pub fn dump_lspci_x(&self)
    {
        println!("{}:{}.{} {}: {} {} (rev {:02})", self.get_bus_num(), self.get_device_num(), self.get_func_num(), self.get_class_name(), self.get_vendor_name(), self.get_device_name(), self.get_revision_id());

        for i in 0..4
        {
            print!("{:02x}: ", i * 16);

            for j in 0..4
            {
                let mut k = 4;
                loop
                {
                    if k > 32
                    {
                        break;
                    }

                    print!("{:01x}", ((self.config_space.raw_data[j + 4 * i] & (0xf << 32 - k)) >> 32 - k) as u8);

                    if k % 8 == 0
                    {
                        print!(" ");
                    }

                    k += 4;
                }
            }

            print!("\n");
        }
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

    pub fn read_pci_config(&self, bus: u8, device: u8, func: u8, offset: u32) -> u32
    {
        // offset is a multiple of 4
        let addr = 0x80000000 | (bus as u32) << 16 | (device as u32) << 11 | (func as u32) << 8 | offset;
        asm::out32(0xcf8, addr);

        return asm::in32(0xcfc);
    }

    pub fn write_pci_config(&self, bus: u8, device: u8, func: u8, offset: u32, data: u32)
    {
        let addr = 0x80000000 | (bus as u32) << 16 | (device as u32) << 11 | (func as u32) << 8 | offset;
        asm::out32(0xcf8, addr);
        asm::out32(0xcfc, data);
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