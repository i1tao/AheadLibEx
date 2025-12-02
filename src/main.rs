#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod dll;
mod gui;
mod ui_events;

use anyhow::Result;

fn main() -> Result<()> {
    gui::launch_gui()
}
