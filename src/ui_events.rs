use std::{env, path::Path};

use crate::dll;
use eframe::egui;
use rfd;

pub const PROJECT_TYPES: [&str; 4] = [
    "Visual Studio 2022",
    "Visual Studio 2019",
    "Visual Studio 2017",
    "CMake",
];

const DEFAULT_LOG: &str =
    "AheadLibEx (Rust)\nAuthor: i1tao\nGitHub: https://github.com/i1tao/AheadLibEx\n------------------------------------------------------";

pub struct UiState {
    pub dll_path: String,
    pub project_dir: String,
    pub project_type: usize,
    pub log: String,
    pub dragging: bool,
    pub success: Option<bool>,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            dll_path: String::new(),
            project_dir: String::new(),
            project_type: 0,
            log: default_log(),
            dragging: false,
            success: None,
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

fn default_log() -> String {
    DEFAULT_LOG.to_string()
}

pub fn pick_dll(state: &mut UiState) {
    if let Some(p) = rfd::FileDialog::new()
        .add_filter("DLL", &["dll"])
        .pick_file()
    {
        state.dll_path = p.display().to_string();
        state.project_dir = default_project_dir(&p)
            .or_else(|| fallback_parent_dir(&p))
            .unwrap_or_default();
    }
}

pub fn pick_dir(state: &mut UiState) {
    if let Some(p) = rfd::FileDialog::new().pick_folder() {
        state.project_dir = p.display().to_string();
    }
}

pub fn generate(state: &mut UiState) {
    if state.dll_path.trim().is_empty() {
        state.log = "Please select a DLL file first".into();
        state.success = Some(false);
        return;
    }

    if state.project_dir.trim().is_empty() {
        state.log = "Please set output directory".into();
        state.success = Some(false);
        return;
    }

    let dll_path = Path::new(state.dll_path.trim());
    match dll::read_exports(dll_path) {
        Ok(info) => {
            let mut exports = info.exports;
            exports.sort_by_key(|e| e.ordinal);

            let mut out = String::with_capacity(state.log.len().max(256));
            use std::fmt::Write;
            let _ = writeln!(out, "DLL: {}", state.dll_path);
            let _ = writeln!(out, "Architecture: {}", info.arch);
            let _ = writeln!(out, "Output Dir: {}", state.project_dir);
            let _ = writeln!(out, "Project Type: {}", PROJECT_TYPES[state.project_type]);
            let _ = writeln!(out, "Exports: {}", exports.len());
            out.push_str("-- Export Table --\n");
            if exports.is_empty() {
                out.push_str("No export symbols\n");
            } else {
                for e in exports {
                    if let Some(fwd) = e.forwarder {
                        let _ = writeln!(out, "#{:>5} {} -> {}", e.ordinal, e.name, fwd);
                    } else {
                        let _ = writeln!(out, "#{:>5} {}", e.ordinal, e.name);
                    }
                }
            }

            state.log = out;
            state.success = Some(true);
        }
        Err(err) => {
            state.log = format!("Failed to parse DLL: {err}");
            state.success = Some(false);
        }
    }
}

pub fn reset(state: &mut UiState) {
    state.dll_path.clear();
    state.project_dir.clear();
    state.project_type = 0;
    state.log = default_log();
    state.success = None;
}

pub fn handle_drop(state: &mut UiState, ctx: &egui::Context) {
    ctx.input(|i| {
        state.dragging = !i.raw.hovered_files.is_empty();
        if let Some(f) = i.raw.dropped_files.first() {
            if let Some(p) = &f.path {
                if p.extension()
                    .map_or(false, |e| e.eq_ignore_ascii_case("dll"))
                {
                    state.dll_path = p.display().to_string();
                    state.project_dir = default_project_dir(p)
                        .or_else(|| fallback_parent_dir(p))
                        .unwrap_or_default();
                    state.log = format!(
                        "Loaded: {}",
                        p.file_name().unwrap_or_default().to_string_lossy()
                    );
                    state.success = None;
                }
            }
        }
    });
}

fn default_project_dir(dll_path: &Path) -> Option<String> {
    let exe_dir = env::current_exe().ok()?;
    let exe_dir = exe_dir.parent()?;
    let dll_name = dll_path.file_stem()?.to_string_lossy();
    let folder = format!("AheadLibEx_{}", dll_name);
    Some(exe_dir.join(folder).display().to_string())
}

fn fallback_parent_dir(dll_path: &Path) -> Option<String> {
    let stem = dll_path.file_stem()?.to_string_lossy();
    Some(
        dll_path
            .parent()
            .unwrap_or(Path::new("."))
            .join(format!("AheadLibEx_{}", stem))
            .display()
            .to_string(),
    )
}
