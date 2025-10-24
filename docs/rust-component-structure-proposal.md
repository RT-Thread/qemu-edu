# RT-Thread Rust组件目录结构设计方案

## 概述

本文档提供RT-Thread集成Rust语言支持的目录结构设计方案，遵循RT-Thread的松耦合、模块化设计理念。

## 一、总体目录结构

建议在`rt-thread/components/`下创建`rust`组件，整体结构如下：

```
rt-thread/components/rust/
├── Kconfig                      # Rust组件配置选项
├── SConscript                   # SCons构建脚本
├── README.md                    # 组件说明文档
├── docs/                        # 详细文档目录
│   ├── api_reference.md        # API参考文档
│   ├── abi_compatibility.md    # ABI兼容性说明
│   └── performance_report.md   # 性能测试报告
├── core/                        # Rust核心支持库
│   ├── Cargo.toml              # Rust项目配置
│   ├── build.rs                # 构建脚本
│   ├── src/
│   │   ├── lib.rs              # 库入口
│   │   ├── bindings/           # C接口绑定
│   │   │   ├── mod.rs
│   │   │   ├── kernel.rs       # 内核API绑定
│   │   │   ├── thread.rs       # 线程相关API
│   │   │   ├── ipc.rs          # IPC机制绑定
│   │   │   ├── memory.rs       # 内存管理API
│   │   │   └── device.rs       # 设备驱动API
│   │   ├── rt_prelude.rs       # RT-Thread预导入模块
│   │   ├── allocator.rs        # 内存分配器实现
│   │   ├── panic.rs            # panic处理器
│   │   └── macros/             # 宏定义
│   │       ├── mod.rs
│   │       ├── thread_entry.rs # 线程入口宏
│   │       └── msh_export.rs   # Shell命令导出宏
│   └── cbindgen.toml           # C头文件生成配置
├── runtime/                     # Rust运行时支持
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── no_std_support.rs   # no_std模式支持
│   │   ├── start.rs            # 启动代码
│   │   └── lang_items.rs       # 语言项实现
│   └── linker/                 # 链接脚本
│       └── rust_module.ld
├── shell/                       # Shell命令支持
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   └── commands.rs         # Rust命令实现
│   └── SConscript
├── examples/                    # 示例代码目录
│   ├── README.md               # 示例说明文档
│   ├── applications/           # 应用程序示例
│   │   ├── hello_world/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── main.rs     # 简单应用示例
│   │   ├── thread_sync/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── main.rs     # 线程同步示例
│   │   └── device_io/
│   │       ├── Cargo.toml
│   │       └── src/
│   │           └── main.rs     # 设备IO示例
│   ├── components/             # 组件/软件包示例
│   │   ├── logger/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── lib.rs      # 日志组件示例
│   │   ├── sensor_driver/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── lib.rs      # 传感器驱动示例
│   │   └── protocol_stack/
│   │       ├── Cargo.toml
│   │       └── src/
│   │           └── lib.rs      # 协议栈示例
│   └── modules/                # 内核动态模块示例
│       ├── simple_module/
│       │   ├── Cargo.toml
│       │   ├── module.ld       # 模块链接脚本
│       │   └── src/
│       │       └── lib.rs      # 简单内核模块
│       └── device_module/
│           ├── Cargo.toml
│           ├── module.ld
│           └── src/
│               └── lib.rs      # 设备驱动模块
├── tools/                       # 工具脚本
│   ├── build_rust.py           # Rust构建辅助脚本
│   ├── gen_bindings.sh         # 生成绑定脚本
│   └── cargo_wrapper.py        # Cargo包装脚本
└── tests/                       # 测试代码
    ├── integration/            # 集成测试
    └── unit/                   # 单元测试
```

## 二、各模块详细说明

### 2.1 核心支持库 (core/)

**目标**: 提供Rust与RT-Thread内核交互的基础设施。

**关键文件**:

- `bindings/`: 使用bindgen或手工编写的C API安全封装
  - 按功能模块划分（thread, ipc, memory, device等）
  - 提供类型安全的Rust接口
  
- `rt_prelude.rs`: 类似于std::prelude，预导入常用类型和trait
  ```rust
  // 示例
  pub use crate::bindings::thread::*;
  pub use crate::bindings::ipc::*;
  pub use crate::macros::*;
  ```

- `allocator.rs`: 实现`GlobalAlloc` trait，对接RT-Thread的内存管理
  ```rust
  #[global_allocator]
  static ALLOCATOR: RTThreadAllocator = RTThreadAllocator;
  ```

- `macros/`: 提供便利宏
  - `thread_entry!`: 将Rust函数标记为线程入口点
  - `msh_cmd_export!`: 导出shell命令
  ```rust
  // 使用示例
  #[msh_cmd_export]
  fn my_command(argc: i32, argv: &[&str]) {
      // 命令实现
  }
  ```

### 2.2 运行时支持 (runtime/)

**目标**: 提供no_std环境下的Rust运行时。

**关键实现**:

- `no_std_support.rs`: 
  - 实现`#![no_std]`环境所需的基础功能
  - 提供panic handler
  - 实现必要的语言项(lang_items)

- `start.rs`: 
  - 提供应用程序启动代码
  - 处理main函数到RT-Thread线程的转换
  ```rust
  #[no_mangle]
  pub extern "C" fn rust_main_wrapper(arg: *mut c_void) {
      // 初始化Rust环境
      // 调用用户main函数
  }
  ```

### 2.3 Shell命令支持 (shell/)

**目标**: 简化Rust程序导出shell命令的过程。

**功能**:
- 提供`MSH_CMD_EXPORT`宏的Rust版本
- 自动处理参数解析
- 支持命令帮助信息

### 2.4 示例代码 (examples/)

#### 2.4.1 应用程序示例 (applications/)

展示如何使用Rust编写RT-Thread应用：

1. **hello_world**: 最简单的Rust应用
   - 演示基础线程创建
   - 演示打印输出

2. **thread_sync**: 线程同步示例
   - 演示互斥锁、信号量使用
   - 演示消息队列通信

3. **device_io**: 设备IO示例
   - 演示设备打开/读写
   - 演示中断处理

#### 2.4.2 组件/软件包示例 (components/)

展示如何使用Rust开发可复用组件：

1. **logger**: 日志组件
   - 实现类似log crate的功能
   - 对接RT-Thread的ulog

2. **sensor_driver**: 传感器驱动
   - 演示设备驱动框架使用
   - 演示I2C/SPI通信

3. **protocol_stack**: 协议栈
   - 演示网络组件开发
   - 演示零拷贝优化

#### 2.4.3 内核模块示例 (modules/)

展示如何开发动态加载的内核模块：

1. **simple_module**: 简单模块
   - 演示模块初始化/清理
   - 演示符号导出

2. **device_module**: 设备驱动模块
   - 演示动态加载设备驱动
   - 演示热插拔支持

## 三、构建系统集成

### 3.1 Kconfig配置

```kconfig
menuconfig RT_USING_RUST
    bool "Rust Language Support"
    default n
    help
        Enable Rust language support for RT-Thread.

if RT_USING_RUST

    config RT_RUST_CORE
        bool "Enable Rust Core Library"
        default y
        help
            Core Rust bindings for RT-Thread kernel.

    config RT_RUST_RUNTIME
        bool "Enable Rust Runtime Support"
        default y
        select RT_RUST_CORE
        help
            Runtime support for no_std Rust applications.

    config RT_RUST_MSH_SUPPORT
        bool "Enable Rust MSH Command Support"
        default y
        depends on RT_USING_FINSH
        help
            Support exporting Rust functions to RT-Thread shell.

    config RT_RUST_ALLOCATOR
        bool "Enable Rust Global Allocator"
        default y
        depends on RT_USING_HEAP
        help
            Use RT-Thread memory allocator for Rust heap allocations.

    menu "Rust Examples"
        config RT_RUST_EXAMPLES_APPS
            bool "Build Application Examples"
            default n

        config RT_RUST_EXAMPLES_COMPONENTS
            bool "Build Component Examples"
            default n

        config RT_RUST_EXAMPLES_MODULES
            bool "Build Module Examples"
            default n
            depends on RT_USING_LWP && RT_USING_MODULE
    endmenu

    config RT_RUST_TOOLCHAIN_PATH
        string "Rust Toolchain Path"
        default ""
        help
            Path to Rust toolchain. Leave empty to use system default.

    choice RT_RUST_TARGET_ARCH
        prompt "Rust Target Architecture"
        default RT_RUST_TARGET_AUTO

        config RT_RUST_TARGET_AUTO
            bool "Auto Detect"

        config RT_RUST_TARGET_RISCV64
            bool "riscv64gc-unknown-none-elf"

        config RT_RUST_TARGET_AARCH64
            bool "aarch64-unknown-none"

        config RT_RUST_TARGET_ARM
            bool "armv7a-none-eabi"
    endchoice

endif
```

### 3.2 SConscript集成

主`SConscript`文件应处理：

1. Cargo项目构建
2. 生成的.a库链接
3. C绑定头文件生成

```python
from building import *
import os

cwd = GetCurrentDir()
src = []
CPPPATH = [cwd]

if GetDepend(['RT_USING_RUST']):
    # 构建Rust核心库
    if GetDepend(['RT_RUST_CORE']):
        os.system('cd ' + cwd + '/core && cargo build --release --target=...')
        src += ['core/target/release/librt_rust_core.a']
    
    # 构建运行时
    if GetDepend(['RT_RUST_RUNTIME']):
        os.system('cd ' + cwd + '/runtime && cargo build --release --target=...')
        src += ['runtime/target/release/librt_rust_runtime.a']
    
    # 构建示例
    if GetDepend(['RT_RUST_EXAMPLES_APPS']):
        src += SConscript('examples/applications/SConscript')

group = DefineGroup('Rust', src, depend=['RT_USING_RUST'], CPPPATH=CPPPATH)

Return('group')
```

## 四、使用方式示例

### 4.1 编写简单的Rust应用

```rust
// examples/applications/hello_world/src/main.rs
#![no_std]
#![no_main]

use rt_rust_core::prelude::*;

#[rt_thread_main]
fn main() {
    rt_println!("Hello from Rust!");
    
    // 创建线程
    let thread = Thread::create("rust_thread", thread_entry, 0, 2048, 20);
    thread.startup();
}

fn thread_entry(_parameter: *mut c_void) {
    loop {
        rt_println!("Rust thread running");
        Thread::delay(1000);
    }
}

// 导出shell命令
#[msh_cmd_export]
fn hello_rust(_argc: i32, _argv: &[&str]) {
    rt_println!("Hello from Rust command!");
}
```

### 4.2 编写Rust组件

```rust
// examples/components/logger/src/lib.rs
#![no_std]

use rt_rust_core::prelude::*;

pub struct RustLogger;

impl RustLogger {
    pub fn log(&self, level: LogLevel, msg: &str) {
        // 实现日志功能
    }
}

// 导出C接口供C代码使用
#[no_mangle]
pub extern "C" fn rust_logger_init() -> *mut RustLogger {
    Box::into_raw(Box::new(RustLogger))
}
```

### 4.3 编写内核模块

```rust
// examples/modules/simple_module/src/lib.rs
#![no_std]

use rt_rust_core::module::*;

#[module_init]
fn module_init() -> i32 {
    rt_println!("Rust module loaded!");
    0
}

#[module_exit]
fn module_exit() {
    rt_println!("Rust module unloaded!");
}

module_info! {
    name: "rust_simple_module",
    author: "RT-Thread",
    license: "Apache-2.0",
}
```

## 五、关键设计考虑

### 5.1 ABI兼容性

- 使用`extern "C"`确保函数调用兼容
- 使用`#[repr(C)]`确保结构体布局兼容
- 提供bindgen配置自动生成绑定

### 5.2 内存安全

- 全局分配器对接RT-Thread内存管理
- 提供安全的所有权模型包装
- 处理FFI边界的内存管理

### 5.3 无标准库支持

- 实现no_std环境必需的trait
- 提供自定义panic handler
- 实现必要的语言项

### 5.4 性能优化

- 零成本抽象设计
- 内联关键函数
- 避免不必要的边界检查

### 5.5 文档和测试

- 提供详细的API文档
- 包含丰富的示例代码
- 编写集成测试和单元测试

## 六、开发路线图

### 阶段一：基础设施 (1-2个月)
- [ ] 完成核心绑定库
- [ ] 实现基本运行时支持
- [ ] 完成构建系统集成
- [ ] 编写hello_world示例

### 阶段二：API完善 (2-3个月)
- [ ] 完善线程、IPC、设备API
- [ ] 实现全局分配器
- [ ] 完善宏系统
- [ ] 编写应用示例

### 阶段三：高级功能 (2-3个月)
- [ ] 支持动态模块
- [ ] 完善shell命令支持
- [ ] 编写组件示例
- [ ] 性能测试和优化

### 阶段四：文档和生态 (持续)
- [ ] 完善文档
- [ ] 编写教程
- [ ] 建立测试体系
- [ ] 社区推广

## 七、与现有组件的交互

### 7.1 与libc的关系

Rust组件可以与RT-Thread的libc共存：
- 在no_std模式下，Rust有自己的核心库
- 需要时可以通过FFI调用libc函数
- 避免符号冲突

### 7.2 与libdl的关系

对于内核模块支持：
- 扩展libdl以支持Rust编译的.so文件
- 处理Rust特定的符号和初始化
- 实现模块加载钩子

### 7.3 与设备驱动框架的关系

Rust可以实现设备驱动：
- 提供设备驱动trait
- 封装设备注册接口
- 支持中断处理

## 八、参考资料

1. RT-Thread组件开发指南
2. [Rust嵌入式开发手册](https://github.com/rust-embedded/book)
3. [Linux内核Rust支持项目](https://github.com/Rust-for-Linux/linux)
4. [Zephyr RTOS](https://github.com/zephyrproject-rtos/zephyr)

## 九、总结

本设计方案遵循RT-Thread的设计哲学：

1. **松耦合**: Rust组件作为独立模块，可选择启用
2. **模块化**: 清晰的目录结构，功能分离
3. **面向对象**: 使用Rust的trait和结构体体现OOP思想
4. **可扩展**: 提供丰富的示例和文档，便于扩展

通过这个结构，Rust可以无缝集成到RT-Thread中，为开发者提供内存安全和高性能的开发体验。
