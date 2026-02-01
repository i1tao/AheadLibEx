use aheadlibex_rs::dll::ExportEntry;
use aheadlibex_rs::templates::{
    render_asm_x64, render_c, render_c_x64, VsGuids, VsTemplateContext,
};

fn dummy_ctx<'a>(exports: &'a [ExportEntry]) -> VsTemplateContext<'a> {
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
        exports,
        guids,
    }
}

#[test]
fn decorated_names_are_preserved_in_exports() {
    let exports = vec![
        ExportEntry {
            name: "?Func@@YAXH@Z".to_string(),
            ordinal: 1,
            forwarder: None,
        },
        ExportEntry {
            name: "@Func@8".to_string(),
            ordinal: 2,
            forwarder: None,
        },
        ExportEntry {
            name: "??0Class@@QAE@XZ".to_string(),
            ordinal: 3,
            forwarder: None,
        },
        ExportEntry {
            name: "#345".to_string(),
            ordinal: 345,
            forwarder: None,
        },
    ];

    let ctx = dummy_ctx(&exports);

    let c_x86 = render_c(&ctx);
    assert!(c_x86.contains(r#"/EXPORT:\"?Func@@YAXH@Z=_AheadLibEx__Func__YAXH_Z,@1\""#));
    assert!(c_x86.contains(r#"/EXPORT:\"@Func@8=_AheadLibEx__Func_8,@2\""#));
    assert!(c_x86.contains(r#"/EXPORT:\"??0Class@@QAE@XZ=_AheadLibEx___0Class__QAE_XZ,@3\""#));
    assert!(c_x86.contains(r#"/EXPORT:\"Noname345=_AheadLibEx_Unnamed345,@345,NONAME\""#));

    let c_x64 = render_c_x64(&ctx);
    assert!(c_x64.contains(r#"/EXPORT:\"?Func@@YAXH@Z=AheadLibEx__Func__YAXH_Z,@1\""#));
    assert!(c_x64.contains(r#"/EXPORT:\"@Func@8=AheadLibEx__Func_8,@2\""#));
    assert!(c_x64.contains(r#"/EXPORT:\"??0Class@@QAE@XZ=AheadLibEx___0Class__QAE_XZ,@3\""#));
    assert!(c_x64.contains(r#"/EXPORT:\"Noname345=AheadLibEx_Unnamed345,@345,NONAME\""#));
}

#[test]
fn asm_uses_sanitized_stub_names() {
    let exports = vec![ExportEntry {
        name: "?Decorated@Name@@@".to_string(),
        ordinal: 7,
        forwarder: None,
    }];
    let ctx = dummy_ctx(&exports);

    let asm = render_asm_x64(&ctx);
    assert!(asm.contains("EXTERN pfnAheadLibEx__Decorated_Name___:dq"));
    assert!(asm.contains("AheadLibEx__Decorated_Name___ PROC"));
}
