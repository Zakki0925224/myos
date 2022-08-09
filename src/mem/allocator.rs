use core::{alloc::{GlobalAlloc, Layout}, ptr::null_mut, cell::UnsafeCell};

use crate::println;

pub const HEAP_AREA_BASE_ADDR: u32 = 0x64000;
pub const HEAP_SIZE: u32 = 100 * 1024; // 100KiB

#[global_allocator]
static ALLOCATOR: Allocator = Allocator { base_addr: UnsafeCell::new(HEAP_AREA_BASE_ADDR) };

pub struct Allocator
{
    base_addr: UnsafeCell<u32>
}

unsafe impl Sync for Allocator {}

unsafe impl GlobalAlloc for Allocator
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8
    {
        let size = layout.size() as u32;
        let align = layout.align() as u32;

        let base_addr = self.base_addr.get();

        if size > HEAP_SIZE
        {
            return null_mut();
        }

        if align > size
        {
            return null_mut();
        }

        //println!("addr: {}, size: {}, align: {}", *base_addr, size, align);

        let offset = (size + ((align - 1) & !(align - 1))) * size;

        if *base_addr + offset > HEAP_AREA_BASE_ADDR + HEAP_SIZE
        {
            return null_mut();
        }

        *base_addr += offset;

        return *base_addr as *mut u8;
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}