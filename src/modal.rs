use crate::dictionary::Dictionary;
use crate::state::State;
use crate::symbol::Symbol;
use eframe::egui;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum ModalState {
    Unknown,
    Guess,
    Known,
    New,
}

#[derive(Debug, PartialEq)]
enum Letter {
    None,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Letter {
    fn from(letter: char) -> Self {
        match letter {
            'A' => Letter::A,
            'B' => Letter::B,
            'C' => Letter::C,
            'D' => Letter::D,
            'E' => Letter::E,
            'F' => Letter::F,
            'G' => Letter::G,
            'H' => Letter::H,
            'I' => Letter::I,
            'J' => Letter::J,
            'K' => Letter::K,
            'L' => Letter::L,
            'M' => Letter::M,
            'N' => Letter::N,
            'O' => Letter::O,
            'P' => Letter::P,
            'Q' => Letter::Q,
            'R' => Letter::R,
            'S' => Letter::S,
            'T' => Letter::T,
            'U' => Letter::U,
            'V' => Letter::V,
            'W' => Letter::W,
            'X' => Letter::X,
            'Y' => Letter::Y,
            'Z' => Letter::Z,
            _ => unimplemented!(),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Letter::A => 'A',
            Letter::B => 'B',
            Letter::C => 'C',
            Letter::D => 'D',
            Letter::E => 'E',
            Letter::F => 'F',
            Letter::G => 'G',
            Letter::H => 'H',
            Letter::I => 'I',
            Letter::J => 'J',
            Letter::K => 'K',
            Letter::L => 'L',
            Letter::M => 'M',
            Letter::N => 'N',
            Letter::O => 'O',
            Letter::P => 'P',
            Letter::Q => 'Q',
            Letter::R => 'R',
            Letter::S => 'S',
            Letter::T => 'T',
            Letter::U => 'U',
            Letter::V => 'V',
            Letter::W => 'W',
            Letter::X => 'X',
            Letter::Y => 'Y',
            Letter::Z => 'Z',
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct ModalConfig {
    pub symbol: Symbol,
    symbol_color: egui::Color32,
    modal_state: ModalState,
    letter: Letter,
    char_availability: HashMap<char, bool>,
}

impl ModalConfig {
    pub fn new(symbol: &Symbol, symbol_color: &egui::Color32, dict: &Dictionary) -> Self {
        let symbol = symbol.clone();
        let mut modal_state = ModalState::Unknown;
        let mut letter = Letter::None;

        let mut char_availability: HashMap<char, bool> = dict
            .translations
            .iter()
            .filter_map(|(dict_symbol, dict_state, _)| match dict_state {
                State::Unknown => None,
                State::Guess(value) => {
                    modal_state = ModalState::Guess;
                    if *dict_symbol == symbol {
                        letter = Letter::from(*value);
                    }
                    Some((*value, *dict_symbol == symbol))
                }
                State::Known(value) => {
                    modal_state = ModalState::Known;
                    if *dict_symbol == symbol {
                        letter = Letter::from(*value);
                    }
                    Some((*value, *dict_symbol == symbol))
                }
                State::New(value) => {
                    modal_state = ModalState::New;
                    if *dict_symbol == symbol {
                        letter = Letter::from(*value);
                    }
                    Some((*value, *dict_symbol == symbol))
                }
            })
            .collect::<HashMap<char, bool>>();

        for value in b'A'..=b'Z' {
            char_availability.entry(value as char).or_insert(true);
        }

        Self {
            symbol: symbol,
            symbol_color: symbol_color.clone(),
            modal_state: modal_state,
            letter: letter,
            char_availability: char_availability,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) -> Option<State> {
        let mut ans: Option<State> = None;

        egui::Modal::new(egui::Id::new("symbol_modal")).show(ui.ctx(), |ui| {
            ui.set_width(200.0);
            ui.vertical_centered(|ui| {
                ui.heading("Symbol Config");
            });

            ui.label("");

            egui::Grid::new("modal_state")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Symbol");
                    self.symbol.render(ui, self.symbol_color, "");
                    ui.end_row();

                    ui.label("Description");
                    ui.label(self.symbol.to_string());
                    ui.end_row();

                    ui.label("Confidence");
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.modal_state, ModalState::Unknown, "Unknown");
                        ui.radio_value(&mut self.modal_state, ModalState::Guess, "Guess");
                        ui.radio_value(&mut self.modal_state, ModalState::Known, "Known");
                        ui.radio_value(&mut self.modal_state, ModalState::New, "New");
                    });
                    ui.end_row();
                });

            ui.separator();

            let mut ui_builder = egui::UiBuilder::new();
            if self.modal_state == ModalState::Unknown {
                ui_builder = ui_builder.disabled();
            }

            ui.scope_builder(ui_builder, |ui| {
                egui::Grid::new("modal_letter")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Letter");
                        ui.horizontal(|ui| {
                            self.render_letter('Q', ui);
                            self.render_letter('W', ui);
                            self.render_letter('E', ui);
                            self.render_letter('R', ui);
                            self.render_letter('T', ui);
                            self.render_letter('Y', ui);
                            self.render_letter('U', ui);
                            self.render_letter('I', ui);
                            self.render_letter('O', ui);
                            self.render_letter('P', ui);
                        });
                        ui.end_row();

                        ui.label("");
                        ui.horizontal(|ui| {
                            self.render_letter('A', ui);
                            self.render_letter('S', ui);
                            self.render_letter('D', ui);
                            self.render_letter('F', ui);
                            self.render_letter('G', ui);
                            self.render_letter('H', ui);
                            self.render_letter('J', ui);
                            self.render_letter('K', ui);
                            self.render_letter('L', ui);
                        });
                        ui.end_row();

                        ui.label("");
                        ui.horizontal(|ui| {
                            self.render_letter('Z', ui);
                            self.render_letter('X', ui);
                            self.render_letter('C', ui);
                            self.render_letter('V', ui);
                            self.render_letter('B', ui);
                            self.render_letter('N', ui);
                            self.render_letter('M', ui);
                            ui.selectable_value(&mut self.letter, Letter::None, "None");
                        });
                        ui.end_row();
                    });
            });

            ui.vertical_centered(|ui| {
                if ui.button("Done").clicked() {
                    if self.modal_state == ModalState::Unknown || self.letter == Letter::None {
                        ans = Some(State::Unknown);
                    } else {
                        ans = match self.modal_state {
                            ModalState::Guess => Some(State::Guess(self.letter.as_char())),
                            ModalState::Known => Some(State::Known(self.letter.as_char())),
                            ModalState::New => Some(State::New(self.letter.as_char())),
                            _ => unimplemented!(),
                        };
                    }
                }
            });
        });

        ans
    }

    fn render_letter(&mut self, letter: char, ui: &mut egui::Ui) {
        let available: bool = *self.char_availability.get(&letter).unwrap();

        if available {
            ui.selectable_value(&mut self.letter, Letter::from(letter), letter.to_string());
        } else {
            ui.colored_label(egui::Color32::RED, letter.to_string());
        }
    }
}
