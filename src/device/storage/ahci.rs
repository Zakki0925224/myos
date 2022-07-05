use core::ptr::{read_volatile, write_volatile};

use modular_bitfield::{bitfield, prelude::*};

use crate::{util::logger::*, device::{pci::{PciDevice, BaseAddressRegister}, PCI}, println, mem::{PHYS_MEM_MANAGER, phys_mem::{MemoryBlockInfo, MEM_BLOCK_SIZE}}};

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

const FIS_H2D_REGS_SIZE: u8 = 20;
const FIS_D2H_REGS_SIZE: u8 = 20;

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

        if let Some(BaseAddressRegister::MemoryAddress32Bit(addr)) = base_addr
        {
            self.hba_base_addr = addr;
        }
        else
        {
            log_warn("AHCI BAR#5 not found");
            return;
        }

        // init port memory space
        for i in 0..32
        {
            if self.get_port_type(i) != None &&
               self.is_impl_port(i) != None
            {
                self.init_port_mem_space(i);
            }
        }

        self.is_init = true;
    }

    fn read_hba_mem_regs(&self) -> HostBusAdapterMemoryRegisters
    {
        unsafe
        {
            let buffer = self.hba_base_addr as *const HostBusAdapterMemoryRegisters;
            return read_volatile(buffer);
        }
    }

    fn write_hba_mem_regs(&self, hba_mem_regs: HostBusAdapterMemoryRegisters)
    {
        unsafe
        {
            let buffer = self.hba_base_addr as *mut HostBusAdapterMemoryRegisters;
            write_volatile(buffer, hba_mem_regs);
        }
    }

    fn read_port_ctrl_regs(&self, port_num: usize) -> PortControlRegisters
    {
        return self.read_hba_mem_regs().port_ctrl_regs[port_num];
    }

    fn write_port_ctrl_regs(&self, port_num: usize, port_ctrl_regs: PortControlRegisters)
    {
        let mut hba_mem_regs = self.read_hba_mem_regs();
        hba_mem_regs.port_ctrl_regs[port_num] = port_ctrl_regs;
        self.write_hba_mem_regs(hba_mem_regs);
    }

    fn read_cmd_header(&self, port_num: usize, header_index: usize) -> CommandHeader
    {
        let addr = self.read_port_ctrl_regs(port_num).cmd_list_base_addr_low + header_index as u32;

        unsafe
        {
            let buffer = addr as *const CommandHeader;
            return read_volatile(buffer);
        }
    }

    fn write_cmd_header(&self, port_num: usize, header_index: usize, cmd_header: CommandHeader)
    {
        let addr = self.read_port_ctrl_regs(port_num).cmd_list_base_addr_low + header_index as u32;

        unsafe
        {
            let buffer = addr as *mut CommandHeader;
            write_volatile(buffer, cmd_header);
        }
    }

    pub fn get_port_type(&self, port_num: usize) -> Option<PortType>
    {
        if port_num > MAX_PORT_COUNT - 1
        {
            return None;
        }

        let status = self.read_port_ctrl_regs(port_num).sata_status;
        let sig = self.read_port_ctrl_regs(port_num).sig;
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

    pub fn read(&self, port_num: usize, cnt: u32)
    {
        if port_num > MAX_PORT_COUNT - 1
        {
            return;
        }

        let mut port_ctrl_regs = self.read_port_ctrl_regs(port_num);
        port_ctrl_regs.int_status = 0xfff1; // clear bits

        let mut spin_lock_timeout_cnt = 0;
        let slot = self.find_cmd_slot(port_num);

        if let None = slot
        {
            return;
        }

        let mut cmd_header = self.read_cmd_header(port_num, slot.unwrap() as usize);
        cmd_header.set_cmd_fis_len(FIS_H2D_REGS_SIZE / 4);
        cmd_header.set_write(0);
        cmd_header.set_phys_region_desc_table_len(((cnt - 1) >> 4) + 1);
    }

    fn find_cmd_slot(&self, port_num: usize) -> Option<u32>
    {
        if port_num > MAX_PORT_COUNT - 1
        {
            return None;
        }

        let port_ctrl_regs = self.read_port_ctrl_regs(port_num);
        let mut slots = port_ctrl_regs.sata_active | port_ctrl_regs.cmd_issue;

        for i in 0..32
        {
            if (slots & 1) == 0
            {
                return Some(i);
            }

            slots >>= 1;
        }

        log_warn("Cannot find free command list entry");
        return None;
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

        return Some(((self.read_hba_mem_regs().port_impl >> port_num) & 0x1) != 0);
    }

    fn init_port_mem_space(&self, port_num: usize)
    {
        if port_num > MAX_PORT_COUNT - 1
        {
            return;
        }

        self.lock_port_cmd(port_num);
        let mut port_ctrl_reg = self.read_port_ctrl_regs(port_num);

        // setup command list memory area
        let mb_info = PHYS_MEM_MANAGER.lock().alloc_single_mem_block();

        if mb_info != None
        {
            port_ctrl_reg.cmd_list_base_addr_low = mb_info.unwrap().mem_block_start_addr;
            port_ctrl_reg.cmd_list_base_addr_high = 0;

            // allocate memory areas for command table
            let mut mem_areas = [MemoryBlockInfo::new(); 16];
            for i in 0..16
            {
                let mem_area = PHYS_MEM_MANAGER.lock().alloc_single_mem_block();

                if mem_area == None
                {
                    log_error("Failed to initialize port memory spaces");
                    self.unlock_port_cmd(port_num);
                    return;
                }

                mem_areas[i] = mem_area.unwrap();
            }

            //set command headers
            for i in 0..32
            {
                let mut cmd_header = self.read_cmd_header(port_num, i);
                cmd_header.set_phys_region_desc_table_len(8); // 8 ptrd entry per command table

                let mut cmd_table_base_addr = mem_areas[i / 2].mem_block_start_addr;

                if i % 2 != 0
                {
                    cmd_table_base_addr += MEM_BLOCK_SIZE / 2;
                }

                cmd_header.set_cmd_table_desc_base_addr_low(cmd_table_base_addr);
                cmd_header.set_cmd_table_desc_base_addr_high(0);

                self.write_cmd_header(port_num, i, cmd_header);
            }
        }
        else
        {
            self.unlock_port_cmd(port_num);
            return;
        }

        // setup FIS struct memory area
        let mb_info = PHYS_MEM_MANAGER.lock().alloc_single_mem_block();
        if mb_info != None
        {
            port_ctrl_reg.fis_base_addr_low = mb_info.unwrap().mem_block_start_addr;
            port_ctrl_reg.fis_base_addr_high = 0;
        }
        else
        {
            println!("Failed to initialize port{} memory space", port_num);
        }

        self.write_port_ctrl_regs(port_num, port_ctrl_reg);
        self.unlock_port_cmd(port_num);

        println!("Port{} memory space initialized", port_num);
    }

    fn lock_port_cmd(&self, port_num: usize)
    {
        if port_num > MAX_PORT_COUNT - 1
        {
            return;
        }

        let mut port_ctrl_regs = self.read_port_ctrl_regs(port_num);

        // clear ST (bit0)
        port_ctrl_regs.cmd &= !PORT_CMD_ST_MASK;
        // clear FRE (bit4)
        port_ctrl_regs.cmd &= !PORT_CMD_FRE_MASK;
        self.write_port_ctrl_regs(port_num, port_ctrl_regs);

        //wait until FR (bit 14) and CR (bit15) are cleared
        loop
        {
            let port_ctrl_regs = self.read_port_ctrl_regs(port_num);
            if (port_ctrl_regs.cmd & PORT_CMD_FR_MASK) == 0 &&
               (port_ctrl_regs.cmd & PORT_CMD_CR_MASK) == 0
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

        // wait until CR (bit15) is cleared
        //while (port_ctrl_reg.cmd & PORT_CMD_CR_MASK) != 0 {};
        loop
        {
            let port_ctrl_regs = self.read_port_ctrl_regs(port_num);

            if (port_ctrl_regs.cmd & PORT_CMD_CR_MASK) == 0
            {
                break;
            }
        }

        let mut port_ctrl_regs = self.read_port_ctrl_regs(port_num);
        // set FRE (bit4)
        port_ctrl_regs.cmd |= PORT_CMD_FRE_MASK;
        // set ST (bit0)
        port_ctrl_regs.cmd |= PORT_CMD_ST_MASK;

        self.write_port_ctrl_regs(port_num, port_ctrl_regs);
    }

    pub fn is_init(&self) -> bool
    {
        return self.is_init;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
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

#[derive(Copy, Clone)]
#[repr(C)]
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

#[derive(Copy, Clone)]
#[bitfield]
#[repr(C)]
pub struct CommandHeader
{
    pub cmd_fis_len: B5,                        // command FIS length
    pub atapi: B1,                              // ATAPI
    pub write: B1,                              // write - 1: host to drive, 0: drive to host
    pub prefet: B1,                             // prefetchable

    pub reset: B1,                              // reset
    pub bist: B1,
    pub clear: B1,                              // clear busy upon R_OK
    reserved0: B1,
    pub port_multi_port: B4,

    pub phys_region_desc_table_len: u16,        // physical region descriptor table length

    pub phys_region_desc_byte_cnt: u32,         // physical region descriptor byte count transferred

    pub cmd_table_desc_base_addr_low: u32,      // command table descriptor base address
    pub cmd_table_desc_base_addr_high: u32,
    reserved1: u32,
    reserved2: u32,
    reserved3: u32,
    reserved4: u32
}

#[derive(Copy, Clone)]
#[bitfield]
#[repr(C)]
pub struct FISHostToDeviceRegisters
{
    // dw0
    pub fis_type: u8,
    pub port_multi: B4,
    reserved0: B3,
    pub c: B1,
    pub cmd: u8,
    pub feature_reg_low: u8,
    // dw1
    pub lba0: u8,
    pub lba1: u8,
    pub lba2: u8,
    pub device: u8,
    // dw2
    pub lba3: u8,
    pub lba4: u8,
    pub lba5: u8,
    pub feature_reg_high: u8,
    // dw3
    pub cnt_reg_low: u8,
    pub cnt_reg_high: u8,
    pub icc: u8,
    pub ctrl_reg: u8,
    // dw4
    reserved1: u32
}

#[derive(Copy, Clone)]
#[bitfield]
#[repr(C)]
pub struct FISDeviceToHostRegisters
{
    // dw0
    pub fis_type: u8,
    pub port_multi: B4,
    reserved0: B2,
    pub int: B1, // interrupt bit
    reserved1: B1,
    pub status: u8,
    pub error: u8,
    // dw1
    pub lba0: u8,
    pub lba1: u8,
    pub lba2: u8,
    pub device: u8,
    // dw2
    pub lba3: u8,
    pub lba4: u8,
    pub lba5: u8,
    reserved2: u8,
    // dw3
    pub cnt_reg_low: u8,
    pub cnt_reg_high: u8,
    reserved3: u16,
    // dw4
    reserved4: u32
}