#![allow(dead_code)]

use crate::dll::ExportEntry;
use std::collections::HashSet;
use std::fmt::Write;

#[derive(Clone, Debug)]
pub struct VsGuids<'a> {
    pub solution: &'a str,
    pub project: &'a str,
    pub filter_source: &'a str,
    pub filter_header: &'a str,
    pub filter_resource: &'a str,
}

#[derive(Clone, Debug)]
pub struct VsTemplateContext<'a> {
    pub project_name: &'a str,
    pub dll_name: &'a str,
    pub base_name: &'a str,
    pub exports: &'a [ExportEntry],
    pub guids: VsGuids<'a>,
}

#[derive(Clone, Debug)]
struct PreparedExport<'a> {
    raw_name: &'a str,
    ordinal: u16,
    forwarder: Option<&'a str>,
    label: String,
    stub: String,
}

const TPL_SOLUTION: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/vs2022/vs2022_solution.sln.tpl"
));
const TPL_VCXPROJ: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/vs2022/vs2022_project.vcxproj.tpl"
));
const TPL_FILTERS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/vs2022/vs2022_filters.vcxproj.filters.tpl"
));
const TPL_USER: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/vs2022/vs2022_project.vcxproj.user.tpl"
));
const TPL_C_X86: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/common/proxy_x86.c.tpl"
));
const TPL_C_X64: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/common/proxy_x64.c.tpl"
));
const TPL_ASM_X64: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/common/proxy_x64_jump.asm.tpl"
));
const TPL_VCXPROJ_2026: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/vs2026/vs2026_project.vcxproj.tpl"
));
const TPL_FILTERS_2026: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/vs2026/vs2026_project.vcxproj.filters.tpl"
));
const TPL_USER_2026: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/vs2026/vs2026_project.vcxproj.user.tpl"
));
const TPL_SLNX_2026: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/vs2026/vs2026_solution.slnx.tpl"
));

fn fill(template: &str, pairs: &[(&str, String)]) -> String {
    let mut out = template.to_string();
    // 支持嵌套占位符，多轮替换直到没有匹配或达到安全上限
    for _ in 0..5 {
        let mut changed = false;
        for (key, val) in pairs {
            let needle = format!("{{{{{}}}}}", key);
            if out.contains(&needle) {
                out = out.replace(&needle, val);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    out
}

fn sanitize_identifier(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        let valid = ch.is_ascii_alphanumeric() || ch == '_';
        if valid {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        "_".to_string()
    } else {
        out
    }
}

fn prepare_exports(entries: &[ExportEntry]) -> Vec<PreparedExport<'_>> {
    let mut exports = entries.iter().collect::<Vec<_>>();
    exports.sort_by_key(|e| (e.ordinal, e.name.clone()));

    let mut used_stubs = HashSet::new();
    let mut prepared = Vec::with_capacity(exports.len());
    for entry in exports {
        let is_noname = entry.name.starts_with('#');
        let label = if is_noname {
            format!("Noname{}", entry.ordinal)
        } else {
            entry.name.clone()
        };

        let mut stub = if is_noname {
            format!("Unnamed{}", entry.ordinal)
        } else {
            sanitize_identifier(&entry.name)
        };

        if used_stubs.contains(&stub) {
            stub = format!("{}_{}", stub, entry.ordinal);
        }
        used_stubs.insert(stub.clone());

        prepared.push(PreparedExport {
            raw_name: &entry.name,
            ordinal: entry.ordinal,
            forwarder: entry.forwarder.as_deref(),
            label,
            stub,
        });
    }

    prepared
}

fn solution_configs(is_x64: bool, project_guid: &str) -> (String, String) {
    if is_x64 {
        (
            "        Debug|x64 = Debug|x64\n        Release|x64 = Release|x64\n".to_string(),
            format!(
                "        {guid}.Debug|x64.ActiveCfg = Debug|x64\n        {guid}.Debug|x64.Build.0 = Debug|x64\n        {guid}.Release|x64.ActiveCfg = Release|x64\n        {guid}.Release|x64.Build.0 = Release|x64\n",
                guid = project_guid
            ),
        )
    } else {
        (
            "        Debug|x86 = Debug|x86\n        Release|x86 = Release|x86\n".to_string(),
            format!(
                "        {guid}.Debug|x86.ActiveCfg = Debug|Win32\n        {guid}.Debug|x86.Build.0 = Debug|Win32\n        {guid}.Release|x86.ActiveCfg = Release|Win32\n        {guid}.Release|x86.Build.0 = Release|Win32\n",
                guid = project_guid
            ),
        )
    }
}

fn project_config_entries(is_x64: bool) -> String {
    if is_x64 {
        r#"    <ProjectConfiguration Include="Debug|x64">
      <Configuration>Debug</Configuration>
      <Platform>x64</Platform>
    </ProjectConfiguration>
    <ProjectConfiguration Include="Release|x64">
      <Configuration>Release</Configuration>
      <Platform>x64</Platform>
    </ProjectConfiguration>
"#
        .to_string()
    } else {
        r#"    <ProjectConfiguration Include="Debug|Win32">
      <Configuration>Debug</Configuration>
      <Platform>Win32</Platform>
    </ProjectConfiguration>
    <ProjectConfiguration Include="Release|Win32">
      <Configuration>Release</Configuration>
      <Platform>Win32</Platform>
    </ProjectConfiguration>
"#
        .to_string()
    }
}

fn cl_item_group(base: &str, is_x64: bool) -> String {
    if is_x64 {
        format!(
            r#"  <ItemGroup>
    <ClCompile Include="{base}_x64.c" />
  </ItemGroup>
"#
        )
    } else {
        format!(
            r#"  <ItemGroup>
    <ClCompile Include="{base}_x86.c" />
  </ItemGroup>
"#
        )
    }
}

fn asm_item_group(base: &str, is_x64: bool) -> String {
    if is_x64 {
        format!(
            r#"  <ItemGroup>
    <MASM Include="{base}_x64_jump.asm" />
  </ItemGroup>
"#
        )
    } else {
        String::new()
    }
}

fn config_groups(toolset: &str, is_x64: bool) -> String {
    if is_x64 {
        format!(
            r#"  <PropertyGroup Condition="'$(Configuration)|$(Platform)'=='Debug|x64'" Label="Configuration">
    <ConfigurationType>DynamicLibrary</ConfigurationType>
    <UseDebugLibraries>true</UseDebugLibraries>
    <PlatformToolset>{toolset}</PlatformToolset>
    <CharacterSet>Unicode</CharacterSet>
  </PropertyGroup>
  <PropertyGroup Condition="'$(Configuration)|$(Platform)'=='Release|x64'" Label="Configuration">
    <ConfigurationType>DynamicLibrary</ConfigurationType>
    <UseDebugLibraries>false</UseDebugLibraries>
    <PlatformToolset>{toolset}</PlatformToolset>
    <WholeProgramOptimization>true</WholeProgramOptimization>
    <CharacterSet>Unicode</CharacterSet>
  </PropertyGroup>
"#
        )
    } else {
        format!(
            r#"  <PropertyGroup Condition="'$(Configuration)|$(Platform)'=='Debug|Win32'" Label="Configuration">
    <ConfigurationType>DynamicLibrary</ConfigurationType>
    <UseDebugLibraries>true</UseDebugLibraries>
    <PlatformToolset>{toolset}</PlatformToolset>
    <CharacterSet>Unicode</CharacterSet>
  </PropertyGroup>
  <PropertyGroup Condition="'$(Configuration)|$(Platform)'=='Release|Win32'" Label="Configuration">
    <ConfigurationType>DynamicLibrary</ConfigurationType>
    <UseDebugLibraries>false</UseDebugLibraries>
    <PlatformToolset>{toolset}</PlatformToolset>
    <WholeProgramOptimization>true</WholeProgramOptimization>
    <CharacterSet>Unicode</CharacterSet>
  </PropertyGroup>
"#
        )
    }
}

fn property_sheets(is_x64: bool) -> String {
    if is_x64 {
        r#"  <ImportGroup Label="PropertySheets" Condition="'$(Configuration)|$(Platform)'=='Debug|x64'">
    <Import Project="$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props" Condition="exists('$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props') " Label="LocalAppDataPlatform" />
  </ImportGroup>
  <ImportGroup Label="PropertySheets" Condition="'$(Configuration)|$(Platform)'=='Release|x64'">
    <Import Project="$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props" Condition="exists('$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props') " Label="LocalAppDataPlatform" />
  </ImportGroup>
"#
        .to_string()
    } else {
        r#"  <ImportGroup Label="PropertySheets" Condition="'$(Configuration)|$(Platform)'=='Debug|Win32'">
    <Import Project="$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props" Condition="exists('$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props') " Label="LocalAppDataPlatform" />
  </ImportGroup>
  <ImportGroup Label="PropertySheets" Condition="'$(Configuration)|$(Platform)'=='Release|Win32'">
    <Import Project="$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props" Condition="exists('$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props') " Label="LocalAppDataPlatform" />
  </ImportGroup>
"#
        .to_string()
    }
}

fn exports_macro(project_name: &str) -> String {
    let mut macro_name = sanitize_identifier(project_name);
    macro_name.make_ascii_uppercase();
    if macro_name.ends_with("_EXPORTS") {
        macro_name
    } else {
        format!("{}_EXPORTS", macro_name)
    }
}

fn item_definitions(exports_macro: &str, is_x64: bool) -> String {
    if is_x64 {
        format!(
            r#"  <ItemDefinitionGroup Condition="'$(Configuration)|$(Platform)'=='Debug|x64'">
    <ClCompile>
      <WarningLevel>Level3</WarningLevel>
      <SDLCheck>true</SDLCheck>
      <PreprocessorDefinitions>_DEBUG;{EXPORTS_MACRO};_WINDOWS;_USRDLL;%(PreprocessorDefinitions)</PreprocessorDefinitions>
      <ConformanceMode>true</ConformanceMode>
      <PrecompiledHeader>NotUsing</PrecompiledHeader>
      <PrecompiledHeaderFile>pch.h</PrecompiledHeaderFile>
    </ClCompile>
    <Link>
      <SubSystem>Windows</SubSystem>
      <GenerateDebugInformation>true</GenerateDebugInformation>
      <EnableUAC>false</EnableUAC>
    </Link>
  </ItemDefinitionGroup>
  <ItemDefinitionGroup Condition="'$(Configuration)|$(Platform)'=='Release|x64'">
    <ClCompile>
      <WarningLevel>Level3</WarningLevel>
      <FunctionLevelLinking>true</FunctionLevelLinking>
      <IntrinsicFunctions>true</IntrinsicFunctions>
      <SDLCheck>true</SDLCheck>
      <PreprocessorDefinitions>NDEBUG;{EXPORTS_MACRO};_WINDOWS;_USRDLL;%(PreprocessorDefinitions)</PreprocessorDefinitions>
      <ConformanceMode>true</ConformanceMode>
      <PrecompiledHeader>NotUsing</PrecompiledHeader>
      <PrecompiledHeaderFile>pch.h</PrecompiledHeaderFile>
    </ClCompile>
    <Link>
      <SubSystem>Windows</SubSystem>
      <EnableCOMDATFolding>true</EnableCOMDATFolding>
      <OptimizeReferences>true</OptimizeReferences>
      <GenerateDebugInformation>true</GenerateDebugInformation>
      <EnableUAC>false</EnableUAC>
    </Link>
  </ItemDefinitionGroup>
"#,
            EXPORTS_MACRO = exports_macro
        )
    } else {
        format!(
            r#"  <ItemDefinitionGroup Condition="'$(Configuration)|$(Platform)'=='Debug|Win32'">
    <ClCompile>
      <WarningLevel>Level3</WarningLevel>
      <SDLCheck>true</SDLCheck>
      <PreprocessorDefinitions>WIN32;_DEBUG;{EXPORTS_MACRO};_WINDOWS;_USRDLL;%(PreprocessorDefinitions)</PreprocessorDefinitions>
      <ConformanceMode>true</ConformanceMode>
      <PrecompiledHeader>NotUsing</PrecompiledHeader>
      <PrecompiledHeaderFile>pch.h</PrecompiledHeaderFile>
    </ClCompile>
    <Link>
      <SubSystem>Windows</SubSystem>
      <GenerateDebugInformation>true</GenerateDebugInformation>
      <EnableUAC>false</EnableUAC>
    </Link>
  </ItemDefinitionGroup>
  <ItemDefinitionGroup Condition="'$(Configuration)|$(Platform)'=='Release|Win32'">
    <ClCompile>
      <WarningLevel>Level3</WarningLevel>
      <FunctionLevelLinking>true</FunctionLevelLinking>
      <IntrinsicFunctions>true</IntrinsicFunctions>
      <SDLCheck>true</SDLCheck>
      <PreprocessorDefinitions>WIN32;NDEBUG;{EXPORTS_MACRO};_WINDOWS;_USRDLL;%(PreprocessorDefinitions)</PreprocessorDefinitions>
      <ConformanceMode>true</ConformanceMode>
      <PrecompiledHeader>NotUsing</PrecompiledHeader>
      <PrecompiledHeaderFile>pch.h</PrecompiledHeaderFile>
    </ClCompile>
    <Link>
      <SubSystem>Windows</SubSystem>
      <EnableCOMDATFolding>true</EnableCOMDATFolding>
      <OptimizeReferences>true</OptimizeReferences>
      <GenerateDebugInformation>true</GenerateDebugInformation>
      <EnableUAC>false</EnableUAC>
    </Link>
  </ItemDefinitionGroup>
"#,
            EXPORTS_MACRO = exports_macro
        )
    }
}

fn extension_settings(is_x64: bool) -> String {
    if is_x64 {
        "    <Import Project=\"$(VCTargetsPath)\\BuildCustomizations\\masm.props\" />\n".to_string()
    } else {
        String::new()
    }
}

fn extension_targets(is_x64: bool) -> String {
    if is_x64 {
        "    <Import Project=\"$(VCTargetsPath)\\BuildCustomizations\\masm.targets\" />\n".to_string()
    } else {
        String::new()
    }
}

fn filter_itemgroups(base: &str, is_x64: bool) -> String {
    if is_x64 {
        format!(
            r#"  <ItemGroup>
    <ClCompile Include="{base}_x64.c">
      <Filter>Source Files</Filter>
    </ClCompile>
  </ItemGroup>
  <ItemGroup>
    <MASM Include="{base}_x64_jump.asm">
      <Filter>Source Files</Filter>
    </MASM>
  </ItemGroup>
"#
        )
    } else {
        format!(
            r#"  <ItemGroup>
    <ClCompile Include="{base}_x86.c">
      <Filter>Source Files</Filter>
    </ClCompile>
  </ItemGroup>
"#
        )
    }
}

fn slnx_platforms(is_x64: bool) -> String {
    if is_x64 {
        "    <Platform Name=\"x64\" />\n".to_string()
    } else {
        "    <Platform Name=\"x86\" />\n".to_string()
    }
}

pub fn render_solution(ctx: &VsTemplateContext, is_x64: bool) -> String {
    let (solution_configs, project_configs) = solution_configs(is_x64, ctx.guids.project);
    fill(
        TPL_SOLUTION,
        &[
            ("PROJECT_NAME", ctx.project_name.to_string()),
            ("PROJECT_GUID", ctx.guids.project.to_string()),
            ("SOLUTION_GUID", ctx.guids.solution.to_string()),
            ("SOLUTION_CONFIGS", solution_configs),
            ("PROJECT_CONFIGS", project_configs),
        ],
    )
}

pub fn render_vcxproj(ctx: &VsTemplateContext, is_x64: bool) -> String {
    let exports_macro = exports_macro(ctx.project_name);
    fill(
        TPL_VCXPROJ,
        &[
            ("BASE", ctx.base_name.to_string()),
            ("PROJECT_GUID", ctx.guids.project.to_string()),
            ("PROJECT_CONFIGS", project_config_entries(is_x64)),
            ("CL_ITEM_GROUP", cl_item_group(ctx.base_name, is_x64)),
            ("ASM_ITEM_GROUP", asm_item_group(ctx.base_name, is_x64)),
            ("CONFIG_GROUPS", config_groups("v143", is_x64)),
            ("PROPERTY_SHEETS", property_sheets(is_x64)),
            ("ITEM_DEFINITIONS", item_definitions(&exports_macro, is_x64)),
            ("EXTENSION_SETTINGS", extension_settings(is_x64)),
            ("EXTENSION_TARGETS", extension_targets(is_x64)),
        ],
    )
}

pub fn render_vcxproj_2026(ctx: &VsTemplateContext, is_x64: bool) -> String {
    let exports_macro = exports_macro(ctx.project_name);
    fill(
        TPL_VCXPROJ_2026,
        &[
            ("PROJECT_NAME", ctx.project_name.to_string()),
            (
                "PROJECT_NAME_UPPER",
                ctx.project_name.to_string().to_ascii_uppercase(),
            ),
            ("PROJECT_GUID", ctx.guids.project.to_string()),
            ("BASE", ctx.base_name.to_string()),
            ("PROJECT_CONFIGS", project_config_entries(is_x64)),
            ("CL_ITEM_GROUP", cl_item_group(ctx.base_name, is_x64)),
            ("ASM_ITEM_GROUP", asm_item_group(ctx.base_name, is_x64)),
            ("CONFIG_GROUPS", config_groups("v145", is_x64)),
            ("PROPERTY_SHEETS", property_sheets(is_x64)),
            ("ITEM_DEFINITIONS", item_definitions(&exports_macro, is_x64)),
            ("EXTENSION_SETTINGS", extension_settings(is_x64)),
            ("EXTENSION_TARGETS", extension_targets(is_x64)),
        ],
    )
}

pub fn render_filters(ctx: &VsTemplateContext, is_x64: bool) -> String {
    fill(
        TPL_FILTERS,
        &[
            ("BASE", ctx.base_name.to_string()),
            ("GUID_SOURCE", ctx.guids.filter_source.to_string()),
            ("GUID_HEADER", ctx.guids.filter_header.to_string()),
            ("GUID_RESOURCE", ctx.guids.filter_resource.to_string()),
            ("FILTER_ITEMGROUPS", filter_itemgroups(ctx.base_name, is_x64)),
        ],
    )
}

pub fn render_filters_2026(ctx: &VsTemplateContext, is_x64: bool) -> String {
    fill(
        TPL_FILTERS_2026,
        &[
            ("BASE", ctx.base_name.to_string()),
            ("GUID_SOURCE", ctx.guids.filter_source.to_string()),
            ("GUID_HEADER", ctx.guids.filter_header.to_string()),
            ("GUID_RESOURCE", ctx.guids.filter_resource.to_string()),
            ("FILTER_ITEMGROUPS", filter_itemgroups(ctx.base_name, is_x64)),
        ],
    )
}

pub fn render_user() -> String {
    TPL_USER.to_string()
}

pub fn render_user_2026() -> String {
    TPL_USER_2026.to_string()
}

pub fn render_slnx_2026(ctx: &VsTemplateContext, is_x64: bool) -> String {
    fill(
        TPL_SLNX_2026,
        &[
            ("PROJECT_NAME", ctx.project_name.to_string()),
            ("PLATFORMS", slnx_platforms(is_x64)),
        ],
    )
}

pub fn render_c(ctx: &VsTemplateContext) -> String {
    let exports = prepare_exports(ctx.exports);

    let mut export_pragmas = String::new();
    for exp in &exports {
        let noname = if exp.label.starts_with("Noname") {
            ",NONAME"
        } else {
            ""
        };
        let entry = format!("{}=AheadLibEx_{},@{}{}", exp.label, exp.stub, exp.ordinal, noname);
        let _ = writeln!(
            export_pragmas,
            "#pragma comment(linker, \"/EXPORT:\\\"{}\\\"\")",
            entry
        );
        let _ = writeln!(
            export_pragmas,
            "#pragma comment(linker, \"/alternatename:AheadLibEx_{}=_AheadLibEx_{}\")",
            exp.stub,
            exp.stub
        );
    }

    let mut forward_decls = String::new();
    for exp in &exports {
        let _ = writeln!(
            forward_decls,
            "AHEADLIB_EXTERN PVOID pfnAheadLibEx_{};",
            exp.stub
        );
    }

    let mut trampolines = String::new();
    for exp in &exports {
        let _ = writeln!(
            trampolines,
            "__declspec(naked) AHEADLIB_EXTERN void __cdecl AheadLibEx_{name}(void) {{ __asm {{ jmp dword ptr [pfnAheadLibEx_{name}] }} }}",
            name = exp.stub
        );
    }

    let mut init_forwarders = String::new();
    for exp in &exports {
        if exp.label.starts_with("Noname") {
            let _ = writeln!(
                init_forwarders,
                "    pfnAheadLibEx_{} = get_address(MAKEINTRESOURCEA({}));",
                exp.stub, exp.ordinal
            );
        } else {
            let _ = writeln!(
                init_forwarders,
                "    pfnAheadLibEx_{} = get_address(\"{}\");",
                exp.stub, exp.raw_name
            );
        }
    }

    fill(
        TPL_C_X86,
        &[
            ("DLL_NAME", ctx.dll_name.to_string()),
            ("EXPORT_PRAGMAS", export_pragmas),
            ("FORWARD_DECLS", forward_decls),
            ("INIT_FORWARDERS", init_forwarders),
            ("X86_TRAMPOLINES", trampolines),
        ],
    )
}

pub fn render_c_x64(ctx: &VsTemplateContext) -> String {
    let exports = prepare_exports(ctx.exports);

    let mut export_pragmas = String::new();
    for exp in &exports {
        let noname = if exp.label.starts_with("Noname") {
            ",NONAME"
        } else {
            ""
        };
        let entry = format!("{}=AheadLibEx_{},@{}{}", exp.label, exp.stub, exp.ordinal, noname);
        let _ = writeln!(
            export_pragmas,
            "#pragma comment(linker, \"/EXPORT:\\\"{}\\\"\")",
            entry
        );
    }

    let mut forward_decls = String::new();
    for exp in &exports {
        let _ = writeln!(
            forward_decls,
            "AHEADLIB_EXTERN PVOID pfnAheadLibEx_{};",
            exp.stub
        );
    }

    let mut init_forwarders = String::new();
    for exp in &exports {
        if exp.label.starts_with("Noname") {
            let _ = writeln!(
                init_forwarders,
                "    pfnAheadLibEx_{} = get_address(MAKEINTRESOURCEA({}));",
                exp.stub, exp.ordinal
            );
        } else {
            let _ = writeln!(
                init_forwarders,
                "    pfnAheadLibEx_{} = get_address(\"{}\");",
                exp.stub, exp.raw_name
            );
        }
    }

    fill(
        TPL_C_X64,
        &[
            ("DLL_NAME", ctx.dll_name.to_string()),
            ("EXPORT_PRAGMAS", export_pragmas),
            ("FORWARD_DECLS", forward_decls),
            ("INIT_FORWARDERS", init_forwarders),
        ],
    )
}

pub fn render_asm_x64(ctx: &VsTemplateContext) -> String {
    let exports = prepare_exports(ctx.exports);

    let mut externs = String::new();
    for exp in &exports {
        let _ = writeln!(externs, "EXTERN pfnAheadLibEx_{}:dq;", exp.stub);
    }

    let mut jumps = String::new();
    for exp in &exports {
        let _ = writeln!(
            jumps,
            "AheadLibEx_{name} PROC\n    jmp pfnAheadLibEx_{name}\nAheadLibEx_{name} ENDP\n",
            name = exp.stub
        );
    }

    fill(
        TPL_ASM_X64,
        &[("ASM_EXTERNS", externs), ("ASM_JUMPS", jumps)],
    )
}
