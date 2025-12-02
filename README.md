# AheadLibEx (Rust)

Rust rewrite this project. It inspects a DLL’s export table and generates the proxy sources you need to build a hijack DLL, plus an optional ready-to-open Visual Studio project.

## Refactor Timeline

- **2025-12-01**: Rebuilt the GUI layer in Rust with a fixed layout, unified theming, and decoupled event handling.
- **2025-12-02**: Flattened modules (ui_events, dll, gui), enforced English-only UI/logs, made the output log read-only, and optimized export log building (fewer clones/allocs).


## Credits

- Original idea and C++ implementation: [AheadLibEx](https://github.com/i1tao/AheadLibEx)
- Based on AheadLib-x86-x64 by [strivexjun](https://github.com/strivexjun/AheadLib-x86-x64)
- Thanks to [JetBrains](https://www.jetbrains.com/?from=i1tao) for providing free licenses such as [RustRover](https://www.jetbrains.com/Rust/?from=i1tao) for my open-source projects.
[<img src="https://resources.jetbrains.com/storage/products/company/brand/logos/RustRover_icon.png" alt="RustRover logo." width=200>](https://www.jetbrains.com/?from=i1tao)