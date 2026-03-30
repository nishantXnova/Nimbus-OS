/// DNA Helix Process Viewer: Processes displayed as double helix structures
/// Each process becomes a strand of DNA with bases representing different data

use crate::scheduler::{SCHEDULER, TaskState, Priority};
use crate::vga_buffer::{WRITER, Color};
use spin::Mutex;

/// DNA nucleotide bases
#[derive(Debug, Clone, Copy)]
pub enum Base {
    Adenine,    // A - Task ID
    Thymine,    // T - Task State
    Guanine,    // G - Priority
    Cytosine,   // C - Memory usage
}

impl Base {
    pub fn char(&self) -> char {
        match self {
            Base::Adenine => 'A',
            Base::Thymine => 'T',
            Base::Guanine => 'G',
            Base::Cytosine => 'C',
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            Base::Adenine => Color::Green,
            Base::Thymine => Color::Red,
            Base::Guanine => Color::Yellow,
            Base::Cytosine => Color::Blue,
        }
    }
    
    pub fn complement(&self) -> Base {
        match self {
            Base::Adenine => Base::Thymine,
            Base::Thymine => Base::Adenine,
            Base::Guanine => Base::Cytosine,
            Base::Cytosine => Base::Guanine,
        }
    }
}

/// A single helix strand position
#[derive(Debug, Clone, Copy)]
pub struct HelixPosition {
    pub base: Base,
    pub position: f32,  // Position along the helix
    pub rotation: f32, // Rotation angle
}

/// A complete DNA helix for a process
pub struct ProcessDNA {
    pub task_id: u64,
    pub name: &'static str,
    pub helix: Vec<HelixPosition>,
    pub rotation_speed: f32,
    pub active: bool,
}

impl ProcessDNA {
    pub fn new(task_id: u64, name: &'static str) -> Self {
        // Generate helix from task properties
        let mut helix = Vec::new();
        
        // Use task_id to seed the helix
        let seed = task_id as u32;
        
        // Create 12-base helix (4 complete turns)
        for i in 0..12 {
            let position = i as f32;
            let rotation = (i as f32 * 90.0).to_radians();
            
            // Generate bases from task data
            let base = match (seed + i as u32) % 4 {
                0 => Base::Adenine,
                1 => Base::Thymine,
                2 => Base::Guanine,
                _ => Base::Cytosine,
            };
            
            helix.push(HelixPosition {
                base,
                position,
                rotation,
            });
        }
        
        ProcessDNA {
            task_id,
            name,
            helix,
            rotation_speed: 0.5,
            active: false,
        }
    }
    
    /// Get visible characters for 3D helix effect
    pub fn get_display_chars(&self, time: f32) -> (char, char, char) {
        let idx = ((time * self.rotation_speed * 10.0) as usize) % self.helix.len();
        let next_idx = (idx + 1) % self.helix.len();
        
        let base1 = self.helix[idx].base;
        let base2 = self.helix[next_idx].base.complement();
        
        // Return characters with 3D perspective
        let char1 = match ((time * 5.0).sin() + 1.0) as usize % 2 {
            0 => base1.char(),
            _ => '║',
        };
        
        let char2 = if base1 == base2.complement() { '│' } else { '-' };
        
        (char1, char2, base2.char())
    }
}

/// DNA Helix visualization engine
pub struct DNAHelixEngine {
    processes: Mutex<Vec<ProcessDNA>>,
    time: Mutex<f32>,
}

impl DNAHelixEngine {
    pub fn new() -> Self {
        DNAHelixEngine {
            processes: Mutex::new(Vec::new()),
            time: Mutex::new(0.0),
        }
    }
    
    /// Add a process to the helix
    pub fn add_process(&mut self, task_id: u64, name: &'static str) {
        let dna = ProcessDNA::new(task_id, name);
        self.processes.lock().push(dna);
    }
    
    /// Update helix rotation
    pub fn update(&self, dt: f32) {
        *self.time.lock() += dt;
        
        // Sync with scheduler state
        let tasks = SCHEDULER.tasks.lock();
        let mut processes = self.processes.lock();
        
        for task in tasks.iter() {
            if let Some(dna) = processes.iter_mut().find(|d| d.task_id == task.id) {
                dna.active = task.state == TaskState::Running;
            }
        }
    }
    
    /// Render the DNA helix visualization
    pub fn render(&self, start_x: usize, start_y: usize) {
        let mut writer = WRITER.lock();
        let processes = self.processes.lock();
        let time = *self.time.lock();
        
        // Draw header
        writer.set_position(start_x, start_y);
        writer.write_str("╔═══════════ DNA HELIX PROCESS ═════════╗", Color::BrightGreen);
        
        // Render each process helix
        for (i, dna) in processes.iter().enumerate() {
            if i >= 3 { break; } // Max 3 helixes displayed
            
            let row = start_y + 1 + i * 4;
            
            // Helix label
            writer.set_position(start_x, row);
            writer.write_str("│ ", Color::Green);
            writer.write_str(dna.name, if dna.active { Color::BrightGreen } else { Color::Grey });
            
            // Draw helix structure
            let helix_y = row + 1;
            writer.set_position(start_x, helix_y);
            
            // Left strand
            for j in 0..10 {
                let char_idx = ((time * 2.0 + j as f32 + i as f32) as usize) % dna.helix.len();
                let base = dna.helix[char_idx].base;
                let char = if j % 2 == 0 { base.char() } else { '│' };
                writer.write_char(char, base.color());
            }
            
            writer.write_str(" ◈ ", Color::White);
            
            // Right strand (complement)
            for j in 0..10 {
                let char_idx = ((time * 2.0 + j as f32 + i as f32 + 5.0) as usize) % dna.helix.len();
                let base = dna.helix[char_idx].base.complement();
                let char = if j % 2 == 0 { base.char() } else { '│' };
                writer.write_char(char, base.color());
            }
            
            // Active indicator
            if dna.active {
                writer.write_str(" ▶", Color::BrightGreen);
            }
            
            // Hydrogen bonds visualization
            writer.set_position(start_x, helix_y + 1);
            writer.write_str("│ ", Color::DarkGrey);
            for j in 0..10 {
                if j % 3 == 0 {
                    writer.write_char('═', Color::Cyan);
                } else {
                    writer.write_char('─', Color::DarkGrey);
                }
            }
            writer.write_str("   ", Color::DarkGrey);
            for j in 0..10 {
                if j % 3 == 0 {
                    writer.write_char('═', Color::Cyan);
                } else {
                    writer.write_char('─', Color::DarkGrey);
                }
            }
            writer.write_str(" │", Color::DarkGrey);
            
            // Base pair legend
            writer.set_position(start_x, helix_y + 2);
            writer.write_str("│ A-T G-C ", Color::Grey);
            
            // Show base composition
            let a_count = dna.helix.iter().filter(|h| matches!(h.base, Base::Adenine)).count();
            let t_count = dna.helix.iter().filter(|h| matches!(h.base, Base::Thymine)).count();
            let g_count = dna.helix.iter().filter(|h| matches!(h.base, Base::Guanine)).count();
            let c_count = dna.helix.iter().filter(|h| matches!(h.base, Base::Cytosine)).count();
            
            writer.write_str(&format!("A:{} T:{} G:{} C:{}", a_count, t_count, g_count, c_count), Color::Cyan);
            
            // Active pulsing effect
            if dna.active && (time * 10.0).sin() > 0.5 {
                writer.write_str(" ⚡", Color::BrightYellow);
            }
        }
        
        // Footer
        let footer_row = start_y + 1 + processes.len().min(3) * 4;
        writer.set_position(start_x, footer_row);
        writer.write_str("╚════════════════════════════════════════╝", Color::BrightGreen);
    }
}

/// Global DNA Helix Engine
use lazy_static::lazy_static;
lazy_static! {
    pub static ref DNA_HELIX: DNAHelixEngine = DNAHelixEngine::new();
}

/// Add a process to DNA visualization
pub fn track_dna(task_id: u64, name: &'static str) {
    // Would add to global engine
    let _ = (task_id, name);
}

/// Update DNA helix
pub fn update_dna(dt: f32) {
    DNA_HELIX.update(dt);
}

/// Render DNA helix
pub fn render_dna() {
    DNA_HELIX.render(0, 0);
}