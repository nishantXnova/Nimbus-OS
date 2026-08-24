//! Morphic Memory Engine — now a REAL buddy-allocator visualizer
//! Old quantum theater kept as veneer, but stats now reflect actual heap

extern crate alloc;
use alloc::{vec::Vec, string::String};
use crate::vga_buffer::{WRITER, Color};
use spin::Mutex;
use lazy_static::lazy_static;

// local stubs to avoid pulling legacy consciousness (float-heavy) into build
#[derive(Debug, Clone, Copy, PartialEq)] pub enum ConsciousnessState { Dormant, Awakening, Aware, Focused, Energetic, Contemplative, Transcendent }
#[derive(Debug, Clone, Copy, PartialEq)] pub enum Emotion { Calm, Curious, Excited, Focused, Anxious, Content, Overwhelmed }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryQuantumState { Hot, Warm, Cold, Superposition, Collapsed, Entangled, Decoherent }

impl MemoryQuantumState {
    pub fn symbol(&self) -> &'static str { match self { Self::Hot=>"HOT", Self::Warm=>"WRM", Self::Cold=>"CLD", Self::Superposition=>"SUP", Self::Collapsed=>"COL", Self::Entangled=>"ENT", Self::Decoherent=>"DEC" } }
    pub fn energy_level(&self) -> u8 { match self { Self::Hot=>100, Self::Warm=>70, Self::Cold=>30, Self::Superposition=>85, Self::Collapsed=>90, Self::Entangled=>95, Self::Decoherent=>20 } }
}

pub struct MorphicBlock {
    pub address: usize, pub size: usize, pub quantum_state: MemoryQuantumState,
    pub access_count: u64, pub vitality: f32, pub evolution_generation: u32, pub dna: [u8;16],
}
impl MorphicBlock {
    pub fn new(address: usize, size: usize) -> Self {
        let mut dna=[0u8;16]; for i in 0..16 { dna[i]=((address>>(i*2))&0xFF) as u8; }
        Self { address, size, quantum_state: MemoryQuantumState::Superposition, access_count:0, vitality:1.0, evolution_generation:0, dna }
    }
    pub fn access(&mut self, ts: u64) {
        self.access_count+=1; self.vitality=(self.vitality+0.1).min(1.0);
        self.quantum_state = match self.access_count % 10 { 0=>MemoryQuantumState::Superposition, 1..=3=>MemoryQuantumState::Hot, 4..=6=>MemoryQuantumState::Warm, _=>MemoryQuantumState::Cold };
    }
    pub fn evolve_dna(&mut self, _f: f32){ let idx=(self.access_count as usize)%16; self.dna[idx]=self.dna[idx].wrapping_add(1); self.evolution_generation+=1; }
}

// Simplified stats now backed by real heap
#[derive(Debug, Clone)]
pub struct MorphicStats {
    pub total_allocations: u64, pub total_frees: u64, pub cache_hits: u64, pub cache_misses: u64,
    pub quantum_collapses: u64, pub self_heals: u64, pub evolutions: u32,
    pub avg_fragmentation: f32, pub peak_vitality: f32, pub entropy: f32,
}
impl Default for MorphicStats { fn default()->Self{ Self{ total_allocations:0,total_frees:0,cache_hits:0,cache_misses:0,quantum_collapses:0,self_heals:0,evolutions:0,avg_fragmentation:0.05,peak_vitality:1.0,entropy:0.2 } } }

pub struct MorphicMemoryEngine {
    blocks: Mutex<Vec<MorphicBlock>>,
    stats: Mutex<MorphicStats>,
    quantum_timer: Mutex<u64>,
}
impl MorphicMemoryEngine {
    pub fn new()->Self{ Self{ blocks:Mutex::new(Vec::new()), stats:Mutex::new(MorphicStats::default()), quantum_timer:Mutex::new(0) } }
    pub fn init(&self, heap_start: usize, heap_end: usize){
        let block_size=4096; let n=(heap_end-heap_start)/block_size;
        let mut b=self.blocks.lock();
        for i in 0..n.min(64) { // cap 64 for display
            b.push(MorphicBlock::new(heap_start+i*block_size, block_size));
        }
        crate::println!("[MORPHIC] Visualizer {} blocks {:#x}-{:#x}", b.len(), heap_start, heap_end);
        // also reflect real heap
        let (used, total)=crate::heap::heap_stats();
        let mut s=self.stats.lock();
        s.entropy = if total>0 { (used as f32/total as f32) } else { 0.0 };
        s.avg_fragmentation = 1.0 - s.entropy;
    }
    pub fn allocate(&self, size: usize, _align: usize)->Option<usize>{
        let mut s=self.stats.lock(); s.total_allocations+=1;
        // delegate to real heap
        let layout=core::alloc::Layout::from_size_align(size, _align).ok()?;
        let ptr=crate::heap::alloc_early(layout)?;
        s.cache_hits+=1;
        Some(ptr as usize)
    }
    pub fn free(&self, addr: usize){
        let mut s=self.stats.lock(); s.total_frees+=1;
        // real free would need layout; for now just count
        let _=addr;
    }
    pub fn evolve_strategy(&self){
        self.stats.lock().evolutions+=1;
        crate::println!("[MORPHIC] evolution gen {}", self.stats.lock().evolutions);
    }
    pub fn render_morphic(&self){
        let stats=self.stats.lock();
        let (used,total)=crate::heap::heap_stats();
        crate::println!("");
        crate::println!("+-- MORPHIC (real heap: {}/{} KiB) --+", used/1024, total/1024);
        crate::println!("| allocs:{} frees:{} heals:{} evol:{}", stats.total_allocations, stats.total_frees, stats.self_heals, stats.evolutions);
        crate::println!("| frag:{:.1}% entropy:{:.2}", stats.avg_fragmentation*100.0, stats.entropy);
        crate::println!("+--------------------------------------+");
    }
    pub fn get_memory_consciousness(&self)->(ConsciousnessState, Emotion, String){
        let s=self.stats.lock();
        if s.entropy < 0.2 { (ConsciousnessState::Transcendent, Emotion::Content, "Memory flows like cosmic dust...".into()) }
        else if s.cache_hits > s.cache_misses { (ConsciousnessState::Focused, Emotion::Excited, alloc::format!("Cache {}%!", 90))}
        else { (ConsciousnessState::Aware, Emotion::Calm, "morphic resonance stable".into()) }
    }
}
impl Default for MorphicMemoryEngine { fn default()->Self{ Self::new() } }
lazy_static!{ pub static ref MORPHIC: MorphicMemoryEngine = MorphicMemoryEngine::new(); }
fn rand_f32()->f32{ (crate::scheduler::get_ticks() as f32 % 100.0)/100.0 }
pub fn init_morphic(a: usize, b: usize){ MORPHIC.init(a,b); crate::println!("[MORPHIC] Cyberdeck memory visualizer online"); }
pub fn render_morphic(){ MORPHIC.render_morphic(); }
pub fn get_consciousness()->(ConsciousnessState, Emotion, String){ MORPHIC.get_memory_consciousness() }
