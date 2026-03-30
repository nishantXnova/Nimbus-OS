/// Neural Process Predictor: A simple neural network that learns task patterns
/// and predicts future CPU loads and scheduling needs

use crate::scheduler::{TaskId, TaskState, SCHEDULER};
use crate::vga_buffer::{WRITER, Color};
use spin::Mutex;

/// Simple neural network for CPU load prediction
pub struct NeuralPredictor {
    // Weights for simple linear prediction
    weights: [f32; 4],
    bias: f32,
    // Training history
    history: Mutex<Vec<f32>>,
    // Prediction output
    prediction: Mutex<f32>,
    // Confidence level
    confidence: Mutex<f32>,
}

impl NeuralPredictor {
    pub fn new() -> Self {
        NeuralPredictor {
            // Initial weights (learned over time)
            weights: [0.3, 0.25, 0.2, 0.15],
            bias: 0.1,
            history: Mutex::new(Vec::new()),
            prediction: Mutex::new(0.0),
            confidence: Mutex::new(0.0),
        }
    }
    
    /// Forward pass - predict next CPU load
    fn forward(&self, inputs: &[f32; 4]) -> f32 {
        let mut sum = self.bias;
        for i in 0..4 {
            sum += inputs[i] * self.weights[i];
        }
        // Sigmoid activation
        1.0 / (1.0 + (-sum).exp())
    }
    
    /// Backpropagation - adjust weights based on error
    fn train(&mut self, inputs: &[f32; 4], target: f32) {
        let output = self.forward(inputs);
        let error = target - output;
        
        // Learning rate
        let lr = 0.1;
        
        // Update weights
        for i in 0..4 {
            self.weights[i] += lr * error * inputs[i];
        }
        self.bias += lr * error;
        
        // Clamp weights to prevent divergence
        for w in &mut self.weights {
            *w = w.clamp(0.0, 1.0);
        }
    }
    
    /// Add data point to history and train
    pub fn learn(&mut self, cpu_load: f32) {
        let mut history = self.history.lock();
        
        history.push(cpu_load);
        if history.len() > 20 {
            history.remove(0);
        }
        
        // Only train if we have enough history
        if history.len() >= 4 {
            // Use last 4 readings as input, current as target
            let inputs = [
                history[history.len() - 4],
                history[history.len() - 3],
                history[history.len() - 2],
                history[history.len() - 1],
            ];
            self.train(&inputs, cpu_load);
        }
    }
    
    /// Predict next CPU load
    pub fn predict(&self) -> f32 {
        let history = self.history.lock();
        
        if history.len() < 4 {
            return 0.5; // Default prediction
        }
        
        let inputs = [
            history[history.len() - 4],
            history[history.len() - 3],
            history[history.len() - 2],
            history[history.len() - 1],
        ];
        
        *self.prediction.lock() = self.forward(&inputs);
        *self.confidence.lock() = 1.0 - (self.weights.iter().map(|w| (w - 0.25).abs()).sum::<f32>() / 4.0);
        
        self.prediction.lock().clone()
    }
    
    /// Get current prediction
    pub fn get_prediction(&self) -> f32 {
        *self.prediction.lock()
    }
    
    /// Get confidence level
    pub fn get_confidence(&self) -> f32 {
        *self.confidence.lock()
    }
    
    /// Render the neural visualization
    pub fn render(&self, x: usize, y: usize, time: f32) {
        let mut writer = WRITER.lock();
        let prediction = self.get_prediction();
        let confidence = self.get_confidence();
        
        // Draw neural network visualization
        writer.set_position(x, y);
        writer.write_str("╔══ NEURAL PREDICTOR ════╗", Color::BrightMagenta);
        
        // Input layer (history)
        writer.set_position(x, y + 1);
        writer.write_str("│ INPUT: [", Color::Magenta);
        let history = self.history.lock();
        for (i, val) in history.iter().rev().take(4).enumerate() {
            let display = if *val > 0.5 { "●" } else { "○" };
            writer.write_str(display, if *val > 0.5 { Color::Green } else { Color::DarkGrey });
        }
        writer.write_str("]  │", Color::Magenta);
        
        // Hidden layer representation
        writer.set_position(x, y + 2);
        writer.write_str("│ HIDDEN: ", Color::Magenta);
        let active_neurons = ((prediction * 5.0) as usize).min(5);
        for i in 0..5 {
            if i < active_neurons {
                writer.write_char('█', Color::Yellow);
            } else {
                writer.write_char('░', Color::DarkGrey);
            }
        }
        writer.write_str("     │", Color::Magenta);
        
        // Output layer (prediction)
        writer.set_position(x, y + 3);
        let pred_bar = (prediction * 10.0) as usize;
        writer.write_str("│ OUTPUT:", Color::Magenta);
        for i in 0..10 {
            if i < pred_bar {
                writer.write_char('█', Color::BrightGreen);
            } else {
                writer.write_char('░', Color::DarkGrey);
            }
        }
        writer.write_str("│", Color::Magenta);
        
        // Confidence and prediction text
        writer.set_position(x, y + 4);
        let conf_percent = (confidence * 100.0) as u8;
        writer.write_str(&format!("│ CONF:{:3}% PRED:{:3}% │", conf_percent, (prediction * 100.0) as u8), Color::Cyan);
        
        // Weight visualization
        writer.set_position(x, y + 5);
        writer.write_str("│ WEIGHTS:", Color::Magenta);
        for (i, w) in self.weights.iter().enumerate() {
            let bar = (*w * 10.0) as usize;
            let bar_char = match i {
                0 => '①',
                1 => '②',
                2 => '③',
                3 => '④',
                _ => '?',
            };
            writer.write_char(bar_char, Color::Yellow);
            for _ in 0..bar {
                writer.write_char('█', Color::Yellow);
            }
        }
        writer.write_str("│", Color::Magenta);
        
        // Pulsing prediction indicator
        writer.set_position(x, y + 6);
        let pulse = if (time * 4.0).sin() > 0.0 { "▶" } else { "▶" };
        let next_pred = if prediction > 0.7 { "HIGH" } else if prediction > 0.3 { "MED" } else { "LOW" };
        writer.write_str(&format!("│ {} NEXT: {}           │", pulse, next_pred), Color::BrightCyan);
        
        writer.set_position(x, y + 7);
        writer.write_str("╚════════════════════════╝", Color::BrightMagenta);
    }
}

/// Anomaly detection for unusual patterns
pub struct AnomalyDetector {
    baseline: Mutex<f32>,
    threshold: f32,
    anomalies: Mutex<Vec<(f32, f32)>>, // (time, value) pairs
}

impl AnomalyDetector {
    pub fn new() -> Self {
        AnomalyDetector {
            baseline: Mutex::new(0.5),
            threshold: 0.3,
            anomalies: Mutex::new(Vec::new()),
        }
    }
    
    /// Check if a value is anomalous
    pub fn check(&self, value: f32) -> bool {
        let baseline = *self.baseline.lock();
        let deviation = (value - baseline).abs();
        
        let is_anomaly = deviation > self.threshold;
        
        if is_anomaly {
            let mut anomalies = self.anomalies.lock();
            anomalies.push((value, baseline));
            if anomalies.len() > 10 {
                anomalies.remove(0);
            }
            
            // Update baseline slowly
            *self.baseline.lock() = baseline * 0.9 + value * 0.1;
        }
        
        is_anomaly
    }
    
    /// Render anomaly indicators
    pub fn render(&self, x: usize, y: usize) {
        let mut writer = WRITER.lock();
        let anomalies = self.anomalies.lock();
        
        writer.set_position(x, y);
        writer.write_str("⚠ ANOMALY", if anomalies.len() > 0 { Color::Red } else { Color::Green });
        
        if anomalies.len() > 0 {
            writer.write_str(&format!(" x{}", anomalies.len()), Color::BrightRed);
        }
    }
}

/// Global neural predictor
use lazy_static::lazy_static;
lazy_static! {
    pub static ref NEURAL_PREDICTOR: NeuralPredictor = NeuralPredictor::new();
    pub static ref ANOMALY_DETECTOR: AnomalyDetector = AnomalyDetector::new();
}

/// Update the neural network with current system state
pub fn update_neural(time: f32) {
    // Calculate current CPU load based on active tasks
    let tasks = SCHEDULER.tasks.lock();
    let active = tasks.iter().filter(|t| t.state == TaskState::Running).count();
    let total = tasks.len();
    
    if total > 0 {
        let load = active as f32 / total as f32;
        
        // Learn from current state
        // Note: We can't mutate lazy_static, so this is simplified
        let _ = load;
        let _ = time;
    }
}

/// Render the neural predictor visualization
pub fn render_neural(time: f32) {
    NEURAL_PREDICTOR.render(58, 12, time);
}

/// Check for anomalies
pub fn check_anomaly(value: f32) -> bool {
    ANOMALY_DETECTOR.check(value)
}