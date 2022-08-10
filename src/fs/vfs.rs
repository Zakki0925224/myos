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
    is_init: bool,
    current_dir_cluster_num: usize
}

impl VirtualFileSystem
{
    pub fn new() -> VirtualFileSystem
    {
        let fat = FatVolume::new(0, 0);
        return VirtualFileSystem { fat_volume: fat, is_init: false, current_dir_cluster_num: 2 };
    }

    pub fn init(&mut self, start_base_addr: u32, end_base_addr: u32)
    {
        self.fat_volume = FatVolume::new(start_base_addr, end_base_addr);
        self.fat_volume.init();
        self.current_dir_cluster_num = self.fat_volume.get_root_dir_cluster_num().unwrap();
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

    // return Vec<(filename, file attribute, pointing cluster num)>
    fn scan(&mut self, start_cluster_num: usize) -> Vec<(String, FileAttribute, usize)>
    {
        let mut result = Vec::new();

        if start_cluster_num < 2
        {
            return result;
        }

        let dir_entries_per_cluster = self.fat_volume.get_dir_entries_per_cluster();
        let max_cluster_num = self.fat_volume.get_dir_entries_max_num() / dir_entries_per_cluster;

        let i = start_cluster_num;
        let mut file_name_buf = String::new();

        // if start_cluster_num == self.fat_volume.get_root_dir_cluster_num().unwrap()
        // {
        //     self.dir_cluster_list.push((start_cluster_num, String::from(PATH_SEPARATOR)));
        // }

        // dir entries in a cluster
        for j in (i - 2) * dir_entries_per_cluster..(i - 2) * dir_entries_per_cluster + dir_entries_per_cluster
        {
            //println!("j: {}", j);
            let de = self.fat_volume.get_dir_entry(j).unwrap();
            let entry_type = de.entry_type();
            let file_attr = de.get_file_attr();

            if entry_type == EntryType::Null &&
                file_attr == None
            {
                continue;
            }

            if file_attr == Some(FileAttribute::LongFileName)
            {
                let lfn = self.fat_volume.get_long_file_name_entry(j).unwrap();
                file_name_buf = format!("{}{}", lfn.get_file_name(), file_name_buf);
                continue;
            }

            if file_name_buf != ""
            {
                //println!("\"{}\"(cn: {})(fcn: {})", file_name_buf, i, de.get_first_cluster_num());

                // if file_attr == Some(FileAttribute::Directory)
                // {
                //     let parent_dir_name = &self.dir_cluster_list[self.dir_cluster_list.len() - 1].1;
                //     self.dir_cluster_list.push((de.get_first_cluster_num(), file_name_buf.clone()));
                //     println!("=={}(start)==", file_name_buf);
                //     //self.scan(de.get_first_cluster_num());
                //     println!("=={}(end)==", file_name_buf);
                // }

                result.push((file_name_buf.clone(), de.get_file_attr().unwrap(), de.get_first_cluster_num()));
                file_name_buf.clear();
            }
        }

        // is skip after cluster?
        // if let Some(next_cluster) = self.fat_volume.get_next_cluster(i)
        // {
        //     //println!("cp: {:?}", next_cluster);
        //     match next_cluster
        //     {
        //         ClusterType::EndOfChain(_) => return false,
        //         _ => return true
        //     }
        // }

        return result;
    }

    // fat32 only
    pub fn ls(&mut self)
    {
        if !self.is_init
        {
            return;
        }

        let a = self.scan(self.current_dir_cluster_num);
        println!("{:?}", a);
    }

    pub fn cd(&mut self, dir_name: &str)
    {
        if !self.is_init
        {
            return;
        }

        let current_dir = self.scan(self.current_dir_cluster_num);
        println!("{:?}", current_dir);

        for file in current_dir
        {
            if file.0 == dir_name &&
               file.1 == FileAttribute::Directory
            {
                self.current_dir_cluster_num = file.2;
                return;
            }
        }

        println!("Directory \"{}\" was not found in current directory", dir_name);
    }
}