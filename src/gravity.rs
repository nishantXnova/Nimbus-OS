/// Gravity Well Scheduler: Tasks orbit around the CPU as a cosmic gravity well
/// High-priority tasks have tight orbits, low-priority tasks drift to the outer rim

use crate::scheduler::{TaskId, TaskState, Priority, SCHEDULER};
use crate::vga_buffer::{WRITER, Color};
use spin::Mutex;
use core::f32::consts::PI;

/// Orbital parameters for each task
pub struct OrbitalBody {
    pub task_id: TaskId,
    pub name: &'static str,
    pub angle: f32,          // Current orbital angle (radians)
    pub radius: f32,         // Current orbital radius
    pub base_radius: f32,    // Base orbital radius (based on priority)
    pub angular_velocity: f32, // Orbital speed
    pub priority: Priority,
    pub state: TaskState,
}

impl OrbitalBody {
    pub fn new(task_id: TaskId, name: &'static str, priority: Priority) -> Self {
        let base_radius = match priority {
            Priority::High => 8.0,
            Priority::Normal => 16.0,
            Priority::Low => 24.0,
        };
        
        let angular_velocity = match priority {
            Priority::High => 0.05,
            Priority::Normal => 0.03,
            Priority::Low => 0.01,
        };
        
        OrbitalBody {
            task_id,
            name,
            angle: (task_id as f32 * 0.7).rem_euclid(2.0 * PI),
            radius: base_radius,
            base_radius,
            angular_velocity,
            priority,
            state: TaskState::Ready,
        }
    }
    
    /// Update orbital position based on state
    pub fn update(&mut self, dt: f32) {
        // Apply time dilation based on state
        let time_factor = match self.state {
            TaskState::Running => 1.0,    // Full speed
            TaskState::Ready => 0.5,      // Half speed
            TaskState::Sleeping => 0.1,   // Slow motion
            TaskState::Terminated => 0.0, // Frozen
        };
        
        // Update angle based on angular velocity and time dilation
        self.angle += self.angular_velocity * dt * time_factor;
        self.angle = self.angle.rem_euclid(2.0 * PI);
        
        // Drift outward if sleeping, inward if running
        let target_radius = match self.state {
            TaskState::Running => self.base_radius - 2.0,  // Closer when active
            TaskState::Ready => self.base_radius,
            TaskState::Sleeping => self.base_radius + 4.0, // Drifts outward
            TaskState::Terminated => self.base_radius + 10.0, // Far away
        };
        
        // Smooth transition to target radius
        self.radius += (target_radius - self.radius) * 0.1;
    }
    
    /// Get screen position (center at screen middle)
    pub fn screen_position(&self, center_x: f32, center_y: f32) -> (usize, usize) {
        let x = center_x + self.radius * self.angle.cos();
        let y = center_y + self.radius * self.angle.sin();
        (x as usize, y as usize)
    }
    
    /// Get orbital trail positions
    pub fn trail_positions(&self, center_x: f32, center_y: f32, trail_length: usize) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for i in 0..trail_length {
            let trail_angle = self.angle - (i as f32 * 0.15);
            let trail_radius = self.radius + (i as f32 * 0.3);
            let x = center_x + trail_radius * trail_angle.cos();
            let y = center_y + trail_radius * trail_angle.sin();
            positions.push((x as usize, y as usize));
        }
        positions
    }
}

/// The Gravity Well visualization engine
pub struct GravityWell {
    bodies: Mutex<Vec<OrbitalBody>>,
    time: Mutex<f32>,
    center_x: f32,
    center_y: f32,
}

impl GravityWell {
    pub fn new() -> Self {
        GravityWell {
            bodies: Mutex::new(Vec::new()),
            time: Mutex::new(0.0),
            center_x: 40.0,  // Center of screen width
            center_y: 12.0,  // Center of screen height
        }
    }
    
    /// Add a task to the gravity well
    pub fn add_orbiter(&mut self, task_id: TaskId, name: &'static str, priority: Priority) {
        let body = OrbitalBody::new(task_id, name, priority);
        self.bodies.lock().push(body);
    }
    
    /// Update all orbital bodies
    pub fn update(&self, dt: f32) {
        *self.time.lock() += dt;
        
        // Sync with scheduler state
        let tasks = SCHEDULER.tasks.lock();
        let mut bodies = self.bodies.lock();
        
        for task in tasks.iter() {
            if let Some(body) = bodies.iter_mut().find(|b| b.task_id == task.id) {
                body.state = task.state;
            }
        }
        
        // Update orbital positions
        for body in bodies.iter_mut() {
            body.update(dt);
        }
    }
    
    /// Render the gravity well visualization
    pub fn render(&self) {
        let mut writer = WRITER.lock();
        let bodies = self.bodies.lock();
        let time = *self.time.lock();
        
        // Draw the CPU gravity well (center)
        writer.set_position(self.center_x as usize - 3, self.center_y as usize);
        writer.write_str("◉ CPU", Color::White);
        
        // Draw orbital rings
        for ring in &[8.0, 16.0, 24.0] {
            // Draw partial ring (simplified circle)
            for angle in (0..360).step_by(15) {
                let rad = (angle as f32).to_radians();
                let x = (self.center_x + ring * rad.cos()) as usize;
                let y = (self.center_y + ring * rad.sin() / 2.0) as usize;
                if x < 80 && y < 25 {
                    writer.set_position(x, y);
                    writer.write_char('○', Color::DarkGrey);
                }
            }
        }
        
        // Draw orbital bodies
        for body in bodies.iter() {
            // Draw trail
            let trail_positions = body.trail_positions(self.center_x, self.center_y, 5);
            let color = match body.state {
                TaskState::Running => Color::Green,
                TaskState::Ready => Color::Yellow,
                TaskState::Sleeping => Color::Blue,
                TaskState::Terminated => Color::DarkGrey,
            };
            
            for (i, (x, y)) in trail_positions.iter().enumerate() {
                if *x < 80 && *y < 25 {
                    writer.set_position(*x, *y);
                    let alpha = 1.0 - (i as f32 / 5.0);
                    let trail_char = match (alpha * 10.0) as usize {
                        0..=3 => '·',
                        4..=7 => '∙',
                        _ => '•',
                    };
                    writer.write_char(trail_char, Color::DarkGrey);
                }
            }
            
            // Draw the body
            let (x, y) = body.screen_position(self.center_x, self.center_y);
            if x < 80 && y < 25 {
                writer.set_position(x, y);
                
                // Body symbol based on priority
                let symbol = match body.priority {
                    Priority::High => '●',
                    Priority::Normal => '◉',
                    Priority::Low => '○',
                };
                
                writer.write_char(symbol, color);
                
                // Pulse effect for running tasks
                if body.state == TaskState::Running && (time * 10.0).sin() > 0.5 {
                    writer.write_char('◈', color);
                }
            }
        }
        
        // Draw time dilation indicator
        writer.set_position(0, 24);
        writer.write_str("⏱ TIME DILATION: ", Color::Cyan);
        
        // Show time speed indicator
        let time_speed = if bodies.iter().any(|b| b.state == TaskState::Running) {
            "NORMAL ◉◉◉"
        } else {
            "SLOW ◇◇◇"
        };
        writer.write_str(time_speed, Color::Green);
    }
    
    /// Gravitational pulse effect when task changes state
    pub fn gravitational_pulse(&self, task_id: TaskId) {
        let mut bodies = self.bodies.lock();
        if let Some(body) = bodies.iter_mut().find(|b| b.task_id == task_id) {
            // Pulse creates a ripple effect
            body.radius -= 2.0; // Brief inward pull
        }
    }
}

/// Global gravity well instance
use lazy_static::lazy_static;
lazy_static! {
    pub static ref GRAVITY_WELL: GravityWell = GravityWell::new();
}

/// Track a task in the gravity well
pub fn track_task(task_id: TaskId, name: &'static str, priority: Priority) {
    GRAVITY_WELL.add_orbiter(task_id, name, priority);
}

/// Update the gravity simulation
pub fn update_gravity(dt: f32) {
    GRAVITY_WELL.update(dt);
}

/// Render the gravity well
pub fn render_gravity() {
    GRAVITY_WELL.render();
}