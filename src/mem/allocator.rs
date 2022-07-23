use core::{alloc::{GlobalAlloc, Layout}, ptr::null_mut};

pub const HEAP_AREA_BASE_ADDR: u32 = 0x64000;
pub const HEAP_SIZE: u32 = 100 * 1024; // 100KiB

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8
    {
        let size = layout.size();
        let align = layout.align();

        let mut base_addr = HEAP_AREA_BASE_ADDR;

        if size > HEAP_SIZE as usize
        {
            return null_mut();
        }

        if align > size
        {
            return null_mut();
        }

        for i in base_addr as usize..base_addr as usize + size
        {
            if i % align != 0
            {
                base_addr += 2;
            }
        }

        return base_addr as *mut u8;
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}