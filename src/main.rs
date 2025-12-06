mod dll;
mod gui;
mod ui_events;
#[allow(dead_code)]
mod templates;

use anyhow::{bail, Result};
use std::{env, path::PathBuf};
use ui_events::{generate_cli, OutputTarget};

#[cfg(windows)]
use windows_sys::Win32::System::Console::FreeConsole;

fn print_usage() {
    println!("AheadLibEx usage:");
    println!("  aheadlibex-rs.exe <source|vs2022|vs2026> <dll_path> <output_dir>");
    println!("Examples:");
    println!("  aheadlibex-rs.exe source  \"C:\\\\path\\\\to\\\\foo.dll\" \"C:\\\\path\\\\to\\\\out\"");
    println!("  aheadlibex-rs.exe vs2022 \"C:\\\\path\\\\to\\\\foo.dll\" \"C:\\\\path\\\\to\\\\out\"");
    println!("  aheadlibex-rs.exe vs2026 \"C:\\\\path\\\\to\\\\foo.dll\" \"C:\\\\path\\\\to\\\\out\"");
    println!("No arguments -> GUI mode (console auto-detached on Windows).");
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

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        detach_console_if_needed(true);
        return gui::launch_gui();
    }

    if args.len() == 1 && matches!(args[0].as_str(), "-h" | "--help" | "help") {
        print_usage();
        return Ok(());
    }

    if args.len() != 3 {
        bail!("Usage: AheadLibEx <source|vs2022|vs2026> <dll_path> <output_dir>");
    }

    let target = match args[0].to_ascii_lowercase().as_str() {
        "source" | "src" | "c" => OutputTarget::Source,
        "vs2022" | "2022" => OutputTarget::Vs2022,
        "vs2026" | "2026" => OutputTarget::Vs2026,
        other => bail!("Unknown target '{}'. Use source|vs2022|vs2026.", other),
    };

    let dll_path = PathBuf::from(&args[1]);
    let output_dir = PathBuf::from(&args[2]);

    let written = generate_cli(target, &dll_path, &output_dir)?;
    println!("Generated {} file(s):", written.len());
    for path in written {
        println!("{path}");
    }

    Ok(())
}
