# Rust Component for RT-Thread

A universal Rust component for RT-Thread RTOS with multi-architecture support.

## Features

- **Multi-Architecture Support**: Automatic target detection for ARM, AArch64, and RISC-V
- **Zero Configuration**: No manual target configuration needed
- **Modular Design**: Clean separation between core modules and examples
- **RT-Thread Integration**: Full access to RT-Thread kernel APIs

## Project Structure

```
rust/
├── Cargo.toml           # Rust project configuration
├── src/
│   ├── lib.rs           # Main library entry point
│   ├── libc.rs          # Standard C library bindings
│   ├── librt.rs         # RT-Thread kernel API bindings
│   ├── init.rs          # Component initialization
│   └── examples/        # Example demonstrations
│       ├── hello.rs     # Basic hello world
│       ├── printf_demo.rs   # Printf formatting
│       ├── string_demo.rs   # String operations
│       ├── memory_demo.rs   # Memory management
│       └── thread_demo.rs   # RT-Thread threading
├── rust_cmd.c           # MSH command registration
├── SConscript           # Build script with auto-detection
├── Kconfig             # Configuration options
```

## Quick Start

### Prerequisites

1. **Install Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Add Target** (based on your architecture)
```bash
# For RISC-V64 (soft-float)
rustup target add riscv64imac-unknown-none-elf

# For ARM Cortex-M4
rustup target add thumbv7em-none-eabi

# For other targets, add the corresponding Rust target per your toolchain/ABI
```

### Build

```bash
# Enable Rust component in menuconfig
menuconfig
# Navigate to: Rust Component Support → Enable

# Build
scons

# Clean
scons -c
```

## Supported Architectures

| Architecture | Target | Auto-Detection |
|-------------|--------|----------------|
| Cortex-M3 | thumbv7m-none-eabi | ✓ |
| Cortex-M4/M7 | thumbv7em-none-eabi | ✓ |
| Cortex-M4F/M7F | thumbv7em-none-eabihf | ✓ |
| ARMv7-A | armv7a-none-eabi | ✓ |
| AArch64 | aarch64-unknown-none | ✓ |
| RISC-V32 | riscv32ima[f]c-unknown-none-elf | ✓ |
| RISC-V64 | riscv64[gc/imac]-unknown-none-elf | ✓ |

The build system automatically detects the correct target from RT-Thread configuration.

## MSH Commands

| Command | Description |
|---------|-------------|
| `rust_hello` | Print hello message |
| `rust_add <a> <b>` | Add two numbers |
| `rust_mul <a> <b>` | Multiply two numbers |
| `rust_strlen <str>` | Calculate string length |
| `rust_printf_demo` | Printf formatting demo |
| `rust_memory_demo` | Memory operations demo |
| `rust_thread_demo` | Threading demo |

## Configuration Options

Configure via `menuconfig` (mapped to Cargo features automatically):

- `RT_USING_RUST` - Enable/disable Rust component
- `RUST_DEBUG_BUILD` - Build with debug symbols
- `RUST_EXAMPLE_*` - Enable specific examples (passed as Cargo features)
- `RUST_INIT_COMPONENT` - Auto-initialize on startup

## Technical Details

- **No-std**: Embedded-friendly `#![no_std]` environment
- **FFI**: Seamless C/Rust interoperability
- **Static Linking**: Produces `.a` library
- **Memory Safety**: Rust's compile-time guarantees
- **Zero-cost**: Performance equivalent to C

## Extending

Add new functionality by:

1. Creating modules in `src/`
2. Adding examples in `src/examples/`
3. Registering commands in `rust_cmd.c`

## License

Apache-2.0

## References

- [Rust Embedded Book](https://docs.rust-embedded.org/)
- [RT-Thread Documentation](https://www.rt-thread.org/document/site/)
- [FFI in Rust](https://doc.rust-lang.org/nomicon/ffi.html)
