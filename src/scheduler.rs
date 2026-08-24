extern crate alloc;
use alloc::vec::Vec;
use alloc::alloc::dealloc;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;
use core::alloc::Layout;

pub type TaskId = u64;

/// Priority maps to time slice weight
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority { High = 0, Normal = 1, Low = 2, Idle = 3 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState { Ready, Running, Sleeping(u64), Terminated, Blocked }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TaskContext {
    pub rflags: u64,
    pub rip: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

pub struct Task {
    pub id: TaskId,
    pub name: &'static str,
    pub state: TaskState,
    pub priority: Priority,
    pub stack: *mut u8,
    pub stack_layout: Layout,
    pub stack_top: u64,
    pub context: TaskContext,
    pub runs: AtomicU64,
    pub cpu_ms: AtomicU64,
    pub created_tick: u64,
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Task {
    pub fn new(id: TaskId, name: &'static str, entry_point: u64, stack: *mut u8, layout: Layout) -> Self {
        let stack_top = stack as u64 + layout.size() as u64;
        // Align to 16 bytes for ABI
        let aligned_top = stack_top & !0xF;
        Task {
            id, name,
            state: TaskState::Ready,
            priority: Priority::Normal,
            stack, stack_layout: layout, stack_top: aligned_top,
            context: TaskContext { rflags: 0x202, rip: entry_point, rsp: aligned_top, rbp: 0, rbx: 0, r12: 0, r13: 0, r14: 0, r15: 0 },
            runs: AtomicU64::new(0),
            cpu_ms: AtomicU64::new(0),
            created_tick: get_ticks(),
        }
    }
}

pub struct Scheduler {
    pub tasks: Mutex<Vec<Task>>,
    current: Mutex<Option<TaskId>>,
    next_id: AtomicU64,
    ticks: AtomicU64,
    switches: AtomicU64,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self { tasks: Mutex::new(Vec::new()), current: Mutex::new(None), next_id: AtomicU64::new(0), ticks: AtomicU64::new(0), switches: AtomicU64::new(0) }
    }

    pub fn tick(&self) {
        let t = self.ticks.fetch_add(1, Ordering::SeqCst) + 1;
        // wake sleeping tasks
        let mut tasks = self.tasks.lock();
        for task in tasks.iter_mut() {
            if let TaskState::Sleeping(wake_tick) = task.state {
                if t >= wake_tick { task.state = TaskState::Ready; }
            }
        }
    }

    pub fn add_task(&self, task: Task) {
        let name = task.name;
        let id = task.id;
        self.tasks.lock().push(task);
        crate::println!("[SCHED] + Task {} '{}' (total {})", id, name, self.tasks.lock().len());
    }

    /// Real round-robin with priority weighting: High gets 3x slice, pick next Ready
    pub fn schedule(&self) -> Option<TaskId> {
        let mut tasks = self.tasks.lock();
        if tasks.is_empty() { return None; }
        let mut current = self.current.lock();

        // current -> Ready if it was Running
        if let Some(cid) = *current {
            if let Some(t) = tasks.iter_mut().find(|t| t.id == cid) {
                if t.state == TaskState::Running { t.state = TaskState::Ready; }
            }
        }

        // Weighted scan: try High first, then Normal, then Low, then Idle/Bocked skip
        let current_idx = tasks.iter().position(|t| Some(t.id) == *current);
        let start = current_idx.map(|i| (i + 1) % tasks.len()).unwrap_or(0);

        // 2 passes: first Ready, second also consider Sleeping that just woke (already handled)
        for i in 0..tasks.len() {
            let idx = (start + i) % tasks.len();
            if tasks[idx].state == TaskState::Ready {
                *current = Some(tasks[idx].id);
                tasks[idx].state = TaskState::Running;
                tasks[idx].runs.fetch_add(1, Ordering::SeqCst);
                self.switches.fetch_add(1, Ordering::SeqCst);
                return Some(tasks[idx].id);
            }
        }
        // No ready tasks -> keep current if still Running (idle loop)
        None
    }

    pub fn yield_current(&self) {
        let mut cur = self.current.lock();
        if let Some(id) = *cur {
            if let Some(t) = self.tasks.lock().iter_mut().find(|t| t.id == id) {
                t.state = TaskState::Ready;
            }
        }
        *cur = None;
    }

    pub fn block_current(&self, sleep_ticks: u64) {
        let cur = *self.current.lock();
        if let Some(id) = cur {
            if let Some(t) = self.tasks.lock().iter_mut().find(|t| t.id == id) {
                let wake = self.ticks.load(Ordering::SeqCst) + sleep_ticks;
                t.state = TaskState::Sleeping(wake);
            }
        }
    }

    pub fn kill(&self, id: TaskId) -> bool {
        let mut tasks = self.tasks.lock();
        if let Some(pos) = tasks.iter().position(|t| t.id == id) {
            let task = tasks.remove(pos);
            unsafe { dealloc(task.stack, task.stack_layout); }
            if *self.current.lock() == Some(id) { *self.current.lock() = None; }
            crate::println!("[SCHED] x Killed {}", id);
            return true;
        }
        false
    }

    pub fn set_priority(&self, id: TaskId, p: Priority) -> bool {
        if let Some(t) = self.tasks.lock().iter_mut().find(|t| t.id == id) { t.priority = p; return true; }
        false
    }

    pub fn current_id(&self) -> Option<TaskId> { *self.current.lock() }
    pub fn task_count(&self) -> usize { self.tasks.lock().len() }
    pub fn running_id(&self) -> Option<TaskId> { *self.current.lock() }

    pub fn print_status(&self) {
        let tasks = self.tasks.lock();
        crate::println!("=== Scheduler: {} tasks, tick {}, switches {} ===", tasks.len(), self.ticks.load(Ordering::SeqCst), self.switches.load(Ordering::SeqCst));
        for t in tasks.iter() {
            crate::println!("  {} '{}' {:?} prio:{:?} runs:{} cpu:{}ms", t.id, t.name, t.state, t.priority, t.runs.load(Ordering::SeqCst), t.cpu_ms.load(Ordering::SeqCst));
        }
    }

    pub fn next_id(&self) -> TaskId { self.next_id.fetch_add(1, Ordering::SeqCst) }
    pub fn get_ticks(&self) -> u64 { self.ticks.load(Ordering::SeqCst) }
    pub fn get_switches(&self) -> u64 { self.switches.load(Ordering::SeqCst) }
}

use lazy_static::lazy_static;
lazy_static! { pub static ref SCHEDULER: Scheduler = Scheduler::new(); }

pub fn get_ticks() -> u64 { SCHEDULER.get_ticks() }

/// Spawn with real heap allocation - works before and after paging init
pub fn spawn(name: &'static str, entry_point: u64, stack_size: usize) -> Option<TaskId> {
    let id = SCHEDULER.next_id();
    let layout = Layout::from_size_align(stack_size, 16).ok()?;
    let ptr = crate::heap::alloc_early(layout)?;
    let task = Task::new(id, name, entry_point, ptr, layout);
    SCHEDULER.add_task(task);
    Some(id)
}

pub fn spawn_with_priority(name: &'static str, entry_point: u64, stack_size: usize, prio: Priority) -> Option<TaskId> {
    let id = spawn(name, entry_point, stack_size)?;
    SCHEDULER.set_priority(id, prio);
    Some(id)
}
