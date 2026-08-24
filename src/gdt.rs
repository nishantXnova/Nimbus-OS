use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;
use lazy_static::lazy_static;

const DOUBLE_FAULT_STACK_SIZE: usize = 4096 * 5;

static mut DOUBLE_FAULT_STACK: [u8; DOUBLE_FAULT_STACK_SIZE] = [0; DOUBLE_FAULT_STACK_SIZE];

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[0] = {
            let stack_start = VirtAddr::new(unsafe { &raw const DOUBLE_FAULT_STACK as u64 });
            stack_start + DOUBLE_FAULT_STACK_SIZE as u64
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, SegmentSelector, SegmentSelector) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code = gdt.append(Descriptor::kernel_code_segment());
        let data = gdt.append(Descriptor::kernel_data_segment());
        gdt.append(Descriptor::tss_segment(&TSS));
        (gdt, code, data)
    };
}

pub fn init_gdt() {
    use x86_64::instructions::segmentation::{CS, DS, Segment};
    use x86_64::instructions::tables::load_tss;
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1);
        DS::set_reg(GDT.2);
        load_tss(GDT.1);
    }
    crate::println!("[GDT] Loaded code={:#x} data={:#x}", GDT.1.0, GDT.2.0);
}
pub fn user_code_segment() -> SegmentSelector { SegmentSelector(3) }
pub fn user_data_segment() -> SegmentSelector { SegmentSelector(4) }
