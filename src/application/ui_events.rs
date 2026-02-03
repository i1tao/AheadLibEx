use std::{env, path::Path};

use crate::dll;
use crate::templates::{
    render_asm_x64, render_asm_x64_gas, render_asm_x86, render_asm_x86_gas, render_c, render_c_x64,
    render_cmake_lists, render_def, render_filters, render_filters_2026, render_solution,
    render_slnx_2026, render_user, render_user_2026, render_vcxproj, render_vcxproj_2026,
    OriginLoadMode, OriginLoadModeOwned, VsGuids, VsTemplateContext,
};
use eframe::egui;
use rfd;
use std::fs;
use uuid::Uuid;

const DEFAULT_LOG: &str =
    "AheadLibEx (Rust)\nAuthor: i1tao\nGitHub: https://github.com/i1tao/AheadLibEx\n------------------------------------------------------";

#[derive(Copy, Clone, Debug)]
pub enum OutputTarget {
    Source,
    Vs2022,
    Vs2026,
    CMake,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OriginModeChoice {
    SystemDir,
    SameDir,
    CustomPath,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UiLanguageChoice {
    English,
    ZhHans,
    ZhHant,
}

impl UiLanguageChoice {
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::ZhHans => "简体中文",
            Self::ZhHant => "繁體中文",
        }
    }
}

pub struct UiState {
    pub dll_path: String,
    pub project_dir: String,
    pub output_source: bool,
    pub output_vs2022: bool,
    pub output_vs2026: bool,
    pub output_cmake: bool,
    pub ui_language: UiLanguageChoice,
    pub origin_mode: OriginModeChoice,
    pub origin_same_dir_name: String,
    pub origin_custom_path: String,
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
            output_cmake: false,
            ui_language: detect_default_ui_language(),
            origin_mode: OriginModeChoice::SystemDir,
            origin_same_dir_name: String::new(),
            origin_custom_path: String::new(),
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

fn detect_default_ui_language() -> UiLanguageChoice {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::Globalization::GetUserDefaultUILanguage;
        let lang_id = unsafe { GetUserDefaultUILanguage() } as u16;
        // https://learn.microsoft.com/windows/win32/intl/language-identifiers
        // LANGID: primary language id is low 10 bits; sublanguage id is high 6 bits.
        let primary = lang_id & 0x03ff;
        let sub = lang_id >> 10;

        // LANG_CHINESE = 0x04
        if primary == 0x04 {
            // SUBLANG_CHINESE_TRADITIONAL = 0x01
            // SUBLANG_CHINESE_SIMPLIFIED = 0x02
            // SUBLANG_CHINESE_HONGKONG   = 0x03
            // SUBLANG_CHINESE_SINGAPORE  = 0x04
            // SUBLANG_CHINESE_MACAU      = 0x05
            return match sub {
                0x01 | 0x03 | 0x05 => UiLanguageChoice::ZhHant,
                0x02 | 0x04 => UiLanguageChoice::ZhHans,
                _ => UiLanguageChoice::ZhHans,
            };
        }
    }

    UiLanguageChoice::English
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
        ensure_default_origin_same_dir_name(state, &p);
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

    if !state.output_source
        && !state.output_vs2022
        && !state.output_vs2026
        && !state.output_cmake
    {
        state.log = "Please select at least one output target (Source/VS2022/VS2026/CMake)".into();
        state.success = Some(false);
        return;
    }

    // enforce single selection
    let selected_count = state.output_source as u8
        + state.output_vs2022 as u8
        + state.output_vs2026 as u8
        + state.output_cmake as u8;
    if selected_count > 1 {
        state.log = "Please select only one output target".into();
        state.success = Some(false);
        return;
    }

    match dll::read_exports(dll_path) {
        Ok(info) => {
            let mut exports = info.exports.clone();
            exports.sort_by_key(|e| e.ordinal);
            let is_x64 = info.arch.eq_ignore_ascii_case("x64");

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
            let dll_stem = dll_path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            let origin_load_mode = match build_origin_load_mode(state, &dll_stem) {
                Ok(v) => v,
                Err(err) => {
                    state
                        .log
                        .push_str(&format!("\n-- Origin DLL config invalid --\n{err}"));
                    state.success = Some(false);
                    return;
                }
            };
            let origin = origin_load_mode.as_borrowed();

            if state.output_source {
                match write_source_files(
                    dll_path,
                    Path::new(state.project_dir.trim()),
                    is_x64,
                    origin,
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
                    is_x64,
                    origin,
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
                match write_vs2026_project(
                    dll_path,
                    Path::new(state.project_dir.trim()),
                    is_x64,
                    origin,
                    &exports_for_write,
                ) {
                    Ok(_) => state
                        .log
                        .push_str("\n-- VS2026 project written successfully --"),
                    Err(err) => {
                        state
                            .log
                            .push_str(&format!("\n-- VS2026 project write failed --\n{err}"));
                        state.success = Some(false);
                    }
                }
            }

            if state.output_cmake {
                match write_cmake_project(
                    dll_path,
                    Path::new(state.project_dir.trim()),
                    is_x64,
                    origin,
                    &exports_for_write,
                ) {
                    Ok(_) => state
                        .log
                        .push_str("\n-- CMake project written successfully --"),
                    Err(err) => {
                        state
                            .log
                            .push_str(&format!("\n-- CMake project write failed --\n{err}"));
                        state.success = Some(false);
                    }
                }
            }
        }
        Err(err) => {
            state.log = format!("Failed to parse DLL: {err}");
            state.success = Some(false);
        }
    }
}

pub fn generate_cli(
    target: OutputTarget,
    dll_path: &Path,
    output_dir: &Path,
    origin_load_mode: OriginLoadModeOwned,
) -> anyhow::Result<Vec<String>> {
    if !dll_path.exists() {
        return Err(anyhow::anyhow!("DLL path not found: {}", dll_path.display()));
    }

    let info = dll::read_exports(dll_path)?;
    let mut exports = info.exports.clone();
    exports.sort_by_key(|e| e.ordinal);
    let is_x64 = info.arch.eq_ignore_ascii_case("x64");

    let origin = origin_load_mode.as_borrowed();
    match target {
        OutputTarget::Source => write_source_files(dll_path, output_dir, is_x64, origin, &exports),
        OutputTarget::Vs2022 => write_vs2022_project(dll_path, output_dir, is_x64, origin, &exports),
        OutputTarget::Vs2026 => write_vs2026_project(dll_path, output_dir, is_x64, origin, &exports),
        OutputTarget::CMake => write_cmake_project(dll_path, output_dir, is_x64, origin, &exports),
    }
}

pub fn reset(state: &mut UiState) {
    state.dll_path.clear();
    state.project_dir.clear();
    state.output_source = false;
    state.output_vs2022 = false;
    state.output_vs2026 = false;
    state.output_cmake = false;
    state.origin_mode = OriginModeChoice::SystemDir;
    state.origin_same_dir_name.clear();
    state.origin_custom_path.clear();
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
                    ensure_default_origin_same_dir_name(state, p);
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

fn ensure_default_origin_same_dir_name(state: &mut UiState, dll_path: &Path) {
    if !state.origin_same_dir_name.trim().is_empty() {
        return;
    }
    let Some(stem) = dll_path.file_stem().map(|s| s.to_string_lossy().to_string()) else {
        return;
    };
    if stem.is_empty() {
        return;
    }
    state.origin_same_dir_name = format!("{stem}_orig.dll");
}

fn build_origin_load_mode(state: &UiState, dll_stem: &str) -> anyhow::Result<OriginLoadModeOwned> {
    match state.origin_mode {
        OriginModeChoice::SystemDir => Ok(OriginLoadModeOwned::system_dir()),
        OriginModeChoice::SameDir => {
            let name = state.origin_same_dir_name.trim();
            if name.is_empty() {
                Ok(OriginLoadModeOwned::same_dir(format!("{dll_stem}_orig.dll")))
            } else {
                Ok(OriginLoadModeOwned::same_dir(name.to_string()))
            }
        }
        OriginModeChoice::CustomPath => {
            let p = state.origin_custom_path.trim();
            if p.is_empty() {
                anyhow::bail!("Please set a custom origin DLL path");
            }
            Ok(OriginLoadModeOwned::custom_path(p.to_string()))
        }
    }
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
    if state.output_cmake {
        v.push("CMake".to_string());
    }
    v
}

fn write_source_files(
    dll_path: &Path,
    output_dir: &Path,
    is_x64: bool,
    origin_load_mode: OriginLoadMode<'_>,
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
        origin_load_mode,
        exports,
        guids,
    };

    let c_src_x86 = if is_x64 { None } else { Some(render_c(&ctx)) };
    let c_src_x64 = if is_x64 {
        Some(render_c_x64(&ctx))
    } else {
        None
    };
    let asm_src_x86 = if is_x64 { None } else { Some(render_asm_x86(&ctx)) };
    let asm_src_x86_gas = if is_x64 {
        None
    } else {
        Some(render_asm_x86_gas(&ctx))
    };
    let asm_src_x64 = if is_x64 { Some(render_asm_x64(&ctx)) } else { None };
    let asm_src_x64_gas = if is_x64 {
        Some(render_asm_x64_gas(&ctx))
    } else {
        None
    };
    let def_src = render_def(&ctx, is_x64);

    fs::create_dir_all(output_dir)?;

    let mut written = Vec::new();
    let mut write_file = |name: &str, content: &str| -> anyhow::Result<()> {
        let path = output_dir.join(name);
        fs::write(&path, content)?;
        written.push(path.display().to_string());
        Ok(())
    };

    if let Some(content) = c_src_x86 {
        write_file(&format!("{}_x86.c", base_name), &content)?;
    }
    if let Some(content) = c_src_x64 {
        write_file(&format!("{}_x64.c", base_name), &content)?;
    }
    if let Some(content) = asm_src_x86 {
        write_file(&format!("{}_x86_jump.asm", base_name), &content)?;
    }
    if let Some(content) = asm_src_x86_gas {
        write_file(&format!("{}_x86_jump.S", base_name), &content)?;
    }
    if let Some(content) = asm_src_x64 {
        write_file(&format!("{}_x64_jump.asm", base_name), &content)?;
    }
    if let Some(content) = asm_src_x64_gas {
        write_file(&format!("{}_x64_jump.S", base_name), &content)?;
    }
    write_file(&format!("{}.def", base_name), &def_src)?;

    Ok(written)
}

fn write_cmake_project(
    dll_path: &Path,
    output_dir: &Path,
    is_x64: bool,
    origin_load_mode: OriginLoadMode<'_>,
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
        origin_load_mode,
        exports,
        guids,
    };

    let cmake_lists = render_cmake_lists(&ctx, is_x64);

    let c_src_x86 = if is_x64 { None } else { Some(render_c(&ctx)) };
    let c_src_x64 = if is_x64 {
        Some(render_c_x64(&ctx))
    } else {
        None
    };
    let asm_src_x86 = if is_x64 { None } else { Some(render_asm_x86(&ctx)) };
    let asm_src_x86_gas = if is_x64 {
        None
    } else {
        Some(render_asm_x86_gas(&ctx))
    };
    let asm_src_x64 = if is_x64 { Some(render_asm_x64(&ctx)) } else { None };
    let asm_src_x64_gas = if is_x64 {
        Some(render_asm_x64_gas(&ctx))
    } else {
        None
    };
    let def_src = render_def(&ctx, is_x64);

    fs::create_dir_all(output_dir)?;

    let mut written = Vec::new();
    let mut write_file = |name: &str, content: &str| -> anyhow::Result<()> {
        let path = output_dir.join(name);
        fs::write(&path, content)?;
        written.push(path.display().to_string());
        Ok(())
    };

    write_file("CMakeLists.txt", &cmake_lists)?;

    if let Some(content) = c_src_x86 {
        write_file(&format!("{}_x86.c", base_name), &content)?;
    }
    if let Some(content) = c_src_x64 {
        write_file(&format!("{}_x64.c", base_name), &content)?;
    }
    if let Some(content) = asm_src_x86 {
        write_file(&format!("{}_x86_jump.asm", base_name), &content)?;
    }
    if let Some(content) = asm_src_x86_gas {
        write_file(&format!("{}_x86_jump.S", base_name), &content)?;
    }
    if let Some(content) = asm_src_x64 {
        write_file(&format!("{}_x64_jump.asm", base_name), &content)?;
    }
    if let Some(content) = asm_src_x64_gas {
        write_file(&format!("{}_x64_jump.S", base_name), &content)?;
    }
    write_file(&format!("{}.def", base_name), &def_src)?;

    Ok(written)
}

fn write_vs2022_project(
    dll_path: &Path,
    output_dir: &Path,
    is_x64: bool,
    origin_load_mode: OriginLoadMode<'_>,
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

    let solution_name = format!("AheadlibEx_{}", dll_stem);
    let project_name = dll_stem;
    let base_name = project_name.clone();

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
        origin_load_mode,
        exports,
        guids,
    };

    let sln = render_solution(&ctx, is_x64);
    let vcxproj = render_vcxproj(&ctx, is_x64);
    let filters = render_filters(&ctx, is_x64);
    let user = render_user();
    let c_src_x86 = if is_x64 { None } else { Some(render_c(&ctx)) };
    let c_src_x64 = if is_x64 {
        Some(render_c_x64(&ctx))
    } else {
        None
    };
    let asm_src_x86 = if is_x64 { None } else { Some(render_asm_x86(&ctx)) };
    let asm_src_x64 = if is_x64 {
        Some(render_asm_x64(&ctx))
    } else {
        None
    };
    let def_src = render_def(&ctx, is_x64);

    fs::create_dir_all(output_dir)?;

    let mut written = Vec::new();
    let mut write_file = |name: &str, content: &str| -> anyhow::Result<()> {
        let path = output_dir.join(name);
        fs::write(&path, content)?;
        written.push(path.display().to_string());
        Ok(())
    };

    write_file(&format!("{}.sln", solution_name), &sln)?;
    write_file(&format!("{}.vcxproj", project_name), &vcxproj)?;
    write_file(&format!("{}.vcxproj.filters", project_name), &filters)?;
    write_file(&format!("{}.vcxproj.user", project_name), &user)?;
    if let Some(content) = c_src_x86 {
        write_file(&format!("{}_x86.c", base_name), &content)?;
    }
    if let Some(content) = c_src_x64 {
        write_file(&format!("{}_x64.c", base_name), &content)?;
    }
    if let Some(content) = asm_src_x86 {
        write_file(&format!("{}_x86_jump.asm", base_name), &content)?;
    }
    if let Some(content) = asm_src_x64 {
        write_file(&format!("{}_x64_jump.asm", base_name), &content)?;
    }
    write_file(&format!("{}.def", base_name), &def_src)?;

    Ok(written)
}

fn write_vs2026_project(
    dll_path: &Path,
    output_dir: &Path,
    is_x64: bool,
    origin_load_mode: OriginLoadMode<'_>,
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

    let solution_name = format!("AheadlibEx_{}", dll_stem);
    let project_name = dll_stem;
    let base_name = project_name.clone();

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
        origin_load_mode,
        exports,
        guids,
    };

    let slnx = render_slnx_2026(&ctx, is_x64);
    let vcxproj = render_vcxproj_2026(&ctx, is_x64);
    let filters = render_filters_2026(&ctx, is_x64);
    let user = render_user_2026();
    let c_src_x86 = if is_x64 { None } else { Some(render_c(&ctx)) };
    let c_src_x64 = if is_x64 {
        Some(render_c_x64(&ctx))
    } else {
        None
    };
    let asm_src_x86 = if is_x64 { None } else { Some(render_asm_x86(&ctx)) };
    let asm_src_x64 = if is_x64 {
        Some(render_asm_x64(&ctx))
    } else {
        None
    };
    let def_src = render_def(&ctx, is_x64);

    fs::create_dir_all(output_dir)?;

    let mut written = Vec::new();
    let mut write_file = |name: &str, content: &str| -> anyhow::Result<()> {
        let path = output_dir.join(name);
        fs::write(&path, content)?;
        written.push(path.display().to_string());
        Ok(())
    };

    write_file(&format!("{}.slnx", solution_name), &slnx)?;
    write_file(&format!("{}.vcxproj", project_name), &vcxproj)?;
    write_file(&format!("{}.vcxproj.filters", project_name), &filters)?;
    write_file(&format!("{}.vcxproj.user", project_name), &user)?;
    if let Some(content) = c_src_x86 {
        write_file(&format!("{}_x86.c", base_name), &content)?;
    }
    if let Some(content) = c_src_x64 {
        write_file(&format!("{}_x64.c", base_name), &content)?;
    }
    if let Some(content) = asm_src_x86 {
        write_file(&format!("{}_x86_jump.asm", base_name), &content)?;
    }
    if let Some(content) = asm_src_x64 {
        write_file(&format!("{}_x64_jump.asm", base_name), &content)?;
    }
    write_file(&format!("{}.def", base_name), &def_src)?;

    Ok(written)
}
