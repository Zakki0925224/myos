use alloc::{vec::Vec, string::{ToString, String}, format};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{util::logger::{log_info, log_warn}, println, fs::fat::{file_allocation_table::ClusterType, dir_entery::FileAttribute}, print};

use super::fat::{FatVolume, dir_entery::EntryType};

const PATH_SEPARATOR: &str = "/";

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

    fn get_full_path(&self, dir_entry_num: usize) -> Option<String>
    {
        if !self.is_init
        {
            return None;
        }

        if let Some(file_name) = self.fat_volume.get_file_name_from_dir_entry_num(dir_entry_num)
        {
            let cluster_num = self.fat_volume.get_cluster_num_from_dir_entry_num(dir_entry_num);
            let root_dir_cluster_num = self.fat_volume.get_root_dir_cluster_num();
            let dirs_dir_entry = self.fat_volume.get_dirs_dir_entry(dir_entry_num);

            if root_dir_cluster_num == None
            {
                return None;
            }

            if cluster_num == root_dir_cluster_num.unwrap()
            {
                let mut str_buf = String::from(PATH_SEPARATOR);
                //println!("file name: {:p} \"{}\"", file_name.as_ptr(), file_name);
                //println!("str buf: {:p} \"{}\"", str_buf.as_ptr(), str_buf);
                str_buf += file_name.as_str();

                return Some(str_buf);
            }
            else
            {
                if let Some((_, parent)) = dirs_dir_entry
                {
                    let name = self.fat_volume.get_file_name_from_dir_entry_num(parent.get_first_cluster_num());
                    println!("parent: {:?}", name);
                }
            }
        }

        return None;
    }

    // fat32 only
    pub fn ls(&self)
    {
        if !self.is_init
        {
            return;
        }

        let mut i = 0;
        while i < self.fat_volume.get_dir_entries_max_num()
        {
            let cluster_num = self.fat_volume.get_cluster_num_from_dir_entry_num(i);
            // skip this cluster?
            if i % self.fat_volume.get_dir_entries_per_cluster() == 0
            {
                if let Some(next_cluster) = self.fat_volume.get_next_cluster(cluster_num)
                {
                    match next_cluster
                    {
                        ClusterType::Data(_) => (),
                        ClusterType::EndOfChain(_) => (),
                        _ =>
                        {
                            i += self.fat_volume.get_dir_entries_per_cluster();
                            continue;
                        }
                    }
                }
                else
                {
                    i += self.fat_volume.get_dir_entries_per_cluster();
                    continue;
                }
            }

            let de = self.fat_volume.get_dir_entry(i).unwrap();
            let entry_type = de.entry_type();
            let file_attr = de.get_file_attr();

            if entry_type == EntryType::Null &&
               file_attr == None
            {
                i += 1;
                continue;
            }

            if let Some(path) = self.get_full_path(i)
            {
                println!("path: \"{}\"", path);
            }

            if let Some(file_name) = self.fat_volume.get_file_name_from_dir_entry_num(i)
            {
                println!("{}(cn: {})", file_name, cluster_num);
            }

            i += 1;
        }
    }
}