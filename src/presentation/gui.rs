use crate::ui_events::{
    generate, handle_drop, pick_dir, pick_dll, reset, OriginModeChoice, UiLanguageChoice, UiState,
};
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
const PATH_CONTROL_HEIGHT: f32 = 36.0;
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
const MIN_WINDOW_W: f32 = 420.0;
const MIN_WINDOW_H: f32 = 620.0;
const MAX_WINDOW_W: f32 = 440.0;
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
            .with_inner_size([420.0, 660.0])
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

fn tr(lang: UiLanguageChoice, en: &'static str, zh_hans: &'static str, zh_hant: &'static str) -> &'static str {
    match lang {
        UiLanguageChoice::English => en,
        UiLanguageChoice::ZhHans => zh_hans,
        UiLanguageChoice::ZhHant => zh_hant,
    }
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

                let w = (monitor_size.x * 0.27).clamp(MIN_WINDOW_W, max_w);
                let h = (monitor_size.y * 0.78).clamp(MIN_WINDOW_H, max_h);

                ctx.send_viewport_cmd(ViewportCommand::InnerSize(egui::vec2(w, h)));
                ctx.send_viewport_cmd(ViewportCommand::OuterPosition(egui::pos2(
                    ((monitor_size.x - w) * 0.5).max(0.0),
                    ((monitor_size.y - h) * 0.5).max(0.0),
                )));
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
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("AheadLibEx")
                            .size(20.0)
                            .strong()
                            .color(colors::TEXT_PRIMARY),
                    );
                    ui.label(
                        RichText::new("v0.2.4")
                            .size(12.0)
                            .color(colors::TEXT_SECONDARY),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        egui::ComboBox::from_id_source("ui_language")
                            .selected_text(self.state.ui_language.display_name())
                            .width(110.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.state.ui_language,
                                    UiLanguageChoice::English,
                                    UiLanguageChoice::English.display_name(),
                                );
                                ui.selectable_value(
                                    &mut self.state.ui_language,
                                    UiLanguageChoice::ZhHans,
                                    UiLanguageChoice::ZhHans.display_name(),
                                );
                                ui.selectable_value(
                                    &mut self.state.ui_language,
                                    UiLanguageChoice::ZhHant,
                                    UiLanguageChoice::ZhHant.display_name(),
                                );
                            });
                        ui.label(
                            RichText::new(tr(self.state.ui_language, "Language", "语言", "語言"))
                                .size(11.0)
                                .color(colors::TEXT_HINT),
                        );
                    });
                });
            });

        let lang = self.state.ui_language;

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
                            || self.state.output_vs2026
                            || self.state.output_cmake;

                        if ui
                            .add_sized(
                                [BUTTON_WIDTH, CONTROL_HEIGHT],
                                egui::Button::new(
                                    RichText::new(tr(lang, "Generate", "生成", "生成"))
                                        .color(Color32::WHITE),
                                )
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
                            .add_sized(
                                [BUTTON_WIDTH, CONTROL_HEIGHT],
                                egui::Button::new(tr(lang, "Reset", "重置", "重置")),
                            )
                            .clicked()
                        {
                            reset(&mut self.state);
                        }

                        if ui
                            .add_sized(
                                [BUTTON_WIDTH, CONTROL_HEIGHT],
                                egui::Button::new(tr(lang, "Exit", "退出", "退出")),
                            )
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
                tr(lang, "Drop DLL here", "将 DLL 拖放到此处", "將 DLL 拖放到此處"),
                egui::FontId::proportional(16.0),
                colors::ACCENT,
            );
        }
    }
}

impl App {
    fn centered_placeholder_text_edit(
        ui: &mut egui::Ui,
        size: [f32; 2],
        text: &mut String,
        placeholder: &str,
    ) -> egui::Response {
        let response = ui.add_sized(
            size,
            egui::TextEdit::singleline(text).vertical_align(egui::Align::Center),
        );

        if text.is_empty() {
            let rect = response.rect;
            let pos = egui::pos2(rect.left() + 6.0, rect.center().y);
            ui.painter().text(
                pos,
                egui::Align2::LEFT_CENTER,
                placeholder,
                egui::FontId::proportional(13.0),
                colors::TEXT_HINT,
            );
        }

        response
    }

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
        cmake: &mut bool,
    ) {
        Self::output_checkbox(ui, "Source", source, &mut [vs2022, vs2026, cmake]);
        Self::output_checkbox(ui, "VS2022", vs2022, &mut [source, vs2026, cmake]);
        Self::output_checkbox(ui, "VS2026", vs2026, &mut [source, vs2022, cmake]);
        Self::output_checkbox(ui, "CMake", cmake, &mut [source, vs2022, vs2026]);
    }

    fn left_panel(&mut self, ui: &mut egui::Ui, card_height: f32) {
        let state = &mut self.state;
        let lang = state.ui_language;

        ui.label(
            RichText::new(tr(lang, "Project Settings", "项目设置", "專案設定"))
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
                    RichText::new(tr(lang, "Input DLL", "输入 DLL", "輸入 DLL"))
                        .size(12.0)
                        .color(colors::TEXT_SECONDARY),
                );
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    Self::centered_placeholder_text_edit(
                        ui,
                        [input_width, PATH_CONTROL_HEIGHT],
                        &mut state.dll_path,
                        tr(lang, "Select or drop DLL...", "选择或拖放 DLL...", "選擇或拖放 DLL..."),
                    );
                    if ui
                        .add_sized(
                            [BROWSE_BTN_WIDTH, PATH_CONTROL_HEIGHT],
                            egui::Button::new(tr(lang, "Browse", "浏览", "瀏覽")),
                        )
                        .clicked()
                    {
                        pick_dll(state);
                    }
                });

                ui.add_space(SPACING);

                ui.label(
                    RichText::new(tr(lang, "Output Directory", "输出目录", "輸出目錄"))
                        .size(12.0)
                        .color(colors::TEXT_SECONDARY),
                );
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    Self::centered_placeholder_text_edit(
                        ui,
                        [input_width, PATH_CONTROL_HEIGHT],
                        &mut state.project_dir,
                        tr(
                            lang,
                            "Project output directory...",
                            "项目输出目录...",
                            "專案輸出目錄...",
                        ),
                    );
                    if ui
                        .add_sized(
                            [BROWSE_BTN_WIDTH, PATH_CONTROL_HEIGHT],
                            egui::Button::new(tr(lang, "Browse", "浏览", "瀏覽")),
                        )
                        .clicked()
                    {
                        pick_dir(state);
                    }
                });

                ui.add_space(SPACING);

                ui.label(
                    RichText::new(tr(lang, "Outputs", "输出", "輸出"))
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
                        &mut state.output_cmake,
                    );
                });

                ui.add_space(SPACING);

                ui.label(
                    RichText::new(tr(lang, "Original DLL", "原始 DLL", "原始 DLL"))
                        .size(12.0)
                        .color(colors::TEXT_SECONDARY),
                );
                ui.add_space(4.0);

                egui::ComboBox::from_id_source("origin_mode")
                    .selected_text(match state.origin_mode {
                        OriginModeChoice::SystemDir => {
                            tr(lang, "System directory", "系统目录", "系統目錄")
                        }
                        OriginModeChoice::SameDir => tr(
                            lang,
                            "Same directory (renamed)",
                            "同目录（改名）",
                            "同目錄（改名）",
                        ),
                        OriginModeChoice::CustomPath => {
                            tr(lang, "Custom path", "自定义路径", "自訂路徑")
                        }
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut state.origin_mode,
                            OriginModeChoice::SystemDir,
                            tr(lang, "System directory", "系统目录", "系統目錄"),
                        );
                        ui.selectable_value(
                            &mut state.origin_mode,
                            OriginModeChoice::SameDir,
                            tr(lang, "Same directory (renamed)", "同目录（改名）", "同目錄（改名）"),
                        );
                        ui.selectable_value(
                            &mut state.origin_mode,
                            OriginModeChoice::CustomPath,
                            tr(lang, "Custom path", "自定义路径", "自訂路徑"),
                        );
                    });

                ui.add_space(6.0);
                match state.origin_mode {
                    OriginModeChoice::SystemDir => {}
                    OriginModeChoice::SameDir => {
                        let w = ui.available_width();
                        Self::centered_placeholder_text_edit(
                            ui,
                            [w, PATH_CONTROL_HEIGHT],
                            &mut state.origin_same_dir_name,
                            tr(lang, "e.g. foo_orig.dll", "例如 foo_orig.dll", "例如 foo_orig.dll"),
                        );
                    }
                    OriginModeChoice::CustomPath => {
                        let w = ui.available_width();
                        Self::centered_placeholder_text_edit(
                            ui,
                            [w, PATH_CONTROL_HEIGHT],
                            &mut state.origin_custom_path,
                            tr(
                                lang,
                                "e.g. C:\\\\path\\\\to\\\\foo.dll or foo_orig.dll",
                                "例如 C:\\\\path\\\\to\\\\foo.dll 或 foo_orig.dll",
                                "例如 C:\\\\path\\\\to\\\\foo.dll 或 foo_orig.dll",
                            ),
                        );
                    }
                }
            });
    }

    fn right_panel(&mut self, ui: &mut egui::Ui, card_height: f32) {
        let state = &mut self.state;
        let lang = state.ui_language;

        ui.horizontal(|ui| {
            ui.label(
                RichText::new(tr(lang, "Output Log", "输出日志", "輸出日誌"))
                    .size(13.0)
                    .color(colors::TEXT_SECONDARY),
            );
            if let Some(ok) = state.success {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (t, c) = if ok {
                        (tr(lang, "Success", "成功", "成功"), colors::SUCCESS)
                    } else {
                        (tr(lang, "Failed", "失败", "失敗"), colors::ERROR)
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
