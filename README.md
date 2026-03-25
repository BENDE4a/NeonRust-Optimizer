# NeonRust Optimizer

Welcome to **NeonRust Optimizer**, the ultimate open-source tool designed specifically to boost FPS, reduce input lag, and fix stuttering in Rust. It utilizes low-level Windows API tweaks and registry editing to provide maximum performance.

## Features

- **Clean PC**: Automatically purges system Temp and Prefetch folders to free up space.
- **Kill Bloatware**: Kills background tasks and telemetry processes that steal CPU time.
- **Deep Rust Optimization (client.cfg)**: Tweaks hidden console variables like `gc.buffer`, `physics.steps`, `global.freezeshortcuts`, and disables shadow cascades for maximum performance and fewer garbage collection spikes.
- **Process Priority**: Forces `RustClient.exe` to high process priority for better CPU scheduling.
- **Clean RAM (Standby List)**: Functions similarly to ISLC, instantly freeing cached standby memory.
- **Lossless Scaling (Borderless Fullscreen)**: Removes window borders and forces Rust to span exactly across your monitor, fixing visual tearing and matching the functionality of external scaling apps.
- **Network Optimization**: Disables Nagle's Algorithm (`TCPNoDelay`) and lifts the `NetworkThrottlingIndex` to ensure the lowest ping and best hit-registration possible.
- **Ultimate Power Plan**: Unlocks and activates Windows' hidden "Ultimate Performance" power plan to prevent aggressive CPU downclocking.
- **Disable Game DVR**: Disables Windows 10/11 built-in game recording overlays that are notorious for causing stutters in Rust.
- **Timer Resolution Fix**: Sets the Windows system timer to 1ms to drastically improve input delay.

## How to Compile

Because it depends on specific local Windows APIs, you should compile it on your own machine.
1. Install [Rust](https://rustup.rs/).
2. Run the build command:
```bash
cargo build --release
```
3. Your executable will be ready in `target/release/rust_optimizer.exe`.

## Supported Languages
The CLI automatically supports switching between:
- English
- Ukrainian
- Russian

## Disclaimer
Always back up your Windows registry before making system-wide changes! The optimizer automatically creates a backup of your `client.cfg` file before modifying it.
