# RT-Thread 教育支持

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![RT-Thread](https://img.shields.io/badge/RT--Thread-OS-orange.svg)](https://www.rt-thread.org/)

本仓库旨在为学校提供基于RT-Thread操作系统的教学资源，包括**移植案例**、**实践教程**、**课程设计题目**以及**竞赛/毕业设计方向**。通过QEMU模拟器和真实硬件结合，帮助学习者快速掌握嵌入式系统、操作系统的开发技能。

## 目录结构

```
.
├── machines          # 支持的板卡移植
├── tutorials         # 教程文档
├── course-projects   # 课程设计题目
├── docs              # 技术文档和设计方案
└── rt-thread         # RT-Thread内核(submodule)
```

## 板卡支持计划

### QEMU模拟器支持
| 架构            | 基础功能支持                  | 网络支持  | 文件系统  |
|-----------------|-----------------------------|----------|----------|
| qemu-vexpress-a9     | 32位arm cortex-a9支持/设备驱动框架         | ✔️        | ✔️        |
| qemu-virt-riscv64    | 64位risc-v内核及命令行       | ✔️        | ✔️        |
| qemu-virt-aarch64    | 64位arm cortex-a53内核及命令行         | ✔️        | ✔️        |
| qemu-loongarch  | 龙芯64位指令集/基础外设          | 开发中    | 开发中    |

### 真实硬件支持介绍

#### STM32F407 星火①号开发板

前向深度嵌入式的arm cortex-m4，stm32f407-rt-spark开发板。

#### K230 RISCV64 AI开发板

包含riscv v指令集1.0的riscv AI开发板。

## 使用说明

```bash
# 以virt-riscv64为例
# 获取仓库及rt-thread代码
$ git clone https://github.com/rt-thread/qemu-edu.git
$ cd qemu-edu
$ git submodule update --init --recursive

$ cd machines/qemu-virt-riscv64
$ scons                 # 编译
$ ./qemu-nographic.sh   # 执行
```

## 教学资源

- [教程文档](tutorials/)
- [课题方向](course-projects/)
- [技术文档](docs/) - 包括Rust组件集成设计方案

## 贡献指南

欢迎高校师生和开发者参与建设：
1. 完善文档时请使用Markdown格式并附示意图
2. 提交Pull Request前需通过基础功能测试

## License

本项目采用与RT-Thread一致的Apache License 2.0：

```text
Copyright (c) 2025 RT-Thread Team
Licensed under the Apache License, Version 2.0
```

## 联系我们
- 社区论坛: https://club.rt-thread.org
- GitHub Issues: 提交技术问题
