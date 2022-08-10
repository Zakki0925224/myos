use core::ptr::read_volatile;

use super::boot_sector::FATType;

#[derive(Debug, PartialEq, Eq)]
pub enum ClusterType
{
    Free(usize),
    Reserved(usize),
    Data(usize),
    Bad(usize),
    EndOfChain(usize),
    Other(usize) // TODO: remove after
}

pub fn get_next_cluster_num(fat_start_base_addr: u32, fat_type: FATType, cluster_num: usize) -> ClusterType
{
    match fat_type
    {
        FATType::FAT12 =>
        {
            let offset = cluster_num + (cluster_num / 2);
            let mut value = unsafe { read_volatile((fat_start_base_addr + offset as u32) as *const u16) };

            if (cluster_num & 0x1) != 0
            {
                value >>= 4;
            }
            else
            {
                value &= 0xfff;
            }

            let result_cluster_num = value as usize;

            if result_cluster_num >= 0xff8
            {
                return ClusterType::EndOfChain(result_cluster_num);
            }
            else if result_cluster_num == 0xff7
            {
                return ClusterType::Bad(result_cluster_num);
            }
            else if result_cluster_num >= 0xff0
            {
                return ClusterType::Reserved(result_cluster_num);
            }
            else if result_cluster_num >= 0x2 &&
                    result_cluster_num <= 0xfef
            {
                return ClusterType::Data(result_cluster_num);
            }
            else if result_cluster_num == 0x1
            {
                return ClusterType::Reserved(result_cluster_num);
            }
            else if result_cluster_num == 0x0
            {
                return ClusterType::Free(result_cluster_num);
            }
            else
            {
                return ClusterType::Other(result_cluster_num);
            }
        },
        FATType::FAT16 =>
        {
            let offset = cluster_num * 2;
            let value = unsafe { read_volatile((fat_start_base_addr + offset as u32) as *const u16) };

            let result_cluster_num = value as usize;

            if result_cluster_num >= 0xfff8
            {
                return ClusterType::EndOfChain(result_cluster_num);
            }
            else if result_cluster_num == 0xfff7
            {
                return ClusterType::Bad(result_cluster_num);
            }
            else if result_cluster_num >= 0xfff0
            {
                return ClusterType::Reserved(result_cluster_num);
            }
            else if result_cluster_num >= 0x2 &&
                    result_cluster_num <= 0xffef
            {
                return ClusterType::Data(result_cluster_num);
            }
            else if result_cluster_num == 0x1
            {
                return ClusterType::Reserved(result_cluster_num);
            }
            else if result_cluster_num == 0x0
            {
                return ClusterType::Free(result_cluster_num);
            }
            else
            {
                return ClusterType::Other(result_cluster_num);
            }
        },
        FATType::FAT32 =>
        {
            let offset = cluster_num * 4;
            let value = unsafe { read_volatile((fat_start_base_addr + offset as u32) as *const u32) };

            let result_cluster_num = value as usize;

            if result_cluster_num >= 0xffffff8
            {
                return ClusterType::EndOfChain(result_cluster_num);
            }
            else if result_cluster_num == 0xffffff7
            {
                return ClusterType::Bad(result_cluster_num);
            }
            else if result_cluster_num >= 0xffffff0
            {
                return ClusterType::Reserved(result_cluster_num);
            }
            else if result_cluster_num >= 0x2 &&
                    result_cluster_num <= 0xfffffef
            {
                return ClusterType::Data(result_cluster_num);
            }
            else if result_cluster_num == 0x1
            {
                return ClusterType::Reserved(result_cluster_num);
            }
            else if result_cluster_num == 0x0
            {
                return ClusterType::Free(result_cluster_num);
            }
            else
            {
                return ClusterType::Other(result_cluster_num);
            }
        },
    }
}