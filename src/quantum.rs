/// Quantum Process States: A visualization system that treats processes
/// as quantum entities with superposition and collapse mechanics

use crate::scheduler::{TaskId, TaskState, Priority};
use crate::vga_buffer::{WRITER, Color};
use spin::Mutex;

/// Quantum state representation of a process
#[derive(Debug, Clone, Copy)]
pub enum QuantumState {
    Superposition,     // Process exists in multiple states
    Running,            // Process is executing (collapsed state)
    Ready,              // Process waiting for CPU
    Sleeping,           // Process blocked on I/O
    Collapsed,          // State just collapsed (visual feedback)
    Entangled,          // Process is linked to another
}

impl QuantumState {
    pub fn symbol(&self) -> char {
        match self {
            QuantumState::Superposition => '⚛',
            QuantumState::Running => '▶',
            QuantumState::Ready => '◐',
            QuantumState::Sleeping => '◑',
            QuantumState::Collapsed => '⬡',
            QuantumState::Entangled => '⫴',
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            QuantumState::Superposition => Color::Magenta,
            QuantumState::Running => Color::Green,
            QuantumState::Ready => Color::Yellow,
            QuantumState::Sleeping => Color::Blue,
            QuantumState::Collapsed => Color::BrightWhite,
            QuantumState::Entangled => Color::Cyan,
        }
    }
}

/// Quantum process descriptor
pub struct QuantumProcess {
    pub id: TaskId,
    pub name: &'static str,
    pub quantum_state: QuantumState,
    pub superposition_prob: f32, // Probability in superposition
    pub energy_level: u32,       // Process energy (priority indicator)
    pub collapse_count: u32,     // Times state collapsed
}

impl QuantumProcess {
    pub fn new(id: TaskId, name: &'static str, priority: Priority) -> Self {
        let energy = match priority {
            Priority::High => 100,
            Priority::Normal => 50,
            Priority::Low => 25,
        };
        
        QuantumProcess {
            id,
            name,
            quantum_state: QuantumState::Superposition,
            superposition_prob: 0.5,
            energy_level: energy,
            collapse_count: 0,
        }
    }
    
    /// Collapse the quantum state to a definite state
    pub fn collapse(&mut self, state: TaskState) {
        self.quantum_state = QuantumState::Collapsed;
        self.collapse_count += 1;
        self.superposition_prob = if state == TaskState::Running { 1.0 } else { 0.0 };
    }
    
    /// Apply wave function decay
    pub fn wave_decay(&mut self) {
        if self.quantum_state == QuantumState::Collapsed {
            self.quantum_state = match self.energy_level {
                75..=100 => QuantumState::Running,
                50..=74 => QuantumState::Ready,
                _ => QuantumState::Sleeping,
            };
        }
    }
}

/// Quantum visualization engine
pub struct QuantumVisualizer {
    processes: Mutex<Vec<QuantumProcess>>,
    wave_offset: Mutex<i32>,
}

impl QuantumVisualizer {
    pub const fn new() -> Self {
        QuantumVisualizer {
            processes: Mutex::new(Vec::new()),
            wave_offset: Mutex::new(0),
        }
    }
    
    /// Add a process to the quantum visualization
    pub fn add_process(&mut self, id: TaskId, name: &'static str, priority: Priority) {
        let proc = QuantumProcess::new(id, name, priority);
        self.processes.lock().push(proc);
    }
    
    /// Render the quantum visualization
    pub fn render(&self) {
        let processes = self.processes.lock();
        let mut writer = WRITER.lock();
        
        // Draw quantum field header
        writer.set_position(42, 22);
        writer.write_str("╔═════ QUANTUM PROCESS ═════╗", Color::Magenta);
        
        // Render each process as a quantum entity
        for (i, proc) in processes.iter().enumerate() {
            if i >= 6 { break; } // Max 6 processes displayed
            
            let row = 23 + i as isize;
            writer.set_position(42, row as usize);
            
            // Draw quantum state symbol
            writer.write_char(proc.quantum_state.symbol(), proc.quantum_state.color());
            
            // Draw process name with quantum styling
            let mut name_chars: [u8; 12] = [b' '; 12];
            for (j, c) in proc.name.as_bytes().iter().enumerate() {
                if j < 12 {
                    name_chars[j] = *c;
                }
            }
            
            // Draw probability indicator (wave function)
            let prob_bar = (proc.superposition_prob * 10.0) as usize;
            let bar_str = match prob_bar {
                0..=3 => "▒▒▒░░░",
                4..=6 => "▓▓▓▒░",
                7..=9 => "▓▓▓▓▓",
                _ => "█████",
            };
            
            writer.write_str(" ", Color::White);
            writer.write_str(core::str::from_utf8(&name_chars).unwrap_or("?"), proc.quantum_state.color());
            writer.write_str(" ", Color::White);
            writer.write_str(bar_str, Color::Cyan);
        }
        
        // Draw quantum footer
        writer.set_position(42, 23 + processes.len().min(6));
        writer.write_str("╚═══════════════════════════╝", Color::Magenta);
        
        // Draw quantum entanglement indicators
        let mut wave = self.wave_offset.lock();
        *wave = (*wave + 1) % 360;
        
        let wave_char = match *wave {
            0..=90 => '~',
            91..=180 => '≈',
            181..=270 => '≋',
            _ => '∿',
        };
        
        writer.set_position(42, 22);
        writer.write_char(wave_char, Color::BrightMagenta);
    }
    
    /// Trigger quantum collapse for a process
    pub fn collapse_process(&self, id: TaskId, state: TaskState) {
        let mut processes = self.processes.lock();
        if let Some(proc) = processes.iter_mut().find(|p| p.id == id) {
            proc.collapse(state);
        }
    }
}

/// Global quantum visualizer
use lazy_static::lazy_static;
lazy_static! {
    pub static ref QUANTUM_VIS: QuantumVisualizer = QuantumVisualizer::new();
}

/// Render all quantum process states
pub fn render_quantum() {
    QUANTUM_VIS.render();
}

/// Add a process to quantum tracking
pub fn track_process(id: TaskId, name: &'static str, priority: Priority) {
    QUANTUM_VIS.add_process(id, name, priority);
}