#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use aheadlibex_rs::{gui, ui_events};
use anyhow::{bail, Result};
use std::{env, path::PathBuf};
use ui_events::{generate_cli, OutputTarget};

#[cfg(windows)]
use windows_sys::Win32::System::Console::{
    AttachConsole, AllocConsole, FreeConsole, ATTACH_PARENT_PROCESS,
};

fn print_usage() {
    println!("AheadLibEx usage:");
    println!("  aheadlibex-rs.exe <source|vs2022|vs2026|cmake> <dll_path> <output_dir> [options]");
    println!("Examples:");
    println!("  aheadlibex-rs.exe source  \"C:\\\\path\\\\to\\\\foo.dll\" \"C:\\\\path\\\\to\\\\out\"");
    println!("  aheadlibex-rs.exe vs2022 \"C:\\\\path\\\\to\\\\foo.dll\" \"C:\\\\path\\\\to\\\\out\"");
    println!("  aheadlibex-rs.exe vs2026 \"C:\\\\path\\\\to\\\\foo.dll\" \"C:\\\\path\\\\to\\\\out\"");
    println!("  aheadlibex-rs.exe cmake  \"C:\\\\path\\\\to\\\\foo.dll\" \"C:\\\\path\\\\to\\\\out\"");
    println!("Options:");
    println!("  --origin-mode <system|samedir|custom>   Where to load the original DLL (default: system).");
    println!("  --origin-name <name.dll>               Used when --origin-mode samedir (default: <stem>_orig.dll).");
    println!("  --origin-path <path>                   Used when --origin-mode custom (absolute path, UNC, or relative to proxy DLL dir).");
    println!("No arguments -> GUI mode (console auto-detached on Windows).");
}

fn print_cli_banner() {
    println!("AheadLibEx (Rust)");
    println!("Author: i1tao");
    println!("GitHub: https://github.com/i1tao/aheadlibex");
}

#[cfg(windows)]
fn detach_console_if_needed(no_args: bool) {
    if no_args {
        // 双击启动默认会带个控制台，这里把它关掉，只保留 GUI
        unsafe {
            let _ = FreeConsole();
        }
    }
}

#[cfg(not(windows))]
fn detach_console_if_needed(_no_args: bool) {}

#[cfg(windows)]
fn ensure_console_for_cli() {
    // GUI 构建为 windows 子系统后，CLI 模式需要主动附着或新建控制台，避免输出被吞
    unsafe {
        if AttachConsole(ATTACH_PARENT_PROCESS) == 0 {
            let _ = AllocConsole();
        }
    }
}

#[cfg(not(windows))]
fn ensure_console_for_cli() {}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let no_args = args.is_empty();

    if no_args {
        detach_console_if_needed(true);
        return gui::launch_gui();
    }

    ensure_console_for_cli();
    print_cli_banner();

    if args.len() == 1 && matches!(args[0].as_str(), "-h" | "--help" | "help") {
        print_usage();
        return Ok(());
    }

    if args.len() < 3 {
        bail!("Usage: AheadLibEx <source|vs2022|vs2026|cmake> <dll_path> <output_dir> [options]");
    }

    let target = match args[0].to_ascii_lowercase().as_str() {
        "source" | "src" | "c" => OutputTarget::Source,
        "vs2022" | "2022" => OutputTarget::Vs2022,
        "vs2026" | "2026" => OutputTarget::Vs2026,
        "cmake" | "cml" => OutputTarget::CMake,
        other => bail!(
            "Unknown target '{}'. Use source|vs2022|vs2026|cmake.",
            other
        ),
    };

    let dll_path = PathBuf::from(&args[1]);
    let output_dir = PathBuf::from(&args[2]);

    let origin_load_mode = parse_origin_load_mode(&args[3..], &dll_path)?;
    let written = generate_cli(target, &dll_path, &output_dir, origin_load_mode)?;
    println!("Generated {} file(s):", written.len());
    for path in written {
        println!("{path}");
    }

    Ok(())
}

fn parse_origin_load_mode(
    args: &[String],
    dll_path: &PathBuf,
) -> Result<aheadlibex_rs::templates::OriginLoadModeOwned> {
    use aheadlibex_rs::templates::OriginLoadModeOwned;

    let mut mode: Option<String> = None;
    let mut origin_name: Option<String> = None;
    let mut origin_path: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        let key = args[i].as_str();
        match key {
            "--origin-mode" | "--origin" => {
                let Some(v) = args.get(i + 1) else {
                    bail!("Missing value for {}", key);
                };
                mode = Some(v.to_string());
                i += 2;
            }
            "--origin-name" => {
                let Some(v) = args.get(i + 1) else {
                    bail!("Missing value for {}", key);
                };
                origin_name = Some(v.to_string());
                i += 2;
            }
            "--origin-path" => {
                let Some(v) = args.get(i + 1) else {
                    bail!("Missing value for {}", key);
                };
                origin_path = Some(v.to_string());
                i += 2;
            }
            "-h" | "--help" | "help" => {
                print_usage();
                std::process::exit(0);
            }
            other => {
                bail!("Unknown option '{}'. Use --help for usage.", other);
            }
        }
    }

    let mode = mode.unwrap_or_else(|| "system".to_string());
    match mode.to_ascii_lowercase().as_str() {
        "system" | "systemdir" | "sys" => Ok(OriginLoadModeOwned::system_dir()),
        "samedir" | "same" | "local" => {
            let default_name = dll_path
                .file_stem()
                .map(|s| format!("{}_orig.dll", s.to_string_lossy()))
                .unwrap_or_else(|| "origin_orig.dll".to_string());
            Ok(OriginLoadModeOwned::same_dir(
                origin_name.unwrap_or(default_name),
            ))
        }
        "custom" | "path" => {
            let Some(p) = origin_path else {
                bail!("--origin-mode custom requires --origin-path <path>");
            };
            Ok(OriginLoadModeOwned::custom_path(p))
        }
        other => bail!("Unknown --origin-mode '{}'. Use system|samedir|custom.", other),
    }
}
