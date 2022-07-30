use core::{ptr::read_volatile, mem::size_of, intrinsics::transmute};

use alloc::{vec::Vec, string::{String, ToString}};
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

impl CpioHeader
{
    pub fn is_valid_header(&self) -> bool
    {
        let magic_num_str = self.get_magic_num_str();

        match &*magic_num_str
        {
            "070701" => return true,
            "070702" => return true,
            _ => return false
        }
    }

    pub fn get_magic_num_str(&self) -> String
    {
        return self.get_str(self.magic_num());
    }

    pub fn get_file_inode_num_str(&self) -> String
    {
        return self.get_str(self.file_inode_num());
    }

    pub fn get_file_mode_str(&self) -> String
    {
        return self.get_str(self.file_mode());
    }

    pub fn get_file_uid_str(&self) -> String
    {
        return self.get_str(self.file_uid());
    }

    pub fn get_file_gid_str(&self) -> String
    {
        return self.get_str(self.file_gid());
    }

    pub fn get_link_num_str(&self) -> String
    {
        return self.get_str(self.link_num());
    }

    pub fn get_mod_time_str(&self) -> String
    {
        return self.get_str(self.mod_time());
    }

    pub fn get_file_size(&self) -> u32
    {
        let file_size_str = self.get_str(self.file_size());
        return u32::from_str_radix(&file_size_str, 16).unwrap();
    }

    pub fn get_major_file_device_num_str(&self) -> String
    {
        return self.get_str(self.major_file_device_num());
    }

    pub fn get_minor_file_device_num_str(&self) -> String
    {
        return self.get_str(self.minor_file_device_num());
    }

    pub fn get_major_device_node_ref_str(&self) -> String
    {
        return self.get_str(self.major_device_node_ref());
    }

    pub fn get_minor_device_node_fef_str(&self) -> String
    {
        return self.get_str(self.minor_device_node_ref());
    }

    pub fn get_file_name_len_str(&self) -> String
    {
        return self.get_str(self.file_name_len());
    }

    pub fn get_checksum_str(&self) -> String
    {
        return self.get_str(self.checksum());
    }

    fn get_str(&self, bytes: u64) -> String
    {
        let mut s = String::from_utf8(bytes.to_le_bytes().to_vec()).unwrap();
        s.retain(|c| c != '\0'); // remove '\0' from string
        return s;
    }
}

#[derive(Debug)]
struct FileInfo
{
    pub header: CpioHeader,
    pub file_name: String,
    pub data_start_base_addr: u32,
    pub data_end_base_addr: u32
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

        let mut base_addr = self.start_base_addr;

        let header = self.get_cpio_header(base_addr);
        println!("{} bytes", header.get_file_size());

        // unsafe
        // {
        //     let ptr = self.start_base_addr as *const CpioHeader;
        //     let header = read_volatile(ptr);
        //     println!("{:?}", header);
        //     println!("Magic num: {}", header.get_magic_num_str());
        //     println!("File size: {}", header.get_file_size_str());
        //     println!("File uid: {}", header.get_file_uid_str());
        //     println!("Mod time: {}", header.get_mod_time_str());
        //     println!("File name len: {}", header.get_file_name_len_str());
        //     println!("Is valid header?: {}", header.is_valid_header());
        // }

        // for i in self.start_base_addr..self.start_base_addr + 100 + size_of::<CpioHeader>() as u32
        // {
        //     unsafe
        //     {
        //         let ptr = i as *const u8;
        //         print!("{}", read_volatile(ptr) as char);
        //     }
        // }

        // print!("\n");
    }

    fn get_cpio_header(&self, base_addr: u32) -> CpioHeader
    {
        unsafe
        {
            let ptr = base_addr as *const CpioHeader;
            return read_volatile(ptr);
        }
    }
}