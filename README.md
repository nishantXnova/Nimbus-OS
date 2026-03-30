# NimbusOS: The Cosmic Core 🌌

NimbusOS is a freestanding, 64-bit operating system kernel written in Rust. This project explores the depths of low-level systems programming through an innovative **cosmic visualization system** that transforms process management into an interactive space simulation.

## 🚀 Phase 4: Cosmic Multitasking (Latest)

NimbusOS has evolved into a visually stunning multitasking kernel with **8 concurrent visualization systems** that make the invisible visible.

### ✧ The Problem Solved
Traditional OS kernels hide processes in invisible text logs. NimbusOS solves this by transforming every aspect of process management into a **real-time cosmic visualization** where tasks orbit, pulse, collapse, and interact like celestial bodies.

## 🔭 Visualization Systems (8 Layers)

### 1. ⚛ Quantum Process Engine
Tasks exist as **quantum entities** with superposition states:
- **⚛ Superposition**: Process in multiple states simultaneously
- **Wave Function Decay**: Visual probability bar (▓▓▓▒░)
- **State Collapse**: Bright flash when process state changes
- **Energy Levels**: Priority mapped to quantum energy (25-100 units)

### 2. ◉ Gravity Well Scheduler
Tasks orbit the CPU as **gravitational bodies**:
- High priority = tight inner orbit
- Low priority = outer drift
- **Orbital trails**: Comet-like paths showing movement history
- **Time dilation**: Tasks slow down or speed up based on cosmic events

### 3. ▣ Neural Prediction Network
A **self-learning prediction system** that anticipates CPU load:
- LSTM-inspired pattern recognition
- **Confidence indicator**: Shows prediction reliability
- **Anomaly detection**: Alerts on unusual system behavior
- **Weight visualization**: See the network "thinking"

### 4. ▤ Holographic Dashboard
3D-inspired terminal UI with **holographic projection effects**:
- **Bar**: Progress with percentage
- **Wave**: Animated sine wave display
- **Pulse**: Pulsing ring indicators
- **Matrix**: Binary rain effect
- **Radar**: Sweeping blip display
- **Glitch effect**: Occasional visual distortion for authenticity

### 5. ◎ Cosmic Event System
Random cosmic phenomena that **affect scheduling**:
| Event | Symbol | Effect |
|-------|--------|--------|
| Solar Flare | ☀ | CPU burst +50% |
| Black Hole | ● | Tasks compressed toward center |
| Neutron Pulse | ✦ | Hyper-fast scheduling |
| Cosmic Ray | ☾ | Random interrupt spike |
| Dark Matter | ◐ | Silent background processing |
| Supernova | ✶ | All tasks burst simultaneously |
| Wormhole | ◎ | Task positions swapped |
| Quantum Foam | ◇ | Reality/time instability |

### 6. ★ Process Nebula
Star constellation view of all tasks:
- **★** High priority (quantum energy 75-100)
- **☆** Normal priority (quantum energy 50-74)
- **○** Sleeping/blocked (quantum energy <50)

### 7. ◇ Time Dilation Engine
Time itself responds to cosmic events:
- Normal: 1.0x speed
- Black hole: 0.1x (dramatic slowdown)
- Neutron pulse: 5.0x (hyper-speed)
- Wormhole: 0.0x (time freeze)

### 8. ═ Space-Time Curvature
Tasks bend reality based on their activity:
- Active tasks curve nearby space
- Idle tasks have minimal curvature
- Black holes pull everything toward center

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
- [x] **Phase 4: Cosmic Multitasking** - 8-layer visualization system.

## 📂 Project Structure

```
NimbusOS/
├── src/
│   ├── main.rs         # Cosmic kernel entry with 5 task types
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
│   └── cosmic.rs       # Cosmic event system & time dilation
├── v86/                # WebAssembly emulator
├── index.html          # Browser emulator UI
└── Cargo.toml          # Rust dependencies
```

## ⚛ The Cosmic Difference

Unlike any other educational OS, NimbusOS makes process management **visible and interactive**:

```
┌─────────── COSMIC EVENTS ───────────┐
│ ◎ WORMHOLE                         │
│ TIME DILATION: [▓▓░░░░░░░░░░] 2.3x │
│ SPACETIME: ①▓▓ ②▓ ③▓ ④ ⑤         │
└──────────────────────────────────────┘

╔══ PROCESS NEBULA ═══════════╗
║  ★ ☆ ○ ○ ☆ ★                    ║  <- Star constellation
╚══════════════════════════════╝

╔══ QUANTUM PROCESS ═════╗
⚛ Solar Core    ▓▓▓▓▓   <- Wave function
▶ Neutron Star  ▓▓▓▒░
◑ Dark Matter   ▒▒▒░░
╚════════════════════════╝

◉ CPU - Orbiting tasks with trails
  ○   ●
    ★
  ☆   ○
```

## 📜 License

MIT License - See [LICENSE](LICENSE) file for details.

---

*Built with passion by nishantXnova. ✧ Where Processes Become Stars.*
