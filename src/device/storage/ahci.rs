use core::ptr::{read_volatile, write_volatile};

use modular_bitfield::{bitfield, prelude::*};

use crate::{util::logger::*, device::{pci::{PciDevice, BaseAddressRegister}, PCI}, println, mem::{PHYS_MEM_MANAGER, phys_mem::{MemoryBlockInfo, MEM_BLOCK_SIZE}}, print};

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
const PORT_CTRL_REGS_SIZE: u8 = 128;
const PRDT_SIZE: u8 = 16;
const CMD_TABLE_SIZE: u8 = 128 + PRDT_SIZE;
const CMD_HEADER_SIZE: u8 = 28;

const HBA_TO_PCR_OFFSET: u32 = 256;
const CMD_TABLE_TO_PRDT_OFFSET: u32 = 128;

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
        for i in 0..MAX_PORT_COUNT
        {
            let port_type = self.get_port_type(i);
            let is_impl_port = self.is_impl_port(i);
            if is_impl_port
            {
                println!("[AHCI]: port{} - {:?}", i, port_type);

                if let Some(_) = port_type
                {
                    self.init_port_mem_space(i);
                }
            }
        }

        self.is_init = true;
    }

    fn read_hba_mem_regs(&self) -> HostBusAdapterMemoryRegisters
    {
        unsafe
        {
            let ptr = self.hba_base_addr as *const HostBusAdapterMemoryRegisters;
            return read_volatile(ptr);
        }
    }

    fn write_hba_mem_regs(&self, hba_mem_regs: HostBusAdapterMemoryRegisters)
    {
        unsafe
        {
            let ptr = self.hba_base_addr as *mut HostBusAdapterMemoryRegisters;
            write_volatile(ptr, hba_mem_regs);
        }
    }

    fn read_port_ctrl_regs(&self, port_num: usize) -> Option<PortControlRegisters>
    {
        if !self.is_available_port_num(port_num)
        {
            return None;
        }

        let hba_base_addr = self.hba_base_addr;
        let offset = HBA_TO_PCR_OFFSET + (PORT_CTRL_REGS_SIZE as u32) * port_num as u32;

        unsafe
        {
            let ptr = (hba_base_addr + offset) as *const PortControlRegisters;
            return Some(read_volatile(ptr));
        }
    }

    fn write_port_ctrl_regs(&self, port_num: usize, port_ctrl_regs: PortControlRegisters)
    {
        if !self.is_available_port_num(port_num)
        {
            return;
        }

        let hba_base_addr = self.hba_base_addr;
        let offset = HBA_TO_PCR_OFFSET + (PORT_CTRL_REGS_SIZE as u32) * port_num as u32;

        unsafe
        {
            let ptr = (hba_base_addr + offset) as *mut PortControlRegisters;
            write_volatile(ptr, port_ctrl_regs);
        }
    }

    fn read_cmd_header(&self, port_num: usize, header_index: u32) -> Option<CommandHeader>
    {
        let port_ctrl_regs = self.read_port_ctrl_regs(port_num);

        if port_ctrl_regs == None
        {
            return None;
        }

        let addr = port_ctrl_regs.unwrap().cmd_list_base_addr_low + CMD_HEADER_SIZE as u32 * header_index;

        unsafe
        {
            let ptr = addr as *const CommandHeader;
            return Some(read_volatile(ptr));
        }
    }

    fn write_cmd_header(&self, port_num: usize, header_index: u32, cmd_header: CommandHeader)
    {
        let port_ctrl_regs = self.read_port_ctrl_regs(port_num);

        if port_ctrl_regs == None
        {
            return;
        }

        let addr = port_ctrl_regs.unwrap().cmd_list_base_addr_low + CMD_HEADER_SIZE as u32 * header_index;

        unsafe
        {
            let ptr = addr as *mut CommandHeader;
            write_volatile(ptr, cmd_header);
        }
    }

    fn read_cmd_table(&self, cmd_header: &CommandHeader) -> CommandTable
    {
        let base_addr = cmd_header.cmd_table_desc_base_addr_low();

        unsafe
        {
            let ptr = base_addr as *const CommandTable;
            return read_volatile(ptr);
        }
    }

    fn write_cmd_table(&self, cmd_header: &CommandHeader, cmd_table: CommandTable)
    {
        let base_addr = cmd_header.cmd_table_desc_base_addr_low();

        unsafe
        {
            let ptr = base_addr as *mut CommandTable;
            write_volatile(ptr, cmd_table);
        }
    }

    pub fn get_port_type(&self, port_num: usize) -> Option<PortType>
    {
        if !self.is_available_port_num(port_num)
        {
            return None;
        }

        let port_ctrl_regs = self.read_port_ctrl_regs(port_num).unwrap();

        let status = port_ctrl_regs.sata_status;
        let sig = port_ctrl_regs.sig;
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
            if self.is_impl_port(i)
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

    pub fn read(&self, port_num: usize, cnt: u16)
    {
        if !self.is_available_port_num(port_num)
        {
            return;
        }

        let mut port_ctrl_regs = self.read_port_ctrl_regs(port_num).unwrap();

        port_ctrl_regs.int_status = 0xffff; // clear interrupt bits
        self.write_port_ctrl_regs(port_num, port_ctrl_regs);
        let slot = self.find_cmd_slot(port_num);

        if slot == None
        {
            return;
        }

        let mut cmd_header = self.read_cmd_header(port_num, slot.unwrap()).unwrap();

        // TODO: why cmd header is all 0?
        cmd_header.set_cmd_fis_len(FIS_H2D_REGS_SIZE / 4);
        cmd_header.set_write(0);
        cmd_header.set_phys_region_desc_table_len(((cnt - 1) >> 4) + 1);
        self.write_cmd_header(port_num, slot.unwrap(), cmd_header);

        let mut cmd_header = self.read_cmd_header(port_num, slot.unwrap()).unwrap();

        println!("{:?}", cmd_header);

        let base_addr = cmd_header.cmd_table_desc_base_addr_low();
        let size = CMD_TABLE_SIZE as u32 + (cmd_header.phys_region_desc_table_len() as u32 - 1) * PRDT_SIZE as u32;
        PHYS_MEM_MANAGER.lock().memset(base_addr, size, 0);

        let mut cmd_table = self.read_cmd_table(&cmd_header);

        println!("{:?}", cmd_table);

    }

    fn find_cmd_slot(&self, port_num: usize) -> Option<u32>
    {
        if !self.is_available_port_num(port_num)
        {
            return None;
        }

        let port_ctrl_regs = self.read_port_ctrl_regs(port_num).unwrap();
        let mut slots = port_ctrl_regs.sata_active | port_ctrl_regs.cmd_issue;

        for i in 0..MAX_PORT_COUNT as u32
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

            if !self.is_impl_port(i) || port_type == None
            {
                continue;
            }

            println!("Port{} type: {:?}", i, port_type.unwrap());
        }
    }

    fn is_impl_port(&self, port_num: usize) -> bool
    {
        if !self.is_available_port_num(port_num)
        {
            return false;
        }

        return ((self.read_hba_mem_regs().port_impl >> port_num) & 0x1) != 0;
    }

    fn init_port_mem_space(&self, port_num: usize)
    {
        if !self.is_available_port_num(port_num)
        {
            return;
        }

        self.lock_port_cmd(port_num);

        // setup command list memory area
        let mb_info = PHYS_MEM_MANAGER.lock().alloc_single_mem_block();
        let mut port_ctrl_regs = self.read_port_ctrl_regs(port_num).unwrap();

        if mb_info != None
        {
            port_ctrl_regs.cmd_list_base_addr_low = mb_info.unwrap().mem_block_start_addr;
            port_ctrl_regs.cmd_list_base_addr_high = 0;

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
            for i in 0..MAX_PORT_COUNT
            {
                let mut cmd_header = self.read_cmd_header(port_num, i as u32).unwrap();
                cmd_header.set_phys_region_desc_table_len(8); // 8 ptrd entry per command table

                let mut cmd_table_base_addr = mem_areas[i / 2].mem_block_start_addr;

                if i % 2 != 0
                {
                    cmd_table_base_addr += MEM_BLOCK_SIZE / 2;
                }

                cmd_header.set_cmd_table_desc_base_addr_low(cmd_table_base_addr);
                cmd_header.set_cmd_table_desc_base_addr_high(0);

                self.write_cmd_header(port_num, i as u32, cmd_header);
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
            port_ctrl_regs.fis_base_addr_low = mb_info.unwrap().mem_block_start_addr;
            port_ctrl_regs.fis_base_addr_high = 0;
        }
        else
        {
            println!("Failed to initialize port{} memory space", port_num);
        }

        self.write_port_ctrl_regs(port_num, port_ctrl_regs);
        self.unlock_port_cmd(port_num);

        println!("[AHCI]: Port{} memory space initialized", port_num);
    }

    fn lock_port_cmd(&self, port_num: usize)
    {
        if !self.is_available_port_num(port_num)
        {
            return;
        }

        let mut port_ctrl_regs = self.read_port_ctrl_regs(port_num).unwrap();

        // clear ST (bit0)
        port_ctrl_regs.cmd &= !PORT_CMD_ST_MASK;
        // clear FRE (bit4)
        port_ctrl_regs.cmd &= !PORT_CMD_FRE_MASK;
        self.write_port_ctrl_regs(port_num, port_ctrl_regs);

        //wait until FR (bit 14) and CR (bit15) are cleared
        loop
        {
            let port_ctrl_regs = self.read_port_ctrl_regs(port_num).unwrap();
            if (port_ctrl_regs.cmd & PORT_CMD_FR_MASK) == 0 &&
               (port_ctrl_regs.cmd & PORT_CMD_CR_MASK) == 0
            {
                break;
            }
        }
    }

    fn unlock_port_cmd(&self, port_num: usize)
    {
        if !self.is_available_port_num(port_num)
        {
            return;
        }

        // wait until CR (bit15) is cleared
        loop
        {
            let port_ctrl_regs = self.read_port_ctrl_regs(port_num).unwrap();

            if (port_ctrl_regs.cmd & PORT_CMD_CR_MASK) == 0
            {
                break;
            }
        }

        let mut port_ctrl_regs = self.read_port_ctrl_regs(port_num).unwrap();
        // set FRE (bit4)
        port_ctrl_regs.cmd |= PORT_CMD_FRE_MASK;
        // set ST (bit0)
        port_ctrl_regs.cmd |= PORT_CMD_ST_MASK;

        self.write_port_ctrl_regs(port_num, port_ctrl_regs);
    }

    fn is_available_port_num(&self, port_num: usize) -> bool
    {
        return port_num < MAX_PORT_COUNT;
    }

    pub fn is_init(&self) -> bool
    {
        return self.is_init;
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct HostBusAdapterMemoryRegisters
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
    pub vendor_spec_regs: [u32; 24]
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
struct PortControlRegisters
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

#[bitfield]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
struct CommandHeader
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

    pub phys_region_desc_table_len: B16,        // physical region descriptor table length

    pub phys_region_desc_byte_cnt: B32,         // physical region descriptor byte count transferred

    pub cmd_table_desc_base_addr_low: B32,      // command table descriptor base address
    pub cmd_table_desc_base_addr_high: B32,
    reserved1: B32,
    reserved2: B32,
    reserved3: B32,
    reserved4: B32
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct CommandTable
{
    pub cmd_fis: [u8; 64],
    pub atapi_cmd: [u8; 16],
    reserved: [u8; 48]
}

#[bitfield]
#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct PhysicalRegionDescriptorTable
{
    pub data_base_addr_low: B32,
    pub data_base_addr_high: B32,
    reserved0: B32,
    pub byte_cnt: B22,
    reserved1: B9,
    pub int_on_comp: B1
}

#[bitfield]
#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct FISHostToDeviceRegisters
{
    // dw0
    pub fis_type: B8,
    pub port_multi: B4,
    reserved0: B3,
    pub c: B1,
    pub cmd: B8,
    pub feature_reg_low: B8,
    // dw1
    pub lba0: B8,
    pub lba1: B8,
    pub lba2: B8,
    pub device: B8,
    // dw2
    pub lba3: B8,
    pub lba4: B8,
    pub lba5: B8,
    pub feature_reg_high: B8,
    // dw3
    pub cnt_reg_low: B8,
    pub cnt_reg_high: B8,
    pub icc: B8,
    pub ctrl_reg: B8,
    // dw4
    reserved1: B32
}

#[bitfield]
#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct FISDeviceToHostRegisters
{
    // dw0
    pub fis_type: B8,
    pub port_multi: B4,
    reserved0: B2,
    pub int: B1, // interrupt bit
    reserved1: B1,
    pub status: B8,
    pub error: B8,
    // dw1
    pub lba0: B8,
    pub lba1: B8,
    pub lba2: B8,
    pub device: B8,
    // dw2
    pub lba3: B8,
    pub lba4: B8,
    pub lba5: B8,
    reserved2: B8,
    // dw3
    pub cnt_reg_low: B8,
    pub cnt_reg_high: B8,
    reserved3: B16,
    // dw4
    reserved4: B32
}