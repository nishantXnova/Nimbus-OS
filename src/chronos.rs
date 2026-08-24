//! CHRONOS — time-travel determinism: record every schedule/irq/key, rewind via ring + snapshots
//! 4096 events = ~40s at 100Hz, plus RAMFS snapshots

use spin::Mutex;
use lazy_static::lazy_static;
use crate::scheduler::{TaskId, TaskState};

#[derive(Clone, Copy, Debug)]
pub struct Event {
    pub tick: u64,
    pub kind: u8, // 0=tick 1=schedule 2=kill 3=spawn 4=key 5=pipe 6=snapshot
    pub id: u64,
    pub extra: u64,
}

const RING: usize = 4096;

pub struct Ring {
    buf: [Option<Event>; RING],
    head: usize,
    len: usize,
    snapshots: u32,
}

impl Ring {
    const fn new() -> Self { Self { buf: [None; RING], head: 0, len: 0, snapshots: 0 } }
    fn push(&mut self, e: Event) {
        self.buf[self.head] = Some(e);
        self.head = (self.head + 1) % RING;
        if self.len < RING { self.len += 1; }
    }
    fn iter(&self) -> impl Iterator<Item=Event> + '_ {
        (0..self.len).filter_map(move |i| {
            let idx = (self.head + RING - self.len + i) % RING;
            self.buf[idx]
        })
    }
    fn at_tick(&self, tick: u64) -> Option<Event> { self.iter().find(|e| e.tick == tick) }
}

lazy_static! { static ref LOG: Mutex<Ring> = Mutex::new(Ring::new()); }

// snapshot: full task list dump to ramfs
#[derive(Clone, Debug)]
pub struct Snapshot {
    pub tick: u64,
    pub tasks: alloc::vec::Vec<(TaskId, alloc::string::String, TaskState)>,
}

lazy_static! { static ref SNAPSHOTS: Mutex<alloc::vec::Vec<Snapshot>> = Mutex::new(alloc::vec::Vec::new()); }

pub fn record(kind: u8, id: u64, extra: u64) {
    let tick = crate::scheduler::get_ticks();
    LOG.lock().push(Event{ tick, kind, id, extra });
}

pub fn record_tick(){ record(0, 0, 0); }
pub fn record_schedule(id: TaskId){ record(1, id, 0); }
pub fn record_kill(id: TaskId){ record(2, id, 0); }
pub fn record_spawn(id: TaskId){ record(3, id, 0); }
pub fn record_key(sc: u8){ record(4, sc as u64, 0); }

pub fn take_snapshot(){
    let tick = crate::scheduler::get_ticks();
    let tasks = crate::scheduler::SCHEDULER.tasks.lock().iter().map(|t| (t.id, alloc::string::String::from(t.name), t.state)).collect();
    let snap = Snapshot{ tick, tasks };
    SNAPSHOTS.lock().push(snap.clone());
    LOG.lock().snapshots += 1;
    // persist to ramfs
    let path = alloc::format!("/snap/{}.snap", tick);
    let data = alloc::format!("tick:{} tasks:{}", tick, snap.tasks.len()).into_bytes();
    crate::ramfs::write_file(&path, data);
    record(6, tick, 0);
    crate::println!("[CHRONOS] snapshot @ tick {} ({} snapshots)", tick, SNAPSHOTS.lock().len());
}

pub fn snapshot_count() -> usize { SNAPSHOTS.lock().len() }
pub fn event_count() -> usize { LOG.lock().len }
pub fn last_event() -> Option<Event> { LOG.lock().iter().last() }

/// Seek: returns cloned task list at given tick by replaying snapshots + events
/// For deck rendering we just return snapshot <= tick
pub fn tasks_at(tick: u64) -> alloc::vec::Vec<(TaskId, alloc::string::String, TaskState)> {
    let snaps = SNAPSHOTS.lock();
    if let Some(s) = snaps.iter().rev().find(|s| s.tick <= tick) {
        return s.tasks.clone();
    }
    // fallback: current live
    crate::scheduler::SCHEDULER.tasks.lock().iter().map(|t| (t.id, alloc::string::String::from(t.name), t.state)).collect()
}

pub fn timeline_events(tick_start: u64, tick_end: u64) -> alloc::vec::Vec<Event> {
    LOG.lock().iter().filter(|e| e.tick >= tick_start && e.tick <= tick_end).collect()
}

pub fn stats_line() -> alloc::string::String {
    let l = LOG.lock();
    let s = SNAPSHOTS.lock().len();
    alloc::format!("events:{} snaps:{} head:{}", l.len, s, l.head)
}
