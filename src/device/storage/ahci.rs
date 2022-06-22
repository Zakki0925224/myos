use core::ptr::{read_volatile, write_volatile};

use crate::{util::logger::{log_warn, log_debug}, device::{pci::{PciDevice, BaseAddressRegister}, PCI}, println};

const PCI_AHCI_BASE_CLASS_CODE: u8 = 0x01;
const PCI_AHCI_SUB_CLASS_CODE: u8 = 0x06;
const PCI_AHCI_BASE_ADDR_INDEX: usize = 5;
const MAX_PORT_COUNT: usize = 32;

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
                self.pci_ahci_device.dump_bar();
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

        log_debug("HBA#0", self.hba_mem_regs.port_ctrl_regs[0]);

        self.is_init = true;
    }

    pub fn is_init(&self) -> bool
    {
        return self.is_init;
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct HostBusAdapterPortControlRegisters
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
    pub sign: u32,
    pub sata_status: u32,
    pub sata_ctrl: u32,
    pub sata_err: u32,
    pub sata_active: u32,
    pub cmd_issue: u32,
    pub sata_notice: u32,
    pub fis_switch_ctrl: u32,
    // reserved
    pub vendor_spec: [u32; 4]
}

impl HostBusAdapterPortControlRegisters
{
    pub fn new() -> HostBusAdapterPortControlRegisters
    {
        return HostBusAdapterPortControlRegisters
        {
            cmd_list_base_addr_low: 0,
            cmd_list_base_addr_high: 0,
            fis_base_addr_low: 0,
            fis_base_addr_high: 0,
            int_status: 0,
            int_enable: 0,
            cmd: 0,
            task_file_data: 0,
            sign: 0,
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
    pub vendor_spec_regs: [u32; 3],

    // 0x100 ~ port control registers
    pub port_ctrl_regs: [HostBusAdapterPortControlRegisters; 32]
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
            vendor_spec_regs: [0; 3],
            port_ctrl_regs: [HostBusAdapterPortControlRegisters::new(); 32]
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
        self.vendor_spec_regs =
        [
            self.read_mem_reg(base_addr, 40),
            self.read_mem_reg(base_addr, 41),
            self.read_mem_reg(base_addr, 42)
        ];

        let port_ctrl_regs_size = 74;

        for i in 0..32
        {
            self.port_ctrl_regs[i] = HostBusAdapterPortControlRegisters
            {
                cmd_list_base_addr_low: self.read_mem_reg(base_addr, 43 + i * port_ctrl_regs_size),
                cmd_list_base_addr_high: self.read_mem_reg(base_addr, 44 + i * port_ctrl_regs_size),
                fis_base_addr_low: self.read_mem_reg(base_addr, 45 + i * port_ctrl_regs_size),
                fis_base_addr_high: self.read_mem_reg(base_addr, 46 + i * port_ctrl_regs_size),
                int_status: self.read_mem_reg(base_addr, 47 + i * port_ctrl_regs_size),
                int_enable: self.read_mem_reg(base_addr, 48 + i * port_ctrl_regs_size),
                cmd: self.read_mem_reg(base_addr, 48 + i * port_ctrl_regs_size),
                task_file_data: self.read_mem_reg(base_addr, 50 + i * port_ctrl_regs_size),
                sign: self.read_mem_reg(base_addr, 51 + i * port_ctrl_regs_size),
                sata_status: self.read_mem_reg(base_addr, 52 + i * port_ctrl_regs_size),
                sata_ctrl: self.read_mem_reg(base_addr, 53 + i * port_ctrl_regs_size),
                sata_err: self.read_mem_reg(base_addr, 54 + i * port_ctrl_regs_size),
                sata_active: self.read_mem_reg(base_addr, 55 + i * port_ctrl_regs_size),
                cmd_issue: self.read_mem_reg(base_addr, 56 + i * port_ctrl_regs_size),
                sata_notice: self.read_mem_reg(base_addr, 57 + i * port_ctrl_regs_size),
                fis_switch_ctrl: self.read_mem_reg(base_addr, 58 + i * port_ctrl_regs_size),
                vendor_spec:
                [
                    self.read_mem_reg(base_addr, 69 + i * 32),
                    self.read_mem_reg(base_addr, 70 + i * 32),
                    self.read_mem_reg(base_addr, 71 + i * 32),
                    self.read_mem_reg(base_addr, 72 + i * 32),
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