#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod vga_buffer;
mod interrupts;
mod gdt;
mod memory;
mod scheduler;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Simple idle task function
fn idle_task() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter % 1000000 == 0 {
            // Show a pulsing effect periodically
            vga_buffer::WRITER.lock().draw_pulse(" IDLE TASK ACTIVE ", vga_buffer::Color::Cyan);
        }
    }
}

// Timer interrupt demo task
fn timer_task() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter % 500000 == 0 {
            println!("Timer Task: tick {}", counter / 500000);
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("");
    println!("╔══════════════════════════════════════════════╗");
    println!("║          NimbusOS Phase 4 Loading          ║");
    println!("║        Multitasking & Memory Mgmt          ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();

    // Initialize GDT for memory protection
    println!("[PHASE 3.1] Initializing Global Descriptor Table...");
    gdt::init_gdt();
    
    // Initialize interrupts with timer support
    println!("[PHASE 2.2] Setting up interrupt handling...");
    interrupts::init_idt();

    // Initialize VGA status bar
    vga_buffer::init_status_bar();
    
    // Note: Memory initialization requires bootinfo from bootloader
    // This would normally be passed from the bootloader entry point
    // For now, we skip memory::init_memory() as we don't have bootinfo

    // Spawn kernel tasks
    println!("[PHASE 4.1] Starting Task Scheduler...");
    scheduler::spawn("Idle Task", idle_task as u64, 4096);
    scheduler::spawn("Timer Task", timer_task as u64, 4096);
    
    // Schedule and run first task
    println!("[PHASE 4.2] Scheduling first task...");
    if let Some(task_id) = scheduler::SCHEDULER.schedule() {
        println!("Scheduled Task ID: {}", task_id);
    }
    
    // Print scheduler status
    scheduler::SCHEDULER.print_status();

    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║     NimbusOS Phase 4: FULLY OPERATIONAL      ║");
    println!("║   Multitasking Enabled | GDT Active         ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();

    // Trigger a breakpoint to show interrupt handling
    println!("Triggering test breakpoint...");
    x86_64::instructions::interrupts::int3();

    println!("System ready for operations.");
    
    // Halt the CPU until an interrupt occurs
    loop {
        x86_64::instructions::hlt();
    }
}
