use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::instructions::segmentation::{CS, DS, SS};
use x86_64::instructions::tables::load_gdt;

/// Custom struct for storing the GDT pointers needed for reload
#[repr(C, packed)]
struct GdtPointer {
    limit: u16,
    base: x86_64::PhysAddr,
}

/// The GDT structure holding both the table and pointers
struct GdtTables {
    table: GlobalDescriptorTable,
    pointer: GdtPointer,
}

// We need to store the code and data segment selectors
static mut GDT_TABLES: Option<GdtTables> = None;

/// Initialize the Global Descriptor Table
/// This sets up the segments for kernel and user mode
pub fn init_gdt() {
    let mut gdt = GlobalDescriptorTable::new();

    // Kernel code segment - present, execution, ring 0, readable
    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    
    // Kernel data segment - present, writable, ring 0
    let data_selector = gdt.add_entry(Descriptor::kernel_data_segment());

    // User code segment - present, execution, ring 3, readable
    let user_code_selector = gdt.add_entry(Descriptor::user_code_segment());
    
    // User data segment - present, writable, ring 3
    let user_data_selector = gdt.add_entry(Descriptor::user_data_segment());

    // TSS segment (for future task switching)
    // For now, we use a null TSS
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(x86_64::structures::tss::TaskStateSegment::new()));

    // Create the pointer for loading
    let pointer = GdtPointer {
        limit: (core::mem::size_of::<GlobalDescriptorTable>() - 1) as u16,
        base: x86_64::PhysAddr::new(core::ptr::addr_of!(gdt) as u64),
    };

    // Store the tables for later access
    unsafe {
        GDT_TABLES = Some(GdtTables { table: gdt, pointer });
    }

    // Load the GDT
    load_gdt(&pointer);

    // Update segment registers
    unsafe {
        CS::set_reg(code_selector);
        DS::set_reg(data_selector);
        SS::set_reg(data_selector);
    }

    crate::println!("GDT Loaded: CodeSel={:#x}, DataSel={:#x}", 
        code_selector.0, data_selector.0);
}

/// Get the code segment selector for user mode
pub fn user_code_segment() -> SegmentSelector {
    SegmentSelector(3) // Ring 3 code selector
}

/// Get the data segment selector for user mode
pub fn user_data_segment() -> SegmentSelector {
    SegmentSelector(4) // Ring 3 data selector
}