# AheadLibEx (Rust) 中文版

Rust 重写的 AheadLibEx，用于解析 DLL 导出表并生成代理源码，以及可直接打开的 Visual Studio 项目。

## 功能
- 解析 PE 导出表，识别 DLL 架构（x86/x64），列出导出名/序号/转发。
- 生成代理源码：x86 输出 C 代理；x64 输出 C 代理 + 跳转 ASM，支持 C/C++ 构建。
- 生成 VS2022/VS2026 项目，按架构裁剪配置/文件，单选输出（源码或 VS 项目）。
- GUI：拖拽 DLL、目录选择、只读日志，英文界面/日志。
- CLI：`aheadlibex-rs.exe <source|vs2022|vs2026> <dll_path> <output_dir>`，支持 `--help`。

## 分层架构
- `domain`：领域模型与 DLL 导出解析（核心逻辑）。
- `application`：用例编排与 UI 事件（生成流程、状态管理）。
- `infrastructure`：模板与文件生成（使用 `CARGO_MANIFEST_DIR` 定位模板）。
- `presentation`：GUI 展示与交互。
- `lib.rs` 对外再导出核心模块，保持原有模块路径兼容。

## 重构时间线
- **2025-12-01**：Rust 重建 GUI，固定布局与主题，解耦事件。
- **2025-12-02 (Part 1)**：模块扁平化，英文 UI/日志，输出日志只读，优化导出日志构建。
- **2025-12-02 (Part 2)**：新增 VS2022 模板与单选输出，整理模板目录。
- **2025-12-03 (Part 1)**：新增 VS2026 模板，GUI 支持 VS2026 生成。
- **2025-12-03 (Part 2)**：按架构裁剪输出与模板占位；x64 生成 C+ASM，x86 仅 C；x86 使用 `AHEADLIB_EXTERN`。
- **2025-12-04**：导出宏来源改为项目名（大写+`_EXPORTS`），VS 模板注入宏。
- **2025-12-07**：CLI 完善（参数/帮助），GUI 启动自动分离控制台，模板统一四空格，VS 命名规范化。
- **2025-12-08**：分层为 enterprise-style（domain/application/infrastructure/presentation）并再导出；模板 include 改用 `CARGO_MANIFEST_DIR`；CLI banner 与 GUI 品牌对齐。
- **2026-02-01**：修正 x64 生成的 `#pragma comment(linker, "/EXPORT:...")` 引号转义。
- **2026-02-01**：调整 GUI 布局：`Project Settings` 上移至 `Output Log` 之上；`Outputs` 复选框改为左右平铺并支持换行；同步调整日志区域与窗口高度，避免底部信息被遮挡。

## 运行
- GUI：直接双击可启动（Windows Release 无控制台闪烁）。
- CLI：在同目录执行 `aheadlibex-rs.exe <target> <dll_path> <output_dir>`，打印生成文件列表。

## 许可证与作者
- 许可证：GPL-3.0-only，详见 `LICENSE`。
- 作者：i1tao；仓库：https://github.com/i1tao/aheadlibex
