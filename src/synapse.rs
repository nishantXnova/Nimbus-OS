//! SYNAPSE — Live Hot-Patch Lab
//! Edit a running task's entry point without reboot. Deck inline editor -> swap rip.

use spin::Mutex;
use lazy_static::lazy_static;
use alloc::{string::String, vec::Vec, format};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PatchKind { BlinkRate, Counter, Loop, Custom }

impl PatchKind {
    pub fn name(&self)->&'static str{ match self{ Self::BlinkRate=>"blink", Self::Counter=>"counter", Self::Loop=>"loop", Self::Custom=>"custom" } }
    pub fn all()->[Self;4]{ [Self::BlinkRate, Self::Counter, Self::Loop, Self::Custom] }
}

pub struct Patch {
    pub tick: u64,
    pub pid: u64,
    pub kind: PatchKind,
    pub desc: String,
}

lazy_static! {
    static ref PATCHES: Mutex<Vec<Patch>> = Mutex::new(Vec::new());
    static ref EDIT_BUF: Mutex<String> = Mutex::new(String::new());
}

// Trampolines we can hot-swap to
pub fn patch_blink() -> ! {
    let mut c:u64=0; let mut fast=false;
    loop { c+=1; if c%3_000_000==0{ fast=!fast; crate::scheduler::SCHEDULER.block_current(if fast{5}else{40}); } core::hint::spin_loop(); }
}
pub fn patch_counter() -> ! {
    let mut n:u64=0; loop{ n+=1; if n%7_000_000==0{ crate::ramfs::append_log(&format!("[PATCH counter] n={}", n)); crate::scheduler::SCHEDULER.block_current(20);} core::hint::spin_loop();}
}
pub fn patch_loop() -> ! {
    loop{ for _ in 0..1_000_000 { core::hint::spin_loop(); } crate::scheduler::SCHEDULER.block_current(10); }
}

fn entry_for(kind: PatchKind)->u64 {
    match kind {
        PatchKind::BlinkRate => patch_blink as *const () as u64,
        PatchKind::Counter => patch_counter as *const () as u64,
        PatchKind::Loop => patch_loop as *const () as u64,
        PatchKind::Custom => crate::wasm::wasm_task_entry as *const () as u64,
    }
}

pub fn hot_patch(pid: u64, kind: PatchKind) -> bool {
    let mut tasks = crate::scheduler::SCHEDULER.tasks.lock();
    if let Some(t) = tasks.iter_mut().find(|t| t.id==pid) {
        let new_rip = entry_for(kind);
        t.context.rip = new_rip;
        t.context.rsp = t.stack_top; // reset stack
        let tick = crate::scheduler::get_ticks();
        PATCHES.lock().push(Patch{ tick, pid, kind, desc: format!("{} -> {}", pid, kind.name()) });
        crate::chronos::record(3, pid, kind as u64);
        crate::println!("[SYNAPSE] hot-patched pid {} -> {} @ {:#x}", pid, kind.name(), new_rip);
        return true;
    }
    false
}

pub fn list_patches()->Vec<Patch>{ PATCHES.lock().clone() }
pub fn patch_count()->usize{ PATCHES.lock().len() }

// Inline editor buffer for deck
pub fn edit_buf()->String{ EDIT_BUF.lock().clone() }
pub fn edit_push(c: char){ EDIT_BUF.lock().push(c); }
pub fn edit_pop(){ EDIT_BUF.lock().pop(); }
pub fn edit_clear(){ EDIT_BUF.lock().clear(); }
pub fn edit_apply_to(pid: u64){
    let buf = EDIT_BUF.lock().clone();
    let kind = if buf.contains("blink"){ PatchKind::BlinkRate } else if buf.contains("count"){ PatchKind::Counter } else if buf.contains("wasm"){ PatchKind::Custom } else { PatchKind::Loop };
    hot_patch(pid, kind);
}
