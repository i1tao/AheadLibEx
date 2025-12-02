#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod dll;
mod gui;
mod ui_events;
#[allow(dead_code)]
mod templates;

use anyhow::Result;

fn main() -> Result<()> {
    gui::launch_gui()
}
