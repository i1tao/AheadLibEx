use std::{env, path::Path};

use crate::dll;
use crate::templates::{
    render_asm, render_c, render_filters, render_solution, render_user, render_vcxproj, VsGuids,
    VsTemplateContext,
};
use eframe::egui;
use rfd;
use std::fs;
use uuid::Uuid;

const DEFAULT_LOG: &str =
    "AheadLibEx (Rust)\nAuthor: i1tao\nGitHub: https://github.com/i1tao/AheadLibEx\n------------------------------------------------------";

pub struct UiState {
    pub dll_path: String,
    pub project_dir: String,
    pub output_source: bool,
    pub output_vs2022: bool,
    pub output_vs2026: bool,
    pub log: String,
    pub dragging: bool,
    pub success: Option<bool>,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            dll_path: String::new(),
            project_dir: String::new(),
            output_source: false,
            output_vs2022: false,
            output_vs2026: false,
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
    state.success = None;
    state.log = "Generating project...".into();

    if !state.output_source && !state.output_vs2022 && !state.output_vs2026 {
        state.log = "Please select at least one output target (Source/VS2022/VS2026)".into();
        state.success = Some(false);
        return;
    }

    // enforce single selection
    let selected_count =
        state.output_source as u8 + state.output_vs2022 as u8 + state.output_vs2026 as u8;
    if selected_count > 1 {
        state.log = "Please select only one output target".into();
        state.success = Some(false);
        return;
    }

    match dll::read_exports(dll_path) {
        Ok(info) => {
            let mut exports = info.exports.clone();
            exports.sort_by_key(|e| e.ordinal);

            let mut out = String::with_capacity(state.log.len().max(256));
            use std::fmt::Write;
            let _ = writeln!(out, "DLL: {}", state.dll_path);
            let _ = writeln!(out, "Architecture: {}", info.arch);
            let _ = writeln!(out, "Output Dir: {}", state.project_dir);
            let targets = selected_targets(state);
            let _ = writeln!(out, "Targets: {}", targets.join(", "));
            let _ = writeln!(out, "Exports: {}", exports.len());
            out.push_str("-- Export Table --\n");
            if exports.is_empty() {
                out.push_str("No export symbols\n");
            } else {
                for e in &exports {
                    if let Some(fwd) = &e.forwarder {
                        let _ = writeln!(out, "#{:>5} {} -> {}", e.ordinal, e.name, fwd);
                    } else {
                        let _ = writeln!(out, "#{:>5} {}", e.ordinal, e.name);
                    }
                }
            }

            state.log = out;
            state.success = Some(true);

            let exports_for_write = exports;

            if state.output_source {
                match write_source_files(
                    dll_path,
                    Path::new(state.project_dir.trim()),
                    &exports_for_write,
                ) {
                    Ok(_) => state.log.push_str("\n-- Source files written successfully --"),
                    Err(err) => {
                        state.log
                            .push_str(&format!("\n-- Source write failed --\n{err}"));
                        state.success = Some(false);
                    }
                }
            }

            if state.output_vs2022 {
                match write_vs2022_project(
                    dll_path,
                    Path::new(state.project_dir.trim()),
                    &exports_for_write,
                ) {
                    Ok(_) => state
                        .log
                        .push_str("\n-- VS2022 project written successfully --"),
                    Err(err) => {
                        state
                            .log
                            .push_str(&format!("\n-- VS2022 project write failed --\n{err}"));
                        state.success = Some(false);
                    }
                }
            }

            if state.output_vs2026 {
                state
                    .log
                    .push_str("\n-- VS2026 generation not implemented yet --");
            }
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
    state.output_source = false;
    state.output_vs2022 = false;
    state.output_vs2026 = false;
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

fn new_guid_braced() -> String {
    format!("{{{}}}", Uuid::new_v4())
}

fn selected_targets(state: &UiState) -> Vec<String> {
    let mut v = Vec::new();
    if state.output_source {
        v.push("Source".to_string());
    }
    if state.output_vs2022 {
        v.push("VS2022".to_string());
    }
    if state.output_vs2026 {
        v.push("VS2026".to_string());
    }
    v
}

fn write_source_files(
    dll_path: &Path,
    output_dir: &Path,
    exports: &[dll::ExportEntry],
) -> anyhow::Result<Vec<String>> {
    let dll_stem = dll_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("Invalid DLL filename"))?;
    let dll_name = dll_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| format!("{dll_stem}.dll"));

    let base_name = dll_stem;

    let guid_solution = new_guid_braced();
    let guid_project = new_guid_braced();
    let guid_source = new_guid_braced();
    let guid_header = new_guid_braced();
    let guid_resource = new_guid_braced();

    let guids = VsGuids {
        solution: &guid_solution,
        project: &guid_project,
        filter_source: &guid_source,
        filter_header: &guid_header,
        filter_resource: &guid_resource,
    };

    let ctx = VsTemplateContext {
        project_name: &base_name,
        dll_name: &dll_name,
        base_name: &base_name,
        exports,
        guids,
    };

    let c_src = render_c(&ctx);
    let asm_src = render_asm(&ctx);

    fs::create_dir_all(output_dir)?;

    let mut written = Vec::new();
    let mut write_file = |name: &str, content: &str| -> anyhow::Result<()> {
        let path = output_dir.join(name);
        fs::write(&path, content)?;
        written.push(path.display().to_string());
        Ok(())
    };

    write_file(&format!("{}.c", base_name), &c_src)?;
    write_file(&format!("{}_jump.asm", base_name), &asm_src)?;

    Ok(written)
}

fn write_vs2022_project(
    dll_path: &Path,
    output_dir: &Path,
    exports: &[dll::ExportEntry],
) -> anyhow::Result<Vec<String>> {
    let dll_stem = dll_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("Invalid DLL filename"))?;
    let dll_name = dll_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| format!("{dll_stem}.dll"));

    let project_name = format!("AheadLibEx_{}", dll_stem);
    let base_name = dll_stem;

    let guid_solution = new_guid_braced();
    let guid_project = new_guid_braced();
    let guid_source = new_guid_braced();
    let guid_header = new_guid_braced();
    let guid_resource = new_guid_braced();

    let guids = VsGuids {
        solution: &guid_solution,
        project: &guid_project,
        filter_source: &guid_source,
        filter_header: &guid_header,
        filter_resource: &guid_resource,
    };

    let ctx = VsTemplateContext {
        project_name: &project_name,
        dll_name: &dll_name,
        base_name: &base_name,
        exports,
        guids,
    };

    let sln = render_solution(&ctx);
    let vcxproj = render_vcxproj(&ctx);
    let filters = render_filters(&ctx);
    let user = render_user();
    let c_src = render_c(&ctx);
    let asm_src = render_asm(&ctx);

    fs::create_dir_all(output_dir)?;

    let mut written = Vec::new();
    let mut write_file = |name: &str, content: &str| -> anyhow::Result<()> {
        let path = output_dir.join(name);
        fs::write(&path, content)?;
        written.push(path.display().to_string());
        Ok(())
    };

    write_file(&format!("{}.sln", project_name), &sln)?;
    write_file(&format!("{}.vcxproj", project_name), &vcxproj)?;
    write_file(&format!("{}.vcxproj.filters", project_name), &filters)?;
    write_file(&format!("{}.vcxproj.user", project_name), &user)?;
    write_file(&format!("{}.c", base_name), &c_src)?;
    write_file(&format!("{}_jump.asm", base_name), &asm_src)?;

    Ok(written)
}
