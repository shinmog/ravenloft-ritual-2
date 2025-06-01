use crate::app::App;
use eframe::egui;

pub mod app;
pub mod dictionary;
pub mod modal;
pub mod options;
pub mod pillar;
pub mod state;
pub mod symbol;

fn main() {
    //let native_options = eframe::NativeOptions::default();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 650.0])
            .with_min_inner_size([100.0, 100.0])
            .with_active(true)
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Ravenloft Ritual Solver",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}
