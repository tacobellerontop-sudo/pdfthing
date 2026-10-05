//! Window chrome: the app-drawn title bar (brand, tabs, actions, window controls) and the right rail.

use egui::{Align, Align2, Color32, CornerRadius, Layout, Rect, Sense, Stroke, vec2};

use crate::canvas::{Fit, PageLayout};
use crate::theme::{self, ThemeKind, Tokens};
use crate::{PrintCraftApp, RightPanel, icons, widgets};

/// The app's own title bar, in place of the system one: brand, document tabs, a few actions, and
/// (where the app draws its own window frame) minimise, maximise and close. Drag it to move the
/// window; double-click it to maximise.
pub fn title_bar(app: &mut PrintCraftApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let left = if cfg!(target_os = "macos") && app.integrated_titlebar { 80 } else { 10 };
    egui::Panel::top("title_bar")
        .exact_size(46.0)
        .frame(
            egui::Frame::NONE.fill(t.titlebar).inner_margin(egui::Margin { left, right: 0, top: 0, bottom: 0 }).stroke(Stroke::new(1.0, t.divider)),
        )
        .show(ui, |ui| {
            let full = ui.max_rect();
            let drag = ui.interact(full, ui.id().with("titledrag"), Sense::click_and_drag());
            if drag.drag_started() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }
            if drag.double_clicked() {
                toggle_maximized(ui.ctx());
            }
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                if brand(ui, &t, app.active.is_none()).clicked() {
                    app.active = None;
                }
                ui.add_space(6.0);
                let mut close = None;
                for i in 0..app.views.len() {
                    let Some(doc) = app.session.get(app.views[i].id) else { continue };
                    let (name, dirty) = (doc.display_name(), doc.dirty);
                    if tab(ui, &t, &name, dirty, app.active == Some(i), &mut close, i).clicked() {
                        app.active = Some(i);
                    }
                }
                if let Some(i) = close {
                    app.request_close_tab(i);
                }
                if icons::button(ui, "plus", 28.0, false, "Open a PDF (⌘O)").clicked() {
                    app.open_dialog();
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;
                    if app.custom_window_controls {
                        window_controls(ui, &t);
                        ui.add_space(6.0);
                    } else {
                        ui.add_space(8.0);
                    }
                    let (icon, next, tip) = match app.theme {
                        ThemeKind::Light => ("moon", ThemeKind::Dark, "Dark theme"),
                        ThemeKind::Dark => ("sun", ThemeKind::Light, "Light theme"),
                    };
                    if icons::button(ui, icon, 32.0, false, tip).clicked() {
                        let ctx = ui.ctx().clone();
                        app.set_theme(&ctx, next);
                    }
                    main_menu(app, ui);
                    if app.active.is_some() {
                        if icons::button(ui, "printer", 32.0, false, "Print (⌘P)").clicked() {
                            app.run_command("print.dialog");
                        }
                        if icons::button(ui, "save", 32.0, false, "Save (⌘S)").clicked() {
                            app.run_command("file.save");
                        }
                        let open = app.left_open;
                        if icons::button(ui, "layout-grid", 32.0, open, "Tools").clicked() {
                            app.left_open = !open;
                            app.left = crate::LeftPanel::AllTools;
                        }
                    }
                    if icons::button(ui, "search", 32.0, false, "Find a tool or command (⌘K)").clicked() {
                        app.palette_open = true;
                    }
                });
            });
        });
}

/// The PDFThing mark and name; goes home.
fn brand(ui: &mut egui::Ui, t: &Tokens, home: bool) -> egui::Response {
    let font = theme::semibold(14.5);
    let w = ui.fonts_mut(|f| f.layout_no_wrap("PDFThing".to_owned(), font.clone(), t.text).size().x);
    let (rect, resp) = ui.allocate_exact_size(vec2(w + 44.0, 34.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, true, home, "Home"));
    if resp.hovered() {
        ui.painter().rect_filled(rect, CornerRadius::same(10), t.hover);
    }
    let mark = Rect::from_min_size(rect.min + vec2(6.0, 5.0), vec2(24.0, 24.0));
    ui.painter().rect_filled(mark, CornerRadius::same(7), t.accent);
    icons::paint(ui, mark, "pencil", 14.0, Color32::WHITE);
    ui.painter().text(rect.left_center() + vec2(36.0, 0.0), Align2::LEFT_CENTER, "PDFThing", font, t.text);
    resp.on_hover_text("Home")
}

fn toggle_maximized(ctx: &egui::Context) {
    let max = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!max));
}

/// Minimise, maximise / restore and close, drawn by the app (right to left).
fn window_controls(ui: &mut egui::Ui, t: &Tokens) {
    let ctx = ui.ctx().clone();
    let control = |ui: &mut egui::Ui, icon: &str, tip: &str, danger: bool| {
        let (rect, resp) = ui.allocate_exact_size(vec2(46.0, 46.0), Sense::click());
        resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, tip));
        let hover = resp.hovered();
        if hover {
            ui.painter().rect_filled(rect, CornerRadius::ZERO, if danger { Color32::from_rgb(0xE8, 0x3B, 0x3B) } else { t.hover });
        }
        icons::paint(ui, rect, icon, 15.0, if hover && danger { Color32::WHITE } else { t.icon });
        resp.on_hover_text(tip).clicked()
    };
    if control(ui, "x", "Close", true) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
    let maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    if control(ui, if maximized { "copy" } else { "square" }, if maximized { "Restore" } else { "Maximise" }, false) {
        toggle_maximized(&ctx);
    }
    if control(ui, "minus", "Minimise", false) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
    }
}

/// Without the system frame, the window edges resize it: show the resize cursor near an edge and
/// start a resize when one is dragged.
pub fn resize_edges(ctx: &egui::Context) {
    use egui::{CursorIcon, ResizeDirection as D};
    const GRIP: f32 = 5.0;
    if ctx.input(|i| i.viewport().maximized.unwrap_or(false) || i.viewport().fullscreen.unwrap_or(false)) {
        return;
    }
    let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) else { return };
    let r = ctx.content_rect();
    let (w, e, n, s) = (pos.x - r.left() < GRIP, r.right() - pos.x < GRIP, pos.y - r.top() < GRIP, r.bottom() - pos.y < GRIP);
    let dir = match (w, e, n, s) {
        (true, _, true, _) => Some((D::NorthWest, CursorIcon::ResizeNorthWest)),
        (_, true, true, _) => Some((D::NorthEast, CursorIcon::ResizeNorthEast)),
        (true, _, _, true) => Some((D::SouthWest, CursorIcon::ResizeSouthWest)),
        (_, true, _, true) => Some((D::SouthEast, CursorIcon::ResizeSouthEast)),
        (true, ..) => Some((D::West, CursorIcon::ResizeWest)),
        (_, true, ..) => Some((D::East, CursorIcon::ResizeEast)),
        (_, _, true, _) => Some((D::North, CursorIcon::ResizeNorth)),
        (.., true) => Some((D::South, CursorIcon::ResizeSouth)),
        _ => None,
    };
    let Some((dir, cursor)) = dir else { return };
    ctx.set_cursor_icon(cursor);
    if ctx.input(|i| i.pointer.primary_pressed()) {
        ctx.send_viewport_cmd(egui::ViewportCommand::BeginResize(dir));
    }
}

fn tab(ui: &mut egui::Ui, t: &Tokens, name: &str, dirty: bool, active: bool, close: &mut Option<usize>, index: usize) -> egui::Response {
    let font = theme::regular(13.0);
    let label: String = if name.chars().count() > 28 { format!("{}…", name.chars().take(27).collect::<String>()) } else { name.to_string() };
    let text_w = ui.fonts_mut(|f| f.layout_no_wrap(label.clone(), font.clone(), t.text).size().x);
    let (rect, resp) = ui.allocate_exact_size(vec2(text_w + 64.0, 30.0), Sense::click());
    let a11y = if dirty { format!("{name} (edited)") } else { name.to_string() };
    resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, true, active, &a11y));
    let bg = if active {
        t.chrome
    } else if resp.hovered() {
        t.hover
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, CornerRadius { nw: 7, ne: 7, sw: 0, se: 0 }, bg);
    icons::paint(
        ui,
        Rect::from_min_size(rect.min + vec2(6.0, 7.0), vec2(16.0, 16.0)),
        "file-text",
        15.0,
        if active { t.accent } else { t.text_muted },
    );
    ui.painter().text(rect.min + vec2(28.0, rect.height() / 2.0), Align2::LEFT_CENTER, label, font, if active { t.text } else { t.text_muted });
    let x_rect = Rect::from_center_size(rect.right_center() - vec2(16.0, 0.0), vec2(20.0, 20.0));
    let x = ui.interact(x_rect, ui.id().with(("tabclose", index)), Sense::click());
    if x.hovered() {
        ui.painter().rect_filled(x_rect, CornerRadius::same(4), t.pressed);
    }
    // Unsaved changes: a dot where the close button sits, until the tab is hovered.
    if dirty && !resp.hovered() && !x.hovered() {
        ui.painter().circle_filled(x_rect.center(), 4.0, if active { t.text } else { t.text_muted });
    } else if active || resp.hovered() || x.hovered() {
        icons::paint(ui, x_rect, "x", 13.0, t.text_muted);
    }
    if x.clicked() {
        *close = Some(index);
    }
    resp.on_hover_text(if dirty { format!("{name} — unsaved changes") } else { name.to_string() })
}

fn main_menu(app: &mut PrintCraftApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let resp = widgets::ghost_button(ui, "ellipsis", "Menu");
    egui::Popup::menu(&resp).show(|ui| {
        ui.set_min_width(230.0);
        ui.menu_button("File", |ui| crate::commands::registry_menu(app, ui, "File"));
        ui.menu_button("Edit", |ui| crate::commands::registry_menu(app, ui, "Edit"));
        ui.menu_button("Pages", |ui| crate::commands::registry_menu(app, ui, "Pages"));
        ui.menu_button("View", |ui| {
            if let Some(i) = app.active {
                let v = &mut app.views[i];
                ui.label(egui::RichText::new("Zoom").color(t.text_faint).small());
                if widgets::menu_item(ui, "Actual size", "⌘1").clicked() {
                    v.set_zoom(1.0);
                }
                if widgets::menu_item(ui, "Zoom to page level", "⌘0").clicked() {
                    v.fit = Fit::Page;
                }
                if widgets::menu_item(ui, "Fit to width", "⌘2").clicked() {
                    v.fit = Fit::Width;
                }
                if widgets::menu_item(ui, "Fit to height", "").clicked() {
                    v.fit = Fit::Height;
                    v.goto = Some((v.current, 0.0));
                }
                if widgets::menu_item(ui, "Fit visible", "⌘3").clicked() {
                    ui.close();
                    app.execute("view.fit_visible");
                    return;
                }
                if widgets::menu_item(ui, "Zoom in", "⌘+").clicked() {
                    v.zoom_step(true);
                }
                if widgets::menu_item(ui, "Zoom out", "⌘−").clicked() {
                    v.zoom_step(false);
                }
                if widgets::menu_item(ui, "Rotate view clockwise", "⇧⌘+").clicked() {
                    v.rotate_view(true);
                }
                if widgets::menu_item(ui, "Rotate view counterclockwise", "⇧⌘−").clicked() {
                    v.rotate_view(false);
                }
                ui.separator();
                ui.label(egui::RichText::new("Page navigation").color(t.text_faint).small());
                if ui.add_enabled(!v.back.is_empty(), egui::Button::new("Previous view").shortcut_text("⌘[")).clicked() {
                    v.view_history(false);
                }
                if ui.add_enabled(!v.forward.is_empty(), egui::Button::new("Next view").shortcut_text("⌘]")).clicked() {
                    v.view_history(true);
                }
                ui.separator();
                ui.label(egui::RichText::new("Page display").color(t.text_faint).small());
                for (l, label) in
                    [(PageLayout::Continuous, "Continuous scrolling"), (PageLayout::TwoUp, "Two-page view"), (PageLayout::Single, "Single page")]
                {
                    if ui.radio(v.layout == l, label).clicked() {
                        v.layout = l;
                        v.goto = Some((v.current, 0.0));
                    }
                }
                if ui.add_enabled(v.layout == PageLayout::TwoUp, egui::Checkbox::new(&mut v.cover, "Show cover page in two-page view")).changed() {
                    v.goto = Some((v.current, 0.0));
                }
                ui.separator();
            }
            crate::commands::registry_menu(app, ui, "View");
            ui.menu_button("Display theme", |ui| {
                let ctx = ui.ctx().clone();
                if ui.radio(app.follow_system_theme, "Use system setting").clicked() {
                    app.follow_system_theme = true;
                }
                if ui.radio(!app.follow_system_theme && app.theme == ThemeKind::Light, "Light gray").clicked() {
                    app.follow_system_theme = false;
                    app.set_theme(&ctx, ThemeKind::Light);
                }
                if ui.radio(!app.follow_system_theme && app.theme == ThemeKind::Dark, "Dark gray").clicked() {
                    app.follow_system_theme = false;
                    app.set_theme(&ctx, ThemeKind::Dark);
                }
            });
            ui.menu_button("Side panels", |ui| {
                for (p, label) in [
                    (RightPanel::Comments, "Comments"),
                    (RightPanel::Bookmarks, "Bookmarks"),
                    (RightPanel::Pages, "Pages"),
                    (RightPanel::Fields, "Fields"),
                    (RightPanel::Layers, "Layers"),
                    (RightPanel::Attachments, "Attachments"),
                    (RightPanel::Signatures, "Signatures"),
                    (RightPanel::Accessibility, "Accessibility Checker"),
                    (RightPanel::Search, "Search"),
                    (RightPanel::Compare, "Compare"),
                ] {
                    if ui.radio(app.right == Some(p), label).clicked() {
                        app.right = Some(p);
                    }
                }
            });
        });
        ui.menu_button("Help", |ui| crate::commands::registry_menu(app, ui, "Help"));
    });
}

pub fn right_rail(app: &mut PrintCraftApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let Some((index, id)) = app.active_ids() else { return };
    let Some(doc) = app.session.get(id) else { return };
    let has_signatures = doc.is_signed();
    let has_check = app.a11y.report.as_ref().is_some_and(|(d, _)| *d == id);
    let (has_comments, has_outline, has_fields, has_layers, has_files) = (
        !doc.info.annotations.is_empty(),
        !doc.info.outline.is_empty(),
        !doc.info.fields.is_empty(),
        !doc.info.layers.is_empty(),
        !doc.info.attachments.is_empty(),
    );
    let page_count = doc.info.pages.len();
    let open_panel = app.right;
    let labels: Vec<String> = doc.info.pages.iter().map(|p| p.label.clone()).collect();
    egui::Panel::right("rail")
        .resizable(false)
        .exact_size(48.0)
        .frame(egui::Frame::NONE.fill(t.chrome).inner_margin(egui::Margin::symmetric(6, 8)).stroke(Stroke::new(1.0, t.divider)))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            let mut rail_button = |ui: &mut egui::Ui, panel: RightPanel, icon: &str, tip: &str, has: bool| {
                let selected = app.right == Some(panel);
                let r = icons::button(ui, icon, 34.0, selected, tip);
                if has && !selected {
                    let c = r.rect.right_top() + vec2(-8.0, 8.0);
                    ui.painter().circle_filled(c, 3.0, t.accent);
                }
                if r.clicked() {
                    app.right = if selected { None } else { Some(panel) };
                }
            };
            rail_button(ui, RightPanel::Comments, "message-square-text", "Comments", has_comments);
            rail_button(ui, RightPanel::Bookmarks, "bookmark", "Bookmarks", has_outline);
            rail_button(ui, RightPanel::Pages, "files", "Page thumbnails", false);
            // The rest only when the document has something to show (or the panel is open).
            for (panel, icon, tip, has) in [
                (RightPanel::Fields, "text-cursor-input", "Form fields", has_fields),
                (RightPanel::Layers, "layers", "Layers", has_layers),
                (RightPanel::Attachments, "paperclip", "Attachments", has_files),
                (RightPanel::Signatures, "signature", "Signatures", has_signatures),
            ] {
                if has || open_panel == Some(panel) {
                    rail_button(ui, panel, icon, tip, has);
                }
            }
            if has_check {
                rail_button(ui, RightPanel::Accessibility, "accessibility", "Accessibility Checker", false);
            }

            // Page navigation cluster at the bottom (as in Acrobat's rail).
            let view = &mut app.views[index];
            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                if icons::button(ui, "zoom-out", 32.0, false, "Zoom out (⌘−)").clicked() {
                    view.zoom_step(false);
                }
                if icons::button(ui, "zoom-in", 32.0, false, "Zoom in (⌘+)").clicked() {
                    view.zoom_step(true);
                }
                if icons::button(ui, "rotate-cw", 32.0, false, "Rotate view clockwise (⇧⌘+)").clicked() {
                    view.rotate_view(true);
                }
                let fit_icon = if view.fit == Fit::Width { "maximize" } else { "columns-2" };
                if icons::button(ui, fit_icon, 32.0, false, "Toggle fit page / fit width").clicked() {
                    view.fit = if view.fit == Fit::Width { Fit::Page } else { Fit::Width };
                    view.goto = Some((view.current, 0.0));
                }
                ui.label(egui::RichText::new(format!("{:.0}%", view.zoom * 100.0)).font(theme::regular(10.5)).color(t.text_faint));
                ui.add_space(6.0);
                if icons::button(ui, "chevron-down", 30.0, false, "Next page").clicked() {
                    view.go_to_page(view.current + 1);
                }
                if icons::button(ui, "chevron-up", 30.0, false, "Previous page").clicked() {
                    view.go_to_page(view.current.saturating_sub(1));
                }
                ui.label(egui::RichText::new(page_count.to_string()).font(theme::regular(11.0)).color(t.text_muted));
                let edit = egui::TextEdit::singleline(&mut view.page_input)
                    .id(egui::Id::new("page-input"))
                    .desired_width(34.0)
                    .horizontal_align(Align::Center)
                    .font(theme::medium(12.0))
                    .margin(vec2(2.0, 4.0));
                let r = egui::Frame::NONE
                    .fill(t.field)
                    .stroke(Stroke::new(1.0, t.border))
                    .corner_radius(CornerRadius::same(5))
                    .show(ui, |ui| ui.add(edit))
                    .inner
                    .on_hover_text("Current page — type a page number or label (such as iv) and press Enter");
                if r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    // A page label first (logical page numbers, as Acrobat), then a number.
                    let typed = view.page_input.clone();
                    if !view.go_to_typed(&typed, &labels) {
                        view.page_input = (view.current + 1).to_string();
                    }
                }
            });
        });
}
