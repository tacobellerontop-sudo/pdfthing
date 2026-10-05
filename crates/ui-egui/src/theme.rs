//! Design tokens and egui style: warm paper chrome, one violet accent, generous rounding; the dark
//! theme keeps pages white. Every custom widget reads `Tokens::get`.

use std::sync::Arc;

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Stroke, Visuals};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum ThemeKind {
    #[default]
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    pub kind: ThemeKind,
    /// Title/tab strip.
    pub titlebar: Color32,
    /// Mode bar, panels.
    pub chrome: Color32,
    pub panel: Color32,
    /// Document area behind pages.
    pub pasteboard: Color32,
    pub card: Color32,
    pub border: Color32,
    pub divider: Color32,
    pub text: Color32,
    pub text_muted: Color32,
    pub text_faint: Color32,
    pub icon: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub selected: Color32,
    pub accent: Color32,
    pub accent_text: Color32,
    pub accent_soft: Color32,
    pub field: Color32,
    pub badge_new: Color32,
    pub page_shadow: Color32,
    pub radius: u8,
}

impl Tokens {
    pub fn for_kind(kind: ThemeKind) -> Self {
        match kind {
            // Warm paper and a violet ink: a sketchbook, not an office suite.
            ThemeKind::Light => Self {
                kind,
                titlebar: Color32::from_rgb(0xF3, 0xF0, 0xEA),
                chrome: Color32::from_rgb(0xFB, 0xFA, 0xF7),
                panel: Color32::from_rgb(0xFB, 0xFA, 0xF7),
                pasteboard: Color32::from_rgb(0xEC, 0xE8, 0xE0),
                card: Color32::from_rgb(0xFF, 0xFF, 0xFF),
                border: Color32::from_rgb(0xDD, 0xD7, 0xCC),
                divider: Color32::from_rgb(0xE6, 0xE1, 0xD8),
                text: Color32::from_rgb(0x24, 0x20, 0x2B),
                text_muted: Color32::from_rgb(0x66, 0x5F, 0x6E),
                text_faint: Color32::from_rgb(0x97, 0x90, 0x9C),
                icon: Color32::from_rgb(0x48, 0x42, 0x50),
                hover: Color32::from_rgb(0xEE, 0xEA, 0xF6),
                pressed: Color32::from_rgb(0xE2, 0xDC, 0xF0),
                selected: Color32::from_rgb(0xEB, 0xE5, 0xFF),
                accent: Color32::from_rgb(0x6C, 0x4B, 0xF0),
                accent_text: Color32::from_rgb(0x58, 0x38, 0xD6),
                accent_soft: Color32::from_rgb(0xEC, 0xE6, 0xFF),
                field: Color32::from_rgb(0xFF, 0xFF, 0xFF),
                badge_new: Color32::from_rgb(0xF0, 0x6B, 0x4B),
                page_shadow: Color32::from_black_alpha(30),
                radius: 9,
            },
            ThemeKind::Dark => Self {
                kind,
                titlebar: Color32::from_rgb(0x17, 0x15, 0x1D),
                chrome: Color32::from_rgb(0x1F, 0x1C, 0x27),
                panel: Color32::from_rgb(0x1F, 0x1C, 0x27),
                pasteboard: Color32::from_rgb(0x12, 0x10, 0x17),
                card: Color32::from_rgb(0x27, 0x23, 0x31),
                border: Color32::from_rgb(0x3A, 0x34, 0x47),
                divider: Color32::from_rgb(0x2F, 0x2A, 0x3A),
                text: Color32::from_rgb(0xEE, 0xEB, 0xF4),
                text_muted: Color32::from_rgb(0xB0, 0xA9, 0xBE),
                text_faint: Color32::from_rgb(0x82, 0x7B, 0x90),
                icon: Color32::from_rgb(0xD8, 0xD3, 0xE4),
                hover: Color32::from_rgb(0x2E, 0x29, 0x3B),
                pressed: Color32::from_rgb(0x39, 0x33, 0x4A),
                selected: Color32::from_rgb(0x36, 0x2B, 0x63),
                accent: Color32::from_rgb(0x8F, 0x74, 0xFF),
                accent_text: Color32::from_rgb(0xB9, 0xA8, 0xFF),
                accent_soft: Color32::from_rgb(0x33, 0x2A, 0x57),
                field: Color32::from_rgb(0x18, 0x16, 0x1F),
                badge_new: Color32::from_rgb(0xF0, 0x7A, 0x5C),
                page_shadow: Color32::from_black_alpha(120),
                radius: 9,
            },
        }
    }

    pub fn get(ctx: &egui::Context) -> Self {
        ctx.data(|d| d.get_temp::<Tokens>(egui::Id::new("printcraft-theme"))).unwrap_or_else(|| Self::for_kind(ThemeKind::Light))
    }

    pub fn dark(&self) -> bool {
        self.kind == ThemeKind::Dark
    }
}

pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    let add = |fonts: &mut FontDefinitions, name: &str, bytes: &'static [u8]| {
        fonts.font_data.insert(name.to_owned(), Arc::new(FontData::from_static(bytes)));
    };
    add(&mut fonts, "Inter", include_bytes!("../../../assets/fonts/Inter-Regular.ttf"));
    add(&mut fonts, "Inter-Medium", include_bytes!("../../../assets/fonts/Inter-Medium.ttf"));
    add(&mut fonts, "Inter-SemiBold", include_bytes!("../../../assets/fonts/Inter-SemiBold.ttf"));
    add(&mut fonts, "JetBrainsMono", include_bytes!("../../../assets/fonts/JetBrainsMono-Regular.ttf"));
    fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "Inter".to_owned());
    fonts.families.entry(FontFamily::Monospace).or_default().insert(0, "JetBrainsMono".to_owned());
    let fallback: Vec<String> = fonts.families[&FontFamily::Proportional].clone();
    for (fam, primary) in [("medium", "Inter-Medium"), ("semibold", "Inter-SemiBold")] {
        let mut stack = vec![primary.to_owned()];
        stack.extend(fallback.iter().cloned());
        fonts.families.insert(FontFamily::Name(fam.into()), stack);
    }
    ctx.set_fonts(fonts);
}

pub fn regular(size: f32) -> FontId {
    FontId::proportional(size)
}
pub fn medium(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("medium".into()))
}
pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("semibold".into()))
}

pub fn apply(ctx: &egui::Context, kind: ThemeKind) {
    let t = Tokens::for_kind(kind);
    ctx.data_mut(|d| d.insert_temp(egui::Id::new("printcraft-theme"), t));
    let mut v = if t.dark() { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.panel;
    v.window_fill = t.card;
    v.window_stroke = Stroke::new(1.0, t.border);
    v.extreme_bg_color = t.field;
    v.faint_bg_color = t.hover;
    v.selection.bg_fill = t.accent_soft;
    v.selection.stroke = Stroke::new(1.0, t.accent);
    v.hyperlink_color = t.accent_text;
    v.override_text_color = Some(t.text);
    v.window_corner_radius = CornerRadius::same(14);
    v.menu_corner_radius = CornerRadius::same(10);
    v.window_shadow = egui::Shadow { offset: [0, 8], blur: 28, spread: 0, color: Color32::from_black_alpha(if t.dark() { 110 } else { 38 }) };
    v.popup_shadow = egui::Shadow { offset: [0, 4], blur: 16, spread: 0, color: Color32::from_black_alpha(if t.dark() { 90 } else { 30 }) };
    for w in [&mut v.widgets.noninteractive, &mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active, &mut v.widgets.open] {
        w.corner_radius = CornerRadius::same(t.radius);
        w.fg_stroke.color = t.text;
    }
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, t.divider);
    v.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    v.widgets.inactive.bg_fill = t.field;
    v.widgets.inactive.bg_stroke = Stroke::NONE;
    v.widgets.hovered.weak_bg_fill = t.hover;
    v.widgets.hovered.bg_fill = t.hover;
    v.widgets.hovered.bg_stroke = Stroke::NONE;
    v.widgets.active.weak_bg_fill = t.pressed;
    v.widgets.active.bg_fill = t.pressed;
    v.widgets.open.weak_bg_fill = t.hover;
    ctx.set_visuals(v);
    ctx.global_style_mut(|s| {
        s.spacing.item_spacing = egui::vec2(8.0, 6.0);
        s.spacing.button_padding = egui::vec2(10.0, 5.0);
        s.spacing.menu_margin = egui::Margin::same(6);
        s.spacing.scroll.bar_width = 8.0;
        s.spacing.scroll.floating = true;
        s.text_styles.insert(egui::TextStyle::Body, regular(13.0));
        s.text_styles.insert(egui::TextStyle::Button, regular(13.0));
        s.text_styles.insert(egui::TextStyle::Small, regular(11.0));
        s.text_styles.insert(egui::TextStyle::Heading, semibold(17.0));
        s.interaction.tooltip_delay = 0.35;
    });
}
