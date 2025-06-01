use eframe::egui;

#[derive(Debug)]
pub struct Options {
    pub font_size: f32,
    pub stroke_size: f32,
    pub theme: egui::ThemePreference,
    pub unknown_color: egui::Color32,
    pub guess_color: egui::Color32,
    pub known_color: egui::Color32,
    pub highlight_color: egui::Color32,
    pub new_color: egui::Color32,
}

impl Options {
    pub fn load() -> Self {
        Options {
            font_size: 1.0,
            stroke_size: 1.5,
            theme: egui::ThemePreference::System,
            unknown_color: egui::Color32::WHITE,
            //guess_color: egui::Color32::CYAN,
            guess_color: egui::Color32::MAGENTA,
            known_color: egui::Color32::LIGHT_BLUE,
            highlight_color: egui::Color32::YELLOW,
            new_color: egui::Color32::GREEN,
        }
    }
}
