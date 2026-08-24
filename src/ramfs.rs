//! Tiny RAM filesystem - spatial desktop files

extern crate alloc;
use alloc::{string::String, vec::Vec, collections::BTreeMap, format};
use spin::Mutex;
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct File { pub data: Vec<u8>, pub created_tick: u64 }

lazy_static! {
    static ref FS: Mutex<BTreeMap<String, File>> = Mutex::new(BTreeMap::new());
}

pub fn init_ramfs() {
    let mut fs = FS.lock();
    fs.insert("/readme.txt".into(), File { data: b"Welcome to CHRONO-VECTIS Cyber-Physical TT OS. Try: help, chronos, nexus".to_vec(), created_tick: 0 });
    fs.insert("/kernel.log".into(), File { data: b"[boot] CHRONO-VECTIS TT deck online\n".to_vec(), created_tick: 0 });
    crate::println!("[RAMFS] mounted / ({} files)", fs.len());
}

pub fn write_file(path: &str, data: Vec<u8>) {
    FS.lock().insert(path.into(), File { data, created_tick: crate::scheduler::get_ticks() });
}
pub fn read_file(path: &str) -> Option<Vec<u8>> { FS.lock().get(path).map(|f| f.data.clone()) }
pub fn list() -> Vec<(String, usize)> {
    FS.lock().iter().map(|(k,v)| (k.clone(), v.data.len())).collect()
}
pub fn append_log(msg: &str) {
    let mut fs = FS.lock();
    if let Some(f) = fs.get_mut("/kernel.log") {
        f.data.extend_from_slice(msg.as_bytes());
        f.data.extend_from_slice(b"\n");
        if f.data.len() > 4096 { f.data.drain(0..1024); }
    }
}
