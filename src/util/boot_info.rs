use multiboot2::{BootInformation, MemoryArea, ModuleTag};

pub fn get_total_mem_size(boot_info: &BootInformation) -> u64
{
    return get_all_mem_areas(boot_info).map(|area| area.size() - 1).sum();
}

pub fn get_available_mem_areas(boot_info: &BootInformation) -> impl Iterator<Item = &MemoryArea>
{
    let mem_map_tag = boot_info.memory_map_tag().expect("No memory map tag");
    return mem_map_tag.memory_areas();
}

pub fn get_all_mem_areas(boot_info: &BootInformation) -> impl Iterator<Item = &MemoryArea>
{
    let mem_map_tag = boot_info.memory_map_tag().expect("No memory map tag");
    return mem_map_tag.all_memory_areas();
}

pub fn get_kernel_addr(boot_info: &BootInformation) -> (u64, u64)
{
    let elf_sections_tag = boot_info.elf_sections_tag().expect("No elf sections tag");
    let kernel_start = elf_sections_tag.sections().map(|s| s.start_address()).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.end_address()).max().unwrap();

    return (kernel_start, kernel_end);
}

pub fn get_kernel_size(boot_info: &BootInformation) -> u64
{
    let (start, end) = get_kernel_addr(boot_info);
    return end - start;
}

pub fn get_multiboot_addr(boot_info: &BootInformation) -> (u64, u64)
{
    let multiboot_start = boot_info.start_address() as u64;
    let multiboot_end = boot_info.end_address() as u64;

    return (multiboot_start, multiboot_end);
}

pub fn get_multiboot_size(boot_info: &BootInformation) -> u64
{
    let (start, end) = get_multiboot_addr(boot_info);
    return end - start;
}

pub fn get_module_tags(boot_info: &BootInformation) -> impl Iterator<Item = &ModuleTag>
{
    let module_tag = boot_info.module_tags();
    return module_tag;
}