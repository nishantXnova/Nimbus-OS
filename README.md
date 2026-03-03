# NimbusOS: The Foundation 🌌

NimbusOS is a freestanding, 64-bit operating system kernel written in Rust. This project aims to explore the depths of low-level systems programming, starting from "First Light" (VGA output) to advanced memory management and multitasking.

## 🚀 Phase 1: The Foundation

NimbusOS currently implements the core essentials of a freestanding environment:
- **`no_std` Environment**: Complete independence from any host operating system.
- **Custom Target Specification**: Tailored `x86_64` JSON target to disable the Red Zone and enable software-based floating point.
- **VGA Text Driver**: A low-level driver communicating directly with hardware memory at `0xb8000`.
- **v86 Browser Emulator**: Integrated WebAssembly emulation for booting directly in a browser.

## 🛠️ Getting Started

### Prerequisites

- **Rust Nightly**: Required for experimental features like `no_std`.
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

1. Build the bootimage as shown above.
2. Rename the output:
   ```bash
   mv target/x86_64-nimbus_os/debug/bootimage-NimbusOS.bin bootimage-NimbusOS.bin
   ```
3. Start a local server:
   ```bash
   # Using Python
   python -m http.server 8000
   ```
4. Open `http://localhost:8000` in your browser.

## 🗺️ Roadmap

- [x] **Phase 1**: "First Light" - VGA Driver & `no_std` Setup.
- [ ] **Phase 2**: Interrupts & Exceptions (IDT).
- [ ] **Phase 3**: Memory Management (GDT & Paging).
- [ ] **Phase 4**: Kernel Multitasking.

## 📜 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

*Built with 💙 by the Nishant Paudel.*
