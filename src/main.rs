#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod vga_buffer;
mod interrupts;
mod gdt;
mod memory;
mod scheduler;
mod heap;
mod nebula;
mod quantum;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Quantum-inspired idle task
fn idle_task() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter % 500000 == 0 {
            // Quantum pulse visualization
            nebula::pulse_nebula();
            vga_buffer::WRITER.lock().draw_pulse(" IDLE QUANTUM ", vga_buffer::Color::Magenta);
        }
    }
}

// High-energy system task
fn system_task() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter % 300000 == 0 {
            println!("System Task: quantum tick {}", counter / 300000);
        }
    }
}

// I/O simulation task (low energy state)
fn io_task() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter % 700000 == 0 {
            nebula::render_nebula();
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("");
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║          NimbusOS QUANTUM CORE LOADING              ║");
    println!("║    Phase 3: Memory | Phase 4: Multitasking          ║");
    println!("║    ⚛ Quantum Process Visualization Active ⚛         ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();

    // Initialize GDT for memory protection
    println!("[MEM] Initializing Global Descriptor Table...");
    gdt::init_gdt();
    
    // Initialize interrupts
    println!("[INT] Setting up quantum interrupt handlers...");
    interrupts::init_idt();

    // Initialize VGA with nebula visualization
    vga_buffer::init_status_bar();
    
    println!();
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║     QUANTUM VISUALIZATION ENGINE INITIALIZED          ║");
    println!("║                                                      ║");
    println!("║  ⚛ Process Nebula: Star-based task constellation     ║");
    println!("║  ⚛ Quantum States: Superposition & Collapse          ║");
    println!("║  ⚛ Wave Function: Real-time probability tracking     ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();

    // Spawn quantum-influenced tasks
    println!("[SCHED] Launching quantum task scheduler...");
    scheduler::spawn("Idle Quantum", idle_task as u64, 4096);
    scheduler::spawn("System Core", system_task as u64, 4096);
    scheduler::spawn("I/O Entangler", io_task as u64, 4096);
    
    // Track tasks in quantum visualizer
    quantum::track_process(0, "Idle Quantum", scheduler::Priority::Normal);
    quantum::track_process(1, "System Core", scheduler::Priority::High);
    quantum::track_process(2, "I/O Entangler", scheduler::Priority::Low);
    
    // Schedule first task
    if let Some(task_id) = scheduler::SCHEDULER.schedule() {
        println!("[SCHED] First task scheduled: ID {}", task_id);
    }
    
    // Render initial visualizations
    nebula::render_nebula();
    quantum::render_quantum();
    
    println!();
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║   NimbusOS QUANTUM CORE: FULLY OPERATIONAL           ║");
    println!("║   ⚛ Multitasking | ⚛ Memory Protection | ⚛ Visual  ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();

    // Test breakpoint to show quantum pulse
    println!("[TEST] Triggering quantum breakpoint...");
    x86_64::instructions::interrupts::int3();

    // Halt until interrupt
    loop {
        x86_64::instructions::hlt();
    }
}
