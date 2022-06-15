use core::ptr::read_volatile;

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

        self.is_init = true;
    }

    pub fn is_init(&self) -> bool
    {
        return self.is_init;
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct HostBusAdapterMemoryRegisters
{
    generic_host_ctrl_reg: u32,
    vendor_specific_regs: u32,
    control_regs: [u32; MAX_PORT_COUNT]
}

impl HostBusAdapterMemoryRegisters
{
    pub fn new() -> HostBusAdapterMemoryRegisters
    {
        return HostBusAdapterMemoryRegisters { generic_host_ctrl_reg: 0, vendor_specific_regs: 0, control_regs: [0; MAX_PORT_COUNT] };
    }

    pub fn init(&mut self, base_addr: u32)
    {
        for i in 0..50
        {
            println!("0x{:x}", self.read_mem_reg(base_addr, i));
        }
    }

    fn read_mem_reg(&self, base_addr: u32, index: usize) -> u32
    {
        unsafe
        {
            let buffer = base_addr as *const u32;
            return read_volatile(buffer.offset(index as isize * 32));
        }
    }
}