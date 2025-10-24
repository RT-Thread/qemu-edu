# RT-Thread Rust Component

A general-purpose Rust component for RT-Thread RTOS, supporting automatic multi-architecture detection.

## Features

- **Multi-architecture support**: Automatically detects ARM, AArch64, and RISC-V target architectures
- **Zero configuration**: No manual platform setup required
- **Modular design**: Core modules and example code are clearly separated
- **RT-Thread integration**: Full access to RT-Thread kernel APIs

## Project Structure

```
macro-rust/  # Macro component: transforms user-defined main into C ABI entry point.
                        
rt-rust/
├── Cargo.toml           # Rust project configuration
├── src/
    ├── bindings         # RT-Thread kernel API FFI bindings
    │   ├── libc.rs
    │   ├── librt.rs
    │   └── mod.rs
    ├── api              # RT-Thread kernel API Rust wrappers
    │   ├── base.rs
    │   ├── mem.rs
    │   └── thread.rs
    ├── lib.rs           # Main library entry point
    ├── init.rs          # Component initialization
    ├── libloader.rs     # Dynamic module loading
    ├── malloc.rs        # Memory allocation
    ├── mutex.rs         # Mutex
    ├── out.rs           # Output functions
    ├── param.rs         # Parameter passing
    ├── puts.rs          # String output
    ├── fs.rs            # File system operations
    ├── queue.rs         # Queue operations
    ├── sem.rs           # Semaphore
    ├── thread.rs        # Thread
    └── time.rs          # Time functions
├── rust_cmd.c           # MSH command registration
├── SConscript           # Build script with auto-detection
├── Kconfig              # Configuration options
```

## Quick Start

### Prerequisites

1. **Install Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Add target platforms** (according to your architecture)
```bash
# RISC-V64 (soft-float)
rustup target add riscv64imac-unknown-none-elf

# ARM Cortex-M4
rustup target add thumbv7em-none-eabi

# For other targets, add the corresponding Rust target based on your toolchain/ABI
```

### Build

```bash
# Enable Rust component in menuconfig
scons --menuconfig
# Navigate to: Rust Component Support → Enable


# Build
scons

# Clean
scons -c
```

## Supported Architectures

| Architecture | Target | Auto Detection |
|--------------|--------|---------------|
| Cortex-M3    | thumbv7m-none-eabi | ✓ |
| Cortex-M4/M7 | thumbv7em-none-eabi | ✓ |
| Cortex-M4F/M7F | thumbv7em-none-eabihf | ✓ |
| ARMv7-A      | armv7a-none-eabi | ✓ |
| AArch64      | aarch64-unknown-none | ✓ |
| RISC-V32     | riscv32ima[f]c-unknown-none-elf | ✓ |
| RISC-V64     | riscv64[gc/imac]-unknown-none-elf | ✓ |

The build system automatically detects the correct target from RT-Thread configuration.

## MSH Commands for User Testing

| Command | Description |
|---------|-------------|
| `rust_param_demo` | Parameter passing demo |
| `rust_thread_demo` | Thread demo |
| `rust_mutex_demo` | Mutex demo |
| `rust_queue_demo` | Queue demo |
| `rust_sem_demo` | Semaphore demo |
| `rust_dl_demo` | Dynamic module loading demo |
| `rust_fs_demo` | File system operations demo (requires logging component enabled) |

## Configuration Options

Configure via `menuconfig`:

- `RT_USING_RUST` - Enable/disable Rust component
- `RUST_INIT_COMPONENT` - Auto-initialize at startup
- `RUST_USING_LOG` - Enable/disable logging component
- `RUST_DEBUG_BUILD` - Enable/disable file system component

## Technical Details

- **No-std**: Embedded-friendly `#![no_std]` environment
- **FFI**: Seamless C/Rust interoperability
- **Static linking**: Generates `.a` library files
- **Memory safety**: Compile-time guarantees from Rust
- **Zero cost**: Performance equivalent to C

## User Programs

Write user applications as follows:

1. `cargo new --lib application`
2. Add dependencies in `Cargo.toml`:

```toml
[dependencies]
macro_main = { path = "path/to/macro-rust" }
rt_rust = { path = "path/to/rt-rust" }
```

3. Update `SConscript` to point to your `application` path

## Application Scenarios

Rust components are especially suitable for:

- **Safety-critical code**: Leverage Rust's memory safety guarantees
- **Complex algorithms**: Utilize Rust's advanced abstractions
- **Device drivers**: Type-safe hardware abstraction
- **Network protocol stacks**: Safe packet processing
- **Cryptography libraries**: Secure implementations preventing memory leaks

## Troubleshooting

### Link Errors

If you encounter "can't link double-float modules with soft-float modules" error:
- The build system should automatically detect the correct ABI
- Check if the compiler's `-mabi` flag matches the Rust target

### Target Not Installed

If prompted that the target is not installed:
```bash
rustup target add <target-name>
```

### Detection Failure

If target architecture cannot be detected:
- Check if RT-Thread configuration is correct
- Review compiler flags in rtconfig.py

## License

Apache-2.0

## References

- [Rust Embedded Programming Book](https://docs.rust-embedded.org/)
- [RT-Thread Documentation Center](https://www.rt-thread.org/document/site/)
- [Rust FFI Programming](https://doc.rust-lang.org/nomicon/ffi.html)
- [RISC-V Specification](https://riscv.org/technical/specifications/)