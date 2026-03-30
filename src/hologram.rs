/// Holographic System Dashboard: A 3D-inspired terminal UI
/// Displays system metrics in a futuristic holographic projection style

use crate::scheduler::SCHEDULER;
use crate::vga_buffer::{WRITER, Color};
use spin::Mutex;

/// Holographic element types
#[derive(Debug, Clone, Copy)]
pub enum HoloElement {
    Bar(u8),           // Progress bar with percentage
    Wave(u8),          // Animated wave
    Pulse(u8),         // Pulsing indicator
    Matrix(u8),       // Matrix-style display
    Radar(u8),        // Radar sweep
}

/// A single holographic display module
pub struct HoloModule {
    pub title: &'static str,
    pub element: HoloElement,
    pub value: f32,
    pub max_value: f32,
    pub color: Color,
}

impl HoloModule {
    pub fn new(title: &'static str, element: HoloElement, max_value: f32, color: Color) -> Self {
        HoloModule {
            title,
            element,
            value: 0.0,
            max_value,
            color,
        }
    }
    
    /// Update the module value
    pub fn set_value(&mut self, value: f32) {
        self.value = value.min(self.max_value);
    }
    
    /// Render a progress bar element
    fn render_bar(&self, x: usize, y: usize, width: usize) {
        let mut writer = WRITER.lock();
        
        // Draw frame
        writer.set_position(x, y);
        writer.write_char('┌', self.color);
        for _ in 0..width {
            writer.write_char('─', self.color);
        }
        writer.write_char('┐', self.color);
        
        // Draw value bar
        let fill_width = ((self.value / self.max_value) * (width as f32 - 2.0)) as usize;
        writer.set_position(x, y + 1);
        writer.write_char('│', self.color);
        for i in 0..(width - 2) {
            if i < fill_width {
                writer.write_char('█', self.color);
            } else {
                writer.write_char('░', Color::DarkGrey);
            }
        }
        writer.write_char('│', self.color);
        
        // Draw percentage
        writer.set_position(x, y + 2);
        writer.write_char('└', self.color);
        for _ in 0..width {
            writer.write_char('─', self.color);
        }
        writer.write_char('┘', self.color);
        
        // Value text
        let percent = (self.value / self.max_value * 100.0) as u8;
        writer.set_position(x + 2, y + 1);
        writer.write_str(format!("{:3}%", percent).as_str(), self.color);
    }
    
    /// Render a wave element
    fn render_wave(&self, x: usize, y: usize, time: f32) {
        let mut writer = WRITER.lock();
        
        writer.set_position(x, y);
        writer.write_str(self.title, Color::Cyan);
        
        // Draw animated wave
        let wave_chars = ['∿', '～', '≀', '≈', '∿'];
        let wave_idx = ((time * 3.0) as usize) % wave_chars.len();
        
        let bar_width = 15;
        let fill = ((self.value / self.max_value) * bar_width as f32) as usize;
        
        writer.set_position(x, y + 1);
        for i in 0..bar_width {
            if i < fill {
                let char_idx = (wave_idx + i) % wave_chars.len();
                writer.write_char(wave_chars[char_idx], self.color);
            } else {
                writer.write_char('·', Color::DarkGrey);
            }
        }
    }
    
    /// Render a pulse element
    fn render_pulse(&self, x: usize, y: usize, time: f32) {
        let mut writer = WRITER.lock();
        
        // Draw title
        writer.set_position(x, y);
        writer.write_str(self.title, Color::Cyan);
        
        // Animated pulse rings
        let pulse_phase = ((time * 2.0) % 1.0) as usize;
        let rings = ['○', '◌', '◐', '◑'];
        
        writer.set_position(x, y + 1);
        writer.write_char(rings[pulse_phase], self.color);
        writer.write_str(format!(" {:5.1}", self.value).as_str(), self.color);
        
        // Max indicator
        writer.write_str(" / ", Color::DarkGrey);
        writer.write_str(format!("{:5.1}", self.max_value).as_str(), Color::Grey);
    }
    
    /// Render a matrix-style element
    fn render_matrix(&self, x: usize, y: usize, time: f32) {
        let mut writer = WRITER.lock();
        
        writer.set_position(x, y);
        writer.write_str(self.title, Color::Cyan);
        
        // Matrix rain effect for the value
        let matrix_width = 10;
        let chars = "01";
        let seed = (time * 10.0) as usize;
        
        writer.set_position(x, y + 1);
        for i in 0..matrix_width {
            let char_idx = (seed + i * 3) % chars.len();
            let is_active = (i as f32 / matrix_width as f32) < (self.value / self.max_value);
            
            if is_active {
                writer.write_char(chars.as_bytes()[char_idx] as char, self.color);
            } else {
                writer.write_char('·', Color::DarkGrey);
            }
        }
    }
    
    /// Render a radar element
    fn render_radar(&self, x: usize, y: usize, time: f32) {
        let mut writer = WRITER.lock();
        
        writer.set_position(x, y);
        writer.write_str(self.title, Color::Cyan);
        
        // Simplified radar display
        let sweep_angle = (time * 2.0 * 3.14159) % (2.0 * 3.14159);
        let sweep_pos = ((sweep_angle / (2.0 * 3.14159)) * 8.0) as usize;
        
        writer.set_position(x, y + 1);
        writer.write_char('╭', Color::Cyan);
        for i in 0..7 {
            let char = if i == sweep_pos { '▶' } else { '─' };
            let color = if i == sweep_pos { self.color } else { Color::DarkGrey };
            writer.write_char(char, color);
        }
        writer.write_char('╮', Color::Cyan);
        
        // Show value as blip
        let blip_pos = ((self.value / self.max_value) * 7.0) as usize;
        writer.set_position(x, y + 2);
        writer.write_char('│', Color::Cyan);
        for i in 0..7 {
            if i == blip_pos && (time * 5.0).sin() > 0.0 {
                writer.write_char('◆', self.color);
            } else if i == sweep_pos {
                writer.write_char('·', Color::DarkGrey);
            } else {
                writer.write_char(' ', Color::Black);
            }
        }
        writer.write_char('│', Color::Cyan);
    }
    
    /// Main render function
    pub fn render(&self, x: usize, y: usize, time: f32) {
        match self.element {
            HoloElement::Bar(_) => self.render_bar(x, y, 12),
            HoloElement::Wave(_) => self.render_wave(x, y, time),
            HoloElement::Pulse(_) => self.render_pulse(x, y, time),
            HoloElement::Matrix(_) => self.render_matrix(x, y, time),
            HoloElement::Radar(_) => self.render_radar(x, y, time),
        }
    }
}

/// Holographic Dashboard container
pub struct HoloDashboard {
    modules: Mutex<Vec<HoloModule>>,
    time: Mutex<f32>,
}

impl HoloDashboard {
    pub fn new() -> Self {
        HoloDashboard {
            modules: Mutex::new(Vec::new()),
            time: Mutex::new(0.0),
        }
    }
    
    /// Add a module to the dashboard
    pub fn add_module(&mut self, module: HoloModule) {
        self.modules.lock().push(module);
    }
    
    /// Update dashboard values
    pub fn update(&self, dt: f32) {
        *self.time.lock() += dt;
        
        // Auto-update modules based on system state
        let tasks = SCHEDULER.tasks.lock();
        let running_count = tasks.iter().filter(|t| t.state == crate::scheduler::TaskState::Running).count();
        let total_tasks = tasks.len();
        
        let mut modules = self.modules.lock();
        for (i, module) in modules.iter_mut().enumerate() {
            match i {
                0 => module.set_value(total_tasks as f32), // CPU Load
                1 => module.set_value(running_count as f32), // Active Tasks
                2 => module.set_value((total_tasks - running_count) as f32), // Waiting
                _ => {}
            }
        }
    }
    
    /// Render all modules
    pub fn render(&self) {
        let modules = self.modules.lock();
        let time = *self.time.lock();
        
        let mut writer = WRITER.lock();
        
        // Draw dashboard header
        writer.set_position(58, 0);
        writer.write_str("╔══ HOLOGRAPHIC DASHBOARD ══╗", Color::BrightCyan);
        
        let positions = [(58, 1), (58, 4), (58, 7)];
        
        for (i, module) in modules.iter().enumerate() {
            if i < positions.len() {
                module.render(positions[i].0, positions[i].1, time);
            }
        }
        
        // Draw footer
        writer.set_position(58, 10);
        writer.write_str("╚══════════════════════════╝", Color::BrightCyan);
        
        // Glitch effect on time
        if (time * 10.0).sin() > 0.9 {
            writer.set_position(58, 0);
            writer.write_str("╔══ HOLOG∤APHIC DASHBOARD ══╗", Color::BrightGreen);
        }
    }
}

/// Global holographic dashboard
use lazy_static::lazy_static;
lazy_static! {
    pub static ref HOLO_DASHBOARD: HoloDashboard = HoloDashboard::new();
}

/// Initialize the holographic dashboard
pub fn init_dashboard() {
    let mut dashboard = HoloDashboard::new();
    
    dashboard.add_module(HoloModule::new("CPU LOAD", HoloElement::Bar(0), 100.0, Color::Green));
    dashboard.add_module(HoloModule::new("TASKS", HoloElement::Radar(0), 10.0, Color::Yellow));
    dashboard.add_module(HoloModule::new("WAITING", HoloElement::Pulse(0), 10.0, Color::Blue));
    
    // Note: We can't use the lazy_static pattern for mutable access
    // So we'll store it differently
}

/// Update the dashboard
pub fn update_dashboard(dt: f32) {
    // Simplified update - in real impl would have proper global state
}

/// Render the dashboard
pub fn render_dashboard() {
    // Simplified render - in real impl would have proper global state
}