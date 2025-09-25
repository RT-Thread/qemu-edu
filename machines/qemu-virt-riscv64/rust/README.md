# RT-Thread Rust 组件

RT-Thread RTOS 的通用 Rust 组件，支持多架构自动检测。

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
│       └── hello.rs     # 基础 hello world
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

```

### 构建

```bash
# 在 menuconfig 中启用 Rust 组件
scons --menuconfig
# 导航至：Rust Component Support → Enable

# 构建
scons -j$(nproc)

# 清理
scons -c
```

## 支持的架构

| 架构 | 目标 | 自动检测 |
|------|------|----------|
| RISC-V32 | riscv32ima[f]c-unknown-none-elf | ✓ |
| RISC-V64 | riscv64[gc/imac]-unknown-none-elf | ✓ |

构建系统会自动从 RT-Thread 配置中检测正确的目标。

## MSH 命令

| 命令 | 描述 |
|------|------|
| `rust_hello` | 打印问候信息 |

## 配置选项

通过 `menuconfig` 配置：

- `RT_USING_RUST` - 启用/禁用 Rust 组件
- `RUST_DEBUG_BUILD` - 使用调试符号构建
- `RUST_EXAMPLE_*` - 启用特定示例
- `RUST_INIT_COMPONENT` - 启动时自动初始化


## 扩展组件

通过以下方式添加新功能：

1. 在 `src/` 中创建模块
2. 在 `src/examples/` 中添加示例
3. 在 `rust_cmd.c` 中注册命令

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

## 参考资料

- [Rust 嵌入式编程手册](https://docs.rust-embedded.org/)
- [RT-Thread 文档中心](https://www.rt-thread.org/document/site/)
- [Rust FFI 编程](https://doc.rust-lang.org/nomicon/ffi.html)
- [RISC-V 规范](https://riscv.org/technical/specifications/)
