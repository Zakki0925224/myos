use core::ptr::{read_volatile, write_volatile};

use crate::{util::logger::{log_warn, log_debug}, device::{pci::{PciDevice, BaseAddressRegister}, PCI}, println, mem::PAGING};

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

const PORT_CMD_ST_MASK: u32 = 0x1;
const PORT_CMD_FRE_MASK: u32 = 0x10;
const PORT_CMD_FR_MASK: u32 = 0x4000;
const PORT_CMD_CR_MASK: u32 = 0x8000;

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
    hba_base_addr: u32
}

impl Ahci
{
    pub fn new() -> Ahci
    {
        let pci_ahci_device = PciDevice::new();

        return Ahci { is_init: false, pci_ahci_device, hba_base_addr: 0 };
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

        if let Some(BaseAddressRegister::MemoryAddress32Bit((addr))) = base_addr
        {
            self.hba_base_addr = addr;
        }
        else
        {
            log_warn("AHCI BAR#5 not found");
            return;
        }

        self.is_init = true;
    }

    pub fn get_hba_mem_regs(&self) -> &mut HostBusAdapterMemoryRegisters
    {
        let hba = unsafe { &mut *(self.hba_base_addr as *mut HostBusAdapterMemoryRegisters) };
        return hba;
    }

    pub fn get_port_type(&self, port_num: usize) -> Option<PortType>
    {
        let status = self.get_hba_mem_regs().port_ctrl_regs[port_num].sata_status;
        let sig = self.get_hba_mem_regs().port_ctrl_regs[port_num].sig;
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

        return Some(((self.get_hba_mem_regs().port_impl >> port_num) & 0x1) != 0);
    }

    fn init_port_mem_space(&self, port_num: usize)
    {
        if port_num > MAX_PORT_COUNT - 1
        {
            return;
        }

        let port_ctrl_reg = &mut self.get_hba_mem_regs().port_ctrl_regs[port_num];

        self.lock_port_cmd(port_num);

        // setup command list memory area
        if let Some(mb_info) = PAGING.lock().alloc_single_page()
        {
            port_ctrl_reg.cmd_list_base_addr_low = mb_info.mem_block_start_addr;
            port_ctrl_reg.cmd_list_base_addr_high = 0;
        }

        // setup FIS memory area
        if let Some(mb_info) = PAGING.lock().alloc_single_page()
        {
            port_ctrl_reg.fis_base_addr_low = mb_info.mem_block_start_addr;
            port_ctrl_reg.fis_base_addr_high = 0;
        }

        // TODO: https://wiki.osdev.org/AHCI#AHCI port memory space initialization

        self.unlock_port_cmd(port_num);
    }

    fn lock_port_cmd(&self, port_num: usize)
    {
        if port_num > MAX_PORT_COUNT - 1
        {
            return;
        }

        let port_ctrl_reg = &mut self.get_hba_mem_regs().port_ctrl_regs[port_num];

        // clear ST (bit0)
        port_ctrl_reg.cmd &= !PORT_CMD_ST_MASK;
        // clear FRE (bit4)
        port_ctrl_reg.cmd &= !PORT_CMD_FRE_MASK;

        // wait until FR (bit 14) and CR (bit15) are cleared
        loop
        {
            if ((port_ctrl_reg.cmd & PORT_CMD_FR_MASK) != 0 ||
                (port_ctrl_reg.cmd & PORT_CMD_CR_MASK) != 0)
            {
                break;
            }
        }
    }

    fn unlock_port_cmd(&self, port_num: usize)
    {
        if port_num > MAX_PORT_COUNT - 1
        {
            return;
        }

        let port_ctrl_reg = &mut self.get_hba_mem_regs().port_ctrl_regs[port_num];

        // wait until CR (bit15) is cleared
        while (port_ctrl_reg.cmd & PORT_CMD_CR_MASK) != 0 {};

        // set FRE (bit4)
        port_ctrl_reg.cmd |= PORT_CMD_FRE_MASK;
        // set ST (bit0)
        port_ctrl_reg.cmd |= PORT_CMD_ST_MASK;
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
    reserved: [u32; 29],

    pub vendor_spec_regs: [u32; 24],
    pub port_ctrl_regs: [PortControlRegisters; MAX_PORT_COUNT]
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
    reserved0: u32,
    pub task_file_data: u32,
    pub sig: u32,
    pub sata_status: u32,
    pub sata_ctrl: u32,
    pub sata_err: u32,
    pub sata_active: u32,
    pub cmd_issue: u32,
    pub sata_notice: u32,
    pub fis_switch_ctrl: u32,
    reserved1: [u32; 11],
    pub vendor_spec: [u32; 4]
}