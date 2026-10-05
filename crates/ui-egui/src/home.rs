//! Home: two ways in (a blank page to draw on, or a PDF to edit), then every other tool, then
//! recent files (local only, never another app's list).

use egui::{Align2, CornerRadius, Rect, Sense, Stroke, vec2};
use printcraft_engine::catalog;

use crate::theme::{self, Tokens};
use crate::{LeftPanel, PrintCraftApp, icons, panels::human_size, widgets};

pub fn show(app: &mut PrintCraftApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        let side = ((ui.available_width() - 760.0) / 2.0).clamp(28.0, 120.0);
        egui::Frame::NONE.inner_margin(egui::Margin { left: side as i8, right: side as i8, top: 40, bottom: 32 }).show(ui, |ui| {
            ui.label(egui::RichText::new("What do you want to make?").font(theme::semibold(26.0)));
            ui.label(egui::RichText::new("Sketch on a blank page, or draw and write on any PDF.").color(t.text_muted).font(theme::regular(14.5)));
            ui.add_space(20.0);

            // The two ways in.
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 16.0;
                let w = ((ui.available_width() - 16.0) / 2.0).max(220.0);
                if start_card(ui, &t, w, "file-plus", "Blank Page", "A fresh page with the pen ready", true).clicked() {
                    app.execute("draw.new");
                }
                if start_card(ui, &t, w, "folder-open", "Edit a PDF", "Open a PDF to draw, write and mark up", false).clicked() {
                    let before = app.views.len();
                    app.open_dialog();
                    if app.views.len() > before {
                        app.pick_up_pen();
                    }
                }
            });

            ui.add_space(30.0);
            ui.label(egui::RichText::new("More tools").font(theme::semibold(16.0)));
            ui.add_space(10.0);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = vec2(10.0, 10.0);
                for g in catalog::TOOL_GROUPS {
                    if tool_tile(ui, &t, g).clicked() {
                        // Tools work on a document: pick one, then the tool's panel opens beside it.
                        app.left = LeftPanel::Tool(g.id);
                        app.left_open = true;
                        app.open_dialog();
                    }
                }
            });
            ui.add_space(26.0);
            ui.label(egui::RichText::new("Recent").font(theme::semibold(17.0)));
            ui.add_space(8.0);
            if app.recent.is_empty() {
                ui.label(egui::RichText::new("Files you open appear here. Drop a PDF anywhere to open it.").color(t.text_muted));
            }
            let mut open = None;
            for r in &app.recent {
                let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 46.0), Sense::click());
                resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &r.name));
                if resp.hovered() {
                    ui.painter().rect_filled(rect, CornerRadius::same(8), t.hover);
                }
                icons::paint(
                    ui,
                    Rect::from_min_size(rect.min + vec2(10.0, 11.0), vec2(24.0, 24.0)),
                    "file-text",
                    22.0,
                    egui::Color32::from_rgb(0xE0, 0x3E, 0x3E),
                );
                ui.painter().text(rect.min + vec2(46.0, 15.0), Align2::LEFT_CENTER, &r.name, theme::medium(13.5), t.text);
                ui.painter().text(rect.min + vec2(46.0, 32.0), Align2::LEFT_CENTER, &r.path, theme::regular(11.0), t.text_faint);
                ui.painter().text(
                    rect.right_center() - vec2(12.0, 0.0),
                    Align2::RIGHT_CENTER,
                    format!("{} pages  ·  {}", r.pages, human_size(r.size)),
                    theme::regular(12.0),
                    t.text_muted,
                );
                if resp.clicked() {
                    open = Some(r.path.clone());
                }
            }
            if let Some(p) = open {
                if let Some(i) = app.views.iter().position(|v| app.session.get(v.id).and_then(|d| d.path.as_deref()) == Some(p.as_str())) {
                    app.active = Some(i);
                } else {
                    #[cfg(not(target_arch = "wasm32"))]
                    app.open_path(&p);
                }
            }
            ui.add_space(20.0);
            widgets::section_title(ui, "Privacy");
            ui.label(
                egui::RichText::new("PDFThing works offline. No telemetry, no account, and no cloud processing unless you add a provider.")
                    .color(t.text_muted),
            );
        });
    });
}

/// One of the two big start buttons.
fn start_card(ui: &mut egui::Ui, t: &Tokens, width: f32, icon: &str, title: &str, blurb: &str, primary: bool) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(width, 150.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, title));
    let (fill, fg, sub) = if primary {
        (if resp.hovered() { t.accent_text } else { t.accent }, egui::Color32::WHITE, egui::Color32::from_white_alpha(210))
    } else {
        (if resp.hovered() { t.hover } else { t.card }, t.text, t.text_muted)
    };
    let stroke = if primary { Stroke::NONE } else { Stroke::new(1.0, t.border) };
    ui.painter().rect(rect, CornerRadius::same(18), fill, stroke, egui::StrokeKind::Inside);
    let badge = Rect::from_min_size(rect.min + vec2(22.0, 22.0), vec2(44.0, 44.0));
    ui.painter().rect_filled(badge, CornerRadius::same(12), if primary { egui::Color32::from_white_alpha(40) } else { t.accent_soft });
    icons::paint(ui, badge, icon, 22.0, if primary { egui::Color32::WHITE } else { t.accent_text });
    ui.painter().text(rect.min + vec2(22.0, 96.0), Align2::LEFT_CENTER, title, theme::semibold(19.0), fg);
    ui.painter().text(rect.min + vec2(22.0, 122.0), Align2::LEFT_CENTER, blurb, theme::regular(13.0), sub);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A small tile for one tool group.
fn tool_tile(ui: &mut egui::Ui, t: &Tokens, g: &catalog::ToolGroup) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(178.0, 48.0), Sense::click());
    // "Edit a PDF" is the start card above; on the tile, say what the tool edits.
    let label = if g.id == "edit" { "Edit text & images" } else { g.label };
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));
    ui.painter().rect(
        rect,
        CornerRadius::same(12),
        if resp.hovered() { t.hover } else { t.card },
        Stroke::new(1.0, t.divider),
        egui::StrokeKind::Inside,
    );
    let color = egui::Color32::from_rgb(g.hue[0], g.hue[1], g.hue[2]);
    icons::paint(ui, Rect::from_min_size(rect.min + vec2(12.0, 14.0), vec2(20.0, 20.0)), g.icon, 18.0, color);
    let galley = ui.fonts_mut(|f| f.layout(label.to_owned(), theme::medium(12.5), t.text, rect.width() - 52.0));
    ui.painter().galley(rect.min + vec2(42.0, (rect.height() - galley.size().y) / 2.0), galley, t.text);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}
