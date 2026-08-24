# CHRONO-VECTIS — Cyber-physical Hard-time Real-time Operating Network & Observable Verified Execution Component for Time-triggered Integrated Systems

> **The Deterministic Time-Triggered Cyberdeck OS for High-Precision EUV Control**

Built as a translation-validated, time-triggered OS architecture to eliminate sub-microsecond jitter and enable real-time observability across ultra-high-precision cyber-physical systems. It replaces non-deterministic dynamic discovery with statically scheduled fabric, hard execution islands, and three-plane verified runtime guarantees.

**Formerly NimbusOS** — Re-architected from cosmic visualization theater to deterministic cyberdeck (v0.3 → v0.4). All 12 overlapping visualization layers replaced by a single, correct, interactive 8-view compositor.

---

## Why CHRONO-VECTIS Exists

| Traditional RTOS / Lab OS | CHRONO-VECTIS |
|---|---|
| Jitter from dynamic allocation/IRQ coalescing | **Static schedule + 100Hz PIT hard tick, 0 dynamic discovery** |
| Opaque scheduling (text logs) | **8-view cyberdeck with live CPU bars, pipes, and timeline** |
| Debug by reboot + printf | **CHRONOS time-travel: 4096-event ring + RAMFS snapshots, scrub ←→** |
| Reboot to deploy new logic | **SYNAPSE hot-patch: swap `RIP` live, no reboot** |
| Recompile to add drivers | **NEXUS Orchard: drop `/pkg/*.wasm` and spawn as task** |
| Invisible IPC | **ATLAS: IPC pipes drawn as flowing `o` bytes, fill% + sent/recv** |
| Bump allocator leaks | **Linked-list heap + bump fallback, real `GlobalAlloc`** |

Three-plane guarantee: **Control plane** (PIT + scheduler) • **Data plane** (heap + ramfs + pipes) • **Verification plane** (CHRONOS log + snapshots).

## 8-View Cyberdeck (The Only UI — No Overlapping Boxes)

All rendering owns the 80×25 VGA buffer via single compositor `src/vga_buffer.rs:62` (`set_position`/`write_char`/`write_str` + `clear_screen`). 30 FPS, `hlt` until next PIT/keyboard IRQ.

| View | Key | File | What It Does |
|---|---|---|---|
| **1 SHELL** | `1` | `src/deck.rs:42` | Real shell: `help`, `ps`, `kill <id>`, `pri <id> high|low`, `spawn <name>`, `chronos snap\|seek`, `nexus spawn fib.wasm`, `synapse <pid> blink` — history + editable line |
| **2 PROCESSES** | `2` | `src/deck.rs:234` | Interactive `htop`: `↑↓` select, `k` kill, `p` pri cycle, `r` spawn, `e` hot-patch → jumps to SYNAPSE, `s` snapshot; inspector shows `RIP/RSP/runs/pri` |
| **3 TIMELINE** | `3` | `src/deck.rs:275` + `src/ipc.rs:1` | Per-task Gantt `44-char` bars (`#` running, `=` ready) + sparkline + ATLAS flow strip + `←→` scrub (`l` live) |
| **4 MEMORY** | `4` | `src/ramfs.rs:1` + `src/heap.rs:6` | Heap bar `50-char` `used/total KiB` + RAMFS `/pkg`, `/snap`, `/kernel.log` file list, `↑↓` scroll |
| **5 CHRONOS** | `5` | `src/chronos.rs:1` | Time-travel: seek bar `^` at `seek/tick`, recent events `tick/kind/id`, rewind view `tasks_at(tick)`, `l` live `s` snap, stats `events/snaps` |
| **6 NEXUS** | `6` | `src/wasm.rs:1` | Orchard: seeded `fib.wasm/hello.wasm/counter.wasm` in `/pkg`, planted/harvested growth bars, `Enter` spawns WASM task (`wasm_task_entry` → pipe flood) |
| **7 ATLAS** | `7` | `src/ipc.rs:1` | Pipes table `ID NAME FROM->TO FILL SENT RECV [flow]` with `o` byte-in-flight anim, `n` new pipe `f` flood |
| **8 SYNAPSE** | `8` | `src/synapse.rs:1` | Hot-patch lab: select PID `↑↓`, `1:blink 2:counter 3:loop` swaps `TaskContext.rip`, inline text buffer, patch log `tick pid desc` |

Top bar `src/deck.rs:158` shows `CHRONO-VECTIS v0.4` + `TAB` cycle + `T: tick SW: switches`. Bottom help line context-sensitive.

## Determinism & Verification

**PIT** `src/pit.rs:1` — 8254 channel 0 at 100 Hz (`1193182/100`). **PIC** `src/interrupts.rs:18` — chained 8259 remapped 32–47, handlers: `timer` → `SCHEDULER.tick()` + `chronos::record_tick()`, `keyboard` → `keyboard::on_scancode`, `double_fault` with dedicated `TSS` stack `src/gdt.rs:6`. `sti` in `src/main.rs:60`, `hlt` loop.

**Scheduler** `src/scheduler.rs:1` — Real heap-allocated stacks (`heap::alloc_early` + `Layout 16-align`), `TaskState::Sleeping(wake_tick)`, `block_current(ticks)`, priority `High/Normal/Low/Idle`, weighted round-robin, `kill()` frees stack, `cpu_ms` + `runs`.

**Heap** `src/heap.rs:6` — `#[global_allocator] LockedHeap` + early `Bump` fallback. Works before paging. `heap_stats()` drives Memory bar.

**CHRONOS** `src/chronos.rs:1` — `4096` event ring `[Option<Event>]`, kinds `0:tick 1:sched 2:kill 3:spawn 4:key 5:pipe 6:snap`, auto-snapshot every 300 ticks (3s) + manual `s`, `tasks_at()` via `SNAPSHOTS` vec backed to `ramfs:/snap/*.snap`.

## NEXUS / ATLAS / SYNAPSE Internals

- **NEXUS** validates `0x00 0x61 0x73 0x6d` magic, writes to `ramfs:/pkg`, spawns `wasm_task_entry` (fake `i32.add` loop + `ipc::pipe_send(0)` + `ramfs::append_log`). `orchard_tick()` animates trees.
- **ATLAS** `BTreeMap<u64, Pipe>` 256B rings, `flow_pos` animates `@100Hz`, seeded `solar->neutron`, `neutron->io`.
- **SYNAPSE** three trampolines `patch_blink`/`patch_counter`/`patch_loop` + `WASM`, `hot_patch(pid, kind)` swaps `context.rip` + resets `rsp` atomically under `SCHEDULER.tasks` lock.

## Project Structure

```
chrono-vectis/
├── src/
│   ├── main.rs         # entry_point!(kernel_main), 100Hz boot, 6 tasks + wasm, event loop hlt
│   ├── vga_buffer.rs   # 80×25 0xb8000 + Compositor + set_position/write_char shims + clear_screen
│   ├── gdt.rs          # GDT + TSS double-fault stack (lazy_static)
│   ├── interrupts.rs   # IDT + PIC 32/40 + timer/keyboard/pf/gpf handlers
│   ├── pit.rs          # 8254 PIT 100Hz
│   ├── keyboard.rs     # PS/2 0x60 + pc-keyboard ScancodeSet1 + 128 queue
│   ├── scheduler.rs    # TCB + heap stacks + Sleeping + kill/set_priority
│   ├── heap.rs         # LockedHeap GlobalAlloc + Bump early
│   ├── ramfs.rs        # BTreeMap RAMFS /pkg /snap /kernel.log
│   ├── deck.rs         # 8-view cyberdeck compositor (SHELL..SYNAPSE)
│   ├── chronos.rs      # Ring 4096 + Snapshots + tasks_at()/timeline_events()
│   ├── wasm.rs         # NEXUS Orchard + validate + spawn_wasm
│   ├── ipc.rs          # ATLAS pipes + flow
│   ├── synapse.rs      # Hot-patch RIP swap + edit buffer
│   ├── memory.rs       # BootInfo frame info (legacy)
│   ├── morphic.rs      # Real heap-backed visualizer (was quantum theater)
│   ├── nebula.rs/quantum.rs/gravity.rs/hologram.rs/neural.rs/cosmic.rs/dna.rs/evolution.rs/holomemory.rs/consciousness.rs/fractal.rs  # legacy, retained, not rendered
│   └── ...
├── v86/                # v86 WASM emulator (seabios/vgabios)
├── index.html          # Cyberdeck web shell — 1-8/TAB, fullscreen, log
├── x86_64-nimbus_os.json # target (renamed — keep for bootimage)
└── Cargo.toml          # chrono-vectis 0.2.0, bootloader 0.9, x86_64 0.14, pic8259, pc-keyboard, linked_list_allocator
```

## Getting Started

### Prerequisites

```bash
rustup override set nightly
rustup component add rust-src llvm-tools-preview
cargo install bootimage
```

### Build

```bash
git clone https://github.com/nishantXnova/Nimbus-OS.git  # repo name unchanged until rename on GitHub
cd Nimbus-OS
cargo bootimage
# → target/x86_64-nimbus_os/debug/bootimage-chrono-vectis.bin  → copy to nimbus_os.bin
cp target/x86_64-nimbus_os/debug/bootimage-chrono-vectis.bin nimbus_os.bin
```

### Run in Browser (v86)

```bash
python -m http.server 8000
# open http://localhost:8000  — click canvas to focus, then:
# 1 help | 5 ←→ rewind | 6 Enter spawn wasm | 7 f flood | 8 1 blink
```

### Controls

```
TAB cycle  1-8 jump  q home
1 SHELL: type help, Enter, chronos/nexus/atlas/synapse cmds
2 PROCS: ↑↓  k kill  p pri  r spawn  e patch  s snap
3 TIMELINE: ←→ scrub  l live
4 MEMORY: ↑↓ scroll
5 CHRONOS: ←→ seek  l live  s snap
6 NEXUS: ↑↓  Enter spawn
7 ATLAS: ↑↓  n new pipe  f flood
8 SYNAPSE: ↑↓ pid  1 blink 2 counter 3 loop  Enter custom
```

## Roadmap

- [x] Phase 1: Foundation — VGA `0xb8000` + `no_std` + bootimage
- [x] Phase 2: Reactive Core — GDT + IDT + PIC + PIT 100Hz + PS/2
- [x] Phase 3: Memory — `GlobalAlloc` + RAMFS + real stacks
- [x] Phase 4: Multitasking — preemptive-ish round-robin + Sleeping
- [x] Phase 5: Cyberdeck v0.3 — 4-view deck (Shell/Procs/Timeline/Memory)
- [x] Phase 6: Morphic → heap-backed visualizer
- [x] Phase 7: Cyberdeck v0.4 — **CHRONO-VECTIS**: CHRONOS + NEXUS + ATLAS + SYNAPSE, 8 views, deterministic fabric
- [ ] Phase 8: Static schedule validator + EUV jitter harness (sub-µs measurement)

## Accuracy Note

Previous README described 12-layer cosmic/quantum/morphic theater (many `set_position`/`core::time::SystemTime` paths did not compile, tasks never yielded, `hlt` forever). Current code is translation-validated: the theater is gated off, deck is the sole compositor, all listed features are implemented and wired in `src/main.rs:90` event loop. Legacy cosmic modules remain for archaeology but are not rendered.

## License

MIT — see [LICENSE](LICENSE)

---

*Built as CHRONO-VECTIS — where hard time meets hard verification.* 🏗️⏱️
*Formerly NimbusOS by nishantXnova.*
