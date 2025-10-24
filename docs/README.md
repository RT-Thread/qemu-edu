# RT-Thread Rust支持文档

本目录包含关于在RT-Thread中集成Rust语言支持的设计文档和建议。

## 文档列表

### [Rust组件目录结构建议](rust-component-structure-recommendation.md)

**简介版本** - 针对Issue的直接回复文档，包含：
- 推荐的目录结构概览
- 各模块功能说明
- 构建系统集成方案
- 使用示例
- 实施路线图

适合快速了解Rust组件的整体架构设计。

### [Rust组件目录结构详细设计方案](rust-component-structure-proposal.md)

**详细版本** - 完整的技术设计文档，包含：
- 详细的目录结构说明
- 各模块深入设计
- 完整的Kconfig和SConscript示例
- 详细的使用示例和代码片段
- 关键设计考虑（ABI兼容性、内存安全、性能优化等）
- 开发路线图
- 与现有组件的交互关系
- 参考资料

适合实际开发时参考。

## 背景

这些文档是针对Issue [当加入rust组件支持时的目录结构](https://github.com/RT-Thread/qemu-edu/issues/XX) 的设计建议，目标是为RT-Thread提供：

1. **Rust本身的基础支持**
   - RT-Thread操作系统的系统服务绑定
   - no_std模式下的运行时支持
   - 宏定义简化开发

2. **Shell命令集成**
   - 导出Rust函数到RT-Thread shell

3. **丰富的示例**
   - 使用Rust编写应用的示例
   - 使用Rust编写组件或软件包的示例
   - 使用Rust编写内核动态模块的示例

## 设计理念

遵循RT-Thread的核心理念：
- ✅ **松耦合** - Rust作为可选组件，不影响现有代码
- ✅ **模块化** - 清晰的目录结构，功能分离
- ✅ **面向对象** - 通过文件、目录结构反映组件特点

## 相关资源

- [RT-Thread官方文档](https://www.rt-thread.org/document/site/)
- [Rust嵌入式开发手册](https://rust-embedded.github.io/book/)
- [课程项目：支持Rust构建RT-Thread安全组件](../course-projects/01-rust-extension.md)
- PR #12: RT-Thread Rust支持的部分实现

## 贡献

欢迎对这些设计方案提出建议和改进意见。请通过Issue或Pull Request参与讨论。

---

**最后更新**: 2025-10-24  
**维护者**: RT-Thread开源社区
