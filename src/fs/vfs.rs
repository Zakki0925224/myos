use alloc::{vec::Vec, string::{ToString, String}, format};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{util::logger::{log_info, log_warn}, println, fs::fat::{file_allocation_table::ClusterType, dir_entery::FileAttribute}, print};

use super::fat::{FatVolume, dir_entery::EntryType};

const LFN_MAX: usize = 255;
const LFN_EMPTY_CHAR: char = '?';

lazy_static!
{
    pub static ref VFS: Mutex<VirtualFileSystem> = Mutex::new(VirtualFileSystem::new());
}

pub struct VirtualFileSystem
{
    fat_volume: FatVolume,
    is_init: bool
}

impl VirtualFileSystem
{
    pub fn new() -> VirtualFileSystem
    {
        let fat = FatVolume::new(0, 0);
        return VirtualFileSystem { fat_volume: fat, is_init: false };
    }

    pub fn init(&mut self, start_base_addr: u32, end_base_addr: u32)
    {
        self.fat_volume = FatVolume::new(start_base_addr, end_base_addr);
        self.fat_volume.init();
        self.is_init = self.fat_volume.is_init();

        if self.is_init
        {
            log_info("VFS initialized");
        }
        else
        {
            log_warn("Failed to initialize VFS");
        }
    }

    pub fn ls(&self)
    {
        if !self.is_init
        {
            return;
        }

        let mut i = 0;
        let dir_entries_per_cluster = self.fat_volume.get_dir_entries_per_cluster();
        let mut long_file_name_buf = [LFN_EMPTY_CHAR; LFN_MAX];
        let mut buf_cnt = LFN_MAX - 1;
        while i < self.fat_volume.get_dir_entries_max_num()
        {
            let de = self.fat_volume.get_dir_entry(i).unwrap();
            let entry_type = de.entry_type();
            let file_attr = de.get_file_attr();

            if entry_type == EntryType::Null &&
               file_attr == None
            {
                i += 1;
                continue;
            }

            if file_attr == Some(FileAttribute::LongFileName)
            {
                let lfn_entry = self.fat_volume.get_long_file_name_entry(i).unwrap();
                let file_name: Vec<char> = lfn_entry.get_file_name().chars().collect();

                for i in (0..file_name.len()).rev()
                {
                    long_file_name_buf[buf_cnt] = file_name[i];

                    buf_cnt -= 1;

                    if buf_cnt == 0
                    {
                        break;
                    }
                }

                i += 1;
                continue;
            }
            else if (file_attr == Some(FileAttribute::Archive) ||
                    file_attr == Some(FileAttribute::Directory)) &&
                    buf_cnt != 0
            {
                let mut str_buf = String::new();

                for i in 0..LFN_MAX
                {
                    if long_file_name_buf[i] != LFN_EMPTY_CHAR
                    {
                        str_buf.push(long_file_name_buf[i]);
                    }
                }

                long_file_name_buf = [LFN_EMPTY_CHAR; LFN_MAX];
                buf_cnt = LFN_MAX - 1;

                if str_buf == ""
                {
                    i += 1;
                    continue;
                }

                println!("[{}/{}]{} ({:?})", i, self.fat_volume.get_dir_entries_max_num(), str_buf, file_attr.unwrap());
            }

            i += 1;
        }
    }
}