//! ATLAS — Visible IPC: pipes you can SEE
//! Zero-copy rings between tasks, rendered as flowing dots in Timeline

use spin::Mutex;
use lazy_static::lazy_static;
use alloc::{vec::Vec, collections::BTreeMap, string::String, format};

const PIPE_CAP: usize = 256;

pub struct Pipe {
    pub id: u64,
    pub from: u64,
    pub to: u64,
    pub name: String,
    buf: Vec<u8>,
    head: usize, tail: usize, len: usize,
    pub sent: u64, pub recv: u64,
    pub flow_pos: u8, // 0..50 for animation
}

impl Pipe {
    fn new(id: u64, from: u64, to: u64, name: String) -> Self {
        Self{ id, from, to, name, buf: alloc::vec![0u8; PIPE_CAP], head:0, tail:0, len:0, sent:0, recv:0, flow_pos:0 }
    }
    fn push(&mut self, data: &[u8]) -> usize {
        let mut n=0;
        for &b in data {
            if self.len >= PIPE_CAP { break; }
            self.buf[self.tail]=b; self.tail=(self.tail+1)%PIPE_CAP; self.len+=1; n+=1; self.sent+=1;
        }
        self.flow_pos = (self.flow_pos + n as u8) % 50;
        n
    }
    fn pop(&mut self, out: &mut [u8]) -> usize {
        let mut n=0;
        for slot in out.iter_mut() {
            if self.len==0 { break; }
            *slot=self.buf[self.head]; self.head=(self.head+1)%PIPE_CAP; self.len-=1; n+=1; self.recv+=1;
        }
        n
    }
    fn fill_pct(&self)->usize{ (self.len*100)/PIPE_CAP }
}

lazy_static! {
    static ref PIPES: Mutex<BTreeMap<u64, Pipe>> = Mutex::new(BTreeMap::new());
    static ref NEXT_ID: Mutex<u64> = Mutex::new(1);
}

pub fn init_atlas(){
    // demo pipes between demo tasks
    pipe_create(1, 2, "solar->neutron");
    pipe_create(2, 3, "neutron->io");
    crate::println!("[ATLAS] {} pipes online", PIPES.lock().len());
}

pub fn pipe_create(from: u64, to: u64, name: &str)->u64 {
    let mut id = NEXT_ID.lock();
    let pid = *id; *id+=1;
    PIPES.lock().insert(pid, Pipe::new(pid, from, to, name.into()));
    crate::chronos::record(5, pid, from);
    pid
}

pub fn pipe_send(pipe_id: u64, data: &[u8])->usize {
    // pipe 0 = broadcast to all
    if pipe_id==0 {
        let mut total=0;
        for p in PIPES.lock().values_mut() { total+=p.push(data); }
        return total;
    }
    PIPES.lock().get_mut(&pipe_id).map(|p| p.push(data)).unwrap_or(0)
}

pub fn pipe_recv(pipe_id: u64, out: &mut [u8])->usize {
    PIPES.lock().get_mut(&pipe_id).map(|p| p.pop(out)).unwrap_or(0)
}

pub fn list_pipes() -> Vec<(u64, String, u64, u64, usize, u64, u64, u8)> {
    PIPES.lock().values().map(|p| (p.id, p.name.clone(), p.from, p.to, p.fill_pct(), p.sent, p.recv, p.flow_pos)).collect()
}

pub fn tick_atlas(cur: u64){
    for p in PIPES.lock().values_mut() {
        p.flow_pos = ((cur as u8).wrapping_add(p.id as u8)) % 50;
    }
}
