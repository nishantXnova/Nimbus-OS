/// Holographic Memory Map: 3D visualization of kernel memory as a holographic projection
/// Different process memory spaces displayed as floating holographic blocks

use crate::vga_buffer::{WRITER, Color};
use spin::Mutex;

/// Memory block types
#[derive(Debug, Clone, Copy)]
pub enum MemoryBlockType {
    Code,
    Data,
    Heap,
    Stack,
    Reserved,
}

impl MemoryBlockType {
    pub fn color(&self) -> Color {
        match self {
            MemoryBlockType::Code => Color::Green,
            MemoryBlockType::Data => Color::Blue,
            MemoryBlockType::Heap => Color::Yellow,
            MemoryBlockType::Stack => Color::Red,
            MemoryBlockType::Reserved => Color::DarkGrey,
        }
    }
    
    pub fn symbol(&self) -> char {
        match self {
            MemoryBlockType::Code => '▣',
            MemoryBlockType::Data => '▤',
            MemoryBlockType::Heap => '▥',
            MemoryBlockType::Stack => '▦',
            MemoryBlockType::Reserved => '░',
        }
    }
}

/// A memory block in the holographic space
#[derive(Debug, Clone)]
pub struct MemoryBlock {
    pub base: u64,
    pub size: u64,
    pub block_type: MemoryBlockType,
    pub process_id: u64,
    pub floating_height: f32,
    pub pulse_phase: f32,
}

impl MemoryBlock {
    pub fn new(base: u64, size: u64, block_type: MemoryBlockType, process_id: u64) -> Self {
        MemoryBlock {
            base,
            size,
            block_type,
            process_id,
            floating_height: (base as f32 % 10.0) + 2.0,
            pulse_phase: (base as f32 * 0.1) % (2.0 * core::f32::consts::PI),
        }
    }
    
    /// Get the visual representation
    pub fn get_hologram_char(&self, time: f32) -> char {
        let pulse = ((time + self.pulse_phase) * 2.0).sin();
        
        if pulse > 0.5 {
            self.block_type.symbol()
        } else {
            match self.block_type {
                MemoryBlockType::Code => '▢',
                MemoryBlockType::Data => '◇',
                MemoryBlockType::Heap => '◈',
                MemoryBlockType::Stack => '◆',
                MemoryBlockType::Reserved => '▭',
            }
        }
    }
}

/// Holographic memory space
pub struct HolographicMemory {
    blocks: Mutex<Vec<MemoryBlock>>,
    time: Mutex<f32>,
    scan_line: Mutex<f32>,
}

impl HolographicMemory {
    pub fn new() -> Self {
        HolographicMemory {
            blocks: Mutex::new(Vec::new()),
            time: Mutex::new(0.0),
            scan_line: Mutex::new(0.0),
        }
    }
    
    /// Add a memory block
    pub fn add_block(&mut self, base: u64, size: u64, block_type: MemoryBlockType, process_id: u64) {
        let block = MemoryBlock::new(base, size, block_type, process_id);
        self.blocks.lock().push(block);
    }
    
    /// Initialize with kernel memory layout
    pub fn init_kernel_memory(&mut self) {
        // Simulate kernel memory segments
        self.add_block(0xFFFF_8000_0000_0000, 0x1000, MemoryBlockType::Code, 0);  // Kernel code
        self.add_block(0xFFFF_8000_0010_0000, 0x1000, MemoryBlockType::Data, 0);  // Kernel data
        self.add_block(0xFFFF_8000_0020_0000, 0x4000, MemoryBlockType::Heap, 0);   // Kernel heap
        self.add_block(0xFFFF_8000_0030_0000, 0x2000, MemoryBlockType::Stack, 0); // Kernel stack
        self.add_block(0xFFFF_8000_0040_0000, 0x8000, MemoryBlockType::Reserved, 0); // Reserved
    }
    
    /// Update hologram animation
    pub fn update(&self, dt: f32) {
        *self.time.lock() += dt;
        *self.scan_line.lock() += dt * 50.0;
    }
    
    /// Render the holographic memory visualization
    pub fn render(&self, x: usize, y: usize) {
        let mut writer = WRITER.lock();
        let blocks = self.blocks.lock();
        let time = *self.time.lock();
        let scan = *self.scan_line.lock() as usize;
        
        // Draw 3D frame header
        writer.set_position(x, y);
        writer.write_str("╔════════ HOLOGRAPHIC MEMORY ════════╗", Color::BrightBlue);
        
        // Draw scan line effect
        for i in 0..18 {
            let scan_pos = (scan + i * 3) % 78;
            writer.set_position(x, y + 1 + i);
            writer.write_char('║', Color::DarkGrey);
            
            if scan_pos < 38 {
                writer.write_char('▓', Color::BrightCyan);
            } else {
                writer.write_char(' ', Color::Black);
            }
            
            // Draw memory blocks in 3D perspective
            for (j, block) in blocks.iter().enumerate() {
                if j < 5 {
                    let char = block.get_hologram_char(time);
                    let color = block.block_type.color();
                    
                    // 3D effect - brighter when closer to scan line
                    let dist_from_scan = ((scan_pos as i32 - (j * 8) as i32).abs()) as usize;
                    let brightness = if dist_from_scan < 10 { 1.0 } else { 0.5 };
                    
                    writer.set_position(x + 2 + j * 7, y + 1 + i);
                    writer.write_char(char, if brightness > 0.8 { color } else { Color::DarkGrey });
                }
            }
            
            writer.set_position(x + 39, y + 1 + i);
            if scan_pos >= 38 && scan_pos < 78 {
                writer.write_char('▓', Color::BrightBlue);
            } else {
                writer.write_char(' ', Color::Black);
            }
            writer.write_char('║', Color::DarkGrey);
        }
        
        // Draw memory layout as text
        writer.set_position(x, y + 1);
        writer.write_str("║ STACK  ", Color::Red);
        writer.write_str("HEAP    ", Color::Yellow);
        writer.write_str("DATA    ", Color::Blue);
        writer.write_str("CODE    ", Color::Green);
        writer.write_str("║", Color::DarkGrey);
        
        // Draw height indicator
        let height_bar = ((time * 2.0).sin() * 5.0 + 8.0) as usize;
        writer.set_position(x, y + 20);
        writer.write_str("║ HEIGHT:", Color::Cyan);
        for i in 0..10 {
            writer.write_char(if i < height_bar { '█' } else { '░' }, Color::BrightCyan);
        }
        writer.write_str(" m ║", Color::Cyan);
        
        // Draw memory usage percentage
        let usage = blocks.iter().filter(|b| b.block_type != MemoryBlockType::Reserved).sum::<u64>() as f32 / 
                     blocks.iter().map(|b| b.size).sum::<u64>().max(1) as f32 * 100.0;
        
        writer.set_position(x, y + 21);
        writer.write_str(&format!("║ USAGE: {:5.1}%", usage), Color::Green);
        
        // Draw process memory allocation
        writer.set_position(x, y + 22);
        writer.write_str("║ PROC0: KERNEL ", Color::Grey);
        
        // Footer
        writer.set_position(x, y + 23);
        writer.write_str("╚════════════════════════════════════╝", Color::BrightBlue);
        
        // Draw floating animation for each block
        for (i, block) in blocks.iter().enumerate() {
            let float_y = (time * 3.0 + block.pulse_phase).sin() as i32;
            let display_y = y + 2 + i + (float_y as usize % 3);
            
            if display_y < y + 18 {
                writer.set_position(x + 1, display_y);
                writer.write_char(block.block_type.symbol(), block.block_type.color());
                
                // Draw memory address
                writer.write_str(&format!("{:016x}", block.base), Color::DarkGrey);
            }
        }
    }
}

/// Global Holographic Memory
use lazy_static::lazy_static;
lazy_static! {
    pub static ref HOLO_MEMORY: HolographicMemory = HolographicMemory::new();
}

/// Initialize holographic memory
pub fn init_holo_memory() {
    let mut memory = HolographicMemory::new();
    memory.init_kernel_memory();
    // Note: In real impl would set up global
}

/// Update holographic memory
pub fn update_holo_memory(dt: f32) {
    HOLO_MEMORY.update(dt);
}

/// Render holographic memory
pub fn render_holo_memory() {
    HOLO_MEMORY.render(0, 0);
}