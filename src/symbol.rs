use eframe::egui;
use eframe::egui::{Color32, Sense, Stroke, Vec2, vec2};
use std::f32::consts::TAU;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Symbol {
    TwoHorizontalLines,
    SquareWithVerticalLine,
    BWithTail,
    CircleWithHorizontalLine,
    TriangleWithCross,
    UpArrow,
    CircleWithCross,
    CircleWithVerticalLine,
    Square,
    UWithCross,
    SquareWithHorizontalLine,
    VWithForwardSlash,
    UWithHorizontalLine,
    UpsideDownUWithCross,
    Triangle,
    X,
    U,
    ThreeHorizontalLines,
    Circle,
    TriangleWithVerticalLine,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TwoHorizontalLines => write!(f, "Two horizontal lines"),
            Self::SquareWithVerticalLine => write!(f, "Square with a vertical line"),
            Self::BWithTail => write!(f, "B with tail"),
            Self::CircleWithHorizontalLine => write!(f, "Circle with a horizontal line"),
            Self::TriangleWithCross => write!(f, "Triangle with a cross"),
            Self::UpArrow => write!(f, "Up arrow"),
            Self::CircleWithCross => write!(f, "Circle with a cross"),
            Self::CircleWithVerticalLine => write!(f, "Circle with a vertical line"),
            Self::Square => write!(f, "Square"),
            Self::UWithCross => write!(f, "U with a cross"),
            Self::SquareWithHorizontalLine => write!(f, "Square with a horizontal line"),
            Self::VWithForwardSlash => write!(f, "V with a forward slash"),
            Self::UWithHorizontalLine => write!(f, "U with a horizontal line"),
            Self::UpsideDownUWithCross => write!(f, "Upside down U with a cross"),
            Self::Triangle => write!(f, "Triangle"),
            Self::X => write!(f, "X"),
            Self::U => write!(f, "U"),
            Self::ThreeHorizontalLines => write!(f, "Three horizontal lines"),
            Self::Circle => write!(f, "Circle"),
            Self::TriangleWithVerticalLine => write!(f, "Triangle with a vertical line"),
        }
    }
}

impl Symbol {
    pub fn tooltip(&self, count: &u8) -> String {
        format!("Desc: {}\nCount: {}", self, count)
    }

    pub fn render(&self, ui: &mut egui::Ui, color: Color32, tooltip: &str) -> bool {
        let size = Vec2::splat(16.0);
        let (mut response, painter) = ui.allocate_painter(size, Sense::click());
        let rect = response.rect;
        let c = rect.center();
        let r = rect.width() / 2.0 - 1.0;
        let stroke = Stroke::new(1.5, color);

        if tooltip != "" {
            response = response.on_hover_text(tooltip);
        }

        match self {
            Self::X => {
                painter.line_segment([c, c + r * Vec2::angled(TAU * 1.0 / 8.0)], stroke);
                painter.line_segment([c, c + r * Vec2::angled(TAU * 3.0 / 8.0)], stroke);
            }
            _ => {
                painter.circle_stroke(c, r, stroke);
                painter.line_segment([c - vec2(0.0, r), c + vec2(0.0, r)], stroke);
                painter.line_segment([c, c + r * Vec2::angled(TAU * 1.0 / 8.0)], stroke);
                painter.line_segment([c, c + r * Vec2::angled(TAU * 3.0 / 8.0)], stroke);
            }
        };

        response.clicked()
    }
}
