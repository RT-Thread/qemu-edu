# RT-Thread Rust组件目录结构建议

## 回复Issue：当加入rust组件支持时的目录结构

基于对RT-Thread现有组件架构的分析（如fal、vbus、utilities等组件），我为Rust组件支持提出以下目录结构建议。

## 一、推荐的目录结构

建议在 `rt-thread/components/rust/` 下创建以下结构：

```
rt-thread/components/rust/
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
├── msh/                         # Shell命令支持
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
   - `thread_entry!` - 标记线程入口函数
   - `msh_cmd_export!` - 导出shell命令的宏
   - 简化在no_std模式下的开发

### 2.2 运行时支持 (runtime/)

**解决no_std模式下的运行时问题：**

- **no_std_support.rs** - no_std环境支持
- **start.rs** - 启动代码，处理main函数到RT-Thread线程的转换
- **lang_items.rs** - 必要的语言项实现
- **linker/** - 链接脚本，用于模块加载

### 2.3 Shell命令支持 (msh/)

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

### 3.2 SConscript集成

```python
from building import *
import os

cwd = GetCurrentDir()
src = []
CPPPATH = [cwd]

if GetDepend(['RT_USING_RUST']):
    if GetDepend(['RT_RUST_CORE']):
        # 构建Rust核心库
        os.system('cd ' + cwd + '/core && cargo build --release')
        src += ['core/target/release/librt_rust_core.a']
    
    if GetDepend(['RT_RUST_RUNTIME']):
        # 构建运行时
        os.system('cd ' + cwd + '/runtime && cargo build --release')
        src += ['runtime/target/release/librt_rust_runtime.a']

group = DefineGroup('Rust', src, depend=['RT_USING_RUST'], CPPPATH=CPPPATH)
Return('group')
```

## 四、使用示例

### 4.1 简单的Rust应用

```rust
#![no_std]
#![no_main]

use rt_rust_core::prelude::*;

#[rt_thread_main]
fn main() {
    rt_println!("Hello from Rust!");
    
    let thread = Thread::create("rust_thread", thread_entry, 0, 2048, 20);
    thread.startup();
}

fn thread_entry(_parameter: *mut c_void) {
    loop {
        rt_println!("Rust thread running");
        Thread::delay(1000);
    }
}

#[msh_cmd_export]
fn hello_rust(_argc: i32, _argv: &[&str]) {
    rt_println!("Hello from Rust command!");
}
```

### 4.2 Rust组件示例

```rust
#![no_std]

use rt_rust_core::prelude::*;

pub struct RustLogger;

impl RustLogger {
    pub fn log(&self, level: LogLevel, msg: &str) {
        // 实现日志功能
    }
}

#[no_mangle]
pub extern "C" fn rust_logger_init() -> *mut RustLogger {
    Box::into_raw(Box::new(RustLogger))
}
```

### 4.3 内核模块示例

```rust
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

## 五、设计理念

本方案遵循RT-Thread的设计理念：

### 5.1 松耦合
- Rust组件作为可选模块，通过Kconfig控制
- 不影响现有C代码
- 可独立开发和测试

### 5.2 面向对象
- 使用Rust的trait体现接口抽象
- 通过模块划分体现对象特征
- 清晰的职责分离

### 5.3 文件/目录结构反映组件特点
- **core/** - 核心功能层
- **runtime/** - 运行时层
- **msh/** - shell集成层
- **examples/** - 按应用场景分类的示例
  - applications/ - 应用层
  - components/ - 组件层
  - modules/ - 模块层

### 5.4 可扩展性
- 提供丰富的示例
- 文档完善
- 易于添加新功能

## 六、与现有组件的关系

### 6.1 与libdl的集成

对于内核模块支持，需要扩展 `rt-thread/components/libc/posix/libdl`：

1. 添加对Rust编译的.so文件的支持
2. 处理Rust特定的符号和初始化
3. 实现模块加载钩子

建议在libdl中添加：
```
rt-thread/components/libc/posix/libdl/
├── dlrust.c          # Rust模块加载支持
└── dlrust.h          # 相关头文件
```

### 6.2 与其他组件的交互

- **finsh** - 通过msh子模块集成shell命令
- **drivers** - 通过bindings/device.rs封装设备驱动接口
- **libc** - 在需要时通过FFI调用libc函数
- **lwp** - 支持Rust编写的用户态程序

## 七、实施路线图

### 阶段一：基础设施
1. 创建core/目录，实现基本的C接口绑定
2. 实现runtime/，支持no_std模式
3. 完成构建系统集成
4. 实现hello_world示例

### 阶段二：功能完善
1. 完善线程、IPC、设备API绑定
2. 实现全局分配器
3. 实现msh命令导出
4. 添加应用示例

### 阶段三：高级功能
1. 扩展libdl支持Rust模块
2. 实现模块加载示例
3. 添加组件示例
4. 性能测试和优化

### 阶段四：文档和生态
1. 编写详细文档
2. 添加更多示例
3. 建立测试体系
4. 社区推广

## 八、参考实现

可以参考PR #12中的部分实现，以及以下开源项目：

1. [Linux内核Rust支持](https://github.com/Rust-for-Linux/linux)
2. [Zephyr RTOS的Rust集成](https://github.com/zephyrproject-rtos/zephyr/tree/main/samples/rust)
3. [嵌入式Rust示例](https://github.com/rust-embedded/rust-embedded)

## 总结

这个目录结构设计：

✅ **符合RT-Thread理念** - 松耦合、模块化、面向对象  
✅ **结构清晰** - 按功能层次划分目录  
✅ **易于扩展** - 提供丰富示例和文档  
✅ **完整覆盖** - 包含应用、组件、模块三个层次  
✅ **构建友好** - 与SCons构建系统良好集成  

希望这个建议对RT-Thread的Rust支持有所帮助！

---

详细的设计文档请参考：[docs/rust-component-structure-proposal.md](docs/rust-component-structure-proposal.md)
