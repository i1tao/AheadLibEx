# AheadLibEx (Rust)

AheadLibEx (Rust) is a Windows DLL proxy generator. It reads a target DLL, parses its export table, and generates a proxy DLL project that forwards exports to the original DLL.

中文文档请见 `README.zh-CN.md`.

## Outputs
- `source`: proxy sources only
- `vs2022`: Visual Studio 2022 solution and project
- `vs2026`: Visual Studio 2026 solution and project
- `cmake`: `CMakeLists.txt` for MSVC or MinGW-w64 builds

## What Gets Generated
- Export forwarding code based on the input DLL’s export table (names, ordinals, and forwarders)
- Proxy sources
  - x86: C proxy source
  - x64: C proxy source + jump table (MASM for MSVC-like toolchains, GAS for GNU-like toolchains)
- A `.def` file for controlling exports when the build system uses it
- Optional project files (Visual Studio or CMake), depending on the selected output

## Project Structure
- `domain`: DLL export parsing and core domain model
- `application`: generation orchestration and UI event logic
- `infrastructure`: templates and file generation
- `presentation`: GUI

## Quick Start
GUI:
- Launch `aheadlibex-rs.exe` with no arguments, then select a DLL and output directory.

CLI:

```text
aheadlibex-rs.exe <source|vs2022|vs2026|cmake> <dll_path> <output_dir> [--origin-mode <system|samedir|custom>] [--origin-name <name.dll>] [--origin-path <path>]
```

Examples (default `system` mode):

```text
aheadlibex-rs.exe source "C:\path\to\foo.dll" "C:\path\to\out"
aheadlibex-rs.exe vs2022 "C:\path\to\foo.dll" "C:\path\to\out"
aheadlibex-rs.exe vs2026 "C:\path\to\foo.dll" "C:\path\to\out"
aheadlibex-rs.exe cmake  "C:\path\to\foo.dll" "C:\path\to\out"
```

## Original DLL Loading
Generated proxy sources must load the original DLL. This project supports multiple load modes.

- `system` (default): load from `%SystemRoot%\System32\<dll>`
- `samedir`: load from the proxy DLL directory using a renamed filename (default name: `<stem>_orig.dll`)
- `custom`: load from a custom path (absolute, UNC, or relative to the proxy DLL directory)

Examples (custom load modes):

```text
aheadlibex-rs.exe vs2022 "C:\path\to\foo.dll" "C:\path\to\out" --origin-mode samedir --origin-name "foo_orig.dll"
aheadlibex-rs.exe source "C:\path\to\foo.dll" "C:\path\to\out" --origin-mode custom --origin-path "\\server\share\foo.dll"
```

Option notes:
- `--origin-name` is used by `--origin-mode samedir`
- `--origin-path` is used by `--origin-mode custom`

## Build Notes
- Visual Studio outputs: open the generated solution and build.
- CMake output: configure and build with your preferred generator. For example:

```text
cmake -S . -B build
cmake --build build --config Release
```

## Generated Files
The generated filenames are based on the input DLL stem (e.g. `version.dll` -> `version`).

`source`:
- x86: `<stem>_x86.c`, `<stem>_x86_jump.asm`, `<stem>_x86_jump.S`, `<stem>.def`
- x64: `<stem>_x64.c`, `<stem>_x64_jump.asm`, `<stem>_x64_jump.S`, `<stem>.def`

`cmake`:
- `CMakeLists.txt`
- Same files as `source` for the detected architecture.

`vs2022`:
- `AheadlibEx_<stem>.sln`
- `<stem>.vcxproj`, `<stem>.vcxproj.filters`, `<stem>.vcxproj.user`
- x86: `<stem>_x86.c`, `<stem>_x86_jump.asm`, `<stem>.def`
- x64: `<stem>_x64.c`, `<stem>_x64_jump.asm`, `<stem>.def`

`vs2026`:
- `AheadlibEx_<stem>.slnx`
- `<stem>.vcxproj`, `<stem>.vcxproj.filters`, `<stem>.vcxproj.user`
- x86: `<stem>_x86.c`, `<stem>_x86_jump.asm`, `<stem>.def`
- x64: `<stem>_x64.c`, `<stem>_x64_jump.asm`, `<stem>.def`

Notes:
- `.asm` is MASM (MSVC/clang-cl toolchains).
- `.S` is GAS (GNU-like toolchains). Visual Studio outputs only include `.asm`.

## Notes
- Export list is generated from the input DLL’s export table.
- xmake project generation has been removed (as of 2026-02-03).

## Author
- Author: i1tao
- Repository: https://github.com/i1tao/aheadlibex

## Credits

- Original idea and C++ implementation: [AheadLibEx](https://github.com/i1tao/AheadLibEx)
- Based on AheadLib-x86-x64 by [strivexjun](https://github.com/strivexjun/AheadLib-x86-x64)
- Thanks to [JetBrains](https://www.jetbrains.com/?from=i1tao) for providing free licenses such as [RustRover](https://www.jetbrains.com/Rust/?from=i1tao) for my open-source projects.
[<img src="https://resources.jetbrains.com/storage/products/company/brand/logos/RustRover_icon.png" alt="RustRover logo." width=200>](https://www.jetbrains.com/?from=i1tao)

## License

GPL-3.0-only. See `LICENSE`.
