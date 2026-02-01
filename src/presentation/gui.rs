use crate::ui_events::{generate, handle_drop, pick_dir, pick_dll, reset, OriginModeChoice, UiState};
use anyhow::Result;
use eframe::egui::{self, Color32, Frame, Id, Order, RichText, Rounding, Stroke, ViewportCommand};

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
const SPACING: f32 = 10.0;
const CARD_PADDING: f32 = 16.0;
const CARD_RADIUS: f32 = 8.0;
const MIN_SETTINGS_CARD_HEIGHT: f32 = 140.0;
const WINDOW_MARGIN: f32 = 16.0;
const SECTION_TITLE_GAP: f32 = 6.0;
const SECTION_TITLE_HEIGHT_ESTIMATE: f32 = 20.0;
const HEADER_MARGIN_Y: f32 = 12.0;
const BOTTOM_MARGIN_Y: f32 = 10.0;
const BOTTOM_ACTIONS_FOOTER_GAP: f32 = 8.0;
const MIN_WINDOW_W: f32 = 820.0;
const MIN_WINDOW_H: f32 = 620.0;
const MAX_WINDOW_W: f32 = 880.0;
const MAX_WINDOW_H: f32 = 760.0;
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
            .with_inner_size([840.0, 660.0])
            .with_min_inner_size([MIN_WINDOW_W, MIN_WINDOW_H])
            .with_resizable(true)
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
    did_apply_initial_viewport_size: bool,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if !self.did_apply_initial_viewport_size {
            if let Some(monitor_size) = ctx.input(|i| i.viewport().monitor_size) {
                let max_w = (monitor_size.x * 0.92).min(MAX_WINDOW_W);
                let max_h = (monitor_size.y * 0.92).min(MAX_WINDOW_H);

                let w = (monitor_size.x * 0.54).clamp(MIN_WINDOW_W, max_w);
                let h = (monitor_size.y * 0.78).clamp(MIN_WINDOW_H, max_h);

                ctx.send_viewport_cmd(ViewportCommand::InnerSize(egui::vec2(w, h)));
                ctx.send_viewport_cmd(ViewportCommand::Maximized(false));
                self.did_apply_initial_viewport_size = true;
            }
        }

        handle_drop(&mut self.state, ctx);

        let header_frame = Frame::none()
            .fill(colors::BG_PRIMARY)
            .inner_margin(egui::Margin::symmetric(WINDOW_MARGIN, HEADER_MARGIN_Y));
        let bottom_frame = Frame::none()
            .fill(colors::BG_PRIMARY)
            .inner_margin(egui::Margin::symmetric(WINDOW_MARGIN, BOTTOM_MARGIN_Y));

        egui::TopBottomPanel::top("header")
            .frame(header_frame)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("AheadLibEx")
                            .size(20.0)
                            .strong()
                            .color(colors::TEXT_PRIMARY),
                    );
                    ui.label(
                        RichText::new("v0.2.3")
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
            });

        egui::TopBottomPanel::bottom("bottom")
            .frame(bottom_frame)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let total_width = BUTTON_WIDTH * 3.0 + SPACING * 2.0;
                        let offset = (ui.available_width() - total_width) / 2.0;
                        ui.add_space(offset.max(0.0));

                        let any_target = self.state.output_source
                            || self.state.output_vs2022
                            || self.state.output_vs2026;

                        if ui
                            .add_sized(
                                [BUTTON_WIDTH, CONTROL_HEIGHT],
                                egui::Button::new(RichText::new("Generate").color(Color32::WHITE))
                                    .fill(if any_target { colors::ACCENT } else { colors::BORDER })
                                    .sense(if any_target {
                                        egui::Sense::click()
                                    } else {
                                        egui::Sense::hover()
                                    }),
                            )
                            .clicked()
                            && any_target
                        {
                            generate(&mut self.state);
                        }

                        if ui
                            .add_sized([BUTTON_WIDTH, CONTROL_HEIGHT], egui::Button::new("Reset"))
                            .clicked()
                        {
                            reset(&mut self.state);
                        }

                        if ui
                            .add_sized([BUTTON_WIDTH, CONTROL_HEIGHT], egui::Button::new("Exit"))
                            .clicked()
                        {
                            ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                        }
                    });

                    ui.add_space(BOTTOM_ACTIONS_FOOTER_GAP);

                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        let total_width = 280.0;
                        let offset = (ui.available_width() - total_width) / 2.0;
                        ui.add_space(offset.max(0.0));
                        ui.label(
                            RichText::new("(c) 2025 i1tao")
                                .size(11.0)
                                .color(colors::TEXT_HINT),
                        );
                        ui.label(RichText::new("|").size(11.0).color(colors::TEXT_HINT));
                        ui.hyperlink_to(
                            RichText::new("github.com/i1tao/aheadlibex")
                                .size(11.0)
                                .color(colors::ACCENT),
                            "https://github.com/i1tao/aheadlibex",
                        );
                    });
                });
            });

        egui::CentralPanel::default()
            .frame(
                Frame::none()
                    .fill(colors::BG_PRIMARY)
                    .inner_margin(egui::Margin::symmetric(WINDOW_MARGIN, 0.0)),
            )
            .show(ctx, |ui| {
                let available_h = ui.available_height();
                let section_title_overhead = SECTION_TITLE_HEIGHT_ESTIMATE + SECTION_TITLE_GAP;
                let desired_settings_total_h = available_h * 0.38;
                let desired_settings_card_h = (desired_settings_total_h - section_title_overhead)
                    .clamp(MIN_SETTINGS_CARD_HEIGHT, 220.0);

                ui.vertical(|ui| {
                    self.left_panel(ui, desired_settings_card_h);
                    ui.add_space(SPACING);

                    let remaining_h = ui.available_height();
                    let desired_log_card_h = (remaining_h - section_title_overhead).max(0.0);
                    self.right_panel(ui, desired_log_card_h);
                });
            });

        if self.state.dragging {
            let rect = ctx.screen_rect();
            let painter = ctx.layer_painter(egui::LayerId::new(
                Order::Foreground,
                Id::new("drop_overlay"),
            ));
            painter.rect_filled(
                rect,
                0.0,
                Color32::from_rgba_unmultiplied(64, 128, 240, 25),
            );
            painter.rect_stroke(
                rect.shrink(20.0),
                Rounding::same(12.0),
                Stroke::new(2.0, colors::ACCENT),
            );
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Drop DLL here",
                egui::FontId::proportional(16.0),
                colors::ACCENT,
            );
        }
    }
}

impl App {
    fn output_checkbox(ui: &mut egui::Ui, label: &str, state: &mut bool, others: &mut [&mut bool]) {
        let clicked = ui.checkbox(state, label).clicked();
        if clicked && *state {
            for o in others.iter_mut() {
                **o = false;
            }
        }
    }

    fn select_only_one(
        ui: &mut egui::Ui,
        source: &mut bool,
        vs2022: &mut bool,
        vs2026: &mut bool,
    ) {
        Self::output_checkbox(ui, "Source", source, &mut [vs2022, vs2026]);
        Self::output_checkbox(ui, "VS2022", vs2022, &mut [source, vs2026]);
        Self::output_checkbox(ui, "VS2026", vs2026, &mut [source, vs2022]);
    }

    fn left_panel(&mut self, ui: &mut egui::Ui, card_height: f32) {
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
                        ui.set_height(card_height);
                        let input_width = ui.available_width() - BROWSE_BTN_WIDTH - SPACING;

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
                        pick_dll(state);
                    }
                });

                ui.add_space(SPACING);
                ui.separator();
                ui.add_space(SPACING);

                ui.label(
                    RichText::new("Outputs")
                        .size(12.0)
                        .color(colors::TEXT_SECONDARY),
                );
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    Self::select_only_one(
                        ui,
                        &mut state.output_source,
                        &mut state.output_vs2022,
                        &mut state.output_vs2026,
                    );
                });

                ui.add_space(SPACING);
                ui.separator();
                ui.add_space(SPACING);

                ui.label(
                    RichText::new("Original DLL")
                        .size(12.0)
                        .color(colors::TEXT_SECONDARY),
                );
                ui.add_space(4.0);

                egui::ComboBox::from_id_source("origin_mode")
                    .selected_text(match state.origin_mode {
                        OriginModeChoice::SystemDir => "System directory",
                        OriginModeChoice::SameDir => "Same directory (renamed)",
                        OriginModeChoice::CustomPath => "Custom path",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut state.origin_mode,
                            OriginModeChoice::SystemDir,
                            "System directory",
                        );
                        ui.selectable_value(
                            &mut state.origin_mode,
                            OriginModeChoice::SameDir,
                            "Same directory (renamed)",
                        );
                        ui.selectable_value(
                            &mut state.origin_mode,
                            OriginModeChoice::CustomPath,
                            "Custom path",
                        );
                    });

                ui.add_space(6.0);
                match state.origin_mode {
                    OriginModeChoice::SystemDir => {}
                    OriginModeChoice::SameDir => {
                        ui.add_sized(
                            [ui.available_width(), CONTROL_HEIGHT],
                            egui::TextEdit::singleline(&mut state.origin_same_dir_name).hint_text(
                                RichText::new("e.g. foo_orig.dll")
                                    .size(13.0)
                                    .color(colors::TEXT_HINT),
                            ),
                        );
                    }
                    OriginModeChoice::CustomPath => {
                        ui.add_sized(
                            [ui.available_width(), CONTROL_HEIGHT],
                            egui::TextEdit::singleline(&mut state.origin_custom_path).hint_text(
                                RichText::new("e.g. C:\\\\path\\\\to\\\\foo.dll or foo_orig.dll")
                                    .size(13.0)
                                    .color(colors::TEXT_HINT),
                            ),
                        );
                    }
                }

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
                        pick_dir(state);
                    }
                });
            });
    }

    fn right_panel(&mut self, ui: &mut egui::Ui, card_height: f32) {
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
                ui.set_height(card_height);
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        egui::TextEdit::multiline(&mut state.log)
                            .desired_width(f32::INFINITY)
                            .font(egui::TextStyle::Monospace)
                            .frame(false)
                            .interactive(false)
                            .show(ui);
                    });
            });
    }
}
