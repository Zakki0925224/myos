use multiboot2::{BootInformation, MemoryArea};

pub mod phys_mem;

pub fn get_total_available_mem_size(boot_info: &BootInformation) -> u64
{
    let mem_map_tag = boot_info.memory_map_tag().expect("No memory map tag");
    let total_size = mem_map_tag.memory_areas().map(|area| area.size()).sum();

    return total_size;
}

pub fn get_all_available_mem_areas(boot_info: &BootInformation) -> impl Iterator<Item = &MemoryArea>
{
    let mem_map_tag = boot_info.memory_map_tag().expect("No memory map tag");
    return mem_map_tag.memory_areas();
}

pub fn get_kernel_addr(boot_info: &BootInformation) -> (u64, u64)
{
    let elf_sections_tag = boot_info.elf_sections_tag().expect("No elf sections tag");
    let kernel_start = elf_sections_tag.sections().map(|s| s.start_address()).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.end_address()).max().unwrap();

    return (kernel_start, kernel_end);
}

pub fn get_multiboot_addr(boot_info: &BootInformation) -> (u64, u64)
{
    let multiboot_start = boot_info.start_address() as u64;
    let multiboot_end = boot_info.end_address() as u64;

    return (multiboot_start, multiboot_end);
}