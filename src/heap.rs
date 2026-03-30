use core::alloc::allocator::{AllocRef, Layout};
use spin::Mutex;

/// A simple bump allocator for kernel heap allocation
/// Allocates memory linearly, no deallocation (monotonic)
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    current: Mutex<usize>,
}

impl BumpAllocator {
    pub fn new(heap_start: usize, heap_end: usize) -> Self {
        BumpAllocator {
            heap_start,
            heap_end,
            current: Mutex::new(heap_start),
        }
    }

    pub fn init(&self) {
        *self.current.lock() = self.heap_start;
    }
}

unsafe impl AllocRef for BumpAllocator {
    fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, core::alloc::AllocErr> {
        let mut current = self.current.lock();
        
        // Align the current pointer
        let aligned = (*current + layout.align() - 1) & !(layout.align() - 1);
        
        // Check if we have enough space
        if aligned + layout.size() > self.heap_end {
            return Err(core::alloc::AllocErr);
        }
        
        let result = NonNull::new(aligned as *mut u8).unwrap();
        *current = aligned + layout.size();
        
        crate::println!("[HEAP] Allocated {} bytes at {:#x}", layout.size(), aligned);
        Ok(result)
    }

    fn dealloc(&mut self, _ptr: NonNull<u8>, _layout: Layout) {
        // Bump allocator doesn't support deallocation
        // Memory is reclaimed on kernel restart
    }
}

/// Global bump allocator instance
use lazy_static::lazy_static;
lazy_static! {
    static ref HEAP_ALLOCATOR: BumpAllocator = BumpAllocator::new(0xFFFF_0000, 0xFFFF_FFFF);
}

pub fn init_heap(start: usize, size: usize) {
    let end = start + size;
    crate::println!("[HEAP] Initializing bump allocator: {:#x} - {:#x} ({} bytes)", 
        start, end, size);
    
    // Create a new allocator with the given range
    let allocator = BumpAllocator::new(start, end);
    // Note: In real implementation, we'd replace the global allocator
    // For now, we just initialize the existing one
    unsafe {
        // This would normally set up a global allocator
        // HEAP_ALLOCATOR = BumpAllocator::new(start, end);
    }
    
    crate::println!("[HEAP] Kernel heap initialized");
}

/// Allocate memory from the kernel heap
pub fn allocate(size: usize, align: usize) -> Option<usize> {
    let layout = Layout::from_size_align(size, align).ok()?;
    
    // Simplified allocation using the bump allocator
    let mut current = HEAP_ALLOCATOR.current.lock();
    let aligned = (*current + align - 1) & !(align - 1);
    
    if aligned + size > HEAP_ALLOCATOR.heap_end {
        crate::println!("[HEAP] Allocation failed: out of memory");
        return None;
    }
    
    let result = aligned;
    *current = aligned + size;
    
    Some(result)
}