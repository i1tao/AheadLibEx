# AheadLibEx (Rust)

AheadLibEx (Rust) 用于解析 DLL 导出表并生成代理 DLL 工程：根据导出表生成转发逻辑与代理源码，并按所选输出类型生成 Visual Studio 或 CMake 构建文件。

English document: `README.md`。

## 输出类型
- `source`：仅生成代理源码
- `vs2022`：生成 Visual Studio 2022 解决方案与工程
- `vs2026`：生成 Visual Studio 2026 解决方案与工程
- `cmake`：生成 `CMakeLists.txt`，用于 MSVC 或 MinGW-w64 构建

## 生成内容
- 基于输入 DLL 的导出表生成导出转发逻辑（导出名、序号、转发项）
- 代理源码
  - x86：仅生成 C 代理源码
  - x64：生成 C 代理源码与跳转表（MSVC 类工具链生成 MASM，GNU 类工具链生成 GAS）
- 生成用于控制导出的 `.def` 文件（在对应构建系统下使用）
- 按输出类型生成工程文件（Visual Studio 或 CMake）

## 快速使用
GUI：
- 无参数启动 `aheadlibex-rs.exe`，选择 DLL 与输出目录，然后选择输出类型并生成。

CLI：

```text
aheadlibex-rs.exe <source|vs2022|vs2026|cmake> <dll_path> <output_dir> [--origin-mode <system|samedir|custom>] [--origin-name <name.dll>] [--origin-path <path>]
```

示例（默认 `system` 模式）：

```text
aheadlibex-rs.exe source "C:\path\to\foo.dll" "C:\path\to\out"
aheadlibex-rs.exe vs2022 "C:\path\to\foo.dll" "C:\path\to\out"
aheadlibex-rs.exe vs2026 "C:\path\to\foo.dll" "C:\path\to\out"
aheadlibex-rs.exe cmake  "C:\path\to\foo.dll" "C:\path\to\out"
```

## 原始 DLL 加载
生成的代理源码需要加载原始 DLL，支持多种加载模式。

- `system`（默认）：从 `%SystemRoot%\System32\<dll>` 加载
- `samedir`：从代理 DLL 所在目录加载改名后的原始 DLL（默认文件名：`<stem>_orig.dll`）
- `custom`：从自定义路径加载（绝对路径、UNC，或相对代理 DLL 目录的相对路径）

示例（自定义加载模式）：

```text
aheadlibex-rs.exe vs2022 "C:\path\to\foo.dll" "C:\path\to\out" --origin-mode samedir --origin-name "foo_orig.dll"
aheadlibex-rs.exe source "C:\path\to\foo.dll" "C:\path\to\out" --origin-mode custom --origin-path "\\server\share\foo.dll"
```

选项说明：
- `--origin-name` 与 `--origin-mode samedir` 配合使用
- `--origin-path` 与 `--origin-mode custom` 配合使用

## 构建说明
- Visual Studio 输出：打开生成的解决方案进行构建。
- CMake 输出：使用常规 CMake 流程配置与构建，例如：

```text
cmake -S . -B build
cmake --build build --config Release
```

## 输出文件清单
生成文件名以输入 DLL 的文件名主体为基准（例如 `version.dll` 的主体为 `version`）。

`source`：
- x86：`<stem>_x86.c`、`<stem>_x86_jump.asm`、`<stem>_x86_jump.S`、`<stem>.def`
- x64：`<stem>_x64.c`、`<stem>_x64_jump.asm`、`<stem>_x64_jump.S`、`<stem>.def`

`cmake`：
- `CMakeLists.txt`
- 以及与 `source` 相同的架构文件集合。

`vs2022`：
- `AheadlibEx_<stem>.sln`
- `<stem>.vcxproj`、`<stem>.vcxproj.filters`、`<stem>.vcxproj.user`
- x86：`<stem>_x86.c`、`<stem>_x86_jump.asm`、`<stem>.def`
- x64：`<stem>_x64.c`、`<stem>_x64_jump.asm`、`<stem>.def`

`vs2026`：
- `AheadlibEx_<stem>.slnx`
- `<stem>.vcxproj`、`<stem>.vcxproj.filters`、`<stem>.vcxproj.user`
- x86：`<stem>_x86.c`、`<stem>_x86_jump.asm`、`<stem>.def`
- x64：`<stem>_x64.c`、`<stem>_x64_jump.asm`、`<stem>.def`

说明：
- `.asm` 为 MASM（MSVC 与 clang-cl 工具链）。
- `.S` 为 GAS（GNU 类工具链）。Visual Studio 输出仅包含 `.asm`。

## 备注
- 导出列表来自输入 DLL 的导出表解析结果。
- 已于 2026-02-03 移除 xmake 输出目标与相关模板。

## 作者
- 作者：i1tao；仓库：https://github.com/i1tao/aheadlibex

## Credits

- Original idea and C++ implementation: [AheadLibEx](https://github.com/i1tao/AheadLibEx)
- Based on AheadLib-x86-x64 by [strivexjun](https://github.com/strivexjun/AheadLib-x86-x64)
- Thanks to [JetBrains](https://www.jetbrains.com/?from=i1tao) for providing free licenses such as [RustRover](https://www.jetbrains.com/Rust/?from=i1tao) for my open-source projects.
[<img src="https://resources.jetbrains.com/storage/products/company/brand/logos/RustRover_icon.png" alt="RustRover logo." width=200>](https://www.jetbrains.com/?from=i1tao)

## License

GPL-3.0-only. See `LICENSE`.
