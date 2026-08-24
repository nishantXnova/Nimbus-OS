//! Kernel heap: linked_list_allocator + bump fallback, real GlobalAlloc
//! Replaces deprecated AllocRef bump that never freed.

use core::alloc::{GlobalAlloc, Layout};
use linked_list_allocator::LockedHeap;
use spin::Mutex;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Bump fallback before paging init (for early scheduler stacks)
struct Bump {
    start: usize,
    end: usize,
    next: Mutex<usize>,
}
impl Bump {
    const fn new() -> Self { Self { start: 0, end: 0, next: Mutex::new(0) } }
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        *self.next.lock() = start;
    }
    fn alloc(&self, layout: Layout) -> Option<*mut u8> {
        let mut n = self.next.lock();
        let aligned = (*n + layout.align() - 1) & !(layout.align() - 1);
        if aligned + layout.size() > self.end { return None; }
        *n = aligned + layout.size();
        Some(aligned as *mut u8)
    }
}
static BUMP: Mutex<Bump> = Mutex::new(Bump::new());

/// Call early from _start before any Box/Vec
pub fn init_heap_early(start: usize, size: usize) {
    BUMP.lock().init(start, size);
    crate::println!("[HEAP] Bump early: {:#x}-{:#x} ({} KiB)", start, start+size, size/1024);
}

/// After paging is ready, init real heap at given virt range
pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    heap_start: usize,
    heap_size: usize,
) -> Result<(), MapToError<Size4KiB>> {
    let heap_start = VirtAddr::new(heap_start as u64);
    let heap_end = heap_start + heap_size as u64;
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    let page_range = Page::range_inclusive(
        Page::containing_address(heap_start),
        Page::containing_address(heap_end - 1u64),
    );
    for page in page_range {
        let frame = frame_allocator.allocate_frame().expect("no frames for heap");
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush(); }
    }

    unsafe { ALLOCATOR.lock().init(heap_start.as_mut_ptr(), heap_size); }
    crate::println!("[HEAP] LinkedList heap: {:#x}-{:#x} ({} KiB) ONLINE", heap_start.as_u64(), heap_end.as_u64(), heap_size/1024);
    Ok(())
}

/// For scheduler stacks before heap init - fall back to bump
pub fn alloc_early(layout: Layout) -> Option<*mut u8> {
    // try global allocator first
    let ptr = unsafe { ALLOCATOR.alloc(layout) };
    if !ptr.is_null() { return Some(ptr); }
    // fallback bump
    BUMP.lock().alloc(layout)
}

pub fn free_early(ptr: *mut u8, layout: Layout) {
    unsafe { ALLOCATOR.dealloc(ptr, layout); }
}

// Simple wrapper so `Box::new([0u8; N])` works after init_heap
pub fn heap_stats() -> (usize, usize) {
    // linked_list_allocator doesn't expose stats easily; estimate via bump
    let b = BUMP.lock();
    let used = *b.next.lock() - b.start;
    (used, b.end - b.start)
}
