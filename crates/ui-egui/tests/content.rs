//! Edit a PDF ▸ Add content in the real shell (egui_kittest): type text onto a page, move it,
//! restyle it from the panel, add an image, delete with the keyboard.

use egui::Pos2;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use printcraft_ui_egui::{PrintCraftApp, QuickTool};

fn harness() -> Harness<'static, PrintCraftApp> {
    harness_with(Harness::builder())
}

fn harness_with(builder: egui_kittest::HarnessBuilder<PrintCraftApp>) -> Harness<'static, PrintCraftApp> {
    let mut h = builder.with_size(egui::vec2(1400.0, 900.0)).build_eframe(|_cc| {
        let mut app = PrintCraftApp::new();
        app.open_bytes("form.pdf", None, include_bytes!("data/form.pdf").to_vec()).unwrap();
        app.set_option("zoom", "150").unwrap();
        app
    });
    for _ in 0..60 {
        h.run_steps(2);
        if !h.state().render_pending() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    h
}

fn at(h: &Harness<'static, PrintCraftApp>, x: f32, y: f32) -> Pos2 {
    let r = h.state().views[0].page_screen_rect(0).expect("on screen");
    let k = r.width() / 300.0;
    r.min + egui::vec2(x * k, y * k)
}

fn click(h: &mut Harness<'static, PrintCraftApp>, p: Pos2) {
    h.hover_at(p);
    h.run_steps(1);
    h.drag_at(p);
    h.run_steps(1);
    h.drop_at(p);
    h.run_steps(3);
}

fn added(h: &Harness<'static, PrintCraftApp>) -> Vec<printcraft_engine::Added> {
    let s = h.state();
    s.session.get(s.views[0].id).unwrap().added.clone()
}

#[test]
fn typing_moving_styling_and_deleting_added_content() {
    let mut h = harness();
    assert!(h.state_mut().execute("edit.text"));
    h.run_steps(2);
    h.get_by_label("Edit a PDF");
    let p = at(&h, 40.0, 300.0);
    click(&mut h, p);
    assert!(h.state().views[0].content.draft.is_some(), "the editor opened");
    h.event(egui::Event::Text("Reviewed".into()));
    h.run_steps(2);
    // Click elsewhere on the page commits and goes back to Select.
    let p = at(&h, 250.0, 60.0);
    click(&mut h, p);
    let a = added(&h);
    assert_eq!(a.len(), 1);
    let printcraft_engine::AddedContent::Text(t) = &a[0].content else { panic!() };
    assert_eq!(t.text, "Reviewed");
    assert_eq!(h.state().quick_tool, QuickTool::Select);
    h.run_steps(2);
    assert_eq!(h.state().views[0].content.selected, Some((0, 0)), "the new text is selected");
    // Bold from the Format panel: one undoable change.
    h.get_by_label("B").click();
    h.run_steps(3);

    let printcraft_engine::AddedContent::Text(t) = &added(&h)[0].content else { panic!() };
    assert!(t.bold);
    {
        let s = h.state();
        assert_eq!(s.session.get(s.views[0].id).unwrap().can_undo(), Some("Edit content"));
    }
    // Delete.
    h.key_press(egui::Key::Delete);
    h.run_steps(3);
    assert!(added(&h).is_empty());
    // An image lands in the middle of the page, selected.
    let mut png = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut png, 30, 20);
        enc.set_color(png::ColorType::Rgb);
        let mut w = enc.write_header().unwrap();
        w.write_image_data(&[90u8; 1800]).unwrap();
    }
    h.state_mut().add_image("logo.png".into(), png);
    h.run_steps(3);
    let a = added(&h);
    assert_eq!(a.len(), 1);
    assert_eq!(a[0].content.rect(), [135.0, 190.0, 165.0, 210.0]);
    assert_eq!(h.state().views[0].content.selected, Some((0, 0)));
    // Edit image: rotate clockwise turns the box around its centre.
    h.get_by_label("Rotate clockwise").click();
    h.run_steps(3);
    let a = added(&h);
    let printcraft_engine::AddedContent::Image(img) = &a[0].content else { panic!() };
    assert_eq!((img.rotation, img.rect), (3, [140.0, 185.0, 160.0, 215.0]));
    h.get_by_label("Flip horizontal").click();
    h.run_steps(3);
    let printcraft_engine::AddedContent::Image(img) = &added(&h)[0].content else { panic!() };
    assert!(img.flip_h);
}

#[test]
fn added_text_is_edited_by_double_clicking_it_with_the_select_tool() {
    // 60 fps steps, so two clicks fall inside egui's double-click window.
    let mut h = harness_with(Harness::builder().with_step_dt(1.0 / 60.0));
    h.state_mut().set_option("left", "closed").unwrap();
    h.state_mut().set_option("quick", "add-text").unwrap();
    h.run_steps(2);
    let p = at(&h, 40.0, 300.0);
    click(&mut h, p);
    h.event(egui::Event::Text("Draft".into()));
    h.run_steps(2);
    let away = at(&h, 250.0, 60.0);
    click(&mut h, away);
    assert_eq!(added(&h).len(), 1);
    assert_eq!(h.state().quick_tool, QuickTool::Select);
    assert!(h.state().views[0].content.draft.is_none());
    // No editing panel is open: a double-click on the text with the Select tool edits it.
    let r = added(&h)[0].content.rect();
    // `rect` is in PDF user space (y up); `at` measures from the top.
    let s = h.state().views[0].page_screen_rect(0).unwrap();
    let page_h = s.height() / (s.width() / 300.0);
    let mid = at(&h, ((r[0] + r[2]) / 2.0) as f32, page_h - ((r[1] + r[3]) / 2.0) as f32);
    h.hover_at(mid);
    // Let the earlier clicks age out, or egui counts a triple click.
    h.run_steps(60);
    for _ in 0..2 {
        h.drag_at(mid);
        h.step();
        h.drop_at(mid);
        h.step();
    }
    h.run_steps(2);
    let draft = h.state().views[0].content.draft.clone();
    assert_eq!(draft.map(|d| (d.index, d.text)), Some((Some(0), "Draft".to_owned())));
}
