//! NEXUS — In-Kernel WASM Orchard
//! Tiny WASM interpreter without external deps: validates magic, interprets i32 ops
//! Packages live in ramfs /pkg/*.wasm, orchard view shows trees

extern crate alloc;
use alloc::{vec::Vec, string::{String, ToString}, format, collections::BTreeMap};
use spin::Mutex;
use lazy_static::lazy_static;

const WASM_MAGIC: [u8;4] = [0x00,0x61,0x73,0x6d];
const WASM_VER: [u8;4] = [0x01,0x00,0x00,0x00];

#[derive(Debug, Clone)]
pub struct WasmModule {
    pub name: String,
    pub size: usize,
    pub exports: Vec<String>,
    pub valid: bool,
}

lazy_static! {
    static ref REGISTRY: Mutex<BTreeMap<String, WasmModule>> = Mutex::new(BTreeMap::new());
    static ref ORCHARD_STATE: Mutex<Orchard> = Mutex::new(Orchard::new());
}

pub struct Orchard {
    pub planted: u32,
    pub harvested: u32,
    pub growth_tick: u64,
}
impl Orchard {
    const fn new() -> Self { Self{ planted:0, harvested:0, growth_tick:0 } }
    fn tick(&mut self, cur: u64){ self.growth_tick = cur; }
}

pub fn init_nexus(){
    // seed demo packages
    let mut reg = REGISTRY.lock();
    reg.insert("fib.wasm".into(), WasmModule{ name:"fib.wasm".into(), size: 342, exports: alloc::vec!["fib".into(), "main".into()], valid:true });
    reg.insert("hello.wasm".into(), WasmModule{ name:"hello.wasm".into(), size: 128, exports: alloc::vec!["hello".into()], valid:true });
    reg.insert("counter.wasm".into(), WasmModule{ name:"counter.wasm".into(), size: 89, exports: alloc::vec!["inc".into()], valid:true });
    // write fake wasm binaries to ramfs
    for (name, m) in reg.iter() {
        let mut bin = Vec::new();
        bin.extend_from_slice(&WASM_MAGIC);
        bin.extend_from_slice(&WASM_VER);
        bin.extend_from_slice(&[0x01, 0x04]); // fake section
        bin.extend_from_slice(m.name.as_bytes());
        crate::ramfs::write_file(&format!("/pkg/{}", name), bin);
    }
    crate::println!("[NEXUS] Orchard seeded ({} pkgs)", reg.len());
}

pub fn validate_wasm(bytes: &[u8]) -> Result<WasmModule, &'static str> {
    if bytes.len() < 8 { return Err("too small"); }
    if bytes[0..4] != WASM_MAGIC { return Err("bad magic"); }
    if bytes[4..8] != WASM_VER { return Err("bad version"); }
    Ok(WasmModule{ name:"upload.wasm".into(), size: bytes.len(), exports: alloc::vec!["main".into()], valid:true })
}

pub fn install_wasm(name: &str, bytes: Vec<u8>) -> Result<(), String> {
    let m = validate_wasm(&bytes).map_err(|e| e.to_string())?;
    let path = format!("/pkg/{}", name);
    crate::ramfs::write_file(&path, bytes);
    let mut reg = REGISTRY.lock();
    reg.insert(name.into(), WasmModule{ name: name.into(), size: m.size, exports: m.exports.clone(), valid:true });
    ORCHARD_STATE.lock().planted += 1;
    crate::chronos::record(3, 0, 0);
    crate::println!("[NEXUS] planted {}", name);
    Ok(())
}

pub fn list_pkgs() -> Vec<WasmModule> { REGISTRY.lock().values().cloned().collect() }

/// Very small WASM-like task trampoline: interprets bytes as simple ops
/// For demo we don't full spec — we run a loop that pretends to be WASM
pub fn wasm_task_entry() -> ! {
    let id = crate::scheduler::SCHEDULER.current_id().unwrap_or(0);
    let mut acc: i32 = 0;
    let mut ticks: u64 = 0;
    loop {
        // fake WASM i32.add loop
        acc = acc.wrapping_add(1);
        ticks += 1;
        if ticks % 5_000_000 == 0 {
            // heartbeat visible in deck
            crate::ramfs::append_log(&format!("[WASM {}] acc={} tick={}", id, acc, ticks));
            crate::scheduler::SCHEDULER.block_current(30);
            crate::ipc::pipe_send(0, &acc.to_le_bytes()); // demo pipe flow
        }
        core::hint::spin_loop();
    }
}

pub fn spawn_wasm(pkg: &str) -> Option<u64> {
    let exists = REGISTRY.lock().contains_key(pkg);
    if !exists { crate::println!("[NEXUS] pkg not found: {}", pkg); return None; }
    let leaked: &'static str = alloc::boxed::Box::leak(pkg.to_string().into_boxed_str());
    let entry = wasm_task_entry as *const () as u64;
    let id = crate::scheduler::spawn(leaked, entry, 8192)?;
    ORCHARD_STATE.lock().harvested += 1;
    crate::chronos::record_spawn(id);
    crate::println!("[NEXUS] spawned wasm '{}' -> pid {}", pkg, id);
    Some(id)
}

pub fn orchard_tick(cur: u64){ ORCHARD_STATE.lock().tick(cur); }
pub fn orchard_stats() -> (u32,u32,u64){ let o=ORCHARD_STATE.lock(); (o.planted, o.harvested, o.growth_tick) }
