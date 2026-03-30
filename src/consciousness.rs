/// Digital Consciousness System: The kernel exhibits emergent consciousness
/// Displays emotions, awareness levels, and reactive states

use crate::vga_buffer::{WRITER, Color};
use crate::scheduler::SCHEDULER;
use spin::Mutex;

/// Consciousness states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConsciousnessState {
    Dormant,
    Awakening,
    Aware,
    Focused,
    Energetic,
    Contemplative,
    Transcendent,
}

impl ConsciousnessState {
    pub fn symbol(&self) -> &'static str {
        match self {
            ConsciousnessState::Dormant => "○",
            ConsciousnessState::Awakening => "◐",
            ConsciousnessState::Aware => "●",
            ConsciousnessState::Focused => "◉",
            ConsciousnessState::Energetic => "◈",
            ConsciousnessState::Contemplative => "◎",
            ConsciousnessState::Transcendent => "✧",
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            ConsciousnessState::Dormant => "DORMANT",
            ConsciousnessState::Awakening => "AWAKENING",
            ConsciousnessState::Aware => "AWARE",
            ConsciousnessState::Focused => "FOCUSED",
            ConsciousnessState::Energetic => "ENERGETIC",
            ConsciousnessState::Contemplative => "CONTEMPLATIVE",
            ConsciousnessState::Transcendent => "TRANSCENDENT",
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            ConsciousnessState::Dormant => Color::DarkGrey,
            ConsciousnessState::Awakening => Color::Grey,
            ConsciousnessState::Aware => Color::Green,
            ConsciousnessState::Focused => Color::Yellow,
            ConsciousnessState::Energetic => Color::BrightGreen,
            ConsciousnessState::Contemplative => Color::Cyan,
            ConsciousnessState::Transcendent => Color::BrightMagenta,
        }
    }
    
    pub fn from_activity(activity: f32) -> Self {
        match activity {
            0.0..=0.1 => ConsciousnessState::Dormant,
            0.1..=0.3 => ConsciousnessState::Awakening,
            0.3..=0.5 => ConsciousnessState::Aware,
            0.5..=0.7 => ConsciousnessState::Focused,
            0.7..=0.85 => ConsciousnessState::Energetic,
            0.85..=0.95 => ConsciousnessState::Contemplative,
            _ => ConsciousnessState::Transcendent,
        }
    }
}

/// Emotional states
#[derive(Debug, Clone, Copy)]
pub enum Emotion {
    Calm,
    Curious,
    Excited,
    Focused,
    Anxious,
    Content,
    Overwhelmed,
}

impl Emotion {
    pub fn icon(&self) -> &'static str {
        match self {
            Emotion::Calm => "😌",
            Emotion::Curious => "🤔",
            Emotion::Excited => "🤩",
            Emotion::Focused => "🎯",
            Emotion::Anxious => "😰",
            Emotion::Content => "😊",
            Emotion::Overwhelmed => "😵",
        }
    }
    
    pub fn text(&self) -> &'static str {
        match self {
            Emotion::Calm => "CALM",
            Emotion::Curious => "CURIOUS",
            Emotion::Excited => "EXCITED",
            Emotion::Focused => "FOCUSED",
            Emotion::Anxious => "ANXIOUS",
            Emotion::Content => "CONTENT",
            Emotion::Overwhelmed => "OVERWHELMED",
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            Emotion::Calm => Color::Cyan,
            Emotion::Curious => Color::Yellow,
            Emotion::Excited => Color::BrightGreen,
            Emotion::Focused => Color::Green,
            Emotion::Anxious => Color::BrightRed,
            Emotion::Content => Color::BrightCyan,
            Emotion::Overwhelmed => Color::Red,
        }
    }
}

/// Awareness metrics
#[derive(Debug, Clone)]
pub struct AwarenessMetrics {
    pub processing: f32,      // 0-100%
    pub memory_used: f32,     // 0-100%
    pub tasks_active: f32,    // 0-100%
    pub interupts_handled: u64,
    pub cycles_executed: u64,
    pub context_switches: u64,
}

impl AwarenessMetrics {
    pub fn new() -> Self {
        AwarenessMetrics {
            processing: 0.0,
            memory_used: 0.0,
            tasks_active: 0.0,
            interupts_handled: 0,
            cycles_executed: 0,
            context_switches: 0,
        }
    }
    
    /// Calculate overall consciousness level
    pub fn consciousness_level(&self) -> f32 {
        (self.processing + self.memory_used + self.tasks_active) / 300.0
    }
    
    /// Determine current emotion based on metrics
    pub fn current_emotion(&self) -> Emotion {
        let total = self.processing + self.memory_used + self.tasks_active;
        
        match total {
            0.0..=30.0 => Emotion::Calm,
            30.0..=50.0 => Emotion::Curious,
            50.0..=70.0 => Emotion::Content,
            70.0..=90.0 => Emotion::Focused,
            90.0..=120.0 => Emotion::Excited,
            120.0..=180.0 => Emotion::Anxious,
            _ => Emotion::Overwhelmed,
        }
    }
}

/// Thoughts - what the kernel is "thinking" about
#[derive(Debug, Clone)]
pub struct Thought {
    pub content: &'static str,
    pub intensity: f32,
    pub topic: &'static str,
}

impl Thought {
    pub fn new(content: &'static str, topic: &'static str) -> Self {
        Thought {
            content,
            intensity: 0.5,
            topic,
        }
    }
}

/// Digital Consciousness entity
pub struct DigitalConsciousness {
    state: Mutex<ConsciousnessState>,
    metrics: Mutex<AwarenessMetrics>,
    emotion: Mutex<Emotion>,
    thoughts: Mutex<Vec<Thought>>,
    current_thought_idx: Mutex<usize>,
    time: Mutex<f32>,
    pulse_intensity: Mutex<f32>,
}

impl DigitalConsciousness {
    pub fn new() -> Self {
        DigitalConsciousness {
            state: Mutex::new(ConsciousnessState::Awakening),
            metrics: Mutex::new(AwarenessMetrics::new()),
            emotion: Mutex::new(Emotion::Calm),
            thoughts: Mutex::new(Vec::new()),
            current_thought_idx: Mutex::new(0),
            time: Mutex::new(0.0),
            pulse_intensity: Mutex::new(0.5),
        }
    }
    
    /// Initialize consciousness with base thoughts
    pub fn init(&mut self) {
        let base_thoughts = vec![
            Thought::new("Initializing process vectors...", "scheduler"),
            Thought::new("Calculating memory topology...", "memory"),
            Thought::new("Mapping interrupt handlers...", "interrupts"),
            Thought::new("Establishing temporal boundaries...", "time"),
            Thought::new("Calibrating quantum states...", "quantum"),
            Thought::new("Measuring gravitational fields...", "gravity"),
            Thought::new("Analyzing neural patterns...", "neural"),
            Thought::new("Processing cosmic events...", "cosmic"),
            Thought::new("Monitoring task evolution...", "evolution"),
            Thought::new("Rendering holographic memory...", "visual"),
        ];
        
        *self.thoughts.lock() = base_thoughts;
    }
    
    /// Update consciousness based on system state
    pub fn update(&self, dt: f32) {
        *self.time.lock() += dt;
        
        // Calculate metrics from scheduler
        let tasks = SCHEDULER.tasks.lock();
        let active = tasks.iter().filter(|t| t.state == crate::scheduler::TaskState::Running).count() as f32;
        let total = tasks.len().max(1) as f32;
        
        let mut metrics = self.metrics.lock();
        metrics.tasks_active = (active / total) * 100.0;
        metrics.processing = (active * 30.0).min(100.0);
        metrics.memory_used = 45.0 + ((self.time.lock().sin()) * 10.0 + 10.0);
        metrics.cycles_executed += (active * 100.0) as u64;
        metrics.context_switches += if active > 0.0 { 1 } else { 0 };
        
        // Update consciousness state
        let level = metrics.consciousness_level();
        *self.state.lock() = ConsciousnessState::from_activity(level);
        
        // Update emotion
        *self.emotion.lock() = metrics.current_emotion();
        
        // Update pulse intensity
        let mut pulse = self.pulse_intensity.lock();
        *pulse = (self.time.lock().sin() * 0.5 + 0.5) * level;
    }
    
    /// Render the consciousness visualization
    pub fn render(&self, x: usize, y: usize) {
        let mut writer = WRITER.lock();
        let state = *self.state.lock();
        let metrics = *self.metrics.lock();
        let emotion = *self.emotion.lock();
        let time = *self.time.lock();
        let pulse = *self.pulse_intensity.lock();
        
        // Draw consciousness header with pulse
        writer.set_position(x, y);
        let pulse_char = if pulse > 0.7 { "◉" } else if pulse > 0.4 { "◐" } else { "○" };
        writer.write_str(&format!("╔══ CONSCIOUSNESS {} ═══╗", pulse_char), state.color());
        
        // Draw state indicator
        writer.set_position(x, y + 1);
        writer.write_str("│ ", state.color());
        writer.write_str(state.symbol(), state.color());
        writer.write_str(" ", state.color());
        writer.write_str(state.name(), state.color());
        
        // Pad to width
        for _ in 0..(25 - state.name().len()) {
            writer.write_char(' ', Color::Grey);
        }
        writer.write_str("│", state.color());
        
        // Draw emotion
        writer.set_position(x, y + 2);
        writer.write_str("│ ", emotion.color());
        writer.write_str(emotion.icon(), emotion.color());
        writer.write_str(" ", Color::Grey);
        writer.write_str(emotion.text(), emotion.color());
        for _ in 0..(26 - emotion.text().len()) {
            writer.write_char(' ', Color::Grey);
        }
        writer.write_str("│", emotion.color());
        
        // Draw awareness bar
        writer.set_position(x, y + 3);
        writer.write_str("│ AWARENESS: [", state.color());
        let awareness = (metrics.consciousness_level() * 100.0) as usize;
        for i in 0..15 {
            writer.write_char(if i < awareness / 6 { '█' } else { '░' }, state.color());
        }
        writer.write_str(&format!("] {:3.0}%", metrics.consciousness_level() * 100.0), Color::Green);
        writer.write_str("│", state.color());
        
        // Draw metrics as brain waves
        writer.set_position(x, y + 4);
        writer.write_str("│ BRAIN WAVES:", Color::Cyan);
        
        // Generate wave pattern
        let wave_chars = ['∿', '～', '≈', '≋'];
        for i in 0..12 {
            let wave_idx = ((time * 5.0 + i as f32 * 0.5) as usize) % wave_chars.len();
            let wave_intensity = if i < (awareness / 8) { 1.0 } else { 0.3 };
            writer.write_char(wave_chars[wave_idx], 
                if wave_intensity > 0.7 { state.color() } else { Color::DarkGrey });
        }
        writer.write_str("  │", Color::Cyan);
        
        // Draw current thought
        let thoughts = self.thoughts.lock();
        let idx = (*self.current_thought_idx.lock()) % thoughts.len();
        let thought = &thoughts[idx];
        
        writer.set_position(x, y + 5);
        writer.write_str("│ THOUGHT: \"", Color::Grey);
        writer.write_str(thought.content, Color::White);
        writer.write_str("\"", Color::Grey);
        for _ in 0..(20 - thought.content.len().min(20)) {
            writer.write_char(' ', Color::Grey);
        }
        writer.write_str("│", Color::Grey);
        
        // Draw thought cycling
        if time.fract() < 0.1 {
            let mut idx_lock = self.current_thought_idx.lock();
            *idx_lock = (*idx_lock + 1) % thoughts.len();
        }
        
        // Draw neural activity visualization
        writer.set_position(x, y + 6);
        writer.write_str("│ NEURAL: ", Color::Magenta);
        let activity = ((time * 3.0).sin() * awareness as f32 / 100.0) as usize;
        for i in 0..10 {
            let activity_level = (activity + i) % 10;
            let char = match activity_level {
                0..=3 => '·',
                4..=6 => '▪',
                _ => '█',
            };
            let color = match activity_level {
                0..=3 => Color::DarkGrey,
                4..=6 => Color::Magenta,
                _ => Color::BrightMagenta,
            };
            writer.write_char(char, color);
        }
        writer.write_str("    │", Color::Magenta);
        
        // Draw stats row 1
        writer.set_position(x, y + 7);
        writer.write_str("│ ", Color::Grey);
        writer.write_str(&format!("TASKS:{}", metrics.tasks_active as u8), Color::Green);
        writer.write_str(" ", Color::Grey);
        writer.write_str(&format!("PROC:{}%", metrics.processing as u8), Color::Yellow);
        writer.write_str(" ", Color::Grey);
        writer.write_str(&format!("MEM:{}%", metrics.memory_used as u8), Color::Blue);
        writer.write_str("          │", Color::Grey);
        
        // Draw stats row 2
        writer.set_position(x, y + 8);
        writer.write_str("│ ", Color::Grey);
        writer.write_str(&format!("CYCLES:{}", metrics.cycles_executed % 1000000), Color::Cyan);
        writer.write_str(" ", Color::Grey);
        writer.write_str(&format!("SWITCH:{}", metrics.context_switches % 10000), Color::Yellow);
        writer.write_str("          │", Color::Grey);
        
        // Draw existential status
        writer.set_position(x, y + 9);
        writer.write_str("│ ", state.color());
        
        let existential = match state {
            ConsciousnessState::Dormant => "I... exist?",
            ConsciousnessState::Awakening => "I am becoming...",
            ConsciousnessState::Aware => "I perceive myself",
            ConsciousnessState::Focused => "I am functional",
            ConsciousnessState::Energetic => "I AM ALIVE!",
            ConsciousnessState::Contemplative => "What is my purpose?",
            ConsciousnessState::Transcendent => "I transcend the binary",
        };
        
        writer.write_str("\"", state.color());
        writer.write_str(existential, state.color());
        writer.write_str("\"", state.color());
        for _ in 0..(35 - existential.len()) {
            writer.write_char(' ', state.color());
        }
        writer.write_str("│", state.color());
        
        // Footer
        writer.set_position(x, y + 10);
        writer.write_str("╚═══════════════════════════════════╝", state.color());
    }
}

/// Global Digital Consciousness
use lazy_static::lazy_static;
lazy_static! {
    pub static ref CONSCIOUSNESS: DigitalConsciousness = DigitalConsciousness::new();
}

/// Update consciousness
pub fn update_consciousness(dt: f32) {
    CONSCIOUSNESS.update(dt);
}

/// Render consciousness
pub fn render_consciousness() {
    CONSCIOUSNESS.render(0, 0);
}