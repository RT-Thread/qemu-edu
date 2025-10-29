# macro-main

## 功能描述

由于在没有Rust标准库的支持下，也就是`no_std`模式下，是无法使用main函数的。于是通过过程宏的方式对编写的代码进行重构，通过使用C ABI调用格式的main函数，就可以正常使用main函数。并自动把命令或应用段信息落到指定链接段，让 RT-Thread 在启动或命令表中发现并调用。

* 入口封装：生成一个 extern "C" 的入口函数，把 C 风格 argv 转成 Vec<ParamItem> ，再调用原始 Rust 函数。
* Component 段：生成的函数指针并落到 `.rti_fn.4` 段，作为**组件**初始化入口。
* App 段：生成的函数指针并落到 `.rti_fn.6` 段，作为**应用**初始化入口。
* 拓展export说明（部分未测试实现）：
  * 预初始化入口：`.rti_fn.2` 段
  * 设备初始化入口：`.rti_fn.3` 段
  * 组件初始化入口：`.rti_fn.4` 段（已测试）
  * 环境初始化入口：`.rti_fn.5` 段
  * 应用初始化入口：`.rti_fn.6` 段（已测试）
* MSH命令导出：生成命令描述结构体，并把该结构体落到 `FSymTab` 段，同时把 相关信息落到 `.rodata.name` 段。这样能扫描到并注册为 MSH 命令。

## 使用方法

1. 在 Rust 代码中引入 `macro_main` 过程宏。
2. 在 `main` 函数上添加 `#[macro_main_use]` 注解。
3. 根据需求注册为组件或应用。
   * 组件：`component = true`，会自动注册为组件初始化入口。
   * 应用：`app = true`，会自动注册为应用初始化入口。
4. 编译 Rust 代码，生成的可执行文件中就会包含 `main` 函数的入口。
5. 当C代码需要调用Rust函数时，需要在C代码中声明Rust函数的原型，并且使用 `extern "C"` 来指定调用约定。例如：`extern "C" void rust_function_name(void);`

## 参考实现

[RUST support for rt-thread](https://github.com/rust-for-rtthread/rtt_rust)
