/// Cosmic Event System: The OS reacts to simulated cosmic events
/// Time dilation, space-time anomalies, and quantum fluctuations affect scheduling

use crate::scheduler::SCHEDULER;
use crate::vga_buffer::{WRITER, Color};
use spin::Mutex;
use core::f32::consts::PI;

/// Cosmic event types
#[derive(Debug, Clone, Copy)]
pub enum CosmicEvent {
    None,
    SolarFlare,        // CPU burst activity
    BlackHoleGravity,   // Tasks get pulled into queue
    NeutronStarPulse,  // Rapid task switching
    CosmicRay,         // Random interrupt spike
    DarkMatter,        // Invisible background processing
    Supernova,         // All tasks burst at once
    Wormhole,          // Task teleportation (random swap)
    QuantumFoam,       // Reality instability
}

impl CosmicEvent {
    pub fn symbol(&self) -> &'static str {
        match self {
            CosmicEvent::None => "✧",
            CosmicEvent::SolarFlare => "☀",
            CosmicEvent::BlackHoleGravity => "●",
            CosmicEvent::NeutronStarPulse => "✦",
            CosmicEvent::CosmicRay => "☾",
            CosmicEvent::DarkMatter => "◐",
            CosmicEvent::Supernova => "✶",
            CosmicEvent::Wormhole => "◎",
            CosmicEvent::QuantumFoam => "◇",
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            CosmicEvent::None => "STABLE",
            CosmicEvent::SolarFlare => "SOLAR FLARE",
            CosmicEvent::BlackHoleGravity => "BLACK HOLE",
            CosmicEvent::NeutronStarPulse => "NEUTRON PULSE",
            CosmicEvent::CosmicRay => "COSMIC RAY",
            CosmicEvent::DarkMatter => "DARK MATTER",
            CosmicEvent::Supernova => "SUPERNOVA",
            CosmicEvent::Wormhole => "WORMHOLE",
            CosmicEvent::QuantumFoam => "QUANTUM FOAM",
        }
    }
    
    pub fn effect(&self) -> &'static str {
        match self {
            CosmicEvent::None => "System stable",
            CosmicEvent::SolarFlare => "CPU burst +50%",
            CosmicEvent::BlackHoleGravity => "Tasks compressed",
            CosmicEvent::NeutronStarPulse => "Fast scheduling",
            CosmicEvent::CosmicRay => "Random IRQ spike",
            CosmicEvent::DarkMatter => "Silent processing",
            CosmicEvent::Supernova => "All tasks burst",
            CosmicEvent::Wormhole => "Task positions swapped",
            CosmicEvent::QuantumFoam => "Reality unstable",
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            CosmicEvent::None => Color::Grey,
            CosmicEvent::SolarFlare => Color::Yellow,
            CosmicEvent::BlackHoleGravity => Color::BrightBlack,
            CosmicEvent::NeutronStarPulse => Color::BrightWhite,
            CosmicEvent::CosmicRay => Color::Cyan,
            CosmicEvent::DarkMatter => Color::DarkGrey,
            CosmicEvent::Supernova => Color::BrightRed,
            CosmicEvent::Wormhole => Color::Magenta,
            CosmicEvent::QuantumFoam => Color::BrightMagenta,
        }
    }
}

/// Time dilation factor (1.0 = normal, >1 = faster, <1 = slower)
#[derive(Debug, Clone, Copy)]
pub struct TimeDilation {
    pub factor: f32,
    pub active: bool,
    pub duration: f32,
}

impl TimeDilation {
    pub fn new() -> Self {
        TimeDilation {
            factor: 1.0,
            active: false,
            duration: 0.0,
        }
    }
    
    pub fn apply(&mut self, event: CosmicEvent) {
        match event {
            CosmicEvent::SolarFlare => self.factor = 2.0,  // Time speeds up
            CosmicEvent::BlackHoleGravity => self.factor = 0.1,  // Time slows dramatically
            CosmicEvent::NeutronStarPulse => self.factor = 5.0,  // Hyper time speed
            CosmicEvent::CosmicRay => self.factor = 0.5,
            CosmicEvent::DarkMatter => self.factor = 0.8,
            CosmicEvent::Supernova => self.factor = 3.0,
            CosmicEvent::Wormhole => self.factor = 0.0,  // Time stops momentarily
            CosmicEvent::QuantumFoam => self.factor = ((core::time::SystemTime::now().duration_since(core::time::UNIX_EPOCH).unwrap().as_nanos() % 100) as f32 / 50.0).max(0.1),
            CosmicEvent::None => self.factor = 1.0,
        }
        self.active = true;
        self.duration = 3.0; // Event lasts 3 seconds
    }
    
    pub fn update(&mut self, dt: f32) {
        if self.active {
            self.duration -= dt;
            if self.duration <= 0.0 {
                self.active = false;
                self.factor = 1.0;
            }
        }
    }
}

/// Space-time coordinates in the scheduling universe
#[derive(Debug, Clone, Copy)]
pub struct SpaceTimeCoord {
    pub x: f32,
    pub y: f32,
    pub z: f32,  // Time dimension
    pub curvature: f32,  // Gravity distortion
}

impl SpaceTimeCoord {
    pub fn new(task_id: u64) -> Self {
        SpaceTimeCoord {
            x: (task_id as f32 * 17.3).rem_euclid(80.0),
            y: (task_id as f32 * 23.7).rem_euclid(25.0),
            z: 0.0,
            curvature: 1.0,
        }
    }
    
    pub fn apply_gravity(&mut self, source: (f32, f32), strength: f32) {
        let dx = source.0 - self.x;
        let dy = source.1 - self.y;
        let dist = (dx * dx + dy * dy).sqrt().max(1.0);
        
        // Inverse square law
        let force = strength / (dist * dist);
        
        self.x += dx * force * 0.1;
        self.y += dy * force * 0.1;
        self.curvature += force * 0.01;
    }
}

/// The cosmic event engine
pub struct CosmicEngine {
    current_event: Mutex<CosmicEvent>,
    time_dilation: Mutex<TimeDilation>,
    space_time: Mutex<Vec<SpaceTimeCoord>>,
    event_timer: Mutex<f32>,
    nebula_background: Mutex<Vec<Star>>,
}

#[derive(Debug, Clone, Copy)]
struct Star {
    x: f32,
    y: f32,
    brightness: f32,
    twinkle_offset: f32,
}

impl CosmicEngine {
    pub fn new() -> Self {
        CosmicEngine {
            current_event: Mutex::new(CosmicEvent::None),
            time_dilation: Mutex::new(TimeDilation::new()),
            space_time: Mutex::new(Vec::new()),
            event_timer: Mutex::new(0.0),
            nebula_background: Mutex::new(Vec::new()),
        }
    }
    
    /// Initialize space-time for tasks
    pub fn init_space_time(&mut self, task_count: usize) {
        let mut space_time = self.space_time.lock();
        for i in 0..task_count {
            space_time.push(SpaceTimeCoord::new(i as u64));
        }
        
        // Generate nebula background stars
        let mut stars = self.nebula_background.lock();
        for _ in 0..50 {
            stars.push(Star {
                x: (core::time::SystemTime::now().duration_since(core::time::UNIX_EPOCH).unwrap().as_nanos() % 80) as f32,
                y: (core::time::SystemTime::now().duration_since(core::time::UNIX_EPOCH).unwrap().as_nanos() % 25) as f32,
                brightness: 0.3,
                twinkle_offset: (core::time::SystemTime::now().duration_since(core::time::UNIX_EPOCH).unwrap().as_nanos() % 1000) as f32 / 1000.0,
            });
        }
    }
    
    /// Trigger a random cosmic event
    pub fn trigger_event(&self) {
        let rand = (core::time::SystemTime::now().duration_since(core::time::UNIX_EPOCH).unwrap().as_nanos() % 8) as usize;
        let event = match rand {
            0 => CosmicEvent::SolarFlare,
            1 => CosmicEvent::BlackHoleGravity,
            2 => CosmicEvent::NeutronStarPulse,
            3 => CosmicEvent::CosmicRay,
            4 => CosmicEvent::DarkMatter,
            5 => CosmicEvent::Supernova,
            6 => CosmicEvent::Wormhole,
            7 => CosmicEvent::QuantumFoam,
            _ => CosmicEvent::None,
        };
        
        *self.current_event.lock() = event;
        self.time_dilation.lock().apply(event);
        
        // Apply event effects
        match event {
            CosmicEvent::BlackHoleGravity => {
                // Pull all tasks toward center
                let mut space_time = self.space_time.lock();
                let center = (40.0, 12.0);
                for coord in space_time.iter_mut() {
                    coord.apply_gravity(center, 10.0);
                }
            }
            CosmicEvent::Wormhole => {
                // Swap two random task positions
                let mut space_time = self.space_time.lock();
                if space_time.len() >= 2 {
                    let len = space_time.len();
                    let i1 = (core::time::SystemTime::now().duration_since(core::time::UNIX_EPOCH).unwrap().as_nanos() % len as u128) as usize;
                    let mut i2 = ((core::time::SystemTime::now().duration_since(core::time::UNIX_EPOCH).unwrap().as_nanos() >> 8) % len as u128) as usize;
                    if i2 == i1 { i2 = (i2 + 1) % len; }
                    
                    space_time.swap(i1, i2);
                }
            }
            _ => {}
        }
    }
    
    /// Update the cosmic engine
    pub fn update(&self, dt: f32) {
        let mut event_timer = self.event_timer.lock();
        *event_timer += dt;
        
        // Trigger new event every 10-15 seconds
        let interval = 10.0 + (core::time::SystemTime::now().duration_since(core::time::UNIX_EPOCH).unwrap().as_nanos() % 5) as f32;
        if *event_timer > interval {
            *event_timer = 0.0;
            self.trigger_event();
        }
        
        // Update time dilation
        self.time_dilation.lock().update(dt);
    }
    
    /// Render the cosmic visualization
    pub fn render(&self, time: f32) {
        let mut writer = WRITER.lock();
        let event = *self.current_event.lock();
        let dilation = *self.time_dilation.lock();
        let space_time = self.space_time.lock();
        
        // Render nebula background stars
        let stars = self.nebula_background.lock();
        for star in stars.iter() {
            let twinkle = ((time + star.twinkle_offset) * 3.0).sin() * 0.5 + 0.5;
            let brightness = (star.brightness * twinkle) as usize;
            let char = match brightness {
                0 => '·',
                1 => '•',
                2 => '◦',
                _ => '○',
            };
            writer.set_position(star.x as usize, star.y as usize);
            writer.write_char(char, Color::DarkGrey);
        }
        
        // Draw cosmic event display
        writer.set_position(0, 18);
        writer.write_str("┌─────────── COSMIC EVENTS ───────────┐", event.color());
        
        // Event header
        writer.set_position(0, 19);
        writer.write_char('│', event.color());
        writer.write_str(format!(" {} ", event.symbol()), event.color());
        writer.write_str(event.name(), event.color());
        
        // Effect description
        for _ in 0..(35 - event.name().len()) {
            writer.write_char(' ', event.color());
        }
        writer.write_char('│', event.color());
        
        // Time dilation bar
        writer.set_position(0, 20);
        writer.write_str("│ TIME DILATION: [", event.color());
        
        let dilation_bar = ((dilation.factor / 5.0) * 15.0) as usize;
        for i in 0..15 {
            if i < dilation_bar {
                let char = match dilation.factor {
                    0.0 => '▓',
                    0.1..=0.5 => '█',
                    0.6..=1.5 => '▓',
                    _ => '░',
                };
                writer.write_char(char, Color::BrightGreen);
            } else {
                writer.write_char('░', Color::DarkGrey);
            }
        }
        writer.write_str("] ", event.color());
        
        let factor_str = format!("{:4.2}x", dilation.factor);
        writer.write_str(&factor_str, Color::Cyan);
        
        for _ in 0..(14 - factor_str.len()) {
            writer.write_char(' ', Color::Grey);
        }
        writer.write_char('│', event.color());
        
        // Space-time curvature visualization
        writer.set_position(0, 21);
        writer.write_str("│ SPACETIME: ", event.color());
        
        for (i, coord) in space_time.iter().take(5).enumerate() {
            let curve = coord.curvature;
            let curve_bar = ((curve - 1.0) * 10.0).abs() as usize;
            let curve_char = match i {
                0 => '①',
                1 => '②',
                2 => '③',
                3 => '④',
                _ => '⑤',
            };
            writer.write_char(curve_char, Color::Yellow);
            for _ in 0..curve_bar.min(3) {
                writer.write_char('▓', Color::Yellow);
            }
            writer.write_char(' ', Color::Grey);
        }
        
        for _ in 0..25 {
            writer.write_char(' ', Color::Grey);
        }
        writer.write_char('│', event.color());
        
        writer.set_position(0, 22);
        writer.write_str("└──────────────────────────────────────┘", event.color());
        
        // Event countdown to next
        let next_in = 10.0 - *self.event_timer.lock();
        writer.set_position(0, 23);
        writer.write_str(&format!("◆ NEXT EVENT IN: {:.1}s", next_in), Color::BrightCyan);
    }
    
    /// Get the current time dilation factor
    pub fn get_dilation(&self) -> f32 {
        self.time_dilation.lock().factor
    }
    
    /// Check if currently in a cosmic event
    pub fn is_event_active(&self) -> bool {
        *self.current_event.lock() != CosmicEvent::None
    }
}

/// Global cosmic engine
use lazy_static::lazy_static;
lazy_static! {
    pub static ref COSMIC_ENGINE: CosmicEngine = CosmicEngine::new();
}

/// Initialize the cosmic system
pub fn init_cosmos(task_count: usize) {
    // Note: lazy_static doesn't support mutable initialization easily
    // In a real implementation, this would set up the engine properly
    let _ = task_count;
}

/// Update cosmic events
pub fn update_cosmos(dt: f32) {
    // Simplified update
    let _ = dt;
}

/// Render the cosmic visualization
pub fn render_cosmos(time: f32) {
    // Would render the cosmic engine
    let _ = time;
}