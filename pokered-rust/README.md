# Pokémon Red/Blue — Rust Rewrite

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-2%2C446%20passing-brightgreen)](#running-tests)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A faithful reimplementation of Pokémon Red and Blue in Rust, based on the [pret/pokered](https://github.com/pret/pokered) disassembly. This project recreates the complete game logic — battles, overworld, events, menus, audio, and link features — as native Rust code with a modern rendering backend.

## What This Is

- **A native Rust game** that faithfully reproduces the Pokémon Red/Blue experience
- **Based on the complete disassembly** — all 151 Pokémon, 248 maps, trainer AI, event scripts, and game mechanics are implemented from the original assembly source
- **Modern rendering** — 160×144 pixel framebuffer displayed via GPU-accelerated window (wgpu/pixels)
- **Cross-platform** — runs on macOS, Windows, and Linux

## What This Is NOT

- **Not a Game Boy emulator** — does not emulate LR35902 CPU, PPU, or any hardware
- **Not byte-identical to the original ROM** — the game logic is equivalent, not binary-identical
- **Not bundled with copyrighted assets** — original graphics and music data must be loaded externally

---

## Architecture

The project is organized as a Cargo workspace with 7 crates, each with a clear responsibility:

```
pokered-rust/
├── Cargo.toml              # Workspace root
├── run-wasm/               # Helper for cargo-run-wasm (WASM dev server)
└── crates/
    ├── pokered-data/       # Static game data (species, moves, maps, items, trainers)
    ├── pokered-core/       # Pure game logic engine (battle, overworld, events, menus)
    ├── pokered-renderer/   # Graphics rendering (framebuffer, tiles, sprites, UI)
    ├── pokered-audio/      # Sound engine (APU emulation, music sequencer, SFX)
    ├── pokered-app/        # Native desktop executable (window creation, game loop, input)
    └── pokered-web/        # WebAssembly build (browser canvas, async GPU init)
```

### Crate Details

| Crate | Role | Key Dependencies |
|-------|------|-----------------|
| **pokered-data** | All static game data: 151 species stats, 165 moves, type chart, 248 map headers, wild encounter tables, trainer rosters, item definitions, evolution/learnset tables | `serde`, `strum`, `num-derive` |
| **pokered-core** | Pure game logic with zero I/O. Battle engine (damage calc, AI, status effects, stat stages), overworld (movement, collisions, warps, HM effects), event/script engine, menu state machines, save system, link battle/trade protocols | `pokered-data`, `serde`, `rand` |
| **pokered-renderer** | GPU-accelerated rendering into a 160×144 RGBA framebuffer. Tile/sprite rendering, text engine, menu boxes, battle scenes, screen transitions, palette management | `pokered-data`, `pokered-core`, `pixels` 0.15, `winit` 0.30, `image` |
| **pokered-audio** | Game Boy APU emulation with 4 sound channels (2× pulse, wave, noise). Music sequencer, SFX playback, audio mixing | `pokered-data`, `bitflags` |
| **pokered-app** | Native entry point. Creates the window, runs the game loop, dispatches input events, coordinates core↔renderer↔audio | All above + `anyhow`, `tracing` |
| **pokered-web** | WebAssembly entry point. Async Pixels init, DOM canvas attachment, browser resize, wasm-bindgen integration | `pokered-core`, `pokered-renderer`, `pixels`, `winit`, `wasm-bindgen`, `web-sys` |

### Dependency Graph

```
pokered-app (native)          pokered-web (wasm/native)
├── pokered-core              ├── pokered-core
│   └── pokered-data          │   └── pokered-data
├── pokered-renderer          ├── pokered-renderer
│   ├── pokered-core          │   ├── pokered-core
│   └── pokered-data          │   └── pokered-data
└── pokered-audio             └── pokered-audio
    └── pokered-data              └── pokered-data
```

> **Design principle**: `pokered-core` and `pokered-data` have no platform dependencies. All platform-specific code lives in `pokered-renderer` (GPU), `pokered-audio` (sound), `pokered-app` (native windowing), and `pokered-web` (browser/WASM).

---

## Prerequisites

### All Platforms

1. **Rust toolchain** (1.70 or later)

   Install via [rustup](https://rustup.rs/):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   Or on Windows, download and run [rustup-init.exe](https://win.rustup.rs/).

2. **Verify installation:**

   ```bash
   rustc --version    # Should show 1.70.0 or later
   cargo --version
   ```

### macOS

The renderer uses [wgpu](https://wgpu.rs/) (via the `pixels` crate) which requires Metal support.

```bash
# Install Xcode Command Line Tools (provides Metal framework + C compiler)
xcode-select --install
```

**Requirements:**
- macOS 10.13 (High Sierra) or later
- Xcode Command Line Tools (for Metal backend)
- No additional libraries needed — Metal and AppKit are provided by the OS

### Windows

```powershell
# Option 1: Install Visual Studio Build Tools (recommended)
# Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
# Select "Desktop development with C++" workload

# Option 2: If you already have Visual Studio 2019/2022 installed
# with the "Desktop development with C++" workload, you're all set
```

**Requirements:**
- Windows 10 (version 1703+) or Windows 11
- Visual Studio Build Tools 2019+ with C++ workload (provides MSVC linker)
- GPU with Vulkan or DX12 support (most GPUs from 2015+)
- No additional package manager installations needed

### Linux

The renderer and audio require several system libraries. Install them with your distribution's package manager:

**Debian / Ubuntu:**

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libxkbcommon-dev \
    libwayland-dev \
    libx11-dev \
    libxrandr-dev \
    libxi-dev \
    libxcursor-dev \
    libvulkan-dev \
    libasound2-dev \
    libudev-dev
```

**Fedora:**

```bash
sudo dnf install -y \
    gcc \
    pkg-config \
    libxkbcommon-devel \
    wayland-devel \
    libX11-devel \
    libXrandr-devel \
    libXi-devel \
    libXcursor-devel \
    vulkan-loader-devel \
    alsa-lib-devel \
    systemd-devel
```

**Arch Linux:**

```bash
sudo pacman -S --needed \
    base-devel \
    pkg-config \
    libxkbcommon \
    wayland \
    libx11 \
    libxrandr \
    libxi \
    libxcursor \
    vulkan-icd-loader \
    alsa-lib \
    systemd-libs
```

**Requirements:**
- Linux kernel 5.0+ recommended
- GPU with Vulkan support (Mesa 21.0+ or proprietary NVIDIA/AMD drivers)
- X11 or Wayland display server
- ALSA for audio output

---

## Building & Running

### Quick Start (All Platforms)

```bash
# Clone the repository (if you haven't already)
git clone https://github.com/pret/pokered.git
cd pokered/pokered-rust

# Build in release mode (recommended for performance)
cargo build --release

# Run the game
cargo run --release
```

### Build Modes

| Command | Description | Use Case |
|---------|-------------|----------|
| `cargo build` | Debug build (fast compile, slow runtime) | Development & debugging |
| `cargo build --release` | Release build (slow compile, fast runtime) | Playing the game |
| `cargo run` | Build and run (debug) | Quick iteration |
| `cargo run --release` | Build and run (release) | Best gameplay experience |

### Platform-Specific Notes

#### macOS

```bash
cd pokered-rust

# Build and run — Metal backend is selected automatically
cargo run --release

# The window opens at 480×432 (160×144 × 3x scale)
# If you see a "Developer Tools Access" prompt, click Allow
```

> **Apple Silicon (M1/M2/M3):** Works natively. Rust's `aarch64-apple-darwin` target is used automatically. No Rosetta needed.

> **Gatekeeper:** If building from source and running, there should be no Gatekeeper issues since you're compiling locally.

#### Windows

```powershell
cd pokered-rust

# Build and run — DX12 backend is preferred, falls back to Vulkan
cargo run --release

# If you see a Windows Defender SmartScreen warning on first run,
# click "More info" → "Run anyway" (expected for locally-built executables)
```

> **GPU Backend Selection:** wgpu automatically selects the best available backend: DX12 → Vulkan → DX11. You can override this by setting the `WGPU_BACKEND` environment variable:
> ```powershell
> $env:WGPU_BACKEND="vulkan"
> cargo run --release
> ```

#### Linux

```bash
cd pokered-rust

# Build and run — Vulkan backend is used
cargo run --release

# If you get a Vulkan error, verify your driver:
vulkaninfo --summary

# Force X11 if Wayland has issues:
WINIT_UNIX_BACKEND=x11 cargo run --release

# Force Wayland:
WINIT_UNIX_BACKEND=wayland cargo run --release
```

> **Headless / SSH:** The game requires a display server. It won't run over plain SSH. Use X11 forwarding (`ssh -X`) or a VNC session if needed.

> **NVIDIA on Linux:** If using proprietary drivers, ensure `libnvidia-gl` is installed and `vulkaninfo` shows your GPU.

### Environment Variables

| Variable | Values | Description |
|----------|--------|-------------|
| `WGPU_BACKEND` | `vulkan`, `dx12`, `dx11`, `metal`, `gl` | Force a specific GPU backend |
| `WINIT_UNIX_BACKEND` | `x11`, `wayland` | Force display server on Linux |
| `RUST_LOG` | `info`, `debug`, `trace` | Set log verbosity (requires `tracing` subscriber) |

### WebAssembly (Browser)

The game can also run in a web browser via WebAssembly. A separate `pokered-web` crate provides the WASM-compatible entry point with async GPU initialization and DOM canvas integration.

#### Prerequisites

```bash
# Add the wasm32 compilation target
rustup target add wasm32-unknown-unknown

# Install cargo-run-wasm (build + serve helper)
cargo install cargo-run-wasm
```

#### Build & Run

```bash
cd pokered-rust

# Build and serve the web version (opens browser automatically)
cargo run-wasm -p pokered-web

# Or build the WASM binary only (for custom deployment)
cargo build -p pokered-web --target wasm32-unknown-unknown --release
```

The `cargo run-wasm` command compiles the game to WASM, generates an HTML page with the `<canvas>` element, and starts a local development server.

#### Build Only (Cross-Compilation)

```bash
# Debug build
cargo build -p pokered-web --target wasm32-unknown-unknown

# Release build (smaller, faster WASM binary)
cargo build -p pokered-web --target wasm32-unknown-unknown --release
```

#### Technical Notes

- Uses `PixelsBuilder::build_async()` for WASM-compatible GPU surface creation
- Texture format is set to `Rgba8Unorm` on web targets (browsers don't support `Rgba8UnormSrgb`)
- The canvas is automatically attached to the document body and resized with the browser viewport
- `getrandom` crate uses the `js` feature for browser-compatible random number generation
- The `pokered-renderer::window` module (native-only) is excluded on wasm32 via `cfg` gating
- Title screen shows `[Web Build]` label to distinguish from native builds

#### Browser Requirements

- Modern browser with WebGPU support (Chrome 113+, Edge 113+, Firefox Nightly with flag)
- Falls back to WebGL2 if WebGPU is not available (via wgpu's automatic backend selection)

---

## Running Tests

The project has **2,446 tests** covering all game systems.

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p pokered-data
cargo test -p pokered-core
cargo test -p pokered-renderer
cargo test -p pokered-audio

# Run a specific test by name
cargo test test_damage_calculation

# Run tests with output (see println! in tests)
cargo test -- --nocapture

# Run only integration tests
cargo test --test game_flow_integration
cargo test --test event_script_integration
cargo test --test npc_trainer_integration
cargo test --test map_transition_integration
cargo test --test overworld_hm_terrain_integration
```

### Test Coverage by System

| System | Tests | Coverage |
|--------|-------|----------|
| Game data (species, moves, types, items) | ~400 | All 151 species, 165 moves, type chart, items |
| Battle engine (damage, AI, status, stat stages) | ~350 | Damage formula, type effectiveness, crits, status effects, trainer AI |
| Overworld (movement, warps, HMs, terrain) | ~300 | Map transitions, warps, Cut/Fly/Surf/Strength/Flash, ledges, dark caves |
| Event system (flags, scripts, NPCs, trainers) | ~250 | Event flags, script execution, NPC interaction, trainer line-of-sight |
| Menu system (bag, PC, mart, battle menus) | ~200 | All menu state machines, item use, Pokémon management |
| Save system (serialization, checksums) | ~50 | Save/load round-trip, original save format compatibility |
| Link system (battle, trade protocols) | ~50 | Protocol handshake, battle/trade state machines |
| Integration tests (full game flows) | ~300 | Start-to-Champion flow, trainer balance, event/map/overworld integration |

### Benchmarks

Performance benchmarks are available via [Criterion.rs](https://github.com/bheisler/criterion.rs):

```bash
# Run all benchmarks
cargo bench -p pokered-core

# Run a specific benchmark suite
cargo bench -p pokered-core --bench battle_bench
cargo bench -p pokered-core --bench overworld_bench
cargo bench -p pokered-core --bench data_bench
```

Benchmark results are saved to `target/criterion/` with HTML reports.

---

## Controls

| Game Boy Button | Keyboard Key | Action |
|----------------|-------------|--------|
| D-Pad Up | ↑ / W | Move up |
| D-Pad Down | ↓ / S | Move down |
| D-Pad Left | ← / A | Move left |
| D-Pad Right | → / D | Move right |
| A Button | Z / Enter | Confirm / Interact |
| B Button | X / Backspace | Cancel / Run |
| Start | Return / Space | Open menu |
| Select | Right Shift | (context-dependent) |

---

## Project Structure

```
pokered-rust/
├── Cargo.toml                          # Workspace configuration
├── Cargo.lock                          # Dependency lockfile
├── README.md                           # This file
│
├── crates/
│   ├── pokered-data/                   # Static game data
│   │   └── src/
│   │       ├── lib.rs                  # Data module exports
│   │       ├── species.rs              # 151 Pokémon base stats, types, catch rates
│   │       ├── moves.rs                # 165 move definitions (power, accuracy, PP, effects)
│   │       ├── type_chart.rs           # Type effectiveness matrix (15 types)
│   │       ├── items.rs                # Item definitions and effects
│   │       ├── maps/                   # 248 map headers, connections, warps
│   │       ├── trainers/               # Trainer rosters by class
│   │       ├── wild_pokemon.rs         # Wild encounter tables per map
│   │       └── evolution.rs            # Evolution chains and learnsets
│   │
│   ├── pokered-core/                   # Game logic (platform-independent)
│   │   ├── src/
│   │   │   ├── lib.rs                  # Core module exports
│   │   │   ├── battle/                 # Battle engine
│   │   │   │   ├── damage.rs           # Damage formula (Gen I)
│   │   │   │   ├── ai.rs              # Trainer AI (move selection, item use)
│   │   │   │   ├── status.rs          # Status effects (burn, freeze, sleep, etc.)
│   │   │   │   └── state.rs           # Battle state machine
│   │   │   ├── overworld/              # Overworld systems
│   │   │   │   ├── movement.rs         # Player/NPC movement and collision
│   │   │   │   ├── warps.rs           # Map transitions and warping
│   │   │   │   └── hm.rs             # HM field effects (Cut, Fly, Surf, etc.)
│   │   │   ├── event/                  # Event/script engine
│   │   │   ├── menu/                   # Menu state machines
│   │   │   ├── save/                   # Save system with checksum validation
│   │   │   └── link/                   # Link battle and trade protocols
│   │   ├── tests/                      # Integration tests
│   │   │   ├── game_flow_integration.rs
│   │   │   ├── trainer_battle_balance.rs
│   │   │   ├── event_script_integration.rs
│   │   │   ├── npc_trainer_integration.rs
│   │   │   ├── map_transition_integration.rs
│   │   │   └── overworld_hm_terrain_integration.rs
│   │   └── benches/                    # Performance benchmarks
│   │       ├── battle_bench.rs
│   │       ├── overworld_bench.rs
│   │       └── data_bench.rs
│   │
│   ├── pokered-renderer/               # Graphics rendering
│   │   └── src/
│   │       ├── lib.rs                  # FrameBuffer, constants (160×144, 3x scale)
│   │       ├── window.rs               # winit window creation and event loop
│   │       ├── tile.rs                 # 8×8 tile rendering
│   │       ├── sprite.rs               # Sprite rendering (OAM-style)
│   │       ├── tilemap.rs              # Background tilemap scrolling
│   │       ├── text_renderer.rs        # Text rendering with game font
│   │       ├── textbox.rs              # Dialog boxes and menus
│   │       ├── battle_scene.rs         # Battle screen layout
│   │       ├── battle_anim.rs          # Move animation effects
│   │       ├── transition.rs           # Screen transitions (fade, slide)
│   │       ├── palette.rs              # DMG 4-shade palettes
│   │       ├── menu.rs                 # Menu rendering
│   │       ├── viewport.rs             # Camera/viewport management
│   │       ├── window_layer.rs         # Window layer compositing
│   │       └── resource.rs             # Asset loading
│   │
│   ├── pokered-audio/                  # Sound engine
│   │   └── src/
│   │       ├── lib.rs                  # Audio module exports
│   │       ├── apu.rs                  # Game Boy APU emulation (4 channels)
│   │       ├── sequencer.rs            # Music sequence playback
│   │       └── sfx.rs                  # Sound effect playback
│   │
│   └── pokered-app/                    # Native desktop executable
│       └── src/
│           └── main.rs                 # Entry point, game loop
│
├── crates/pokered-web/                 # WebAssembly build
│   └── src/
│       └── main.rs                     # WASM entry point, async Pixels, DOM canvas
│
└── run-wasm/                           # cargo-run-wasm helper
    └── src/
        └── main.rs                     # Delegates to cargo_run_wasm
```

---

## Troubleshooting

### Build Errors

**"linker not found" (Windows)**
→ Install Visual Studio Build Tools with "Desktop development with C++" workload.

**"failed to find pkg-config" (Linux)**
→ `sudo apt install pkg-config` (Debian/Ubuntu) or `sudo dnf install pkg-config` (Fedora).

**"error: could not find libxkbcommon" (Linux)**
→ Install the missing system library. See the [Linux prerequisites](#linux) section.

### Runtime Errors

**"Failed to create GPU surface" / "No suitable GPU adapter"**
→ Your GPU may not support Vulkan/Metal/DX12. Try:
```bash
# Force OpenGL fallback (lower performance)
WGPU_BACKEND=gl cargo run --release
```

**"Wayland is not available" (Linux)**
→ Force X11: `WINIT_UNIX_BACKEND=x11 cargo run --release`

**Window opens but is blank/white**
→ GPU driver issue. Update your graphics drivers and try a different backend via `WGPU_BACKEND`.

**High CPU usage**
→ Ensure you're using `--release` mode. Debug builds are significantly slower.

---

## Game Fidelity

This rewrite aims for high fidelity with the original Pokémon Red/Blue:

- **Battle mechanics**: Gen I damage formula, critical hit rates, type effectiveness, stat stage modifiers, and all status effects match the original
- **Trainer AI**: Faithful reimplementation of the original trainer AI logic and move selection
- **Known bugs preserved**: Certain original game bugs (e.g., Focus Energy reducing crit rate, 1/256 miss chance) are faithfully reproduced for authenticity
- **Event system**: All 248 maps with correct event flags, script triggers, trainer sight lines, and item pickups
- **Save format**: Compatible with the original save checksum algorithm

---

## Related Projects

- [pret/pokered](https://github.com/pret/pokered) — The original disassembly this project is based on
- [pret.github.io](https://pret.github.io/) — Other pret disassembly projects

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

**Note:** This project does not include any copyrighted material from Nintendo, Game Freak, or Creatures Inc. The game logic was reimplemented from the publicly available disassembly. Original game assets (graphics, music) are not included and must be sourced separately.

