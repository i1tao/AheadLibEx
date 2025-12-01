#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod gui;

use anyhow::Result;

fn main() -> Result<()> {
    gui::launch_gui()
}
