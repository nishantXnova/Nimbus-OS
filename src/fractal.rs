/// Fractal Process Tree: Processes displayed as recursive fractal patterns
/// Each task spawns a fractal branch representing its child processes

use crate::scheduler::{TaskId, TaskState, Priority, SCHEDULER};
use crate::vga_buffer::{WRITER, Color};
use spin::Mutex;
use core::f32::consts::PI;

/// Fractal types for process visualization
#[derive(Debug, Clone, Copy)]
pub enum FractalType {
    Sierpinski,   // Triangle-based fractal
    Mandelbrot,   // Set-based fractal
    Tree,         // Binary tree fractal
    Koch,         // Snowflake fractal
    Dragon,       // Dragon curve fractal
}

impl FractalType {
    pub fn symbol(&self) -> char {
        match self {
            FractalType::Sierpinski => '△',
            FractalType::Mandelbrot => '⬡',
            FractalType::Tree => '🌲',
            FractalType::Koch => '❄',
            FractalType::Dragon => '🐉',
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            FractalType::Sierpinski => Color::Green,
            FractalType::Mandelbrot => Color::Magenta,
            FractalType::Tree => Color::Yellow,
            FractalType::Koch => Color::Cyan,
            FractalType::Dragon => Color::BrightRed,
        }
    }
    
    pub fn from_id(id: u64) -> Self {
        match id % 5 {
            0 => FractalType::Sierpinski,
            1 => FractalType::Mandelbrot,
            2 => FractalType::Tree,
            3 => FractalType::Koch,
            _ => FractalType::Dragon,
        }
    }
}

/// A fractal node representing a process
#[derive(Debug, Clone)]
pub struct FractalNode {
    pub task_id: TaskId,
    pub name: &'static str,
    pub fractal_type: FractalType,
    pub depth: usize,
    pub angle: f32,
    pub length: f32,
    pub children: Vec<FractalNode>,
    pub active: bool,
}

impl FractalNode {
    pub fn new(task_id: TaskId, name: &'static str, depth: usize, angle: f32) -> Self {
        let fractal_type = FractalType::from_id(task_id);
        
        FractalNode {
            task_id,
            name,
            fractal_type,
            depth,
            angle,
            length: 20.0 - (depth as f32 * 3.0).max(5.0),
            children: Vec::new(),
            active: false,
        }
    }
    
    /// Add a child fractal
    pub fn add_child(&mut self, task_id: TaskId, name: &'static str, angle_offset: f32) {
        let child = FractalNode::new(
            task_id, 
            name, 
            self.depth + 1, 
            self.angle + angle_offset
        );
        self.children.push(child);
    }
    
    /// Get branch characters for rendering
    pub fn get_branch_chars(&self, depth: usize, time: f32) -> Vec<(char, Color, usize)> {
        let mut chars = Vec::new();
        
        if depth > 4 { return chars; }
        
        let branch_char = match depth % 3 {
            0 => '╲',
            1 => '╱',
            _ => '│',
        };
        
        let color = if self.active {
            self.fractal_type.color()
        } else {
            Color::DarkGrey
        };
        
        chars.push((branch_char, color, depth));
        
        // Add fractal decorations
        let pulse = ((time * 2.0 + self.task_id as f32 * 0.5).sin() * 0.5 + 0.5) as usize;
        if pulse > 0 && self.active {
            chars.push(('◈', self.fractal_type.color(), depth));
        }
        
        // Recurse for children
        for child in &self.children {
            chars.extend(child.get_branch_chars(depth + 1, time));
        }
        
        chars
    }
}

/// Fractal tree visualization
pub struct FractalTree {
    root: Mutex<Option<FractalNode>>,
    time: Mutex<f32>,
    max_depth: usize,
}

impl FractalTree {
    pub fn new() -> Self {
        FractalTree {
            root: Mutex::new(None),
            time: Mutex::new(0.0),
            max_depth: 5,
        }
    }
    
    /// Initialize with scheduler tasks
    pub fn init(&mut self) {
        let tasks = SCHEDULER.tasks.lock();
        
        if !tasks.is_empty() {
            // Create root with first task
            let root_task = &tasks[0];
            let mut root = FractalNode::new(
                root_task.id,
                root_task.name,
                0,
                0.0
            );
            root.active = root_task.state == TaskState::Running;
            
            // Add children for remaining tasks
            for (i, task) in tasks.iter().enumerate().skip(1) {
                let angle = (i as f32 * PI / 3.0) - PI / 2.0;
                root.add_child(task.id, task.name, angle);
            }
            
            *self.root.lock() = Some(root);
        }
    }
    
    /// Update fractal animation
    pub fn update(&self, dt: f32) {
        *self.time.lock() += dt;
        
        // Sync with scheduler state
        let tasks = SCHEDULER.tasks.lock();
        let mut root = self.root.lock();
        
        if let Some(ref mut node) = *root {
            // Update root state
            if let Some(task) = tasks.iter().find(|t| t.id == node.task_id) {
                node.active = task.state == TaskState::Running;
            }
            
            // Update children
            for child in &mut node.children {
                if let Some(task) = tasks.iter().find(|t| t.id == child.task_id) {
                    child.active = task.state == TaskState::Running;
                }
            }
        }
    }
    
    /// Render the fractal tree
    pub fn render(&self, x: usize, y: usize) {
        let mut writer = WRITER.lock();
        let time = *self.time.lock();
        
        // Draw header
        writer.set_position(x, y);
        writer.write_str("╔════════ FRACTAL PROCESS TREE ══════╗", Color::BrightGreen);
        
        // Draw fractal visualization
        let root = self.root.lock();
        
        if let Some(ref node) = *root {
            // Draw root node
            writer.set_position(x + 2, y + 2);
            writer.write_char(node.fractal_type.symbol(), node.fractal_type.color());
            writer.write_str(&format!(" {}", node.name), Color::White);
            
            // Draw fractal branches
            let branches = node.get_branch_chars(0, time);
            let mut current_depth = 0;
            let mut col = x + 4;
            
            for (char, color, depth) in branches {
                if depth != current_depth {
                    col = x + 4 + depth * 2;
                    current_depth = depth;
                }
                
                let row = y + 3 + current_depth;
                if row < y + 20 && col < x + 40 {
                    writer.set_position(col, row);
                    writer.write_char(char, color);
                    col += 2;
                }
            }
            
            // Draw child nodes
            let mut child_row = y + 3;
            for child in node.children.iter().take(4) {
                writer.set_position(x + 4, child_row);
                writer.write_char(match child.depth {
                    1 => '├',
                    2 => '│',
                    _ => '└',
                }, Color::DarkGrey);
                writer.write_char(child.fractal_type.symbol(), child.fractal_type.color());
                writer.write_str(&format!(" {}", child.name), 
                    if child.active { Color::White } else { Color::Grey });
                child_row += 1;
            }
            
            // Draw fractal type legend
            writer.set_position(x, y + 18);
            writer.write_str("│ FRACTAL TYPES:", Color::Grey);
            
            let types = [
                (FractalType::Sierpinski, "△"),
                (FractalType::Mandelbrot, "⬡"),
                (FractalType::Tree, "🌲"),
                (FractalType::Koch, "❄"),
                (FractalType::Dragon, "🐉"),
            ];
            
            let mut col = x + 18;
            for (ft, symbol) in types.iter() {
                writer.set_position(col, y + 18);
                writer.write_char(symbol.as_bytes()[0] as char, ft.color());
                col += 3;
            }
            
            writer.write_str("  │", Color::Grey);
            
            // Draw recursion depth indicator
            writer.set_position(x, y + 19);
            writer.write_str("│ DEPTH:", Color::Cyan);
            for i in 0..self.max_depth {
                let depth_bar = (i as f32 * 2.0 + time * 0.5).sin() > 0.3;
                writer.write_char(if depth_bar { '▓' } else { '░' }, Color::Cyan);
            }
            writer.write_str(&format!(" {} levels", self.max_depth), Color::Grey);
            writer.write_str("    │", Color::Grey);
            
            // Draw complexity indicator
            let complexity = node.children.len() * 2 + 1;
            writer.set_position(x, y + 20);
            writer.write_str(&format!("│ COMPLEXITY: {} nodes", complexity), Color::Yellow);
            
            // Fractal iteration counter
            let iteration = ((time * 3.0).floor() as u32) % 100;
            writer.set_position(x, y + 21);
            writer.write_str(&format!("│ ITERATION: {}", iteration), Color::Magenta);
            
            // Pulsing effect on active node
            let pulse = ((time * 4.0).sin() * 0.5 + 0.5) as usize;
            if node.active && pulse > 0 {
                writer.set_position(x + 2, y + 2);
                writer.write_char('◈', node.fractal_type.color());
            }
            
        } else {
            // No tasks to display
            writer.set_position(x + 5, y + 10);
            writer.write_str("No processes to fractalize", Color::DarkGrey);
        }
        
        // Draw sidebar with fractal info
        writer.set_position(x, y + 3);
        writer.write_str("║", Color::DarkGrey);
        
        // Fractal generation status
        writer.set_position(x, y + 4);
        writer.write_str("║ GEN:", Color::Grey);
        writer.write_str(&format!("{:3}", ((time * 2.0) as u32) % 100), Color::BrightGreen);
        
        writer.set_position(x, y + 5);
        writer.write_str("║ RECURSION:", Color::Grey);
        
        let recursion_level = ((time * 1.5).sin() * 2.5 + 2.5) as usize;
        for i in 0..5 {
            writer.write_char(if i < recursion_level { '█' } else { '░' }, 
                if i < recursion_level { Color::Green } else { Color::DarkGrey });
        }
        
        writer.set_position(x, y + 6);
        writer.write_str("║ CHAOS:", Color::Grey);
        
        let chaos = ((time * 2.5).sin() * 0.5 + 0.5) * 100.0;
        writer.write_str(&format!("{:5.1}%", chaos), 
            if chaos > 70.0 { Color::Red } else if chaos > 40.0 { Color::Yellow } else { Color::Green });
        
        // Footer
        writer.set_position(x, y + 22);
        writer.write_str("╚════════════════════════════════════╝", Color::BrightGreen);
    }
    
    /// Get the current fractal type for a task
    pub fn get_task_fractal(&self, task_id: TaskId) -> Option<FractalType> {
        let root = self.root.lock();
        
        if let Some(ref node) = *root {
            if node.task_id == task_id {
                return Some(node.fractal_type);
            }
            for child in &node.children {
                if child.task_id == task_id {
                    return Some(child.fractal_type);
                }
            }
        }
        None
    }
}

/// Global Fractal Tree
use lazy_static::lazy_static;
lazy_static! {
    pub static ref FRACTAL_TREE: FractalTree = FractalTree::new();
}

/// Initialize fractal tree
pub fn init_fractal() {
    let mut tree = FractalTree::new();
    tree.init();
    // Note: Would set global
}

/// Update fractal tree
pub fn update_fractal(dt: f32) {
    FRACTAL_TREE.update(dt);
}

/// Render fractal tree
pub fn render_fractal() {
    FRACTAL_TREE.render(40, 14);
}