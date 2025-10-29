# RT-Thread Rust Examples

This directory contains comprehensive examples demonstrating various aspects of Rust development for RT-Thread.

## Directory Structure

```
examples/
├── applications/          # User application examples
│   ├── hello_world/      # Basic parameter handling example
│   ├── thread_sync/      # Thread synchronization example
│   ├── fs_operations/    # File system operations
│   ├── ipc_demo/         # Inter-process communication (mutex, semaphore, queue)
│   └── dynamic_lib/      # Dynamic library loading example
├── components/           # RT-Thread component examples
│   ├── logger/          # Logging component
│   ├── sensor_driver/   # Component registry example
│   └── protocol_stack/  # Protocol stack component
└── modules/             # Dynamic module examples
    ├── simple_module/   # Basic dynamic module
    ├── device_module/   # Device driver module
    └── dynamic_lib/     # Dynamic library module
```

## Applications

### hello_world
Basic example demonstrating parameter handling and RT-Thread integration.
- **Command**: `rust_param_demo`
- **Features**: Parameter parsing, basic output

### thread_sync
Thread synchronization and management example.
- **Command**: `rust_thread_demo`
- **Features**: Thread creation, synchronization

### fs_operations
File system operations demonstration.
- **Command**: `rust_fs_demo`
- **Features**: File I/O, directory operations

### ipc_demo
Inter-process communication mechanisms.
- **Command**: `rust_ipc_demo [mutex|semaphore|queue]`
- **Features**: 
  - Mutex synchronization
  - Semaphore signaling
  - Message queue communication

### dynamic_lib
Dynamic library loading and usage.
- **Command**: `rust_loadlib_demo`
- **Features**: Dynamic module loading, symbol resolution

## Components

### logger
Logging component with different log levels.
- **Features**: Debug, info, warning, error logging

### sensor_driver
Component registry system demonstration.
- **Features**: Component registration, lifecycle management

### protocol_stack
Network protocol stack component.
- **Command**: `protocol_demo`
- **Features**: Layered protocol processing

## Modules

### simple_module
Basic dynamic module template.
- **Type**: Dynamic library (cdylib)
- **Features**: Module initialization/cleanup

### device_module
Device driver module example.
- **Command**: `device_demo`
- **Features**: Device initialization, I/O operations

### dynamic_lib
Advanced dynamic library with symbol export.
- **Features**: C-compatible exports, runtime loading

## Building Examples

### Prerequisites
- RT-Thread with Rust support enabled
- Rust toolchain configured for target architecture
- SCons build system

### Build Commands

```bash
# Build all examples
scons --rust-examples

# Build specific category
scons --rust-apps          # Applications only
scons --rust-components    # Components only
scons --rust-modules       # Modules only

# Build individual example
scons --rust-example=hello_world
```

### Configuration

Enable Rust examples in RT-Thread configuration:

```
RT-Thread Components  --->
    Rust Support  --->
        [*] Enable Rust Examples
        [*]   Build Applications
        [*]   Build Components  
        [*]   Build Modules
```

## Usage

1. **Flash the firmware** with Rust examples enabled
2. **Connect to RT-Thread shell** via serial console
3. **Run examples** using the command names listed above

Example session:
```
msh /> rust_ipc_demo mutex
=== Mutex Demo ===
Mutex counter: 1
Mutex counter: 2
...

msh /> protocol_demo
=== Protocol Stack Component Demo ===
Added layer: Physical to TCP/IP
Added layer: Data Link to TCP/IP
...
```

## Development Guidelines

### Adding New Examples

1. **Choose appropriate category** (applications/components/modules)
2. **Create directory structure**:
   ```
   category/example_name/
   ├── Cargo.toml
   ├── src/
   │   └── lib.rs
   └── README.md (optional)
   ```
3. **Configure dependencies** with correct relative paths
4. **Update build scripts** if necessary
5. **Document the example** in this README

### Best Practices

- Use `#![no_std]` for embedded compatibility
- Include proper copyright headers
- Use `macro_main_use` for shell commands
- Handle errors gracefully
- Provide clear documentation
- Follow RT-Thread naming conventions

## Troubleshooting

### Common Issues

1. **Build failures**: Check Cargo.toml paths and dependencies
2. **Runtime errors**: Verify RT-Thread configuration
3. **Missing commands**: Ensure examples are built and flashed

### Debug Tips

- Use `rt_rust::println!` for debugging output
- Check RT-Thread logs for initialization errors
- Verify memory allocation settings for complex examples

## Contributing

When contributing new examples:

1. Follow the existing directory structure
2. Include comprehensive documentation
3. Test on target hardware
4. Update this README with new examples
5. Ensure compatibility with RT-Thread standards

## License

All examples are licensed under Apache-2.0, consistent with RT-Thread licensing.