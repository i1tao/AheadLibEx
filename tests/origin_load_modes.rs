use aheadlibex_rs::dll::ExportEntry;
use aheadlibex_rs::templates::{render_c, render_c_x64, OriginLoadMode, VsGuids, VsTemplateContext};

fn dummy_ctx<'a>(exports: &'a [ExportEntry], mode: OriginLoadMode<'a>) -> VsTemplateContext<'a> {
    let guids = VsGuids {
        solution: "{S}",
        project: "{P}",
        filter_source: "{FS}",
        filter_header: "{FH}",
        filter_resource: "{FR}",
    };
    VsTemplateContext {
        project_name: "Foo",
        dll_name: "Foo.dll",
        base_name: "Foo",
        origin_load_mode: mode,
        exports,
        guids,
    }
}

#[test]
fn system_dir_mode_uses_system_directory() {
    let exports = vec![ExportEntry {
        name: "Bar".to_string(),
        ordinal: 1,
        forwarder: None,
    }];
    let ctx = dummy_ctx(&exports, OriginLoadMode::SystemDir);

    let c = render_c(&ctx);
    assert!(c.contains("load_original_module(HMODULE module)"));
    assert!(c.contains("load_original_module(module)"));
    assert!(c.contains("GetSystemDirectory("));
    assert!(c.contains(r#"TEXT("Foo.dll")"#));
}

#[test]
fn same_dir_mode_uses_proxy_directory_and_original_name() {
    let exports = vec![ExportEntry {
        name: "Bar".to_string(),
        ordinal: 1,
        forwarder: None,
    }];
    let ctx = dummy_ctx(
        &exports,
        OriginLoadMode::SameDir {
            original_name: "Foo_orig.dll",
        },
    );

    let c = render_c(&ctx);
    assert!(c.contains("GetModuleFileName("));
    assert!(c.contains(r#"TEXT("Foo_orig.dll")"#));
}

#[test]
fn custom_path_mode_embeds_origin_cfg() {
    let exports = vec![ExportEntry {
        name: "Bar".to_string(),
        ordinal: 1,
        forwarder: None,
    }];
    let ctx = dummy_ctx(
        &exports,
        OriginLoadMode::CustomPath {
            path: r"C:\path\to\Foo.dll",
        },
    );

    let c = render_c_x64(&ctx);
    assert!(c.contains(r#"origin_cfg[] = TEXT("C:\\path\\to\\Foo.dll")"#));
}

