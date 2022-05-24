use crate::{arch::asm, println};

const DEVICE_NOT_EXIST: u32 = 0xffffffff;

pub fn init()
{
    debug(0, 3, 0);
    // debug(0, 1, 0);
    // debug(0, 1, 1);
    // debug(0, 1, 3);
    // debug(0, 2, 0);
    // debug(0, 3, 0);
}

fn debug(bus: u8, device: u8, func: u8)
{
    let data0 = read_pci_config(bus, device, func, 0);
    let data1 = read_pci_config(bus, device, func, 4);
    let data2 = read_pci_config(bus, device, func, 8);
    let data3 = read_pci_config(bus, device, func, 16);
    let data4 = read_pci_config(bus, device, func, 32);
    let data5 = read_pci_config(bus, device, func, 64);
    let data6 = read_pci_config(bus, device, func, 128);
    let data7 = read_pci_config(bus, device, func, 256);
    let data8 = read_pci_config(bus, device, func, 512);
    let data9 = read_pci_config(bus, device, func, 1024);
    let data10 = read_pci_config(bus, device, func, 2048);
    let data11 = read_pci_config(bus, device, func, 4096);

    println!("Bus: {}, Device: {}, Function: {}", bus, device, func);

    if is_exist_pci_device(bus, device, func)
    {
        // device id(31-16), vendor id(15-0)
        println!("[D| |V| ]{:32b}", data0);
        // status(31-16), command(15-0)
        println!("[S| |C| ]{:32b}", data1);
        // class code(31-8), revision id(7-0)
        println!("[C| | |R]{:32b}", data2);
        // BIST(31-24), header type(23-16), lat. timer(15-8), cache line S.(7-0)
        println!("[B|H|L|C]{:32b}", data3);
        // base address registers(31-0)
        println!("[--BAR--]{:32b}", data4);
        println!("[--BAR--]{:32b}", data5);
        println!("[--BAR--]{:32b}", data6);
        println!("[--BAR--]{:32b}", data7);
        println!("[--BAR--]{:32b}", data8);
        println!("[--BAR--]{:32b}", data9);
        // cardbus CIS pointer(31-0)
        println!("[-CCISP-]{:32b}", data10);
        // subsystem id(31-16), subsystem vendor id(15-0)
        println!("[S| |SV-]{:32b}", data11);
    }
    else
    {
        println!("Device not found.");
    }
}

fn read_pci_config(bus: u8, device: u8, func: u8, offset: u32) -> u32
{
    // offset is a multiple of 4
    let addr = 0x80000000 | (bus as u32) << 16 | (device as u32) << 11 | (func as u32) << 8 | offset;
    asm::out32(0xcf8, addr);

    return asm::in32(0xcfc);
}

fn is_exist_pci_device(bus: u8, device: u8, func: u8) -> bool
{
    let data0 = read_pci_config(bus, device, func, 0);
    let data1 = read_pci_config(bus, device, func, 4);
    let data2 = read_pci_config(bus, device, func, 8);
    let data3 = read_pci_config(bus, device, func, 16);
    let data4 = read_pci_config(bus, device, func, 32);
    let data5 = read_pci_config(bus, device, func, 64);
    let data6 = read_pci_config(bus, device, func, 128);
    let data7 = read_pci_config(bus, device, func, 256);
    let data8 = read_pci_config(bus, device, func, 512);
    let data9 = read_pci_config(bus, device, func, 1024);
    let data10 = read_pci_config(bus, device, func, 2048);
    let data11 = read_pci_config(bus, device, func, 4096);

    if data0 != DEVICE_NOT_EXIST ||
       data1 != DEVICE_NOT_EXIST ||
       data2 != DEVICE_NOT_EXIST ||
       data3 != DEVICE_NOT_EXIST ||
       data4 != DEVICE_NOT_EXIST ||
       data5 != DEVICE_NOT_EXIST ||
       data6 != DEVICE_NOT_EXIST ||
       data7 != DEVICE_NOT_EXIST ||
       data8 != DEVICE_NOT_EXIST ||
       data9 != DEVICE_NOT_EXIST ||
       data10 != DEVICE_NOT_EXIST ||
       data11 != DEVICE_NOT_EXIST
    {
        return true;
    }
    else
    {
        return false;
    }
}