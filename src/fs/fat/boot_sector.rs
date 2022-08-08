use core::ptr::read_volatile;
use alloc::{string::String, vec::Vec};
use modular_bitfield::{bitfield, prelude::*};

use crate::println;

#[derive(Debug, PartialEq, Eq)]
pub enum FATType
{
    FAT12,
    FAT16,
    FAT32
}

#[bitfield]
#[derive(Debug)]
#[repr(C)]
pub struct BootSector
{
    jmp_instr: B24,
    oem_name: B64,

    // BIOS paramater block
    // sector size (512B | 1024B | 2048B | 4096B)
    bpb_bytes_per_sector: B16,
    // cluster size (1 | 2 | 4 | 8 | ... | 128)
    bpb_sectors_per_cluster: B8,
    // count of reserved logical sectors (must not be 0)
    bpb_reserved_sectors_cnt: B16,
    // number of FAT copies (must be 2)
    bpb_num_fats: B8,
    // number of directory entries in the root directory (unused for FAT32)
    bpb_root_entries_cnt: B16,
    // total number of sectors for FAT12/16 (0 for FAT32)
    bpb_total_sectors: B16,
    // media type
    bpb_media_type: B8,
    // FAT size in sectors for FAT12/FAT16 (0 for FAT32)
    bpb_fat_size: B16,
    // track size in sectors
    bpb_sectors_per_track: B16,
    // number of heads
    bpb_num_heads: B16,
    // number of hidden sectors before this volume
    bpb_num_hidden_sectors: B32,

    // FAT32 only fields
    // Total number of sectors
    bpb_fat32_total_sectors: B32,
    // FAT size in sectors
    bpb_fat32_fat_size: B32,
    bpb_fat32_ext_flags: B16,
    bpb_fat32_fs_ver: B16,
    // claster number of the root directory
    bpb_fat32_num_root_clasters: B32,
    // sector number of the FSINFO (must be 1)
    bpb_fat32_fs_info_sector_num: B16,
    // sector number of boot sector backup
    bpb_fat32_boot_sector_backup_sector_num: B16,
    bpb_fat32_reserved: B96,

    drive_number: B8,
    reserved: B8,
    boot_sig: B8,
    volume_id: B32,
    volume_label: B88,
    fs_type_name: B64
}

impl BootSector
{
    pub fn read(base_addr: u32) -> BootSector
    {
        return unsafe { read_volatile(base_addr as *const BootSector) };
    }

    pub fn get_oem_name(&self) -> String
    {
        let mut char_buf = Vec::new();

        for  i in 0..8
        {
            char_buf.push((self.oem_name() >> 8 * i) as u8 as char);
        }

        return char_buf.iter().collect();
    }

    pub fn get_volume_label(&self) -> String
    {
        let mut char_buf = Vec::new();

        for i in 0..11
        {
            char_buf.push((self.volume_label() >> 8 * i) as u8 as char);
        }

        return char_buf.iter().collect();
    }

    pub fn get_fs_type_name(&self) -> String
    {
        let mut char_buf = Vec::new();

        for i in 0..8
        {
            char_buf.push((self.fs_type_name() >> 8 * i) as u8 as char);
        }

        return char_buf.iter().collect();
    }

    pub fn get_sector_size(&self) -> usize
    {
        return self.bpb_bytes_per_sector() as usize;
    }

    pub fn get_cluster_size(&self) -> usize
    {
        return self.bpb_sectors_per_cluster() as usize;
    }

    pub fn get_reserved_sectors_cnt(&self) -> usize
    {
        return self.bpb_reserved_sectors_cnt() as usize;
    }

    pub fn get_num_fats(&self) -> usize
    {
        return self.bpb_num_fats() as usize;
    }

    pub fn get_root_entries_cnt(&self) -> usize
    {
        return self.bpb_root_entries_cnt() as usize;
    }

    pub fn get_total_sectors_cnt(&self) -> usize
    {
        let cnt = self.bpb_total_sectors() as usize;

        match cnt
        {
            0 => return self.bpb_fat32_total_sectors() as usize,    // FAT32
            _ => return cnt                                         // not FAT32
        }
    }

    pub fn get_media_type(&self) -> usize
    {
        return self.bpb_media_type() as usize;
    }

    pub fn get_sectors_cnt_per_fat(&self) -> usize
    {
        let cnt = self.bpb_fat_size() as usize;

        match cnt
        {
            0 => return self.bpb_fat32_fat_size() as usize, // FAT32
            _ => return cnt                                 // not FAT32
        }
    }

    pub fn get_sectors_cnt_per_track(&self) -> usize
    {
        return self.bpb_sectors_per_track() as usize;
    }

    pub fn get_heads_cnt(&self) -> usize
    {
        return self.bpb_num_heads() as usize;
    }

    pub fn get_hidden_sectors_cnt(&self) -> usize
    {
        return self.bpb_num_hidden_sectors() as usize;
    }

    pub fn get_fat32_ext_flags(&self) -> usize
    {
        return self.bpb_fat32_ext_flags() as usize;
    }

    pub fn get_fat32_fs_ver(&self) -> usize
    {
        return self.bpb_fat32_fs_ver() as usize;
    }

    pub fn get_fat32_root_dir_cluster_num(&self) -> usize
    {
        return self.bpb_fat32_num_root_clasters() as usize;
    }

    pub fn get_fat32_fs_info_sector_num(&self) -> usize
    {
        return self.bpb_fat32_fs_info_sector_num() as usize;
    }

    pub fn get_fat32_boot_sector_backup_sector_num(&self) -> usize
    {
        return self.bpb_fat32_boot_sector_backup_sector_num() as usize;
    }

    pub fn get_drive_num(&self) -> u8
    {
        return self.drive_number();
    }

    pub fn get_boot_sig(&self) -> u8
    {
        return self.boot_sig();
    }

    pub fn get_volume_id(&self) -> u32
    {
        return self.volume_id();
    }

    pub fn total_sectors_cnt(&self) -> usize
    {
        return self.get_total_sectors_cnt();
    }

    pub fn reserved_area_sectors_cnt(&self) -> usize
    {
        return self.get_reserved_sectors_cnt();
    }

    pub fn fat_area_sectors_cnt(&self) -> usize
    {
        return self.get_num_fats() * self.get_sectors_cnt_per_fat();
    }

    pub fn root_dir_area_sectors_cnt(&self) -> usize
    {
        let bps = self.get_sector_size();
        return (self.get_root_entries_cnt() * 32 + bps - 1) / bps;
    }

    pub fn data_area_sectors_cnt(&self) -> usize
    {
        match self.is_fat32()
        {
            false => return self.total_sectors_cnt() - self.reserved_area_sectors_cnt() - self.fat_area_sectors_cnt() - self.root_dir_area_sectors_cnt(),
            true => return self.total_sectors_cnt() - self.reserved_area_sectors_cnt() - self.fat_area_sectors_cnt()
        }
    }

    pub fn reserved_area_start_sector_num(&self) -> usize
    {
        return 0;
    }

    pub fn fat_area_start_sector_num(&self) -> usize
    {
        return self.reserved_area_sectors_cnt();
    }

    pub fn root_dir_area_start_sector_num(&self) -> usize
    {
        return self.fat_area_start_sector_num() + self.fat_area_sectors_cnt();
    }

    pub fn data_area_start_sector_num(&self) -> usize
    {
        match self.is_fat32()
        {
            false => return self.root_dir_area_start_sector_num() + self.root_dir_area_sectors_cnt(),
            true => return self.root_dir_area_start_sector_num()
        }
    }

    pub fn fat_type(&self) -> FATType
    {
        let clusters_cnt = self.data_area_sectors_cnt() / self.get_cluster_size();

        if clusters_cnt >= 65526
        {
            return FATType::FAT32;
        }
        else if clusters_cnt >= 4086
        {
            return FATType::FAT16;
        }
        else
        {
            return FATType::FAT12;
        }
    }

    fn is_fat32(&self) -> bool
    {
        let root_entry_cnt = self.bpb_root_entries_cnt();
        let total_sectors = self.bpb_total_sectors();
        let fat_size = self.bpb_fat_size();

        if (root_entry_cnt | total_sectors | fat_size) == 0
        {
            return true;
        }
        else
        {
            return false;
        }
    }
}