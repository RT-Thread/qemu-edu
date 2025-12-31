# 项目名称 - 支持Rust构建RT-Thrad安全组件

描述：随着Rust语言在系统级开发中的广泛应用，本项目要求参赛者为RT-Thread嵌入式实时操作系统扩展Rust语言支持能力。通过开发Rust语言绑定的内核接口、实现Rust编写的系统组件，探索Rust在嵌入式场景中实现内存安全与高性能并重的实践路径，为OS课程教学和工业应用提供新型开发范式。

* 难度：中高
* 支持单位：RT-Thread开源社区

[状态] ✅ 随着开源之夏的推进这份题目已经完结。

## 题目要求
- 基于[RT-Thread/qemu-edu](https://github.com/RT-Thread/qemu-edu)仓库完成实现；
- 在`rt-thread/components`下加入rust组件(目录)，实现Rust相关的支撑接口；
- 完善`rt-thread/components/libc/posix/libdl`加载内核模块的支持；
- 需保证Rust组件与C语言内核的ABI兼容性；
- 在QEMU/riscv64，K230/riscv64平台上完成实现和验证；
- 需包含说明文档，使用文档及性能相关文档；

### 特征
- 实现Rust到RT-Thread内核的系统调用安全绑定
- 使用Rust实现RT-Thread内核的功能组件，软件包；
- 使用Rust实现RT-Thread/Smart的内核模块；
- 开发至少3个Rust编写的核心组件（如文件系统、网络协议栈、设备驱动）

### 预期目标
- 建立Rust与RT-Thread内核交互的FFI标准规范
- 实现关键组件的零额外内存泄漏验证
- 相较C语言实现版本，Rust组件需达到类似的性能指标
- 输出可复用的Rust嵌入式开发教学案例库

### License
- Apache-2.0 License

### 参考资料
- [RT-Thread文档中心](https://www.rt-thread.org/document/site/#/)
- [Rust嵌入式手册](https://rust-embedded.github.io/book/)
- [C2Rust迁移工具](https://github.com/immunant/c2rust)
- [RT-Thread的Rust绑定实验项目](https://github.com/vito-chl/rtt_rs2)
- [RT-Thread的Rust支持](https://github.com/listentodella/rt-thread-rs)
- [RT-Thread/Smart的Rust用户态程序](https://github.com/diandianjunA/RT-Thread-Smart-Rust)
