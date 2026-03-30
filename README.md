# NimbusOS: The Reactive Core 🌌

NimbusOS is a freestanding, 64-bit operating system kernel written in Rust. This project explores the depths of low-level systems programming, from "First Light" (VGA output) to advanced memory management and multitasking.

## 🚀 Phase 4: Multitasking & Memory Management (Latest)

NimbusOS has evolved into a fully operational multitasking kernel with comprehensive memory protection.

### Key Technical Achievements:

#### Phase 3: Memory Management
- **Global Descriptor Table (GDT)**: Implemented a complete 4-segment GDT with kernel and user mode separation.
  - Kernel code and data segments (Ring 0)
  - User code and data segments (Ring 3)
  - Task State Segment (TSS) support for future task switching
- **Memory Protection**: Segments provide hardware-enforced memory protection with distinct privilege levels.
- **Frame Allocation**: BootInfo-based memory frame allocator for dynamic memory allocation.

#### Phase 4: Multitasking
- **Task Scheduler**: Round-robin scheduler with priority support and task state management.
- **Task Control Block (TCB)**: Full task context with saved registers, stack pointers, and execution tracking.
- **Task Management**: Spawn, schedule, and yield operations for kernel tasks.
- **Context Switching Foundation**: TaskContext structure ready for hardware context switching.

#### Core Features (Phases 1-2)
- **Interrupt Descriptor Table (IDT)**: 256-entry IDT for CPU exceptions and hardware interrupts.
- **Exception Handling**: Dedicated handlers with detailed stack frame information.
- **Thread-Safe VGA Driver**: Global `WRITER` using `lazy_static` and `spin` Mutexes.
- **"The Pulse" USP**: Dynamic visual dashboard that reacts to kernel events in real-time.

## ✨ Unique Selling Proposition (USP): "The Pulse"

NimbusOS features a **Dynamic Visual Dashboard** integrated directly into the VGA buffer. Instead of interrupts being hidden in background logs, NimbusOS makes them **visual**:

- **Real-time Feedback**: The top status bar acts as a "pulse," reacting instantly to kernel events.
- **Visual Callbacks**: When a Breakpoint or system exception fires, the status bar flashes **RED** with a signal message (`! PULSE DETECTED !`), bridging the gap between hardware execution and human observation.

## 🛠️ Getting Started

### Prerequisites

- **Rust Nightly**: Required for experimental features like `no_std` and `abi_x86_interrupt`.
  ```bash
  rustup override set nightly
  rustup component add rust-src llvm-tools-preview
  ```
- **Bootimage Tool**: To create bootable disk images.
  ```bash
  cargo install bootimage
  ```

### Building Locally

1. Clone the repository:
   ```bash
   git clone https://github.com/nishantXnova/Nimbus-OS.git
   cd Nimbus-OS
   ```
2. Build the kernel image:
   ```bash
   cargo bootimage
   ```

### Running in the Browser (v86)

NimbusOS is optimized for the **v86 WebAssembly Emulator**.

1. Once built, ensure `nimbus_os.bin` is in the root directory.
2. Start a local server:
   ```bash
   python -m http.server 8000
   ```
3. Open `http://localhost:8000`. You will see the brand-new diagnostic loader and the kernel boot process in real-time.

## 🗺️ Roadmap

- [x] **Phase 1: The Foundation** - VGA Driver & `no_std` Setup.
- [x] **Phase 2: The Reactive Core** - IDT, Interrupts, and "The Pulse" USP.
- [x] **Phase 3: Memory Management** - Global Descriptor Table (GDT) & Memory Protection.
- [x] **Phase 4: Multitasking** - Kernel threads, task scheduler, and context switching foundation.

## 📂 Project Structure

```
NimbusOS/
├── src/
│   ├── main.rs        # Kernel entry point & initialization
│   ├── vga_buffer.rs  # VGA text mode driver with "The Pulse"
│   ├── interrupts.rs  # IDT and exception handlers
│   ├── gdt.rs         # Global Descriptor Table setup
│   ├── memory.rs      # Memory management & frame allocation
│   └── scheduler.rs   # Task scheduler & TCB implementation
├── v86/               # WebAssembly emulator files
├── index.html         # Browser-based emulator UI
└── Cargo.toml         # Rust dependencies and build config
```

## 📜 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

*Built with passion by nishantXnova.*
