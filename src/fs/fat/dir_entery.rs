use core::{ptr::read_volatile, char::{decode_utf16, REPLACEMENT_CHARACTER}};

use alloc::{string::{String, ToString}, vec::Vec};
use modular_bitfield::{bitfield, prelude::*};

pub const CURRENT_DIR_FILE_NAME: &str = ".          ";
pub const PARENT_DIR_FILE_NAME: &str = "..         ";

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FileAttribute
{
    ReadOnly    = 0x01,
    Hidden      = 0x02,
    System      = 0x04,
    VolumeLabel = 0x08,
    LongFileName= 0x0f,
    Directory   = 0x10,
    Archive     = 0x20,
    Device      = 0x40
}

#[derive(Debug, PartialEq, Eq)]
pub enum EntryType
{
    Null,
    Unused,
    LongFileName,
    Data
}

#[bitfield]
#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct DirectoryEntry
{
    file_short_name: B88,
    file_attr: B8,
    win_nt_reserved: B8,
    create_time_ms: B8,
    create_time: B16,
    create_date: B16,
    last_access_time: B16,
    first_cluster_num_high: B16,
    last_modified_time: B16,
    last_modified_date: B16,
    first_cluster_num_low: B16,
    file_size: B32
}

impl DirectoryEntry
{
    pub fn read(base_addr: u32) -> DirectoryEntry
    {
        return unsafe { read_volatile(base_addr as *const DirectoryEntry) };
    }

    pub fn get_file_short_name(&self) -> String
    {
        let mut str_buf = Vec::new();

        for i in 0..11
        {
            str_buf.push((self.file_short_name() >> 8 * i) as u8 as char);
        }

        return str_buf.iter().collect();
    }

    pub fn get_file_attr(&self) -> Option<FileAttribute>
    {
        match self.file_attr()
        {
            0x01 => return Some(FileAttribute::ReadOnly),
            0x02 => return Some(FileAttribute::Hidden),
            0x04 => return Some(FileAttribute::System),
            0x08 => return Some(FileAttribute::VolumeLabel),
            0x0f => return Some(FileAttribute::LongFileName),
            0x10 => return Some(FileAttribute::Directory),
            0x20 => return Some(FileAttribute::Archive),
            0x40 => return Some(FileAttribute::Device),
            _ => return None
        }
    }

    pub fn get_first_cluster_num(&self) -> usize
    {
        let low = self.first_cluster_num_low() as usize;
        let high = (self.first_cluster_num_high() as usize) << 16;

        return high | low;
    }

    // return (year, month, date, hour, minutes, second)
    // pub fn get_create_time(&self) -> (u8, u8, u8, u8, u8, u8)
    // {
    //     let date = self.create_date();
    //     let time = self.create_time();
    //     let time_ms = self.create_time_ms();

    //     let y = 1980 + (date >> 8) as u8;
    //     let m = ((date & 0xf0) >> 4) as u8;
    //     let d = date as u8;
    //     let h = (time >> 8) as u8;
    //     let m = ((time & 0x) >> 4)

    //     return (y, m, d, time_ms);
    // }

    // pub fn get_last_mod_time(&self) -> (u8, u8, u8)
    // {
    //     let date = self.last_modified_date();
    //     let time = self.last_modified_time();

    //     let y = 1980 + (date >> 8) as u8;
    //     let m = ((date & 0xf0) >> 4) as u8;
    //     let d =
    // }

    pub fn entry_type(&self) -> EntryType
    {
        let first_byte = (self.file_short_name() >> 80) as u8;

        match first_byte
        {
            0x00 => return EntryType::Null,
            0xe5 => return EntryType::Unused,
            _ => ()
        }

        let last_byte = self.file_short_name() as u8;

        match last_byte
        {
            0x0f => return EntryType::LongFileName,
            _ => return EntryType::Data
        }
    }
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LongFileNameEntry
{
    sequence_num: B8,
    // filename (char 1~5, utf-16)
    file_name1: B80,
    // must be 0x0f (LongFileName)
    file_attr: B8,
    // must be 0
    lfn_type: B8,
    checksum: B8,
    // filename (char 6~11, utf-16)
    file_name2: B96,
    first_cluster_num_low: B16,
    // filename (char 12~13, utf-16)
    file_name3: B32
}

impl LongFileNameEntry
{
    pub fn read(base_addr: u32) -> LongFileNameEntry
    {
        return unsafe { read_volatile(base_addr as *const LongFileNameEntry) };
    }

    pub fn is_valid_entry(&self) -> bool
    {
        match self.get_file_attr()
        {
            Some(FileAttribute::LongFileName) => return true,
            _ => return false
        }
    }

    pub fn get_sequence_num(&self) -> u8
    {
        return self.sequence_num();
    }

    pub fn get_file_attr(&self) -> Option<FileAttribute>
    {
        match self.file_attr()
        {
            0x01 => return Some(FileAttribute::ReadOnly),
            0x02 => return Some(FileAttribute::Hidden),
            0x04 => return Some(FileAttribute::System),
            0x08 => return Some(FileAttribute::VolumeLabel),
            0x0f => return Some(FileAttribute::LongFileName),
            0x10 => return Some(FileAttribute::Directory),
            0x20 => return Some(FileAttribute::Archive),
            0x40 => return Some(FileAttribute::Device),
            _ => return None
        }
    }

    pub fn get_file_name(&self) -> String
    {
        let mut utf16_buf = Vec::new();

        // file_name1
        for i in 0..5
        {
            let c = (self.file_name1() >> i * 16) as u16;
            utf16_buf.push(c);
        }

        // file_name2
        for i in 0..6
        {
            let c = (self.file_name2() >> i * 16) as u16;
            utf16_buf.push(c);
        }

        // file_name3
        for i in 0..2
        {
            let c = (self.file_name3() >> i * 16) as u16;
            utf16_buf.push(c);
        }

        let string: String = decode_utf16(utf16_buf.iter().take_while(|&v| *v != 0).cloned())
            .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
            .collect();
        return string;
    }
}