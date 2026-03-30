# NimbusOS: The Quantum Core 🌌

NimbusOS is a freestanding, 64-bit operating system kernel written in Rust. This project explores the depths of low-level systems programming, from "First Light" (VGA output) to advanced memory management and quantum-inspired multitasking visualization.

## 🚀 Phase 4: Quantum Multitasking (Latest)

NimbusOS has evolved into a fully operational multitasking kernel with quantum-inspired process visualization.

### Key Technical Achievements:

#### Phase 3: Memory Management
- **Global Descriptor Table (GDT)**: Complete 4-segment GDT with kernel and user mode separation.
  - Kernel code and data segments (Ring 0)
  - User code and data segments (Ring 3)
  - Task State Segment (TSS) support
- **Memory Protection**: Hardware-enforced memory protection with distinct privilege levels.
- **Bump Allocator**: Kernel heap allocation with alignment support.

#### Phase 4: Quantum Multitasking
- **Task Scheduler**: Round-robin scheduler with priority-based energy levels.
- **Process Nebula**: Star-based visualization where each task appears as a star.
  - ★ for high priority, ☆ for normal, ○ for sleeping tasks
  - Constellation-style process display at screen bottom
- **Quantum Process States**: Revolutionary process visualization treating tasks as quantum entities:
  - **Superposition State**: Process exists in multiple states simultaneously (⚛)
  - **Wave Function Decay**: Visual probability representation (▓▓▓░░)
  - **State Collapse**: Real-time process state transition animation
  - **Energy Levels**: Priority mapped to quantum energy (25-100 units)
  - **Entanglement Indicators**: Visual coupling between related processes

#### Core Features (Phases 1-2)
- **Interrupt Descriptor Table (IDT)**: 256-entry IDT for CPU exceptions.
- **"The Pulse" USP**: Dynamic visual dashboard reacting to kernel events in real-time.

## ⚛ The Quantum Difference

Unlike traditional OS kernels that hide processes in text logs, NimbusOS makes process management **visible** through quantum visualization:

```
╔═══════════ PROCESS NEBULA ═══════════╗
║  ★ ☆ ○ ○ ☆ ★                              ║  <- Star constellation
╚════════════════════════════════════╝

╔═════ QUANTUM PROCESS ═════╗
⚛ Idle Quantum    ▓▓▓▓▓     <- Wave probability
▶ System Core     ▓▓▓▒░
◑ I/O Entangler   ▒▒▒░░
╚═══════════════════════════╝
```

## 🛠️ Getting Started

### Prerequisites

- **Rust Nightly**: Required for `no_std` and `abi_x86_interrupt`.
  ```bash
  rustup override set nightly
  rustup component add rust-src llvm-tools-preview
  ```
- **Bootimage Tool**:
  ```bash
  cargo install bootimage
  ```

### Building Locally

```bash
git clone https://github.com/nishantXnova/Nimbus-OS.git
cd Nimbus-OS
cargo bootimage
```

### Running in the Browser (v86)

```bash
# After building, start local server
python -m http.server 8000
# Open http://localhost:8000
```

## 🗺️ Roadmap

- [x] **Phase 1: The Foundation** - VGA Driver & `no_std` Setup.
- [x] **Phase 2: The Reactive Core** - IDT, Interrupts, and "The Pulse" USP.
- [x] **Phase 3: Memory Management** - Global Descriptor Table (GDT) & Memory Protection.
- [x] **Phase 4: Quantum Multitasking** - Task scheduler with quantum process visualization.

## 📂 Project Structure

```
NimbusOS/
├── src/
│   ├── main.rs        # Kernel entry with quantum initialization
│   ├── vga_buffer.rs  # VGA driver with "The Pulse" visualization
│   ├── interrupts.rs  # IDT and exception handlers
│   ├── gdt.rs         # Global Descriptor Table setup
│   ├── memory.rs      # Memory management & frame allocation
│   ├── scheduler.rs   # Task scheduler with energy levels
│   ├── heap.rs        # Kernel bump allocator
│   ├── nebula.rs      # Process Nebula star visualization
│   └── quantum.rs     # Quantum process state engine
├── v86/               # WebAssembly emulator files
├── index.html         # Browser-based emulator UI
└── Cargo.toml         # Rust dependencies and build config
```

## 📜 License

MIT License - See [LICENSE](LICENSE) file for details.

---

*Built with passion by nishantXnova. ⚛ Quantum Reactivity.*
