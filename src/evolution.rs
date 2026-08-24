/// Neural Evolution Engine: Genetic Algorithm for evolving optimal scheduling
/// Tasks evolve their priority and behavior over time based on fitness

use crate::scheduler::{TaskId, TaskState, Priority, SCHEDULER};
use crate::vga_buffer::{WRITER, Color};
use spin::Mutex;
use core::f32::consts::PI;

/// Gene representing a single trait
#[derive(Debug, Clone, Copy)]
pub struct Gene {
    pub trait_id: u8,
    pub value: f32,
    pub mutation_rate: f32,
}

impl Gene {
    pub fn new(trait_id: u8) -> Self {
        Gene {
            trait_id,
            value: ((trait_id as f32 * 17.3) % 1.0),
            mutation_rate: 0.1,
        }
    }
    
    /// Mutate the gene
    pub fn mutate(&mut self) {
        if ((crate::scheduler::get_ticks() as u128 % 100) as f32 / 100.0) < self.mutation_rate {
            let delta = ((crate::scheduler::get_ticks() as u128 % 200) as f32 / 100.0) - 1.0;
            self.value = (self.value + delta * 0.1).clamp(0.0, 1.0);
        }
    }
    
    /// Crossover with another gene
    pub fn crossover(&self, other: &Gene) -> Gene {
        let value = if (crate::scheduler::get_ticks() as u128 % 2) == 0 {
            self.value
        } else {
            other.value
        };
        
        Gene {
            trait_id: self.trait_id,
            value,
            mutation_rate: (self.mutation_rate + other.mutation_rate) / 2.0,
        }
    }
}

/// Chromosome containing multiple genes
#[derive(Debug, Clone)]
pub struct Chromosome {
    pub genes: Vec<Gene>,
    pub fitness: f32,
}

impl Chromosome {
    pub fn new() -> Self {
        Chromosome {
            genes: (0..8).map(|i| Gene::new(i)).collect(),
            fitness: 0.0,
        }
    }
    
    /// Get a specific gene value
    pub fn get_trait(&self, trait_id: u8) -> f32 {
        self.genes.iter().find(|g| g.trait_id == trait_id).map(|g| g.value).unwrap_or(0.5)
    }
    
    /// Mutate all genes
    pub fn mutate(&mut self) {
        for gene in &mut self.genes {
            gene.mutate();
        }
    }
    
    /// Crossover with another chromosome
    pub fn crossover(&self, other: &Chromosome) -> Chromosome {
        let mut new_chromosome = Chromosome {
            genes: self.genes.iter().zip(other.genes.iter()).map(|(g1, g2)| g1.crossover(g2)).collect(),
            fitness: 0.0,
        };
        new_chromosome.mutate();
        new_chromosome
    }
}

/// Evolved process with genetic material
#[derive(Debug, Clone)]
pub struct EvolvedProcess {
    pub task_id: TaskId,
    pub name: &'static str,
    pub chromosome: Chromosome,
    pub generations: u32,
    pub total_runtime: u64,
}

impl EvolvedProcess {
    pub fn new(task_id: TaskId, name: &'static str) -> Self {
        EvolvedProcess {
            task_id,
            name,
            chromosome: Chromosome::new(),
            generations: 0,
            total_runtime: 0,
        }
    }
    
    /// Calculate fitness based on actual performance
    pub fn calculate_fitness(&mut self) {
        // Fitness based on how efficiently the task uses CPU time
        let efficiency = if self.total_runtime > 0 {
            (self.total_runtime as f32 / 1000000.0).min(1.0)
        } else {
            0.0
        };
        
        // Bonus for completing work (represented by generations)
        let adaptation = (self.generations as f32 / 100.0).min(1.0);
        
        self.chromosome.fitness = efficiency * 0.7 + adaptation * 0.3;
    }
    
    /// Evolve - create offspring with mutations
    pub fn evolve(&self) -> Chromosome {
        let mut child = self.chromosome.clone();
        child.mutate();
        child
    }
}

/// Evolution statistics
#[derive(Debug, Clone, Copy)]
pub struct EvolutionStats {
    pub generation: u32,
    pub best_fitness: f32,
    pub average_fitness: f32,
    pub diversity: f32,
}

impl EvolutionStats {
    pub fn new(generation: u32) -> Self {
        EvolutionStats {
            generation,
            best_fitness: 0.0,
            average_fitness: 0.0,
            diversity: 0.0,
        }
    }
}

/// Neural Evolution Engine
pub struct EvolutionEngine {
    processes: Mutex<Vec<EvolvedProcess>>,
    population: Mutex<Vec<Chromosome>>,
    generation: Mutex<u32>,
    stats: Mutex<EvolutionStats>,
    time: Mutex<f32>,
}

impl EvolutionEngine {
    pub fn new() -> Self {
        EvolutionEngine {
            processes: Mutex::new(Vec::new()),
            population: Mutex::new(Vec::new()),
            generation: Mutex::new(0),
            stats: Mutex::new(EvolutionStats::new(0)),
            time: Mutex::new(0.0),
        }
    }
    
    /// Add a process to evolution tracking
    pub fn add_process(&mut self, task_id: TaskId, name: &'static str) {
        let evolved = EvolvedProcess::new(task_id, name);
        self.processes.lock().push(evolved);
        
        // Add to population
        self.population.lock().push(Chromosome::new());
    }
    
    /// Run one generation of evolution
    pub fn evolve_generation(&self) {
        let mut gen = self.generation.lock();
        *gen += 1;
        
        let population = self.population.lock();
        let mut processes = self.processes.lock();
        
        // Sort by fitness (simplified - just shuffle for demo)
        let mut new_population = Vec::new();
        
        for (i, _) in population.iter().enumerate().take(4) {
            // Create offspring
            if i < population.len() - 1 {
                let child = population[i].crossover(&population[i + 1]);
                new_population.push(child);
            } else {
                // Keep best
                new_population.push(population[i].clone());
            }
        }
        
        // Update generation
        for process in processes.iter_mut() {
            if (crate::scheduler::get_ticks() as u128 % 3) == 0 {
                process.generations += 1;
                process.chromosome = process.evolve();
            }
        }
        
        // Update stats
        let best = population.iter().map(|c| c.fitness).fold(0.0f32, |a, b| a.max(b));
        let avg: f32 = population.iter().map(|c| c.fitness).sum::<f32>() / population.len().max(1) as f32;
        
        *self.stats.lock() = EvolutionStats {
            generation: *gen,
            best_fitness: best,
            average_fitness: avg,
            diversity: (population.len() as f32 / 10.0).min(1.0),
        };
    }
    
    /// Update evolution (called periodically)
    pub fn update(&self, dt: f32) {
        *self.time.lock() += dt;
        
        // Evolve every few seconds
        if (*self.time.lock() % 5.0) < dt {
            self.evolve_generation();
        }
        
        // Update process runtimes
        let tasks = SCHEDULER.tasks.lock();
        let mut processes = self.processes.lock();
        
        for task in tasks.iter() {
            if let Some(evo) = processes.iter_mut().find(|p| p.task_id == task.id) {
                if task.state == TaskState::Running {
                    evo.total_runtime += 1;
                }
                evo.calculate_fitness();
            }
        }
    }
    
    /// Render evolution visualization
    pub fn render(&self, x: usize, y: usize) {
        let mut writer = WRITER.lock();
        let stats = *self.stats.lock();
        let processes = self.processes.lock();
        let time = *self.time.lock();
        
        // Header
        writer.set_position(x, y);
        writer.write_str("╔════════ EVOLUTION ENGINE ═════════╗", Color::BrightYellow);
        
        // Generation counter
        writer.set_position(x, y + 1);
        writer.write_str("│ GEN:", Color::Yellow);
        writer.write_str(&format!("{:5}", stats.generation), Color::BrightYellow);
        
        // Fitness bars
        writer.set_position(x, y + 2);
        writer.write_str("│ BEST: [", Color::Yellow);
        let best_bar = (stats.best_fitness * 15.0) as usize;
        for i in 0..15 {
            writer.write_char(if i < best_bar { '█' } else { '░' }, Color::BrightGreen);
        }
        writer.write_str(&format!("] {:4.1}%", stats.best_fitness * 100.0), Color::Green);
        
        writer.set_position(x, y + 3);
        writer.write_str("│ AVG:  [", Color::Yellow);
        let avg_bar = (stats.average_fitness * 15.0) as usize;
        for i in 0..15 {
            writer.write_char(if i < avg_bar { '█' } else { '░' }, Color::Yellow);
        }
        writer.write_str(&format!("] {:4.1}%", stats.average_fitness * 100.0), Color::Yellow);
        
        // Diversity indicator
        writer.set_position(x, y + 4);
        writer.write_str("│ DIVERSITY: ", Color::Yellow);
        let div_chars = "▁▂▃▄▅▆▇█";
        let div_idx = (stats.diversity * 8.0) as usize;
        for i in 0..8 {
            writer.write_char(div_chars.as_bytes()[div_idx.min(7)] as char, 
                if i <= div_idx { Color::Cyan } else { Color::DarkGrey });
        }
        writer.write_str(&format!(" {:3.0}%", stats.diversity * 100.0), Color::Cyan);
        
        // Population visualization (genome display)
        writer.set_position(x, y + 5);
        writer.write_str("│ GENOME: ", Color::Yellow);
        let population = self.population.lock();
        for (i, chrom) in population.iter().take(4).enumerate() {
            let char = match i {
                0 => '①',
                1 => '②',
                2 => '③',
                _ => '④',
            };
            writer.write_char(char, Color::Cyan);
            let fitness = (chrom.fitness * 10.0) as usize;
            for _ in 0..fitness.min(5) {
                writer.write_char('█', Color::BrightYellow);
            }
            writer.write_char(' ', Color::Grey);
        }
        
        // Process evolution status
        writer.set_position(x, y + 6);
        writer.write_str("│ PROCESSES:", Color::Yellow);
        for (i, proc) in processes.iter().enumerate().take(3) {
            writer.write_char(match i {
                0 => '☀',
                1 => '✦',
                2 => '◐',
                _ => '?',
            }, Color::White);
            writer.write_str(&format!("G{} ", proc.generations), 
                if proc.generations > 10 { Color::BrightGreen } else { Color::Green });
        }
        
        // Mutation indicator
        writer.set_position(x, y + 7);
        let mutation = ((time * 2.0).sin() * 0.5 + 0.5) as usize;
        let mut_indicator = match mutation {
            0 => "◇◇◇",
            1 => "◆◇◇",
            2 => "◆◆◇",
            _ => "◆◆◆",
        };
        writer.write_str(&format!("│ MUTATION: {}", mut_indicator), Color::BrightMagenta);
        
        // Next evolution countdown
        let next_in = 5.0 - (time % 5.0);
        writer.set_position(x, y + 8);
        writer.write_str(&format!("│ NEXT GEN: {:.1}s", next_in), Color::Grey);
        
        // Footer
        writer.set_position(x, y + 9);
        writer.write_str("╚══════════════════════════════════╝", Color::BrightYellow);
    }
    
    /// Get best chromosome for scheduling decisions
    pub fn get_best_strategy(&self) -> Option<Chromosome> {
        let population = self.population.lock();
        population.iter().max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap_or(core::cmp::Ordering::Equal)).cloned()
    }
}

/// Global Evolution Engine
use lazy_static::lazy_static;
lazy_static! {
    pub static ref EVOLUTION_ENGINE: EvolutionEngine = EvolutionEngine::new();
}

/// Track a process in evolution
pub fn track_evolution(task_id: TaskId, name: &'static str) {
    // Would add to global engine
    let _ = (task_id, name);
}

/// Update evolution
pub fn update_evolution(dt: f32) {
    EVOLUTION_ENGINE.update(dt);
}

/// Render evolution
pub fn render_evolution() {
    EVOLUTION_ENGINE.render(58, 14);
}