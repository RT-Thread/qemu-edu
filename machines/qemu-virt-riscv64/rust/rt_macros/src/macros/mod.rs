/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2025-01-XX     foxglove     Modular macro structure
 */

//! RT-Thread Rust 宏模块
//! 
//! 提供模块化的宏定义，包括：
//! - `rt_thread_main!` - 程序入口宏
//! - `rt_component_export!` - 组件导出宏
//! - `rt_app_export!` - 应用导出宏
//! - `msh_cmd_export!` - Shell命令导出宏

pub mod main;
pub mod component;
pub mod app;
pub mod cmd;

// 模块化宏实现，供主文件调用