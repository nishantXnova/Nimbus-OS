#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod vga_buffer;
mod interrupts;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("NimbusOS: Phase 2 Initializing...");

    interrupts::init_idt();
    vga_buffer::init_status_bar();

    println!("IDT Loaded. Triggering Breakpoint Exception...");
    
    // Trigger a breakpoint exception
    x86_64::instructions::interrupts::int3();

    println!("Continuing after breakpoint. System is Reactive.");

    loop {}
}
