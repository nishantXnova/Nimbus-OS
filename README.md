# NimbusOS: The Reactive Core 🌌

NimbusOS is a freestanding, 64-bit operating system kernel written in Rust. This project aims to explore the depths of low-level systems programming, from "First Light" (VGA output) to advanced memory management and multitasking.

## 🚀 Phase 2: The Reactive Core (Latest)

NimbusOS has transitioned from a static display to a **Reactive Environment**. 

### Key Technical Achievements:
- **Interrupt Descriptor Table (IDT)**: Implemented a 256-entry IDT to handle CPU exceptions and hardware interrupts.
- **Exception Handling**: Dedicated handlers for Breakpoint exceptions (`int3`), providing detailed stack frame information.
- **Thread-Safe VGA Driver**: Refactored the VGA driver into a global `static WRITER` using `lazy_static` and `spin` Mutexes for safe concurrency.
- **Enhanced v86 Emulator**: Integrated persistent diagnostic logging and brand identity (`logo.png`) into the bootloader interface.

## ✨ Unique Selling Proposition (USP): "The Pulse"

NimbusOS features a **Dynamic Visual Dashboard** integrated directly into the VGA buffer. Instead of interrupts being hidden in background logs, NimbusOS makes them **visual**:

- **Real-time Feedback**: The top status bar acts as a "pulse," reacting instantly to kernel events.
- **Visual Callbacks**: When a Breakpoint or system exception fires, the status bar flashes **RED** with a signal message (`! PULSE DETECTED !`), bridges the gap between hardware execution and human observation.

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
- [ ] **Phase 3: Memory Management** - Global Descriptor Table (GDT) & Paging.
- [ ] **Phase 4: Multitasking** - Kernel threads and context switching.

## 📜 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

*Built with passion by nishantXnova.*
