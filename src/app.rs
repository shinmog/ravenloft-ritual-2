use crate::options::Options;
//use crate::state::State;
use crate::dictionary::Dictionary;
use crate::modal::ModalConfig;
use crate::pillar;
use crate::state::State;
use crate::symbol::Symbol;
use eframe::egui;

#[derive(Debug)]
pub struct App {
    options: Options,
    dict: Dictionary,
    modal: Option<ModalConfig>,
    highlight_symbol: Option<Symbol>,
    pillars: [[[Symbol; 3]; 4]; 5],
    configure_font_size: bool,
}

impl App {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self {
            options: Options::load(),
            dict: Dictionary::load(),
            modal: None,
            highlight_symbol: None,
            pillars: pillar::load(),
            configure_font_size: true,
        }
    }

    fn render_pillar(&mut self, ui: &mut egui::Ui, pillar_idx: usize) {
        let cell_space: f32 = 16.0 * self.options.font_size;
        let dp: egui::Pos2 = match pillar_idx {
            0 => egui::Pos2 { x: 600.0, y: 50.0 },
            1 => egui::Pos2 { x: 780.0, y: 200.0 },
            2 => egui::Pos2 { x: 690.0, y: 350.0 },
            3 => egui::Pos2 { x: 510.0, y: 350.0 },
            4 => egui::Pos2 { x: 420.0, y: 200.0 },
            _ => unreachable!(),
        };

        let _ = egui::Window::new(format!("Group {}", pillar_idx + 1))
            .vscroll(false)
            .resizable([true, false])
            .pivot(egui::Align2::CENTER_TOP)
            .default_pos(dp)
            .default_height(cell_space * (self.pillars[pillar_idx].len() + 1) as f32)
            .default_width(cell_space * (self.pillars[pillar_idx][0].len() + 1) as f32)
            .show(ui.ctx(), |ui| {
                egui::Grid::new(format!("grid_{}", pillar_idx))
                    .num_columns(self.pillars[pillar_idx][0].len())
                    .show(ui, |ui| {
                        for row_idx in 0..self.pillars[pillar_idx].len() {
                            for column_idx in 0..self.pillars[pillar_idx][row_idx].len() {
                                self.render_cell(ui, pillar_idx, row_idx, column_idx);
                            }
                            ui.end_row();
                        }
                    });
            });
    }

    fn render_cell(
        &mut self,
        ui: &mut egui::Ui,
        pillar_idx: usize,
        row_idx: usize,
        column_idx: usize,
    ) {
        let dict_idx: usize = self
            .dict
            .lookup(&self.pillars[pillar_idx][row_idx][column_idx]);
        let (symbol, state, count) = &self.dict.translations[dict_idx];

        let mut cell_color: egui::Color32 = match state {
            State::Unknown => self.options.unknown_color,
            State::Guess(_) => self.options.guess_color,
            State::Known(_) => self.options.known_color,
            State::New(_) => self.options.new_color,
        };

        match &self.highlight_symbol {
            Some(highlight_symbol) => {
                if *symbol == *highlight_symbol {
                    cell_color = self.options.highlight_color;
                }
            }
            _ => (),
        }

        let tooltip: String = symbol.tooltip(count);

        let was_clicked = match state {
            State::Unknown => symbol.render(ui, cell_color, &tooltip),
            State::Guess(letter) => ui
                .add(egui::Link::new(egui::WidgetText::RichText(
                    egui::RichText::new(letter.to_string()).color(cell_color),
                )))
                .on_hover_text(tooltip)
                .clicked(),
            State::Known(letter) => ui
                .add(egui::Link::new(egui::WidgetText::RichText(
                    egui::RichText::new(letter.to_string()).color(cell_color),
                )))
                .on_hover_text(tooltip)
                .clicked(),
            State::New(letter) => ui
                .add(egui::Link::new(egui::WidgetText::RichText(
                    egui::RichText::new(letter.to_string()).color(cell_color),
                )))
                .on_hover_text(tooltip)
                .clicked(),
        };

        if was_clicked {
            self.modal = Some(ModalConfig::new(
                symbol,
                &self.options.known_color,
                &self.dict,
            ));
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let mut should_quit: bool = false;
        ctx.set_theme(self.options.theme);
        if self.configure_font_size {
            ctx.set_pixels_per_point(self.options.font_size);
            self.configure_font_size = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("top_panel")
                .resizable(false)
                .min_height(16.0 * self.options.font_size)
                .show_inside(ui, |ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        if ui.button("Exit").clicked() {
                            should_quit = true;
                        }
                    });
                });

            egui::SidePanel::left("left_panel")
                .resizable(true)
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let mut hovered: bool = false;

                        for (symbol, _, count) in &self.dict.translations {
                            // maybe label instead..?
                            let resp = ui.button(symbol.to_string()).on_hover_ui(|ui| {
                                hovered = true;
                                self.highlight_symbol = Some(symbol.clone());
                                ui.label(symbol.tooltip(count));
                            });
                            if resp.clicked() {
                                self.modal = Some(ModalConfig::new(
                                    symbol,
                                    &self.options.known_color,
                                    &self.dict,
                                ));
                            }
                        }

                        if !hovered {
                            self.highlight_symbol = None;
                        }

                        ui.label("");
                        ui.separator();
                        ui.label("");

                        egui::Grid::new("options").num_columns(2).show(ui, |ui| {
                            ui.label("Font Size");
                            let resp = ui.add(
                                egui::DragValue::new(&mut self.options.font_size)
                                    .range(0.0..=5.0)
                                    .speed(0.1)
                                    .update_while_editing(false),
                            );
                            if (resp.changed() && !resp.dragged()) || resp.drag_stopped() {
                                self.configure_font_size = true;
                            }
                            ui.end_row();

                            ui.label("Known Color");
                            ui.color_edit_button_srgba(&mut self.options.known_color);
                            ui.end_row();

                            ui.label("Unknown Color");
                            ui.color_edit_button_srgba(&mut self.options.unknown_color);
                            ui.end_row();

                            ui.label("Guess Color");
                            ui.color_edit_button_srgba(&mut self.options.guess_color);
                            ui.end_row();

                            ui.label("Highlight Color");
                            ui.color_edit_button_srgba(&mut self.options.highlight_color);
                            ui.end_row();

                            ui.label("New Color");
                            ui.color_edit_button_srgba(&mut self.options.new_color);
                            ui.end_row();

                            ui.label("Theme");
                            ui.horizontal(|ui| {
                                ui.selectable_value(
                                    &mut self.options.theme,
                                    egui::ThemePreference::Light,
                                    "☀ Light",
                                );
                                ui.selectable_value(
                                    &mut self.options.theme,
                                    egui::ThemePreference::Dark,
                                    "🌙 Dark",
                                );
                                ui.selectable_value(
                                    &mut self.options.theme,
                                    egui::ThemePreference::System,
                                    "💻 System",
                                );
                            });
                            ui.end_row();
                        });
                    });
                });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                for pillar_idx in 0..self.pillars.len() {
                    self.render_pillar(ui, pillar_idx);
                }

                // Modal render.
                if let Some(m) = &mut self.modal {
                    match m.render(ui) {
                        None => (),
                        Some(value) => {
                            for (e_symbol, e_state, _) in self.dict.translations.iter_mut() {
                                if e_symbol == &m.symbol {
                                    *e_state = value;
                                    self.modal = None;
                                    break;
                                }
                            }
                        }
                    }
                }
            });
        });

        if should_quit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}
