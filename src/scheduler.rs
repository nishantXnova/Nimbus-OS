use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

/// Task ID type
pub type TaskId = u64;

/// Task priority levels
#[derive(Debug, Clone, Copy)]
pub enum Priority {
    High = 0,
    Normal = 1,
    Low = 2,
}

/// Task state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Sleeping,
    Terminated,
}

/// Task control block (TCB)
pub struct Task {
    pub id: TaskId,
    pub name: &'static str,
    pub state: TaskState,
    pub priority: Priority,
    pub stack: &'static mut [u8],
    pub stack_top: u64,
    // Saved context for context switching
    pub context: TaskContext,
    // Execution tracking
    pub runs: AtomicU64,
}

#[repr(C)]
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

impl Task {
    pub fn new(id: TaskId, name: &'static str, entry_point: u64, stack: &'static mut [u8]) -> Self {
        let stack_top = stack.as_ptr() as u64 + stack.len() as u64;
        
        Task {
            id,
            name,
            state: TaskState::Ready,
            priority: Priority::Normal,
            stack,
            stack_top,
            context: TaskContext {
                rflags: 0x202, // IF flag set
                rip: entry_point,
                rsp: stack_top,
                rbp: 0,
                rbx: 0,
                r12: 0,
                r13: 0,
                r14: 0,
                r15: 0,
            },
            runs: AtomicU64::new(0),
        }
    }
}

/// Simple round-robin scheduler
pub struct Scheduler {
    tasks: Mutex<Vec<Task>>,
    current_task: Mutex<Option<TaskId>>,
    next_id: AtomicU64,
}

impl Scheduler {
    pub const fn new() -> Self {
        Scheduler {
            tasks: Mutex::new(Vec::new()),
            current_task: Mutex::new(None),
            next_id: AtomicU64::new(0),
        }
    }

    pub fn add_task(&self, task: Task) {
        self.tasks.lock().push(task);
        crate::println!("Task '{}' added to scheduler", task.name);
    }

    pub fn schedule(&self) -> Option<TaskId> {
        let mut tasks = self.tasks.lock();
        let mut current = self.current_task.lock();
        
        // Find next ready task using round-robin
        let current_idx = tasks.iter().position(|t| Some(t.id) == *current);
        
        // Find next ready task
        let start = current_idx.map(|i| (i + 1) % tasks.len()).unwrap_or(0);
        for i in 0..tasks.len() {
            let idx = (start + i) % tasks.len();
            if tasks[idx].state == TaskState::Ready {
                *current = Some(tasks[idx].id);
                tasks[idx].state = TaskState::Running;
                tasks[idx].runs.fetch_add(1, Ordering::SeqCst);
                return Some(tasks[idx].id);
            }
        }
        
        None
    }

    pub fn yield_current(&self) {
        let mut current = self.current_task.lock();
        if let Some(id) = *current {
            let mut tasks = self.tasks.lock();
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.state = TaskState::Ready;
                crate::println!("Task '{}' yielded (ran {} times)", task.name, task.runs.load(Ordering::SeqCst));
            }
        }
        *current = None;
    }

    pub fn get_current_context(&self) -> Option<&'static mut TaskContext> {
        let current = self.current_task.lock();
        if let Some(id) = *current {
            let tasks = self.tasks.lock();
            let task = tasks.iter().find(|t| t.id == id)?;
            // SAFETY: We're returning a mutable reference to a specific task's context
            // This is safe as long as no other code accesses this context concurrently
            Some(unsafe { &mut *(core::ptr::addr_of!(task.context) as *mut TaskContext) })
        } else {
            None
        }
    }

    pub fn print_status(&self) {
        let tasks = self.tasks.lock();
        crate::println!("=== Task Scheduler Status ===");
        crate::println!("Total tasks: {}", tasks.len());
        for task in tasks.iter() {
            crate::println!("  Task {}: '{}' - {:?} (ran {} times)", 
                task.id, task.name, task.state, task.runs.load(Ordering::SeqCst));
        }
        crate::println!("============================");
    }

    pub fn next_id(&self) -> TaskId {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }
}

/// Global scheduler instance
use lazy_static::lazy_static;
lazy_static! {
    pub static ref SCHEDULER: Scheduler = Scheduler::new();
}

/// Spawn a new task
pub fn spawn(name: &'static str, entry_point: u64, stack_size: usize) -> Option<TaskId> {
    let id = SCHEDULER.next_id();
    
    // Allocate a stack for the task
    let stack = Box::leak(Box::new([0u8; stack_size]));
    let stack_ref: &'static mut [u8] = unsafe { core::mem::transmute(stack) };
    
    let task = Task::new(id, name, entry_point, stack_ref);
    SCHEDULER.add_task(task);
    
    Some(id)
}