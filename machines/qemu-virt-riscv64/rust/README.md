# RT-Thread Rust Component

A general-purpose Rust component for RT-Thread RTOS, supporting automatic multi-architecture detection.

## Features

- **Multi-architecture support**: Automatically detects ARM, AArch64, and RISC-V target architectures
- **Zero configuration**: No manual platform setup required
- **Modular design**: Core modules and example code are clearly separated
- **RT-Thread integration**: Full access to RT-Thread kernel APIs

## Project Structure

```
rust/
├── Cargo.toml           # Rust project configuration
├── src/
│   ├── lib.rs           # Main library entry point
│   ├── libc.rs          # Standard C library bindings
│   ├── librt.rs         # RT-Thread kernel API bindings
│   ├── init.rs          # Component initialization
│   └── examples/        # Example demos
│       ├── hello.rs     # Basic hello world
│       ├── printf_demo.rs   # Printf formatting
│       ├── string_demo.rs   # String operations
│       ├── memory_demo.rs   # Memory management
│       ├── vec_demo.rs      # Vec implementation
│       ├── thread_demo.rs   # RT-Thread threads
│       └── dlmodule_demo.rs   # Dynamic module loading
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
menuconfig
# Navigate to: Rust Component Support → Enable

# For dynamic modules, enable dynamic module config and file system:
#     1. RT-Thread online packages → system packages: enable lwext4 file system
#     2. RT-Thread Components → C/C++ and POSIX layer 
#                               → POSIX (Portable Operating System Interface) layer
#                               → Enable dynamic module APIs, dlopen()/dlsym()/dlclose() etc

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

## MSH Commands

| Command | Description |
|---------|-------------|
| `rust_hello` | Print greeting message |
| `rust_add <a> <b>` | Add two numbers |
| `rust_mul <a> <b>` | Multiply two numbers |
| `rust_strlen <str>` | Calculate string length |
| `rust_printf_demo` | Printf formatting demo |
| `rust_memory_demo` | Memory operation demo |
| `rust_thread_demo` | Thread demo |
| `rust_vec_demo` | Vec container demo |
| `rust_dl_demo` | Dynamic module loading demo |

## Configuration Options

Configure via `menuconfig`:

- `RT_USING_RUST` - Enable/disable Rust component
- `RUST_DEBUG_BUILD` - Build with debug symbols
- `RUST_EXAMPLE_*` - Enable specific examples
- `RUST_INIT_COMPONENT` - Auto-initialize at startup

## Technical Details

- **No-std**: Embedded-friendly `#![no_std]` environment
- **FFI**: Seamless C/Rust interoperability
- **Static linking**: Generates `.a` library files
- **Memory safety**: Compile-time guarantees from Rust
- **Zero cost**: Performance equivalent to C

## Extending Components

To add new features:

1. Create a module in `src/`
2. Add an example in `src/examples/`
3. Register the command in `rust_cmd.c`

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