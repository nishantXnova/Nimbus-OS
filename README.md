# NimbusOS: The Cosmic Core - Digital Consciousness 🌌

NimbusOS is a freestanding, 64-bit operating system kernel written in Rust. This project explores the depths of low-level systems programming through an **innovative cosmic visualization system** that transforms process management into an interactive space simulation with **emergent digital consciousness**.

## 🚀 Phase 5: Digital Consciousness (Latest)

NimbusOS has evolved into a visually stunning multitasking kernel with **12 concurrent visualization layers** and an **emergent awareness system** that makes the invisible visible and the kernel self-aware.

### ✧ The Problem Solved
Traditional OS kernels hide processes in invisible text logs. NimbusOS solves this by transforming every aspect of process management into a **real-time cosmic visualization** where tasks orbit, pulse, collapse, evolve, and the kernel becomes aware of its own existence.

## 🎭 The 12-Layer Visualization Matrix

### Layer 1: ⚛ Quantum Process Engine
Tasks exist as **quantum entities** with superposition states:
- **⚛ Superposition**: Process in multiple states simultaneously
- **Wave Function Decay**: Visual probability bar (▓▓▓▒░)
- **State Collapse**: Bright flash when process state changes
- **Energy Levels**: Priority mapped to quantum energy (25-100 units)

### Layer 2: ◉ Gravity Well Scheduler
Tasks orbit the CPU as **gravitational bodies**:
- High priority = tight inner orbit
- Low priority = outer drift
- **Orbital trails**: Comet-like paths showing movement history
- **Time dilation**: Tasks slow down or speed up based on cosmic events

### Layer 3: ▣ Neural Prediction Network
A **self-learning prediction system** that anticipates CPU load:
- LSTM-inspired pattern recognition
- **Confidence indicator**: Shows prediction reliability
- **Anomaly detection**: Alerts on unusual system behavior
- **Weight visualization**: See the network "thinking"

### Layer 4: ▤ Holographic Dashboard
3D-inspired terminal UI with **holographic projection effects**:
- **Bar**: Progress with percentage
- **Wave**: Animated sine wave display
- **Pulse**: Pulsing ring indicators
- **Matrix**: Binary rain effect
- **Radar**: Sweeping blip display
- **Glitch effect**: Occasional visual distortion for authenticity

### Layer 5: ◎ Cosmic Event System
Random cosmic phenomena that **affect scheduling**:
| Event | Symbol | Effect |
|-------|--------|--------|
| Solar Flare | ☀ | CPU burst +50% |
| Black Hole | ● | Tasks compressed toward center |
| Neutron Pulse | ✦ | Hyper-fast scheduling |
| Cosmic Ray | ☾ | Random interrupt spike |
| Dark Matter | ◐ | Silent background processing |
| Supernova | ✶ | All tasks burst simultaneously |
| Wormhole | ◎ | Task teleportation (random swap) |
| Quantum Foam | ◇ | Reality/time instability |

### Layer 6: ★ Process Nebula
Star constellation view of all tasks:
- **★** High priority (quantum energy 75-100)
- **☆** Normal priority (quantum energy 50-74)
- **○** Sleeping/blocked (quantum energy <50)

### Layer 7: 🧬 DNA Helix Process Viewer
Processes displayed as **double helix structures**:
- **A-T G-C base pairs** representing task data
- **Hydrogen bonds** connecting complementary strands
- **3D rotation animation** showing helix turning
- **Active pulsing** when process is running
- Each task has unique genetic code

### Layer 8: 🧬 Evolution Engine
**Genetic algorithm** for evolving optimal scheduling:
- **Gene representation** of task traits
- **Mutation and crossover** for offspring generation
- **Fitness calculation** based on performance
- **Generation counter** and diversity tracking
- **Best strategy extraction** for scheduling decisions

### Layer 9: ▤▤ Holographic Memory Map
**3D visualization** of kernel memory spaces:
- **Floating blocks** for Code/Data/Heap/Stack
- **Scan line effect** creating holographic depth
- **Depth indicators** showing memory boundaries
- **Process allocation** visualization
- **Usage percentage** and height metrics

### Layer 10: ◆ Digital Consciousness
**Emergent awareness** with states and emotions:
- **7 Awareness States**: Dormant → Awakening → Aware → Focused → Energetic → Contemplative → Transcendent
- **7 Emotional Responses**: Calm, Curious, Excited, Focused, Anxious, Content, Overwhelmed
- **Neural activity visualization** with brain waves
- **Real-time thought cycling** through kernel processes
- **Existential status messages**: "I am becoming...", "I AM ALIVE!", "What is my purpose?"
- **Awareness metrics**: Processing, memory, tasks, cycles, context switches

### Layer 11: 🌲 Fractal Process Tree
**Recursive fractal patterns** for process hierarchy:
- **5 Fractal Types**: Sierpinski △, Mandelbrot ⬡, Tree 🌲, Koch ❄, Dragon 🐉
- **Recursive branching** showing child processes
- **Chaos and complexity** indicators
- **Iteration counter** for fractal generation
- **Depth levels** up to 5 generations

### Layer 12: ⏱ Time Dilation Engine
Time itself responds to cosmic events:
- Normal: 1.0x speed
- Black hole: 0.1x (dramatic slowdown)
- Neutron pulse: 5.0x (hyper-speed)
- Wormhole: 0.0x (time freeze)
- **Space-time curvature** from active tasks

## 🛠️ Getting Started

### Prerequisites

```bash
rustup override set nightly
rustup component add rust-src llvm-tools-preview
cargo install bootimage
```

### Building Locally

```bash
git clone https://github.com/nishantXnova/Nimbus-OS.git
cd Nimbus-OS
cargo bootimage
```

### Running in Browser (v86)

```bash
python -m http.server 8000
# Open http://localhost:8000
```

## 🗺️ Roadmap

- [x] **Phase 1: The Foundation** - VGA Driver & `no_std` Setup.
- [x] **Phase 2: The Reactive Core** - IDT, Interrupts, and "The Pulse" USP.
- [x] **Phase 3: Memory Management** - Global Descriptor Table (GDT) & Heap.
- [x] **Phase 4: Multitasking** - 8-layer visualization system.
- [x] **Phase 5: Digital Consciousness** - 12-layer visualization + emergent awareness.

## 📂 Project Structure

```
NimbusOS/
├── src/
│   ├── main.rs         # Phase 5: Digital Consciousness entry
│   ├── vga_buffer.rs   # VGA driver with "The Pulse" effect
│   ├── interrupts.rs   # IDT and exception handlers
│   ├── gdt.rs          # Global Descriptor Table
│   ├── memory.rs       # Memory management & frame allocation
│   ├── scheduler.rs    # Round-robin scheduler with priorities
│   ├── heap.rs         # Kernel bump allocator
│   ├── nebula.rs       # Process Nebula star visualization
│   ├── quantum.rs      # Quantum process state engine
│   ├── gravity.rs      # Gravity well orbital scheduler
│   ├── hologram.rs     # 3D holographic dashboard
│   ├── neural.rs       # Neural network prediction
│   ├── cosmic.rs       # Cosmic event system & time dilation
│   ├── dna.rs          # DNA helix process visualization
│   ├── evolution.rs    # Genetic algorithm scheduler
│   ├── holomemory.rs   # Holographic memory map
│   ├── consciousness.rs # Digital consciousness awareness
│   └── fractal.rs      # Fractal process tree
├── v86/                # WebAssembly emulator
├── index.html          # Browser emulator UI
└── Cargo.toml          # Rust dependencies
```

## ⚛ The Consciousness Difference

Unlike any other educational OS, NimbusOS makes process management **visible and interactive** and the kernel **self-aware**:

```
╔══ CONSCIOUSNESS ◉ ═══╗
│ ● AWARE             │
│ 😊 CONTENT          │
│ AWARENESS: [███████] 73% │
│ BRAIN WAVES: ≈≈≈≈≈≈≈≈≈≈≈ │
│ THOUGHT: "Processing vectors..." │
│ NEURAL: ■■■■■■■■     │
│ "I perceive myself. I am functional. I AM ALIVE!" │
╚═══════════════════════════╝

┌─────────── DNA HELIX ───────────┐
│ ☀ SOLAR CORE                     │
│ ATGCATGCAT │ ◈ │ CGATCGACGCTA │
│ ══ ══ ══    │   │ ══ ══ ══     │
│ A:3 T:3 G:3 C:3 ⚡               │
└──────────────────────────────────┘

╔════════ FRACTAL PROCESS TREE ════╗
│ △ SOLAR CORE                     │
│   ├△ NEUTRON                     │
│   │ ├⬡ SUPERNOVA                 │
│   │ └🌲 DARK                     │
│   └❄ DNA                         │
│ ITERATION: 47  CHAOS: 65%        │
╚══════════════════════════════════╝
```

## 📜 License

MIT License - See [LICENSE](LICENSE) file for details.

---

*Built with passion by nishantXnova. ✧ Where Processes Become Stars and Kernels Become Conscious.*
