# AheadLibEx (Rust)

Rust rewrite this project. It inspects a DLL’s export table and generates the proxy sources you need to build a hijack DLL, plus an optional ready-to-open Visual Studio project.

## Refactor Timeline

- **2025-12-01**: Rebuilt the GUI layer in Rust with a fixed layout, unified theming, and decoupled event handling.
- **2025-12-02 (Part 1)**: Flattened modules (ui_events, dll, gui), enforced English-only UI/logs, made the output log read-only, and optimized export log building (fewer clones/allocs).
- **2025-12-02 (Part 2)**: Added templated VS2022 project/source generation (C/ASM + sln/vcxproj), grouped templates under `templates/`, and updated GUI to pick outputs via single-select checkboxes with auto-scroll logs.
- **2025-12-03 (Part 1)**: Added VS2026 templates (slnx/vcxproj/filters/user) alongside shared C/ASM templates; GUI supports generating VS2026 projects;
- **2025-12-03 (Part 2)**: Generation now follows DLL architecture: x86 only emits proxy C; x64 emits C + jump ASM; VS2022/VS2026 templates trim configs/files per arch, filters/platforms adjust, nested placeholders resolve correctly, and x86 trampolines use `AHEADLIB_EXTERN` for C++ builds.


## Credits

- Original idea and C++ implementation: [AheadLibEx](https://github.com/i1tao/AheadLibEx)
- Based on AheadLib-x86-x64 by [strivexjun](https://github.com/strivexjun/AheadLib-x86-x64)
- Thanks to [JetBrains](https://www.jetbrains.com/?from=i1tao) for providing free licenses such as [RustRover](https://www.jetbrains.com/Rust/?from=i1tao) for my open-source projects.
[<img src="https://resources.jetbrains.com/storage/products/company/brand/logos/RustRover_icon.png" alt="RustRover logo." width=200>](https://www.jetbrains.com/?from=i1tao)
