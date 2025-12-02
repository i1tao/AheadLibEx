use std::{fs::File, path::Path};

use anyhow::{Context, Result};
use goblin::pe::{export::Reexport, PE};
use memmap2::MmapOptions;

#[derive(Debug, Clone)]
pub struct ExportEntry {
    pub name: String,
    pub ordinal: u16,
    pub forwarder: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DllExports {
    pub arch: String,
    pub exports: Vec<ExportEntry>,
}

pub fn read_exports(path: &Path) -> Result<DllExports> {
    let file =
        File::open(path).with_context(|| format!("Failed to open DLL: {}", path.display()))?;
    let mmap = unsafe {
        MmapOptions::new()
            .map(&file)
            .with_context(|| format!("Failed to mmap DLL: {}", path.display()))?
    };

    let pe = PE::parse(&mmap).with_context(|| format!("Failed to parse PE: {}", path.display()))?;
    let export_data = pe
        .export_data
        .as_ref()
        .context("DLL missing export table")?;
    let ordinal_base = export_data.export_directory_table.ordinal_base;
    let ordinals = &export_data.export_ordinal_table;
    let arch = if pe.is_64 { "x64" } else { "x86" }.to_string();

    let exports = pe
        .exports
        .iter()
        .enumerate()
        .map(|(idx, e)| {
            let ordinal = ordinals
                .get(idx)
                .map(|o| ordinal_base.saturating_add(*o as u32))
                .unwrap_or(ordinal_base);

            let name = e
                .name
                .map(|n| n.to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| format!("#{}", ordinal));

            let forwarder = match &e.reexport {
                Some(Reexport::DLLName { lib, export }) => Some(format!("{}!{}", lib, export)),
                Some(Reexport::DLLOrdinal { lib, ordinal }) => {
                    Some(format!("{}!#{}", lib, ordinal))
                }
                None => None,
            };

            ExportEntry {
                name,
                ordinal: ordinal.min(u16::MAX as u32) as u16,
                forwarder,
            }
        })
        .collect();

    Ok(DllExports { arch, exports })
}
