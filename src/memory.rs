use x86_64::structures::paging::{Mapper, Size4KiB, FrameAllocator};
use x86_64::structures::paging::mapper::MapperAllSizes;
use x86_64::PhysAddr;
use x86_64::VirtAddr;

/// Memory frame allocator for the bootloader-provided memory map
pub struct BootInfoFrameAllocator {
    memory_map: &'static x86_64::structures::paging::BootInfoFrameAllocator,
}

impl BootInfoFrameAllocator {
    pub fn new(boot_info: &'static x86_64::bootinfo::BootInfo) -> Self {
        BootInfoFrameAllocator {
            memory_map: unsafe {
                x86_64::structures::paging::BootInfoFrameAllocator::init(boot_info.memory_map())
            },
        }
    }
}

impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size4KiB>> {
        self.memory_map.allocate()
    }
}

/// Initialize the memory management subsystem
/// Sets up identity mapping for the kernel
pub fn init_memory(boot_info: &'static x86_64::bootinfo::BootInfo) {
    // The bootloader already maps the kernel, but we can add more mappings here
    let mut frame_allocator = BootInfoFrameAllocator::new(boot_info);

    // Get the start and end addresses of the kernel
    let kernel_start = VirtAddr::new(boot_info.kernel_start.as_u64());
    let kernel_end = VirtAddr::new(boot_info.kernel_end.as_u64());

    crate::println!("Memory Management Initialized");
    crate::println!("Kernel start: {:#x}", kernel_start);
    crate::println!("Kernel end: {:#x}", kernel_end);
    crate::println!("Available memory: {} bytes", 
        boot_info.memory_map.iter()
            .filter(|r| r.region_type == x86_64::bootinfo::MemoryRegionType::Usable)
            .map(|r| r.range.end - r.range.start)
            .sum::<u64>());

    // Show memory regions
    for (i, region) in boot_info.memory_map.iter().enumerate() {
        if region.region_type == x86_64::bootinfo::MemoryRegionType::Usable {
            crate::println!("Region {}: {:#x} - {:#x} ({} bytes)", 
                i, region.range.start, region.range.end, region.range.end - region.range.start);
        }
    }
}

/// Allocate a new memory frame
pub fn allocate_frame() -> Option<PhysAddr> {
    // This is a simplified version - full implementation would use the frame allocator
    None
}