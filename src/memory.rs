use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr::null_mut;

#[inline]
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: UnsafeCell<usize>,
}

impl BumpAllocator {
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        BumpAllocator {
            heap_start,
            heap_end: heap_start + heap_size,
            next: UnsafeCell::new(heap_start),
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let current = *self.next.get();
        let alloc_start = align_up(current, layout.align());
        let alloc_end = alloc_start.saturating_add(layout.size());
        if alloc_end > self.heap_end {
            null_mut()
        } else {
            *self.next.get() = alloc_end;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
    }
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new(0x4444_0000, 1024 * 1024);

pub mod paging {
    use x86_64::{
        structures::paging::{Mapper, Page, PageTableFlags, Size4KiB},
        PhysAddr, VirtAddr,
    };

    pub unsafe fn map_page(
        mapper: &mut impl Mapper<Size4KiB>,
        phys_addr: PhysAddr,
        virt_addr: VirtAddr,
    ) {
        let page = Page::<Size4KiB>::containing_address(virt_addr);
        let frame = x86_64::structures::paging::PhysFrame::containing_address(phys_addr);
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        let mut frame_allocator = DummyFrameAllocator {};
        let _ = mapper.map_to(page, frame, flags, &mut frame_allocator)
            .expect("map_to faily")
            .flush();
    }

    pub struct DummyFrameAllocator;

    unsafe impl x86_64::structures::paging::FrameAllocator<Size4KiB> for DummyFrameAllocator {
        fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame> {
            panic!("DummyFrameAllocator aint suppin alloczz");
        }
    }
}
