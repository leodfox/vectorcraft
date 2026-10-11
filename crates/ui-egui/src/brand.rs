//! The Ardoise mark: a plain slate tile drawn in code (the upstream app icon is not used).

use egui::{Color32, CornerRadius, Rect, Stroke, StrokeKind, Ui};

/// Paint the mark into `r` (square).
pub fn paint_mark(ui: &Ui, r: Rect) {
    let radius = CornerRadius::same((r.width() * 0.18).round().clamp(2.0, 12.0) as u8);
    ui.painter().rect_filled(r, radius, Color32::from_rgb(0x2f, 0x3a, 0x4a));
    ui.painter().rect_stroke(r.shrink(r.width() * 0.22), CornerRadius::same(2), Stroke::new(1.5, Color32::from_rgb(0xdd, 0xe4, 0xee)), StrokeKind::Inside);
}
