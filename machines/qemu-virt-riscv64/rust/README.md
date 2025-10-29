# RT-Thread Rust组件目录结构建议

## 一、目录结构

 `rust/` 为以下结构：

```
rust/
├── Kconfig                      # 组件配置
├── SConscript                   # 构建脚本
├── README.md                    # 组件说明
├── docs/                        # 文档
│   ├── api_reference.md
│   ├── abi_compatibility.md
│   └── performance_report.md
├── core/                        # 核心支持库
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── bindings/           # C接口绑定
│   │   │   ├── kernel.rs       # 内核API
│   │   │   ├── thread.rs       # 线程API
│   │   │   ├── ipc.rs          # IPC机制
│   │   │   ├── memory.rs       # 内存管理
│   │   │   └── device.rs       # 设备API
│   │   ├── rt_prelude.rs       # 预导入模块
│   │   ├── allocator.rs        # 内存分配器
│   │   ├── panic.rs            # panic处理
│   │   └── macros/             # 宏支持
│   │       ├── thread_entry.rs # 线程入口宏
│   │       └── msh_export.rs   # Shell命令导出宏
├── runtime/                     # 运行时支持
│   ├── Cargo.toml
│   ├── src/
│   │   ├── no_std_support.rs   # no_std模式
│   │   ├── start.rs            # 启动代码
│   │   └── lang_items.rs       # 语言项
│   └── linker/
│       └── rust_module.ld      # 链接脚本
├── shell/                       # Shell命令支持
│   ├── Cargo.toml
│   └── src/
│       └── commands.rs
├── examples/                    # 示例代码
│   ├── README.md
│   ├── applications/           # 应用示例
│   │   ├── hello_world/
│   │   ├── thread_sync/
│   │   └── device_io/
│   ├── components/             # 组件示例
│   │   ├── logger/
│   │   ├── sensor_driver/
│   │   └── protocol_stack/
│   └── modules/                # 内核模块示例
│       ├── simple_module/
│       └── device_module/
├── tools/                       # 工具脚本
│   ├── build_rust.py
│   ├── gen_bindings.sh
│   └── cargo_wrapper.py
└── tests/                       # 测试
    ├── integration/
    └── unit/
```

## 二、各部分功能说明

### 2.1 核心支持库 (core/)

**提供Rust本身的基础支持和RT-Thread系统服务绑定：**

1. **bindings/** - C接口的安全封装
   - 按功能模块划分（kernel、thread、ipc、memory、device）
   - 使用bindgen自动生成或手工编写
   - 提供类型安全的Rust接口

2. **rt_prelude.rs** - 预导入模块
   - 类似std::prelude，导入常用类型和trait
   - 简化Rust代码编写

3. **allocator.rs** - 全局内存分配器
   - 实现`GlobalAlloc` trait
   - 对接RT-Thread的内存管理系统

4. **macros/** - 宏定义
   - `rt_thread_main!` - 程序入口宏，标记Rust的main函数
   - `rt_component_export!` - 导出组件初始化入口的宏
   - `rt_app_export!` - 导出应用初始化入口的宏
   - `msh_cmd_export!` - 导出shell命令的宏
   - 简化在no_std模式下的开发

### 2.2 运行时支持 (runtime/)

**解决no_std模式下的运行时问题：**

- **no_std_support.rs** - no_std环境支持
- **start.rs** - 启动代码，处理main函数到RT-Thread线程的转换
- **lang_items.rs** - 必要的语言项实现
- **linker/** - 链接脚本，用于模块加载

### 2.3 Shell命令支持 (shell/)

**导出Rust命令到RT-Thread shell：**

- 提供`MSH_CMD_EXPORT`宏的Rust版本
- 自动处理参数解析
- 与finsh组件集成

### 2.4 示例代码 (examples/)

**包含三类示例：**

1. **applications/** - 使用Rust编写应用
   - `hello_world/` - 基础示例
   - `thread_sync/` - 线程同步示例
   - `device_io/` - 设备IO示例

2. **components/** - 使用Rust编写组件/软件包
   - `logger/` - 日志组件
   - `sensor_driver/` - 传感器驱动
   - `protocol_stack/` - 协议栈

3. **modules/** - 使用Rust编写内核动态模块
   - `simple_module/` - 简单模块示例
   - `device_module/` - 设备驱动模块示例

## 三、构建系统集成

### 3.1 Kconfig配置示例

```kconfig
menuconfig RT_USING_RUST
    bool "Rust Language Support"
    default n
if RT_USING_RUST
    config RT_RUST_CORE
        bool "Enable Rust Core Library"
        default y
    config RT_RUST_RUNTIME
        bool "Enable Rust Runtime Support"
        default y
        select RT_RUST_CORE
    config RT_RUST_MSH_SUPPORT
        bool "Enable Rust MSH Command Support"
        default y
        depends on RT_USING_FINSH
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
endif
```