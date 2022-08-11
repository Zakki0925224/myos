use core::{mem::size_of, ptr::read_volatile};

use alloc::{vec::Vec, string::{ToString, String}, format};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{util::logger::{log_info, log_warn, log_debug}, println, fs::fat::{file_allocation_table::ClusterType, dir_entery::{FileAttribute, PARENT_DIR_FILE_NAME, DirectoryEntry}}, mem::PHYS_MEM_MANAGER, print};

use super::fat::{FatVolume, dir_entery::EntryType};

pub const PATH_SEPARATOR: &str = "/";
pub const PARENT_DIR_PATH: &str = "../";

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

        if !self.is_init
        {
            return result;
        }

        let dir_entries_per_cluster = self.fat_volume.get_dir_entries_per_cluster();
        let max_cluster_num = self.fat_volume.get_dir_entries_max_num() / dir_entries_per_cluster;

        let mut file_name_buf = String::new();
        let cluster_chain_list = self.fat_volume.get_cluster_chain_list(start_cluster_num);

        for cluster_num in cluster_chain_list
        {
            // dir entries in a cluster
            for i in (cluster_num - 2) * dir_entries_per_cluster..(cluster_num - 2) * dir_entries_per_cluster + dir_entries_per_cluster
            {
                //println!("j: {}", j);
                let de = self.fat_volume.get_dir_entry(i).unwrap();
                let entry_type = de.entry_type();
                let file_attr = de.get_file_attr();

                if entry_type == EntryType::Null &&
                    file_attr == None
                {
                    continue;
                }

                if file_attr == Some(FileAttribute::LongFileName)
                {
                    let lfn = self.fat_volume.get_long_file_name_entry(i).unwrap();
                    file_name_buf = format!("{}{}", lfn.get_file_name(), file_name_buf);
                    continue;
                }

                if file_name_buf != ""
                {
                    result.push((file_name_buf.clone(), de.get_file_attr().unwrap(), de.get_first_cluster_num()));
                    file_name_buf.clear();
                }
            }
        }

        return result;
    }

    pub fn cat(&mut self, file_name: &str)
    {
        let mut read_cnt = 0;

        for file in self.scan(self.current_dir_cluster_num).iter().filter(|f| f.1 == FileAttribute::Archive)
        {
            if file_name == file.0
            {
                // file size is 0byte
                if file.2 == 0
                {
                    println!("This file is null");
                    return;
                }

                let mut base_addr_list = Vec::new();
                let mut size = 0;
                let cluster_chain_list = self.fat_volume.get_cluster_chain_list(file.2);

                for cluster_num in cluster_chain_list
                {
                    for i in 0..self.fat_volume.get_dir_entries_per_cluster()
                    {
                        let de = self.fat_volume.get_dir_entry((cluster_num - 2) * self.fat_volume.get_dir_entries_per_cluster() + i).unwrap();
                        //log_debug("file", &de);

                        if de.entry_type() != EntryType::Data
                        {
                            break;
                        }

                        if i == 0
                        {
                            let addr = self.fat_volume.get_dir_entry_base_addr((cluster_num - 2) * self.fat_volume.get_dir_entries_per_cluster() + i).unwrap();
                            base_addr_list.push(addr);
                        }

                        size += size_of::<DirectoryEntry>();
                    }
                }

                println!("base addrs: {:?}", base_addr_list);
                println!("size: {}B", size);

                for base_addr in base_addr_list
                {
                    let mut buf = Vec::new();

                    for i in 0..self.fat_volume.get_dir_entries_per_cluster() * size_of::<DirectoryEntry>()
                    {
                        unsafe
                        {
                            let ptr = (base_addr + i as u32) as *const u8;
                            let c = read_volatile(ptr) as char;
                            buf.push(c);
                        }
                    }

                    print!("{}", buf.into_iter().collect::<String>());
                }

                read_cnt += 1;
                continue;
            }
        }

        if read_cnt == 0
        {
            println!("File \"{}\" was not found in current directory", file_name);
        }
    }

    // fat32 only
    pub fn ls(&mut self)
    {
        if !self.is_init
        {
            return;
        }

        //let len = self.fat_volume.get_clusters_chain_length(5419);
        //println!("{:?}", len);
        //let c = self.fat_volume.get_next_cluster(5419);

        let current_dir = self.scan(self.current_dir_cluster_num);
        log_debug("current", &current_dir);

        for file in current_dir
        {
            println!("{}", file.0);
        }
    }

    pub fn cd(&mut self, dir_name: &str)
    {
        if !self.is_init
        {
            return;
        }

        if dir_name == PARENT_DIR_PATH
        {
            if self.current_dir_cluster_num == self.fat_volume.get_root_dir_cluster_num().unwrap()
            {
                return;
            }

            if let Some(de) = self.fat_volume.get_dir_entry((self.current_dir_cluster_num - 2) * self.fat_volume.get_dir_entries_per_cluster() + 1)
            {
                if de.get_file_short_name().as_str() == PARENT_DIR_FILE_NAME
                {
                    let cluster_num = de.get_first_cluster_num();

                    if cluster_num == 0
                    {
                        self.current_dir_cluster_num = self.fat_volume.get_root_dir_cluster_num().unwrap();
                    }
                    else
                    {
                        self.current_dir_cluster_num = de.get_first_cluster_num();
                    }

                    //println!("go to parent dir!");
                }
            }

            return;
        }

        let current_dir = self.scan(self.current_dir_cluster_num);

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