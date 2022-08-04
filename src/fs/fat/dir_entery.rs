use alloc::{string::String, vec::Vec};
use modular_bitfield::{bitfield, prelude::*};

#[derive(Debug)]
#[repr(u8)]
pub enum FileAttribute
{
    ReadOnly    = 0x01,
    Hidden      = 0x02,
    System      = 0x04,
    VolumeLabel = 0x08,
    Directory   = 0x10,
    Archive     = 0x20,
    Device      = 0x40
}

#[bitfield]
#[derive(Debug)]
#[repr(C)]
pub struct DirectoryEntry
{
    file_short_name: B64,
    file_ex: B24,
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
    pub fn get_file_short_name(&self) -> String
    {
        let mut str_buf = Vec::new();

        for i in 0..8
        {
            str_buf.push((self.file_short_name() >> 8 * i) as u8 as char);
        }

        return str_buf.iter().collect();
    }

    pub fn get_file_ex(&self) -> String
    {
        let mut str_buf = Vec::new();

        for i in 0..3
        {
            str_buf.push((self.file_ex() >> 8 * i) as u8 as char);
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
            0x10 => return Some(FileAttribute::Directory),
            0x20 => return Some(FileAttribute::Archive),
            0x40 => return Some(FileAttribute::Device),
            _ => return None
        }
    }
}