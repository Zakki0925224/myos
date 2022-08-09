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
    dir_cluster_list: Vec<(usize, String)>
}

impl VirtualFileSystem
{
    pub fn new() -> VirtualFileSystem
    {
        let fat = FatVolume::new(0, 0);
        return VirtualFileSystem { fat_volume: fat, is_init: false, dir_cluster_list: Vec::new() };
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

    // return is continuing claster
    fn scan(&mut self, start_cluster_num: usize) -> bool
    {
        if start_cluster_num < 2
        {
            return false;
        }

        let dir_entries_per_cluster = self.fat_volume.get_dir_entries_per_cluster();
        let max_cluster_num = self.fat_volume.get_dir_entries_max_num() / dir_entries_per_cluster;

        let i = start_cluster_num;
        let mut file_name_buf = String::new();

        if start_cluster_num == self.fat_volume.get_root_dir_cluster_num().unwrap()
        {
            self.dir_cluster_list.push((start_cluster_num, String::from(PATH_SEPARATOR)));
        }

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
                println!("\"{}\"(cn: {})(fcn: {})", file_name_buf, i, de.get_first_cluster_num());

                if file_attr == Some(FileAttribute::Directory)
                {
                    let parent_dir_name = &self.dir_cluster_list[self.dir_cluster_list.len() - 1].1;
                    self.dir_cluster_list.push((de.get_first_cluster_num(), file_name_buf.clone()));
                    println!("=={}(start)==", file_name_buf);
                    self.scan(de.get_first_cluster_num());
                    println!("=={}(end)==", file_name_buf);
                }

                file_name_buf.clear();
            }
        }

        // is skip after cluster?
        if let Some(next_cluster) = self.fat_volume.get_next_cluster(i)
        {
            //println!("cp: {:?}", next_cluster);
            match next_cluster
            {
                ClusterType::EndOfChain(_) => return false,
                _ => return true
            }
        }

        return false;
    }

    // fat32 only
    pub fn ls(&mut self)
    {
        if !self.is_init
        {
            return;
        }

        self.scan(2);
        //println!("{:?}", self.dir_cluster_list);

        // let mut c_num = self.fat_volume.get_root_dir_cluster_num().unwrap();
        // while c_num < /*self.fat_volume.get_dir_entries_max_num() / self.fat_volume.get_dir_entries_per_cluster()*/ 10
        // {
        //     //println!("c_num: {}", c_num);
        //     let result = self.scan(c_num);
        //     //println!("c_num: {}", result);
        //     c_num = result + 1;
        // }

        // let mut i = 0;
        // while i < self.fat_volume.get_dir_entries_max_num()
        // {
        //     let cluster_num = self.fat_volume.get_cluster_num_from_dir_entry_num(i);
        //     // skip this cluster?
        //     if i % self.fat_volume.get_dir_entries_per_cluster() == 0
        //     {
        //         if let Some(next_cluster) = self.fat_volume.get_next_cluster(cluster_num)
        //         {
        //             match next_cluster
        //             {
        //                 ClusterType::Data(_) => (),
        //                 ClusterType::EndOfChain(_) => (),
        //                 _ =>
        //                 {
        //                     i += self.fat_volume.get_dir_entries_per_cluster();
        //                     continue;
        //                 }
        //             }
        //         }
        //         else
        //         {
        //             i += self.fat_volume.get_dir_entries_per_cluster();
        //             continue;
        //         }
        //     }

        //     let de = self.fat_volume.get_dir_entry(i).unwrap();
        //     let entry_type = de.entry_type();
        //     let file_attr = de.get_file_attr();

        //     if entry_type == EntryType::Null &&
        //        file_attr == None
        //     {
        //         i += 1;
        //         continue;
        //     }

        //     if let Some(path) = self.get_full_path(i)
        //     {
        //         println!("path: \"{}\"", path);
        //     }

        //     if let Some(file_name) = self.fat_volume.get_file_name_from_dir_entry_num(i)
        //     {
        //         println!("{}(de_cnt: {})", file_name, i);
        //     }

        //     i += 1;
        // }
    }
}