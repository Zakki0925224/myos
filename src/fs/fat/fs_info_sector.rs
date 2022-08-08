use core::ptr::read_volatile;

use alloc::{string::String, vec::Vec};

#[derive(Debug)]
#[repr(C)]
pub struct FsInfoSector
{
    signature1: u32,      //for FAT12 and FAT16
    reserved1: [u32; 120], // 480B
    signature2: u32,      // for FAT32
    // last known number of free data clusters on the volume
    // unknown -> 0xffff ffff
    free_data_clusters_cnt: u32,
    // number of the most recently known to be allocated data cluster
    allocated_data_clusters_cnt: u32,
    reserved2: [u8; 12],
    signature3: u32
}

impl FsInfoSector
{
    pub fn read(base_addr: u32) -> FsInfoSector
    {
        return unsafe { read_volatile(base_addr as *const FsInfoSector) };
    }

    pub fn get_signatures(&self) -> [u32; 3]
    {
        return [self.signature1, self.signature2, self.signature3];
    }

    pub fn get_free_data_clusters_cnt(&self) -> u32
    {
        return self.free_data_clusters_cnt;
    }

    pub fn get_allocated_data_clusters_cnt(&self) -> u32
    {
        return self.allocated_data_clusters_cnt;
    }
}