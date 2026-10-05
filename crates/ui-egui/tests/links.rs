//! External links: the app ships none of its own, and documents only open web and email links.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use printcraft_ui_egui::comments::CommentTool;
use printcraft_ui_egui::{Dialog, PrintCraftApp, QuickTool};

fn harness(setup: impl FnOnce(&mut PrintCraftApp) + 'static) -> Harness<'static, PrintCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PrintCraftApp::new();
        setup(&mut app);
        app
    });
    h.run_steps(4);
    h
}

#[test]
fn the_app_ships_no_external_links() {
    let h = harness(|_| {});
    for gone in ["Discord", "Join our Discord", "Join the ArtCraft community", "PrintCraft on GitHub", "ArtCraft website"] {
        assert_eq!(h.query_all_by_label(gone).count(), 0, "{gone}");
    }
    for id in ["help.discord", "help.app_page", "help.github", "help.website"] {
        assert!(printcraft_engine::commands::command(id).is_none(), "{id}");
    }
}

#[test]
fn about_dialog_has_no_links() {
    let pdf = b"%PDF-1.7\n1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj\n3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] >> endobj\ntrailer << /Root 1 0 R >>\n%%EOF";
    let h = harness(move |app| {
        app.open_bytes("one.pdf", None, pdf.to_vec()).unwrap();
        app.dialog = Some(Dialog::About);
    });
    assert_eq!(h.query_all_by_label("Join our Discord").count(), 0);
    assert_eq!(h.state().last_opened_url, None);
}

#[test]
fn new_drawing_opens_a_blank_page_with_the_pen() {
    let mut h = harness(|_| {});
    h.get_by_label("Blank Page").click();
    h.run_steps(3);
    let app = h.state();
    assert_eq!(app.views.len(), 1);
    assert_eq!(app.quick_tool, QuickTool::Comment(CommentTool::Ink));
    assert!(app.comment_prefs.pinned, "the pen stays selected between strokes");
}

#[test]
fn documents_only_open_web_and_email_links() {
    let mut app = PrintCraftApp::new();
    for bad in ["file:///C:/Windows/System32/calc.exe", "\\\\attacker.example\\share\\x.exe", "ms-msdt:/id PCWDiagnostic", "javascript:alert(1)"] {
        app.open_document_url(bad);
        assert_eq!(app.last_opened_url, None, "{bad}");
    }
    app.open_document_url("https://example.org/");
    assert_eq!(app.last_opened_url.as_deref(), Some("https://example.org/"));
}
