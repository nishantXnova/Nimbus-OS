#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod vga_buffer;
mod interrupts;
mod gdt;
// mod memory; // legacy, not used in v0.4 (bootloader handles)
mod scheduler;
mod heap;
mod pit;
mod keyboard;
mod deck;
mod ramfs;
mod chronos;
mod wasm;
mod ipc;
mod synapse;
// legacy cosmic modules — kept as files for archaeology, not compiled in v0.4
// mod nebula; mod quantum; mod gravity; mod hologram; mod neural; mod cosmic; mod dna; mod evolution; mod holomemory; mod consciousness; mod fractal;
mod morphic;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::println!("[PANIC] {}", info);
    loop { x86_64::instructions::hlt(); }
}

pub fn dummy_task() -> ! {
    let mut c:u64=0;
    loop{ c=c.wrapping_add(1); if c%10_000_000==0{ crate::scheduler::SCHEDULER.block_current(50); x86_64::instructions::hlt(); } core::hint::spin_loop(); }
}
fn solar_task()->!{ let mut c:u64=0; loop{ c+=1; if c%5_000_000==0{ crate::ramfs::append_log(&alloc::format!("[SOLAR] flare {}", c/5_000_000)); crate::ipc::pipe_send(1, b"flare"); crate::scheduler::SCHEDULER.block_current(20);} core::hint::spin_loop(); } }
fn neutron_task()->!{ let mut c:u64=0; loop{ c+=1; if c%8_000_000==0{ crate::ipc::pipe_send(2, b"pulse"); crate::scheduler::SCHEDULER.block_current(10);} core::hint::spin_loop(); } }
fn io_task()->!{ let mut c:u64=0; loop{ c+=1; if c%12_000_000==0{ crate::scheduler::SCHEDULER.block_current(100);} core::hint::spin_loop(); } }

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    heap::init_heap_early(0x4444_4444_0000 as usize, 2*1024*1024);
    vga_buffer::clear();
    println!("");
    println!("  _   _ _           _                 ___  ____   ");
    println!(" | \\ | (_)_ __ ___ | |__  _   _ ___ / _ \\/ ___|  ");
    println!(" |  \\| | | '_ ` _ \\| '_ \\| | | / __| | | \\___ \\  ");
    println!(" | |\\  | | | | | | | |_) | |_| \\__ \\ |_| |___) | ");
    println!(" |_| \\_|_|_| |_| |_|_.__/ \\__,_|___/\\___/|____/  ");
    println!("");
    println!("  CYBERDECK v0.4 — CHRONOS+NEXUS+ATLAS+SYNAPSE");
    println!("");

    crate::println!("[BOOT] BootInfo @ {:p} (CHRONO-VECTIS TT)", boot_info as *const _);
    // bootloader 0.9 BootInfo has no physical_memory_offset field — bump heap already set
    gdt::init_gdt();
    interrupts::init_idt();
    interrupts::init_pics();
    pit::init_pit_100hz();
    keyboard::init_keyboard();
    ramfs::init_ramfs();
    chronos::take_snapshot(); // t0
    wasm::init_nexus();
    ipc::init_atlas();

    vga_buffer::init_status_bar();

    println!("[SCHED] Spawning deck tasks...");
    scheduler::spawn_with_priority("solar", solar_task as *const () as u64, 8192, scheduler::Priority::High);
    scheduler::spawn_with_priority("neutron", neutron_task as *const () as u64, 4096, scheduler::Priority::Normal);
    scheduler::spawn_with_priority("io-wait", io_task as *const () as u64, 4096, scheduler::Priority::Low);
    scheduler::spawn_with_priority("idle", dummy_task as *const () as u64, 4096, scheduler::Priority::Idle);
    scheduler::spawn("worker-a", dummy_task as *const () as u64, 4096);
    scheduler::spawn("worker-b", dummy_task as *const () as u64, 4096);
    wasm::spawn_wasm("fib.wasm");

    println!("[DECK] v0.4 online — 1-8 views, TAB cycle");
    println!("[CHRONOS] tick log + snapshots armed");
    println!("[NEXUS] orchard + wasm ready");
    println!("[ATLAS] pipes visible");
    println!("[SYNAPSE] hot-patch armed");

    x86_64::instructions::interrupts::enable();

    let mut last_tick:u64=0;
    let mut snap_timer:u64=0;
    loop{
        deck::deck_handle_keys();
        let cur=scheduler::get_ticks();
        if cur!=last_tick{
            last_tick=cur;
            // chronos record
            chronos::record_tick();
            if let Some(id)=scheduler::SCHEDULER.current_id(){ chronos::record_schedule(id); }
            deck::deck_tick(cur);
            wasm::orchard_tick(cur);
            ipc::tick_atlas(cur);
            // auto snapshot every 300 ticks (3s)
            snap_timer+=1;
            if snap_timer>=300{ snap_timer=0; chronos::take_snapshot(); }
            if let Some(_id)=scheduler::SCHEDULER.schedule(){
                if let Some(cid)=scheduler::SCHEDULER.current_id(){
                    if let Some(t)=scheduler::SCHEDULER.tasks.lock().iter().find(|t| t.id==cid){
                        t.cpu_ms.fetch_add(10, core::sync::atomic::Ordering::Relaxed);
                    }
                }
            }
            if cur%500==0{
                ramfs::append_log(&alloc::format!("[TICK {}] sw={} ev={} snaps={} pipes={}", cur, scheduler::SCHEDULER.get_switches(), chronos::event_count(), chronos::snapshot_count(), ipc::list_pipes().len()));
            }
        }
        if cur%3==0{ deck::deck_render(); }
        x86_64::instructions::hlt();
    }
}
