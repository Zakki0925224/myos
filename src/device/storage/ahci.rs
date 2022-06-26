use core::ptr::{read_volatile, write_volatile};

use crate::{util::logger::{log_warn, log_debug}, device::{pci::{PciDevice, BaseAddressRegister}, PCI}, println};

const PCI_AHCI_BASE_CLASS_CODE: u8 = 0x01;
const PCI_AHCI_SUB_CLASS_CODE: u8 = 0x06;
const PCI_AHCI_BASE_ADDR_INDEX: usize = 5;
const MAX_PORT_COUNT: usize = 32;

const PORT_IPM_ACTIVE: u8 = 0x1;
const PORT_DET_ACTIVE: u8 = 0x3;

const PORT_SIG_ATA: u32 = 0x101;        // SATA drive
const PORT_SIG_ATAPI: u32 = 0xeb140101; // SATAPI drive
const PORT_SIG_SEMB: u32 = 0xc33c0101;  // enclosure management bridge
const PORT_SIG_PM: u32 = 0x96690101;    // port multiplier

#[derive(Debug, PartialEq)]
enum PortType
{
    SataDrive,
    SatapiDrive,
    EnclosureManagementBridge,
    PortMultiplier
}

#[derive(PartialEq)]
pub struct Ahci
{
    is_init: bool,
    pci_ahci_device: PciDevice,
    hba_mem_regs: HostBusAdapterMemoryRegisters
}

impl Ahci
{
    pub fn new() -> Ahci
    {
        let pci_ahci_device = PciDevice::new();

        return Ahci { is_init: false, pci_ahci_device, hba_mem_regs: HostBusAdapterMemoryRegisters::new() };
    }

    pub fn init(&mut self)
    {
        for device in PCI.lock().get_devices()
        {
            if device.get_base_class_code() == PCI_AHCI_BASE_CLASS_CODE &&
               device.get_sub_class_code() == PCI_AHCI_SUB_CLASS_CODE
            {
                self.pci_ahci_device = device;
            }
        }

        if !self.pci_ahci_device.is_exist()
        {
            log_warn("AHCI not found");
            return;
        }

        let base_addr = self.pci_ahci_device.get_base_addr(PCI_AHCI_BASE_ADDR_INDEX);

        match base_addr
        {
            Some(BaseAddressRegister::MemoryAddress32Bit(addr)) => self.hba_mem_regs.init(addr),
            _ =>
            {
                log_warn("AHCI BAR#5 not found");
                return;
            }
        }

        self.is_init = true;
    }

    pub fn get_port_type(&self, port_num: usize) -> Option<PortType>
    {
        let status = self.hba_mem_regs.port_ctrl_regs[port_num].sata_status;
        let sig = self.hba_mem_regs.port_ctrl_regs[port_num].sig;
        let ipm = ((status >> 8) & 0xf) as u8;
        let det = (status & 0xf) as u8;

        if ipm != PORT_IPM_ACTIVE || det != PORT_DET_ACTIVE
        {
            return None;
        }

        match sig
        {
            PORT_SIG_ATAPI => return Some(PortType::SatapiDrive),
            PORT_SIG_SEMB => return Some(PortType::EnclosureManagementBridge),
            PORT_SIG_PM => return Some(PortType::PortMultiplier),
            _ => return Some(PortType::SataDrive)
        }
    }

    pub fn get_impl_ports_cnt(&self) -> usize
    {
        let mut cnt = 0;

        for i in 0..MAX_PORT_COUNT
        {
            if self.is_impl_port(i).unwrap()
            {
                cnt += 1;
            }
        }

        return cnt;
    }

    pub fn get_active_ports_cnt(&self) -> usize
    {
        let mut cnt = 0;

        for i in 0..self.get_impl_ports_cnt()
        {
            if let Some(_) = self.get_port_type(i)
            {
                cnt += 1;
            }
        }

        return cnt;
    }

    pub fn ahci_info(&self)
    {
        if !self.is_init()
        {
            println!("AHCI wasn't initialized");
            return;
        }

        println!("Implemented ports count: {}", self.get_impl_ports_cnt());
        println!("Active ports count: {}", self.get_active_ports_cnt());

        for i in 0..MAX_PORT_COUNT
        {
            let port_type = self.get_port_type(i);

            if !self.is_impl_port(i).unwrap() || port_type == None
            {
                continue;
            }

            println!("Port{} type: {:?}", i, port_type.unwrap());
        }
    }

    fn is_impl_port(&self, port_num: usize) -> Option<bool>
    {
        if port_num > MAX_PORT_COUNT - 1
        {
            return None;
        }

        return Some(((self.hba_mem_regs.port_impl >> port_num) & 0x1) != 0);
    }

    pub fn is_init(&self) -> bool
    {
        return self.is_init;
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct HostBusAdapterMemoryRegisters
{
    // 0x0 - 0x2b generic host control
    pub host_cap: u32,
    pub global_host_ctrl: u32,
    pub int_status: u32,
    pub port_impl: u32,
    pub version: u32,
    pub ccc_ctrl: u32,
    pub ccc_ports: u32,
    pub enc_man_loc: u32,
    pub enc_man_ctrl: u32,
    pub host_cap2: u32,
    pub bios_ho_ctrl: u32,

    // 0x2c - 0x9f reserved

    // 0xa0 - 0xff vendor specific registers
    pub vendor_spec_regs: [u32; 24],

    // 0x100 ~ port control registers
    pub port_ctrl_regs: [PortControlRegisters; MAX_PORT_COUNT]
}

impl HostBusAdapterMemoryRegisters
{
    pub fn new() -> HostBusAdapterMemoryRegisters
    {
        return HostBusAdapterMemoryRegisters
        {
            host_cap: 0,
            global_host_ctrl: 0,
            int_status: 0,
            port_impl: 0,
            version: 0,
            ccc_ctrl: 0,
            ccc_ports: 0,
            enc_man_loc: 0,
            enc_man_ctrl: 0,
            host_cap2: 0,
            bios_ho_ctrl: 0,
            vendor_spec_regs: [0; 24],
            port_ctrl_regs: [PortControlRegisters::new(); MAX_PORT_COUNT]
        }
    }

    pub fn init(&mut self, base_addr: u32)
    {
        self.host_cap = self.read_mem_reg(base_addr, 0);
        self.global_host_ctrl = self.read_mem_reg(base_addr, 1);
        self.int_status = self.read_mem_reg(base_addr, 2);
        self.port_impl = self.read_mem_reg(base_addr, 3);
        self.version = self.read_mem_reg(base_addr, 4);
        self.ccc_ctrl = self.read_mem_reg(base_addr, 5);
        self.ccc_ports = self.read_mem_reg(base_addr, 6);
        self.enc_man_loc = self.read_mem_reg(base_addr, 7);
        self.enc_man_ctrl = self.read_mem_reg(base_addr, 8);
        self.host_cap2 = self.read_mem_reg(base_addr, 9);
        self.bios_ho_ctrl = self.read_mem_reg(base_addr, 10);
        // 4 * 29 bytes reserved
        for i in 0..24
        {
            self.vendor_spec_regs[i] = self.read_mem_reg(base_addr, 40 + i);
        }

        let port_ctrl_regs_size = 32;

        for i in 0..MAX_PORT_COUNT
        {
            self.port_ctrl_regs[i] = PortControlRegisters
            {
                cmd_list_base_addr_low: self.read_mem_reg(base_addr, 64 + i * port_ctrl_regs_size),
                cmd_list_base_addr_high: self.read_mem_reg(base_addr, 65 + i * port_ctrl_regs_size),
                fis_base_addr_low: self.read_mem_reg(base_addr, 66 + i * port_ctrl_regs_size),
                fis_base_addr_high: self.read_mem_reg(base_addr, 67 + i * port_ctrl_regs_size),
                int_status: self.read_mem_reg(base_addr, 68 + i * port_ctrl_regs_size),
                int_enable: self.read_mem_reg(base_addr, 69 + i * port_ctrl_regs_size),
                cmd: self.read_mem_reg(base_addr, 70 + i * port_ctrl_regs_size),
                task_file_data: self.read_mem_reg(base_addr, 72 + i * port_ctrl_regs_size),
                sig: self.read_mem_reg(base_addr, 73 + i * port_ctrl_regs_size),
                sata_status: self.read_mem_reg(base_addr, 74 + i * port_ctrl_regs_size),
                sata_ctrl: self.read_mem_reg(base_addr, 75 + i * port_ctrl_regs_size),
                sata_err: self.read_mem_reg(base_addr, 76 + i * port_ctrl_regs_size),
                sata_active: self.read_mem_reg(base_addr, 77 + i * port_ctrl_regs_size),
                cmd_issue: self.read_mem_reg(base_addr, 78 + i * port_ctrl_regs_size),
                sata_notice: self.read_mem_reg(base_addr, 79 + i * port_ctrl_regs_size),
                fis_switch_ctrl: self.read_mem_reg(base_addr, 80 + i * port_ctrl_regs_size),
                vendor_spec:
                [
                    self.read_mem_reg(base_addr, 100 + i * MAX_PORT_COUNT),
                    self.read_mem_reg(base_addr, 101 + i * MAX_PORT_COUNT),
                    self.read_mem_reg(base_addr, 102 + i * MAX_PORT_COUNT),
                    self.read_mem_reg(base_addr, 103 + i * MAX_PORT_COUNT),
                ]
            }
        }
    }

    fn read_mem_reg(&self, base_addr: u32, index: usize) -> u32
    {
        unsafe
        {
            let buffer = (base_addr + index as u32 * 4) as *const u32;
            return read_volatile(buffer);
        }
    }

    fn write_mem_reg(&self, base_addr: u32, index: usize, data: u32)
    {
        unsafe
        {
            let buffer = (base_addr + index as u32 * 4) as *mut u32;
            write_volatile(buffer, data);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PortControlRegisters
{
    pub cmd_list_base_addr_low: u32,
    pub cmd_list_base_addr_high: u32,
    pub fis_base_addr_low: u32,
    pub fis_base_addr_high: u32,
    pub int_status: u32,
    pub int_enable: u32,
    pub cmd: u32,
    // reserved
    pub task_file_data: u32,
    pub sig: u32,
    pub sata_status: u32,
    pub sata_ctrl: u32,
    pub sata_err: u32,
    pub sata_active: u32,
    pub cmd_issue: u32,
    pub sata_notice: u32,
    pub fis_switch_ctrl: u32,
    // reserved * 11
    pub vendor_spec: [u32; 4]
}

impl PortControlRegisters
{
    pub fn new() -> PortControlRegisters
    {
        return PortControlRegisters
        {
            cmd_list_base_addr_low: 0,
            cmd_list_base_addr_high: 0,
            fis_base_addr_low: 0,
            fis_base_addr_high: 0,
            int_status: 0,
            int_enable: 0,
            cmd: 0,
            task_file_data: 0,
            sig: 0,
            sata_status: 0,
            sata_ctrl: 0,
            sata_err: 0,
            sata_active: 0,
            cmd_issue: 0,
            sata_notice: 0,
            fis_switch_ctrl: 0,
            vendor_spec: [0; 4]
        }
    }
}