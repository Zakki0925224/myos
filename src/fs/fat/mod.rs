use core::{ptr::read_volatile, mem::size_of};
use crate::{print, println, fs::fat::{fs_info_sector::FsInfoSector, dir_entery::DirectoryEntry, boot_sector::FATType}};

use self::boot_sector::BootSector;

mod boot_sector;
mod fs_info_sector;
mod dir_entery;

pub struct Fat
{
    start_base_addr: u32,
    end_base_addr: u32
}

impl Fat
{
    pub fn new(start_base_addr: u32, end_base_addr: u32) -> Fat
    {
        return Fat { start_base_addr, end_base_addr };
    }

    pub fn test(&self)
    {
        unsafe
        {
            let bs = read_volatile(self.start_base_addr as *const BootSector);
            println!("{:?}", bs);
            println!("OEM name: \"{}\"", bs.get_oem_name());
            println!("Volume label: \"{}\"", bs.get_volume_label());
            println!("Volume id: 0x{:x}", bs.get_volume_id());
            println!("FS type name: \"{}\"", bs.get_fs_type_name());
            println!("FAT type: {:?}", bs.fat_type());

            if bs.fat_type() == FATType::FAT32
            {
                let fis = read_volatile((self.start_base_addr + (bs.get_fat32_fs_info_sector_num() * bs.get_sector_size()) as u32) as *const FsInfoSector);
                println!("{:?}", fis);
            }

            println!("data area start sector num: {}", bs.data_area_start_sector_num());
            println!("data area sectors cnt: {}", bs.data_area_sectors_cnt());

            let mut cnt = 0;

            for i in 0../*bs.data_area_sectors_cnt()*/10
            {
                let de = read_volatile((self.start_base_addr + bs.get_sector_size() as u32 * (bs.data_area_start_sector_num() + i) as u32) as *const DirectoryEntry);
                let file_attr = de.get_file_attr();

                // if file_attr == None
                // {
                //     continue;
                // }

                println!("{:?}", de);
                println!("File short name: {}", de.get_file_short_name());
                println!("File attribute: {:?}", file_attr);

                cnt += 1;
            }

            println!("got {} available data", cnt);
        }
    }
}