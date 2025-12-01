use anyhow::Result;
use eframe::egui::{self, Color32, Frame, RichText, Rounding, Stroke, ViewportCommand};

mod ui_events;
use ui_events::{self as events, UiState};

mod colors {
    use eframe::egui::Color32;

    pub const BG_PRIMARY: Color32 = Color32::from_rgb(245, 247, 250);
    pub const BG_CARD: Color32 = Color32::from_rgb(255, 255, 255);
    pub const ACCENT: Color32 = Color32::from_rgb(64, 128, 240);
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(32, 36, 44);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(100, 108, 120);
    pub const TEXT_HINT: Color32 = Color32::from_rgb(160, 168, 180);
    pub const BORDER: Color32 = Color32::from_rgb(220, 224, 232);
    pub const SUCCESS: Color32 = Color32::from_rgb(48, 164, 80);
    pub const ERROR: Color32 = Color32::from_rgb(220, 64, 56);
}

const CONTROL_HEIGHT: f32 = 32.0;
const BUTTON_WIDTH: f32 = 88.0;
const BROWSE_BTN_WIDTH: f32 = 56.0;
const SPACING: f32 = 12.0;
const CARD_PADDING: f32 = 20.0;
const CARD_RADIUS: f32 = 8.0;
const CARD_HEIGHT: f32 = 320.0;
const FONT_CANDIDATES: [&str; 6] = [
    "C:\\Windows\\Fonts\\msyh.ttc",
    "C:\\Windows\\Fonts\\msyh.ttf",
    "C:\\Windows\\Fonts\\msyhbd.ttc",
    "C:\\Windows\\Fonts\\simhei.ttf",
    "C:\\Windows\\Fonts\\segoeui.ttf",
    "C:\\Windows\\Fonts\\arial.ttf",
];

pub fn launch_gui() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 580.0])
            .with_resizable(false)
            .with_drag_and_drop(true),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "AheadLibEx",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            setup_style(&cc.egui_ctx);
            Ok(Box::new(App::default()))
        }),
    )
    .map_err(|e| anyhow::anyhow!("Failed to launch GUI: {e}"))
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    // 这个SB字体兼容需求得照顾win7-win10，所以多试几个系统自带字体
    for path in FONT_CANDIDATES {
        if let Ok(bytes) = std::fs::read(path) {
            let name = path.to_string();
            fonts
                .font_data
                .insert(name.clone(), egui::FontData::from_owned(bytes));
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, name.clone());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, name);
            break;
        }
    }
    ctx.set_fonts(fonts);
}

fn setup_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.spacing.item_spacing = egui::vec2(SPACING, SPACING);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);

    style.visuals = egui::Visuals::light();
    style.visuals.window_fill = colors::BG_PRIMARY;
    style.visuals.panel_fill = colors::BG_PRIMARY;
    style.visuals.override_text_color = Some(colors::TEXT_PRIMARY);

    let rounding = Rounding::same(6.0);
    style.visuals.widgets.inactive.rounding = rounding;
    style.visuals.widgets.hovered.rounding = rounding;
    style.visuals.widgets.active.rounding = rounding;
    style.visuals.widgets.noninteractive.rounding = rounding;

    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, colors::BORDER);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, colors::ACCENT);

    ctx.set_style(style);
}

#[derive(Default)]
struct App {
    state: UiState,
}

const PROJECT_TYPES: [&str; 4] = [
    "Visual Studio 2022",
    "Visual Studio 2019",
    "Visual Studio 2017",
    "CMake",
];

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        events::handle_drop(&mut self.state, ctx);

        egui::CentralPanel::default()
            .frame(Frame::none().fill(colors::BG_PRIMARY).inner_margin(20.0))
            .show(ctx, |ui| {
                // Title
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("AheadLibEx")
                            .size(20.0)
                            .strong()
                            .color(colors::TEXT_PRIMARY),
                    );
                    ui.label(
                        RichText::new("v2.0")
                            .size(12.0)
                            .color(colors::TEXT_SECONDARY),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new("Rust Edition")
                                .size(11.0)
                                .color(colors::TEXT_HINT),
                        );
                    });
                });

                ui.add_space(12.0);

                // Main panels
                let available = ui.available_size();
                let panel_width = (available.x - SPACING) / 2.0;

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.set_width(panel_width);
                        self.left_panel(ui);
                    });

                    ui.vertical(|ui| {
                        ui.set_width(panel_width);
                        self.right_panel(ui);
                    });
                });

                ui.add_space(12.0);

                // Buttons
                ui.horizontal(|ui| {
                    let total_width = BUTTON_WIDTH * 3.0 + SPACING * 2.0;
                    let offset = (ui.available_width() - total_width) / 2.0;
                    ui.add_space(offset.max(0.0));

                    if ui
                        .add_sized(
                            [BUTTON_WIDTH, CONTROL_HEIGHT],
                            egui::Button::new(RichText::new("Generate").color(Color32::WHITE))
                                .fill(colors::ACCENT),
                        )
                        .clicked()
                    {
                        events::generate(&mut self.state);
                    }

                    if ui
                        .add_sized([BUTTON_WIDTH, CONTROL_HEIGHT], egui::Button::new("Reset"))
                        .clicked()
                    {
                        events::reset(&mut self.state);
                    }

                    if ui
                        .add_sized([BUTTON_WIDTH, CONTROL_HEIGHT], egui::Button::new("Exit"))
                        .clicked()
                    {
                        ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                    }
                });

                ui.add_space(8.0);

                // Copyright (centered)
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    let total_width = 280.0; // approximate width of copyright text
                    let offset = (ui.available_width() - total_width) / 2.0;
                    ui.add_space(offset.max(0.0));
                    ui.label(RichText::new("© 2025 i1tao").size(11.0).color(colors::TEXT_HINT));
                    ui.label(RichText::new("|").size(11.0).color(colors::TEXT_HINT));
                    ui.hyperlink_to(
                        RichText::new("github.com/i1tao/aheadlibex").size(11.0).color(colors::ACCENT),
                        "https://github.com/i1tao/aheadlibex",
                    );
                });

                // Drag overlay
                if self.state.dragging {
                    let rect = ctx.screen_rect();
                    ui.painter().rect_filled(
                        rect,
                        0.0,
                        Color32::from_rgba_unmultiplied(64, 128, 240, 25),
                    );
                    ui.painter().rect_stroke(
                        rect.shrink(20.0),
                        Rounding::same(12.0),
                        Stroke::new(2.0, colors::ACCENT),
                    );
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "Drop DLL here",
                        egui::FontId::proportional(16.0),
                        colors::ACCENT,
                    );
                }
            });
    }
}

impl App {
    fn left_panel(&mut self, ui: &mut egui::Ui) {
        let state = &mut self.state;

        ui.label(
            RichText::new("Project Settings")
                .size(13.0)
                .color(colors::TEXT_SECONDARY),
        );
        ui.add_space(6.0);

        Frame::none()
            .fill(colors::BG_CARD)
            .rounding(CARD_RADIUS)
            .stroke(Stroke::new(1.0, colors::BORDER))
            .inner_margin(CARD_PADDING)
            .show(ui, |ui| {
                ui.set_height(CARD_HEIGHT);
                let input_width = ui.available_width() - BROWSE_BTN_WIDTH - SPACING;

                // DLL Input
                ui.label(
                    RichText::new("Input DLL")
                        .size(12.0)
                        .color(colors::TEXT_SECONDARY),
                );
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.add_sized(
                        [input_width, CONTROL_HEIGHT],
                        egui::TextEdit::singleline(&mut state.dll_path).hint_text(
                            RichText::new("Select or drop DLL...")
                                .size(13.0)
                                .color(colors::TEXT_HINT),
                        ),
                    );
                    if ui
                        .add_sized(
                            [BROWSE_BTN_WIDTH, CONTROL_HEIGHT],
                            egui::Button::new("Browse"),
                        )
                        .clicked()
                    {
                        events::pick_dll(state);
                    }
                });

                ui.add_space(SPACING);
                ui.separator();
                ui.add_space(SPACING);

                // Project Type
                ui.label(
                    RichText::new("Project Type")
                        .size(12.0)
                        .color(colors::TEXT_SECONDARY),
                );
                ui.add_space(4.0);
                egui::ComboBox::from_id_source("proj_type")
                    .selected_text(PROJECT_TYPES[state.project_type])
                    .width(200.0)
                    .show_ui(ui, |ui| {
                        for (i, name) in PROJECT_TYPES.iter().enumerate() {
                            ui.selectable_value(&mut state.project_type, i, *name);
                        }
                    });

                ui.add_space(SPACING);

                // Output Directory
                ui.label(
                    RichText::new("Output Directory")
                        .size(12.0)
                        .color(colors::TEXT_SECONDARY),
                );
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.add_sized(
                        [input_width, CONTROL_HEIGHT],
                        egui::TextEdit::singleline(&mut state.project_dir).hint_text(
                            RichText::new("Project output directory...")
                                .size(13.0)
                                .color(colors::TEXT_HINT),
                        ),
                    );
                    if ui
                        .add_sized(
                            [BROWSE_BTN_WIDTH, CONTROL_HEIGHT],
                            egui::Button::new("Browse"),
                        )
                        .clicked()
                    {
                        events::pick_dir(state);
                    }
                });
            });
    }

    fn right_panel(&mut self, ui: &mut egui::Ui) {
        let state = &mut self.state;

        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Output Log")
                    .size(13.0)
                    .color(colors::TEXT_SECONDARY),
            );
            if let Some(ok) = state.success {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (t, c) = if ok {
                        ("Success", colors::SUCCESS)
                    } else {
                        ("Failed", colors::ERROR)
                    };
                    ui.label(RichText::new(t).size(12.0).color(c));
                });
            }
        });
        ui.add_space(6.0);

        Frame::none()
            .fill(colors::BG_CARD)
            .rounding(CARD_RADIUS)
            .stroke(Stroke::new(1.0, colors::BORDER))
            .inner_margin(CARD_PADDING)
            .show(ui, |ui| {
                ui.set_height(CARD_HEIGHT);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut state.log)
                            .desired_width(f32::INFINITY)
                            .font(egui::TextStyle::Monospace)
                            .frame(false),
                    );
                });
            });
    }
}
