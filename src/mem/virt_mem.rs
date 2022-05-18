const VA_PDE_INDEX_MASK: u32 = 0xffc00000;
const VA_PDE_INDEX_MAX: u32 = 0x3ff;
const VA_PDE_INDEX_SHIFT: u32 = 22;
const VA_PTE_INDEX_MASK: u32 = 0x3ff000;
const VA_PTE_INDEX_MAX: u32 = 0x3ff;
const VA_PTE_INDEX_SHIFT: u32 = 12;
const VA_PAGE_OFFSET_MASK: u32 = 0xfff;
const VA_PAGE_OFFSET_MAX: u32 = 0xfff;
const VA_PAGE_OFFSET_SHIFT: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub struct VirtualAddress
{
    inner: u32
}

impl VirtualAddress
{
    pub fn new(inner: u32) -> VirtualAddress
    {
        return VirtualAddress { inner };
    }

    pub fn set(page_directory_entry_index: u32,
               page_table_entry_index: u32,
               page_offset: u32) -> VirtualAddress
    {
        if page_directory_entry_index > VA_PDE_INDEX_MAX
        {
            panic!("page_directory_entry_addr is out of range");
        }

        if page_table_entry_index > VA_PTE_INDEX_MAX
        {
            panic!("page_table_entry_addr is out of range");
        }

        if page_offset > VA_PAGE_OFFSET_MAX
        {
            panic!("page_addr is out of range");
        }

        let inner = page_directory_entry_index << VA_PDE_INDEX_SHIFT |
                        page_table_entry_index << VA_PTE_INDEX_SHIFT |
                        page_offset << VA_PAGE_OFFSET_SHIFT;

        return VirtualAddress { inner };
    }

    pub fn get_page_directory_entry_index(&self) -> u32
    {
        return (self.inner & VA_PDE_INDEX_MASK) >> VA_PDE_INDEX_SHIFT;
    }

    pub fn get_page_table_entry_index(&self) -> u32
    {
        return (self.inner & VA_PTE_INDEX_MASK) >> VA_PTE_INDEX_SHIFT;
    }

    pub fn get_page_offset(&self) -> u32
    {
        return (self.inner & VA_PAGE_OFFSET_MASK) >> VA_PAGE_OFFSET_SHIFT;
    }
}