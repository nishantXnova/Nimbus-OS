use crate::vga_buffer::{WRITER, Color};
use crate::scheduler::{SCHEDULER, TaskId, TaskState};
use spin::Mutex;

/// Process Nebula: A visual representation of the task constellation
/// Each task appears as a star in the "nebula" of system processes
pub struct ProcessNebula {
    nebula_col: usize,
    nebula_row: usize,
    pulse_timer: Mutex<u32>,
}

impl ProcessNebula {
    pub const fn new() -> Self {
        ProcessNebula {
            nebula_col: 2,
            nebula_row: 23, // Bottom of the screen
            pulse_timer: Mutex::new(0),
        }
    }

    /// Render the process nebula visualization
    pub fn render(&self) {
        let tasks = SCHEDULER.tasks.lock();
        let mut writer = WRITER.lock();
        
        // Draw header
        writer.set_position(0, self.nebula_row);
        writer.write_str("╔═══════════ PROCESS NEBULA ═══════════╗", Color::Cyan);
        
        // Draw stars for each task
        let star_count = tasks.len().min(20);
        let mut col = 1;
        let mut row = self.nebula_row + 1;
        
        for (i, task) in tasks.iter().enumerate() {
            if i >= 20 { break; } // Max 20 stars
            
            // Choose star color based on state
            let color = match task.state {
                TaskState::Running => Color::Green,
                TaskState::Ready => Color::Yellow,
                TaskState::Sleeping => Color::Blue,
                TaskState::Terminated => Color::DarkGrey,
            };
            
            // Star character based on priority
            let star = match task.priority {
                crate::scheduler::Priority::High => '★',
                crate::scheduler::Priority::Normal => '☆',
                crate::scheduler::Priority::Low => '○',
            };
            
            // Draw the star
            writer.set_position(col, row);
            writer.write_char(star, color);
            
            col += 2;
            if col >= 40 {
                col = 1;
                row += 1;
            }
        }
        
        // Fill remaining space
        while row <= self.nebula_row + 2 {
            writer.set_position(0, row);
            writer.write_str("║                                    ║", Color::Cyan);
            row += 1;
        }
        
        // Draw footer
        writer.set_position(0, self.nebula_row + 3);
        writer.write_str("╚════════════════════════════════════╝", Color::Cyan);
        
        // Draw legend
        writer.set_position(1, self.nebula_row + 4);
        writer.write_str("★High ★Normal ☆Low ○Sleep", Color::Grey);
    }

    /// Update the nebula with pulse effect
    pub fn pulse(&self) {
        let mut timer = self.pulse_timer.lock();
        *timer = (*timer + 1) % 100;
        
        if *timer < 20 {
            // Pulse animation
            let writer = &mut *WRITER.lock();
            writer.set_position(0, self.nebula_row);
            writer.write_str("╔═══════════ PROCESS NEBULA ═══════════╗", Color::BrightCyan);
        }
    }
}

/// Global process nebula instance
use lazy_static::lazy_static;
lazy_static! {
    pub static ref PROCESS_NEBULA: ProcessNebula = ProcessNebula::new();
}

/// Render the process nebula (call periodically)
pub fn render_nebula() {
    PROCESS_NEBULA.render();
}

/// Pulse the nebula effect
pub fn pulse_nebula() {
    PROCESS_NEBULA.pulse();
}