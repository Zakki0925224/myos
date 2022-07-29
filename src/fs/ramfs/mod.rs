use core::{ptr::read_volatile, mem::size_of};

use bitfield;
use modular_bitfield::prelude::*;

use crate::{println, print};

#[bitfield(bits=880)]
#[derive(Debug)]
#[repr(C)]
struct CpioHeader
{
    magic_num: B48,
    file_inode_num: B64,
    file_mode: B64,
    file_uid: B64,
    file_gid: B64,
    link_num: B64,
    mod_time: B64,
    file_size: B64,
    major_file_device_num: B64,
    minor_file_device_num: B64,
    major_device_node_ref: B64,
    minor_device_node_ref: B64,
    file_name_len: B64,
    checksum: B64
}

#[derive(Debug)]
pub struct Ramfs
{
    start_base_addr: u32,
    end_base_addr: u32,
    size: u32
}

impl Ramfs
{
    pub fn new(start_base_addr: u32, end_base_addr: u32, size: u32) -> Ramfs
    {
        return Ramfs { start_base_addr, end_base_addr, size };
    }

    pub fn test(&self)
    {
        println!("0x{:x}-0x{:x}", self.start_base_addr, self.end_base_addr);
        println!("{}bytes", self.size);

        unsafe
        {
            let ptr = self.start_base_addr as *const CpioHeader;
            let header = read_volatile(ptr);
            println!("{:?}", header);
        }

        for i in self.start_base_addr..self.start_base_addr + 100 + size_of::<CpioHeader>() as u32
        {
            unsafe
            {
                let ptr = i as *const u8;
                print!("{}", read_volatile(ptr) as char);
            }
        }

        print!("\n");
    }
}