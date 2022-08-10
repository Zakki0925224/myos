use core::{ptr::read_volatile, mem::size_of};

use alloc::{vec::Vec, string::{String, ToString}};

use crate::{print, println, fs::fat::{fs_info_sector::FsInfoSector, dir_entery::{DirectoryEntry, FileAttribute, EntryType}, boot_sector::FATType}};

use self::{boot_sector::BootSector, file_allocation_table::ClusterType, dir_entery::LongFileNameEntry};

pub mod boot_sector;
pub mod fs_info_sector;
pub mod dir_entery;
pub mod file_allocation_table;

pub struct FatVolume
{
    start_base_addr: u32,
    end_base_addr: u32,
    boot_sector: BootSector,
    is_init: bool
}

impl FatVolume
{
    pub fn new(start_base_addr: u32, end_base_addr: u32) -> FatVolume
    {
        return FatVolume
        {
            start_base_addr,
            end_base_addr,
            boot_sector: BootSector::new(),
            is_init: false
        };
    }

    pub fn init(&mut self)
    {
        let bs = BootSector::read(self.start_base_addr);

        if bs.get_volume_id() != 0
        {
            self.is_init = true;
        }

        self.boot_sector = bs;
    }

    pub fn is_init(&self) -> bool
    {
        return self.is_init;
    }

    pub fn get_fat_type(&self) -> FATType
    {
        return self.boot_sector.fat_type();
    }

    pub fn get_dir_entries_per_cluster(&self) -> usize
    {
        return self.boot_sector.get_cluster_size() * self.boot_sector.get_sector_size() / size_of::<DirectoryEntry>();
    }

    pub fn get_dir_entries_max_num(&self) -> usize
    {
        let cluster_num = self.boot_sector.data_area_sectors_cnt() / self.boot_sector.get_cluster_size();
        return cluster_num * self.get_dir_entries_per_cluster();
    }

    pub fn get_cluster_num_from_dir_entry_num(&self, dir_entry_num: usize) -> usize
    {
        return dir_entry_num / self.get_dir_entries_per_cluster() + 2;
    }

    pub fn get_fs_info_sector(&self) -> Option<FsInfoSector>
    {
        if self.boot_sector.fat_type() != FATType::FAT32
        {
            return None;
        }

        let base_addr = self.start_base_addr + (self.boot_sector.get_fat32_fs_info_sector_num() * self.boot_sector.get_sector_size()) as u32;
        return Some(FsInfoSector::read(base_addr));
    }

    pub fn get_root_dir_cluster_num(&self) -> Option<usize>
    {
        // TODO: remove option
        if self.get_fat_type() == FATType::FAT32
        {
            return Some(self.boot_sector.get_fat32_root_dir_cluster_num())
        }
        else
        {
            return None;
        }
    }

    pub fn get_root_dir_entry(&self) -> DirectoryEntry
    {
        if self.get_fat_type() == FATType::FAT32
        {
            let entries_per_cluster = self.get_dir_entries_per_cluster();
            let entry_num = (self.get_root_dir_cluster_num().unwrap() - 2) * entries_per_cluster;
            return self.get_dir_entry(entry_num).unwrap();
        }

        let base_addr = self.start_base_addr + (self.boot_sector.root_dir_area_start_sector_num() * self.boot_sector.get_sector_size()) as u32;
        return DirectoryEntry::read(base_addr);
    }

    pub fn get_dir_entry(&self, entry_num: usize) -> Option<DirectoryEntry>
    {
        if let Some(base_addr) = self.get_dir_entry_base_addr(entry_num)
        {
            return Some(DirectoryEntry::read(base_addr));
        }

        return None;
    }

    pub fn get_dir_entry_base_addr(&self, entry_num: usize) -> Option<u32>
    {
        if entry_num > self.get_dir_entries_max_num()
        {
            return None;
        }

        let entries_per_cluster = self.get_dir_entries_per_cluster();

        let data_area_start_offset = self.boot_sector.data_area_start_sector_num() * self.boot_sector.get_sector_size();
        let offset = data_area_start_offset + (entry_num * size_of::<DirectoryEntry>());
        let base_addr = self.start_base_addr + offset as u32;

        return Some(base_addr);
    }

    pub fn get_long_file_name_entry(&self, entry_num: usize) -> Option<LongFileNameEntry>
    {
        if let Some(base_addr) = self.get_dir_entry_base_addr(entry_num)
        {
            let entry = LongFileNameEntry::read(base_addr);

            match entry.is_valid_entry()
            {
                true => return Some(entry),
                false => return None,
            }
        }
        else
        {
            return None;
        }
    }

    // TODO
    // pub fn get_clusters_chain_length(&self, start_cluster_num: usize) -> usize
    // {
    //     if start_cluster_num < 2
    //     {
    //         return 0;
    //     }

    //     let fat_clusters_cnt = self.boot_sector.data_area_sectors_cnt() / self.boot_sector.get_cluster_size() - 2;

    //     if fat_clusters_cnt < start_cluster_num
    //     {
    //         return 0;
    //     }

    //     let mut i = start_cluster_num;

    //     while i < fat_clusters_cnt - start_cluster_num
    //     {
    //         if let Some(cluster_type) = self.get_next_cluster(i)
    //         {
    //             match cluster_type
    //             {
    //                 ClusterType::EndOfChain(_) => return i - start_cluster_num + 1,
    //                 _ => ()
    //             }
    //         }
    //         else
    //         {
    //             return 0;
    //         }

    //         i += 1;
    //     }

    //     return i;
    // }

    pub fn get_next_cluster(&self, cluster_num: usize) -> Option<ClusterType>
    {
        let fat_clusters_cnt = self.boot_sector.fat_area_sectors_cnt() / self.boot_sector.get_cluster_size();

        if fat_clusters_cnt < cluster_num
        {
            return None;
        }

        let fat_start_sector = self.boot_sector.fat_area_start_sector_num();
        let fat_start_base_addr = self.start_base_addr + (fat_start_sector * self.boot_sector.get_sector_size()) as u32;
        return Some(file_allocation_table::get_next_cluster_num(fat_start_base_addr, self.get_fat_type(), cluster_num));
    }
}