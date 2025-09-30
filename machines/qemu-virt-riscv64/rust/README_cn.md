# RT-Thread Rust 组件

RT-Thread RTOS 的通用 Rust 组件，支持多架构自动检测。

## 特性

- **多架构支持**：自动检测 ARM、AArch64 和 RISC-V 目标架构
- **零配置**：无需手动配置目标平台
- **模块化设计**：核心模块与示例代码清晰分离
- **RT-Thread 集成**：完整访问 RT-Thread 内核 API

## 项目结构

```
rust/
├── Cargo.toml           # Rust 项目配置
├── src/
│   ├── lib.rs           # 主库入口点
│   ├── libc.rs          # 标准 C 库绑定
│   ├── librt.rs         # RT-Thread 内核 API 绑定
│   ├── init.rs          # 组件初始化
│   └── examples/        # 示例演示
│       ├── hello.rs     # 基础 hello world
│       ├── printf_demo.rs   # Printf 格式化
│       ├── string_demo.rs   # 字符串操作
│       ├── memory_demo.rs   # 内存管理
│       ├── vec_demo.rs      # vec实现
│       ├── thread_demo.rs   # RT-Thread 线程
│       └── dlmodule_demo.rs   # 动态模块加载
├── rust_cmd.c           # MSH 命令注册
├── SConscript           # 带自动检测的构建脚本
├── Kconfig             # 配置选项
```

## 快速开始

### 前置要求

1. **安装 Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **添加目标平台**（根据您的架构）
```bash
# RISC-V64（软浮点）
rustup target add riscv64imac-unknown-none-elf

# ARM Cortex-M4
rustup target add thumbv7em-none-eabi

# 其他目标请根据实际工具链/ABI 添加对应的 Rust target
```

### 构建

```bash
# 在 menuconfig 中启用 Rust 组件
scons --menuconfig
# 导航至：Rust Component Support → Enable

# 动态模块需要打开加载动态模块配置和文件系统：
#     1. RT-Thread online packages → system packages 打开lwext4文件系统
#     2. RT-Thread Components → C/C++ and POSIX layer 
#                               → POSIX (Portable Operating System Interface) layer
#                               → Enable dynamic module APIs, dlopen()/dlsym()/dlclose() etc

# 构建
scons

# 清理
scons -c
```

## 支持的架构

| 架构 | 目标 | 自动检测 |
|------|------|----------|
| Cortex-M3 | thumbv7m-none-eabi | ✓ |
| Cortex-M4/M7 | thumbv7em-none-eabi | ✓ |
| Cortex-M4F/M7F | thumbv7em-none-eabihf | ✓ |
| ARMv7-A | armv7a-none-eabi | ✓ |
| AArch64 | aarch64-unknown-none | ✓ |
| RISC-V32 | riscv32ima[f]c-unknown-none-elf | ✓ |
| RISC-V64 | riscv64[gc/imac]-unknown-none-elf | ✓ |

构建系统会自动从 RT-Thread 配置中检测正确的目标。

## MSH 命令

| 命令 | 描述 |
|------|------|
| `rust_hello` | 打印问候信息 |
| `rust_add <a> <b>` | 两数相加 |
| `rust_mul <a> <b>` | 两数相乘 |
| `rust_strlen <str>` | 计算字符串长度 |
| `rust_printf_demo` | Printf 格式化演示 |
| `rust_memory_demo` | 内存操作演示 |
| `rust_thread_demo` | 线程演示 |
| `rust_vec_demo` | vec容器演示|
| `rust_dl_demo` | 动态模块加载演示|

## 配置选项

通过 `menuconfig` 配置：

- `RT_USING_RUST` - 启用/禁用 Rust 组件
- `RUST_DEBUG_BUILD` - 使用调试符号构建
- `RUST_EXAMPLE_*` - 启用特定示例
- `RUST_INIT_COMPONENT` - 启动时自动初始化

## 技术细节

- **No-std**：嵌入式友好的 `#![no_std]` 环境
- **FFI**：无缝的 C/Rust 互操作性
- **静态链接**：生成 `.a` 库文件
- **内存安全**：Rust 的编译时保证
- **零成本**：性能等同于 C

## 扩展组件

通过以下方式添加新功能：

1. 在 `src/` 中创建模块
2. 在 `src/examples/` 中添加示例
3. 在 `rust_cmd.c` 中注册命令

## 应用场景

Rust 组件特别适合以下场景：

- **安全关键代码**：利用 Rust 的内存安全保证
- **复杂算法**：利用 Rust 的高级抽象能力
- **设备驱动**：类型安全的硬件抽象
- **网络协议栈**：安全的数据包处理
- **加密库**：防止内存泄露的安全实现

## 故障排除

### 链接错误

如果遇到 "can't link double-float modules with soft-float modules" 错误：
- 构建系统应该自动检测正确的 ABI
- 检查编译器的 `-mabi` 标志是否与 Rust 目标匹配

### 目标未安装

如果提示目标未安装：
```bash
rustup target add <目标名称>
```

### 检测失败

如果无法检测目标架构：
- 检查 RT-Thread 配置是否正确
- 查看 rtconfig.py 中的编译器标志

## 许可证

Apache-2.0

## 参考资料

- [Rust 嵌入式编程手册](https://docs.rust-embedded.org/)
- [RT-Thread 文档中心](https://www.rt-thread.org/document/site/)
- [Rust FFI 编程](https://doc.rust-lang.org/nomicon/ffi.html)
- [RISC-V 规范](https://riscv.org/technical/specifications/)