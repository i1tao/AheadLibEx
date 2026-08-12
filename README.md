# AheadLibEx (Rust)

AheadLibEx (Rust) is a Windows DLL proxy generator. It reads a target DLL, parses its export table, and generates a proxy DLL project that forwards exports to the original DLL.

中文文档请见 `README.zh-CN.md`.

## Disclaimer

AheadLibEx is intended to simplify DLL export analysis and proxy project generation. This program is provided for software compatibility work, reverse-engineering research, teaching, and authorized security testing. DLL proxying can also be abused; making this project publicly available does not imply approval of any unlawful or unauthorized use.

This program may only be used on software and systems owned by the user or covered by clear authorization from the owner. Work performed for a client or employer must remain within the agreed scope. Users are responsible for complying with the laws of their country or region and with any applicable contracts, software licenses, workplace rules, and confidentiality requirements.

This project must not be used for unauthorized access, persistence on someone else's system, credential theft, data destruction, malware delivery, or bypassing security products or access controls without permission. Help will not be provided for those purposes, and related issues, discussions, or contributions may be closed or removed.

AheadLibEx only generates source code and project files. The generated content has not been audited and should not be treated as production-ready. It should be reviewed, tested in an isolated environment, and used only after recoverable backups have been made. A DLL proxy changes how a program loads code; mistakes can cause crashes, data loss, security problems, or conflicts with security software. A successful parse or build does not prove that the result is safe, correct, compatible, or lawful for a particular use.

Users must have the right to use the DLL and any other files supplied to the tool. This repository's license does not grant rights to third-party DLLs, commercial software, system files, trademarks, private data, or generated code containing third-party material. Private binaries, credentials, personal information, customer data, and confidential logs must not be uploaded to public issues or pull requests.

This project is provided **AS IS**, with no promise that it will work for a particular purpose or be free of defects. Anyone who builds, modifies, runs, shares, or deploys the program or its output does so at their own risk. To the extent allowed by applicable law, the authors and contributors are not responsible for data loss, service interruption, system damage, security incidents, third-party claims, legal consequences, or other losses caused by using this project or generated code. This does not exclude any responsibility that cannot lawfully be excluded.

This statement explains the intended use and responsibility boundaries of the project. It is not legal advice and does not change the `GPL-3.0-only` license. The full license, including its warranty and liability terms, is in `LICENSE`. If permission to use or distribute any material is unclear, resolve that question before proceeding.

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
