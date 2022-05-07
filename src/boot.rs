pub const MULTIBOOT_MAGIC_NUM: u32 = 0x36d76289;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MultibootInfo
{
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms1: u32,
    pub syms2: u32,
    pub syms3: u32,
    pub mmap_length: u32,
    pub mmap_addr: u32,
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name: u32,
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u32,
    pub vbe_interface_seg: u32,
    pub vbe_interface_off: u32,
    pub vbe_interface_len: u32
}