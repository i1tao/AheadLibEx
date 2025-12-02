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

const TPL_SOLUTION: &str = include_str!("../templates/vs2022/vs2022_solution.sln.tpl");
const TPL_VCXPROJ: &str = include_str!("../templates/vs2022/vs2022_project.vcxproj.tpl");
const TPL_FILTERS: &str = include_str!("../templates/vs2022/vs2022_filters.vcxproj.filters.tpl");
const TPL_USER: &str = include_str!("../templates/vs2022/vs2022_project.vcxproj.user.tpl");
const TPL_C: &str = include_str!("../templates/common/proxy.c.tpl");
const TPL_ASM: &str = include_str!("../templates/common/proxy_jump.asm.tpl");

fn fill(template: &str, pairs: &[(&str, String)]) -> String {
    let mut out = template.to_string();
    for (key, val) in pairs {
        let needle = format!("{{{{{}}}}}", key);
        out = out.replace(&needle, val);
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

pub fn render_solution(ctx: &VsTemplateContext) -> String {
    fill(
        TPL_SOLUTION,
        &[
            ("PROJECT_NAME", ctx.project_name.to_string()),
            ("PROJECT_GUID", ctx.guids.project.to_string()),
            ("SOLUTION_GUID", ctx.guids.solution.to_string()),
        ],
    )
}

pub fn render_vcxproj(ctx: &VsTemplateContext) -> String {
    fill(
        TPL_VCXPROJ,
        &[
            ("BASE", ctx.base_name.to_string()),
            ("PROJECT_GUID", ctx.guids.project.to_string()),
        ],
    )
}

pub fn render_filters(ctx: &VsTemplateContext) -> String {
    fill(
        TPL_FILTERS,
        &[
            ("BASE", ctx.base_name.to_string()),
            ("GUID_SOURCE", ctx.guids.filter_source.to_string()),
            ("GUID_HEADER", ctx.guids.filter_header.to_string()),
            ("GUID_RESOURCE", ctx.guids.filter_resource.to_string()),
        ],
    )
}

pub fn render_user() -> String {
    TPL_USER.to_string()
}

pub fn render_c(ctx: &VsTemplateContext) -> String {
    let exports = prepare_exports(ctx.exports);

    let mut export_pragmas = String::new();
    export_pragmas.push_str("#if defined(_WIN64)\n");
    for exp in &exports {
        let noname = if exp.label.starts_with("Noname") {
            ",NONAME"
        } else {
            ""
        };
        let _ = writeln!(
            export_pragmas,
            "#pragma comment(linker, \"/EXPORT:{}=AheadLibEx_{},@{}{}\")",
            exp.label, exp.stub, exp.ordinal, noname
        );
    }
    export_pragmas.push_str("#else\n");
    for exp in &exports {
        let noname = if exp.label.starts_with("Noname") {
            ",NONAME"
        } else {
            ""
        };
        let _ = writeln!(
            export_pragmas,
            "#pragma comment(linker, \"/EXPORT:{}=_AheadLibEx_{},@{}{}\")",
            exp.label, exp.stub, exp.ordinal, noname
        );
    }
    export_pragmas.push_str("#endif\n");

    let mut forward_decls = String::new();
    for exp in &exports {
        let _ = writeln!(
            forward_decls,
            "AHEADLIB_EXTERN PVOID pfnAheadLibEx_{};",
            exp.stub
        );
    }

    let mut x86_trampolines = String::new();
    for exp in &exports {
        let _ = writeln!(
            x86_trampolines,
            "EXTERN_C __declspec(naked) void __cdecl AheadLibEx_{}(void)\n{{\n\t__asm jmp pfnAheadLibEx_{};\n}}\n",
            exp.stub, exp.stub
        );
    }

    let mut init_forwarders = String::new();
    for exp in &exports {
        if exp.label.starts_with("Noname") {
            let _ = writeln!(
                init_forwarders,
                "\tpfnAheadLibEx_{} = get_address(MAKEINTRESOURCEA({}));",
                exp.stub, exp.ordinal
            );
        } else {
            let _ = writeln!(
                init_forwarders,
                "\tpfnAheadLibEx_{} = get_address(\"{}\");",
                exp.stub, exp.raw_name
            );
        }
    }

    fill(
        TPL_C,
        &[
            ("DLL_NAME", ctx.dll_name.to_string()),
            ("EXPORT_PRAGMAS", export_pragmas),
            ("FORWARD_DECLS", forward_decls),
            ("X86_TRAMPOLINES", x86_trampolines),
            ("INIT_FORWARDERS", init_forwarders),
        ],
    )
}

pub fn render_asm(ctx: &VsTemplateContext) -> String {
    let exports = prepare_exports(ctx.exports);

    let mut externs = String::new();
    for exp in &exports {
        let _ = writeln!(externs, "EXTERN pfnAheadLibEx_{}:dq;", exp.stub);
    }

    let mut jumps = String::new();
    for exp in &exports {
        let _ = writeln!(
            jumps,
            "AheadLibEx_{name} PROC\n\tjmp pfnAheadLibEx_{name}\nAheadLibEx_{name} ENDP\n",
            name = exp.stub
        );
    }

    fill(
        TPL_ASM,
        &[("ASM_EXTERNS", externs), ("ASM_JUMPS", jumps)],
    )
}
