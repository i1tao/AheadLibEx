use std::{env, path::Path};

use eframe::egui;
use rfd;

use super::PROJECT_TYPES;

#[derive(Default)]
pub struct UiState {
    pub dll_path: String,
    pub project_dir: String,
    pub project_type: usize,
    pub log: String,
    pub dragging: bool,
    pub success: Option<bool>,
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

    state.log = format!(
        "Configuration complete\nDLL: {}\nOutput: {}\nType: {}",
        state.dll_path, state.project_dir, PROJECT_TYPES[state.project_type]
    );
    state.success = Some(true);
}

pub fn reset(state: &mut UiState) {
    state.dll_path.clear();
    state.project_dir.clear();
    state.project_type = 0;
    state.log = "Reset".into();
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
