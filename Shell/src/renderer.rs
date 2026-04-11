// renderer.rs — All drawing via tiny-skia

use tiny_skia::*;
use crate::ui::UiState;
use crate::tabs::TabManager;

const BG:         u32 = 0xFF0C0C0E;
const PANEL:      u32 = 0xFF111115;
const TAB_BG:     u32 = 0xFF1A1A20;
const TAB_ACTIVE: u32 = 0xFF242430;
const ACCENT:     u32 = 0xFFC8F04E;
const TEXT:       u32 = 0xFFE8E8EC;
const TEXT_DIM:   u32 = 0xFF55555F;
const ISLAND_BG:  u32 = 0xFF131318;
const ISLAND_BDR: u32 = 0xFF2A2A35;

fn hex(c: u32) -> Color {
    Color::from_rgba8(
        ((c >> 16) & 0xFF) as u8,
        ((c >>  8) & 0xFF) as u8,
        ( c        & 0xFF) as u8,
        ((c >> 24) & 0xFF) as u8,
    )
}

fn paint_col(color: u32, alpha: f32) -> Paint<'static> {
    let c = hex(color);
    let mut p = Paint::default();
    p.set_color(Color::from_rgba(
        c.red(), c.green(), c.blue(),
        (alpha * c.alpha()).clamp(0.0, 1.0)
    ).unwrap_or(c));
    p.anti_alias = true;
    p
}

fn rrect(x: f32, y: f32, w: f32, h: f32, r: f32) -> Option<Path> {
    if w <= 0.0 || h <= 0.0 { return None; }
    let r = r.min(w / 2.0).min(h / 2.0);
    let mut pb = PathBuilder::new();
    pb.move_to(x + r, y);
    pb.line_to(x + w - r, y);
    pb.quad_to(x + w, y,     x + w, y + r);
    pb.line_to(x + w, y + h - r);
    pb.quad_to(x + w, y + h, x + w - r, y + h);
    pb.line_to(x + r, y + h);
    pb.quad_to(x,     y + h, x,     y + h - r);
    pb.line_to(x,     y + r);
    pb.quad_to(x,     y,     x + r, y);
    pb.close();
    pb.finish()
}

fn fill(pixmap: &mut Pixmap, path: Option<Path>, color: u32, alpha: f32) {
    if let Some(p) = path {
        pixmap.fill_path(&p, &paint_col(color, alpha),
                         FillRule::Winding, Transform::identity(), None);
    }
}

fn stroke_path(pixmap: &mut Pixmap, path: Option<Path>, color: u32, alpha: f32, width: f32) {
    if let Some(p) = path {
        let mut s = Stroke::default();
        s.width = width;
        pixmap.stroke_path(&p, &paint_col(color, alpha), &s,
                           Transform::identity(), None);
    }
}

pub struct Renderer {
    pub width:  u32,
    pub height: u32,
    font: fontdue::Font,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        let font = fontdue::Font::from_bytes(
            include_bytes!("../assets/Inter_18pt-Medium.ttf") as &[u8],
            fontdue::FontSettings::default(),
        ).expect("Font load failed");
        Self { width, height, font }
    }

    pub fn draw(&self, pixmap: &mut Pixmap, ui: &UiState, tabs: &TabManager) {
        pixmap.fill(hex(BG));
        self.draw_page(pixmap, tabs);
        self.draw_settings_btn(pixmap, ui);

        let strip_a = ui.tab_strip.tween.eased();
        if strip_a > 0.001 { self.draw_tab_strip(pixmap, tabs, strip_a); }

        let island_a = ui.island.tween.eased();
        if island_a > 0.001 { self.draw_island(pixmap, ui, island_a); }

        let settings_a = ui.settings.tween.eased();
        if settings_a > 0.001 { self.draw_settings(pixmap, ui, settings_a); }
    }

    fn draw_page(&self, pixmap: &mut Pixmap, tabs: &TabManager) {
        let cx = self.width  as f32 / 2.0;
        let cy = self.height as f32 / 2.0;
        if let Some(tab) = tabs.active_tab() {
            self.text_center(pixmap, &tab.title, cx, cy - 8.0,  22.0, TEXT,     1.0);
            self.text_center(pixmap, &tab.url,   cx, cy + 22.0, 11.0, TEXT_DIM, 0.6);
        }
        self.text_center(
            pixmap,
            "Middle click to search   |   Ctrl + Scroll for tabs",
            cx, self.height as f32 - 28.0, 10.0, TEXT_DIM, 0.3,
        );
    }

    fn draw_tab_strip(&self, pixmap: &mut Pixmap, tabs: &TabManager, alpha: f32) {
        let pw: f32 = 255.0;
        let h = self.height as f32;

        fill(pixmap, rrect(0.0, 0.0, pw, h, 0.0), PANEL, alpha * 0.97);

        let mut pb = PathBuilder::new();
        pb.move_to(pw, 0.0); pb.line_to(pw, h);
        stroke_path(pixmap, pb.finish(), 0xFF1E1E28, alpha, 1.0);

        self.text(pixmap, "TABS", 18.0, 30.0, 10.0, ACCENT, alpha);
        self.text(pixmap, &format!("{} open", tabs.tabs.len()),
                  120.0, 30.0, 10.0, TEXT_DIM, alpha * 0.55);

        let item_h = 44.0_f32;
        let gap    =  4.0_f32;
        let pad    = 10.0_f32;
        let start  = 46.0_f32;

        for (i, tab) in tabs.tabs.iter().enumerate() {
            let tab_a  = tabs.tab_opacity(i) * alpha;
            if tab_a < 0.02 { continue; }
            let y      = start + i as f32 * (item_h + gap);
            let active = i == tabs.active;
            let bg     = if active { TAB_ACTIVE } else { TAB_BG };
            let tw     = pw - pad * 2.0;

            fill(pixmap, rrect(pad, y, tw, item_h, 8.0), bg, tab_a);

            if active {
                fill(pixmap,
                     rrect(pad, y + item_h * 0.2, 3.0, item_h * 0.6, 2.0),
                     ACCENT, tab_a);
            }

            let tc = if active { TEXT } else { TEXT_DIM };
            self.text(pixmap, &tab.title,
                      pad + 14.0, y + item_h * 0.5 + 5.0, 11.0, tc, tab_a);
        }
    }

    fn draw_island(&self, pixmap: &mut Pixmap, ui: &UiState, alpha: f32) {
        let w = self.width  as f32;
        let h = self.height as f32;

        fill(pixmap, rrect(0.0, 0.0, w, h, 0.0), 0xFF000000, alpha * 0.50);

        let scale = ui.island.tween.spring();
        let iw    = 580.0 * scale;
        let ih    =  58.0 * scale;
        let ix    = (w - iw) / 2.0;
        let iy    = (h - ih) / 2.0 - 60.0;

        fill(pixmap,       rrect(ix - 2.0, iy + 6.0, iw + 4.0, ih + 4.0, 14.0),
             0xFF000000, 0.30 * alpha);
        fill(pixmap,       rrect(ix, iy, iw, ih, 12.0), ISLAND_BG,  alpha);
        stroke_path(pixmap, rrect(ix, iy, iw, ih, 12.0), ISLAND_BDR, alpha, 1.0);

        let tx = ix + 42.0;
        let ty = iy + ih * 0.63;

        if ui.island.input.is_empty() {
            self.text(pixmap, "Search or enter address...",
                      tx, ty, 14.0, 0xFF333348, alpha);
        } else {
            self.text(pixmap, &ui.island.input, tx, ty, 14.0, TEXT, alpha);
        }

        // Cursor
        if ui.island.cursor_on {
            let cx = if ui.island.input.is_empty() {
                tx
            } else {
                tx + self.text_width(&ui.island.input, 14.0)
            };
            fill(pixmap,
                 rrect(cx + 1.0, iy + ih * 0.22, 2.0, ih * 0.56, 1.0),
                 ACCENT, alpha * 0.9);
        }

        self.text(pixmap, "esc",
                  ix + iw - 34.0, ty, 9.0, TEXT_DIM, alpha * 0.35);
    }

    fn draw_settings(&self, pixmap: &mut Pixmap, ui: &UiState, alpha: f32) {
        let h     = self.height as f32;
        let pw    = 290.0_f32;
        let ph    = 380.0_f32;
        let px    =  16.0_f32;
        let slide = ui.settings.tween.eased();
        let py    = h - 52.0 - ph * slide;

        fill(pixmap,
             rrect(px - 3.0, py + 8.0, pw + 6.0, ph, 18.0),
             0xFF000000, 0.35 * alpha);
        fill(pixmap,        rrect(px, py, pw, ph, 14.0), PANEL,      alpha);
        stroke_path(pixmap, rrect(px, py, pw, ph, 14.0), ISLAND_BDR, alpha, 1.0);

        self.text(pixmap, "Settings",   px + 20.0, py + 34.0,  14.0, TEXT,     alpha);
        self.text(pixmap, "General",    px + 20.0, py + 72.0,  12.0, TEXT_DIM, alpha);
        self.text(pixmap, "Privacy",    px + 20.0, py + 100.0, 12.0, TEXT_DIM, alpha);
        self.text(pixmap, "Appearance", px + 20.0, py + 128.0, 12.0, TEXT_DIM, alpha);
        self.text(pixmap, "Nova  v0.1.0-dev",
                  px + 20.0, py + ph - 16.0, 9.0, TEXT_DIM, alpha * 0.35);

        let mut pb = PathBuilder::new();
        pb.move_to(px + 10.0, py + 50.0);
        pb.line_to(px + pw - 10.0, py + 50.0);
        stroke_path(pixmap, pb.finish(), 0xFF1E1E28, alpha, 1.0);
    }

    fn draw_settings_btn(&self, pixmap: &mut Pixmap, ui: &UiState) {
        let h     = self.height as f32;
        let bx    = 14.0_f32;
        let by    = h - 42.0;
        let bs    = 28.0_f32;
        let open  = ui.settings.is_open();
        let hover = ui.settings_btn_hovered;
        let alpha = if hover || open { 0.9 } else { 0.28 };
        let bg    = if hover || open { TAB_ACTIVE } else { TAB_BG };

        fill(pixmap,        rrect(bx, by, bs, bs, 7.0), bg,         alpha);
        stroke_path(pixmap, rrect(bx, by, bs, bs, 7.0), ISLAND_BDR, alpha * 0.5, 1.0);

        let cx = bx + bs / 2.0;
        let cy = by + bs / 2.0;

        // Gear teeth
        for i in 0..6 {
            let angle = i as f32 * std::f32::consts::TAU / 6.0;
            let x1 = cx + 4.5 * angle.cos();
            let y1 = cy + 4.5 * angle.sin();
            let x2 = cx + 7.5 * angle.cos();
            let y2 = cy + 7.5 * angle.sin();
            let mut pb = PathBuilder::new();
            pb.move_to(x1, y1);
            pb.line_to(x2, y2);
            stroke_path(pixmap, pb.finish(), TEXT, alpha, 2.5);
        }

        // Gear center
        let mut pb = PathBuilder::new();
        pb.push_circle(cx, cy, 3.5);
        if let Some(p) = pb.finish() {
            fill(pixmap, Some(p), TEXT, alpha);
        }
    }

    // ── Font rendering ────────────────────────────────────────────────────────
    fn text(&self, pixmap: &mut Pixmap,
            s: &str, x: f32, y: f32,
            size: f32, color: u32, alpha: f32) {
        let r = ((color >> 16) & 0xFF) as f32 / 255.0;
        let g = ((color >>  8) & 0xFF) as f32 / 255.0;
        let b = ( color        & 0xFF) as f32 / 255.0;
        let pw = pixmap.width();
        let ph = pixmap.height();
        let mut cx = x;

        for ch in s.chars() {
            let (metrics, bitmap) = self.font.rasterize(ch, size);
            if bitmap.is_empty() { cx += metrics.advance_width; continue; }

            for row in 0..metrics.height {
                for col in 0..metrics.width {
                    let cov = bitmap[row * metrics.width + col] as f32 / 255.0;
                    if cov < 0.01 { continue; }

                    let fx = cx as i32 + col as i32 + metrics.xmin;
                    let fy = y  as i32 - metrics.height as i32
                                       + row  as i32
                                       - metrics.ymin;
                    if fx < 0 || fy < 0 { continue; }
                    let fx = fx as u32;
                    let fy = fy as u32;
                    if fx >= pw || fy >= ph { continue; }

                    let idx = (fy * pw + fx) as usize * 4;
                    let buf = pixmap.data_mut();
                    if idx + 3 >= buf.len() { continue; }

                    let a   = (cov * alpha).clamp(0.0, 1.0);
                    let inv = 1.0 - a;
                    // tiny-skia RGBA: R=idx, G=idx+1, B=idx+2, A=idx+3
                    buf[idx]     = (r * 255.0 * a + buf[idx]     as f32 * inv) as u8;
                    buf[idx + 1] = (g * 255.0 * a + buf[idx + 1] as f32 * inv) as u8;
                    buf[idx + 2] = (b * 255.0 * a + buf[idx + 2] as f32 * inv) as u8;
                    buf[idx + 3] = 255;
                }
            }
            cx += metrics.advance_width;
        }
    }

    fn text_center(&self, pixmap: &mut Pixmap,
                   s: &str, cx: f32, y: f32,
                   size: f32, color: u32, alpha: f32) {
        self.text(pixmap, s, cx - self.text_width(s, size) / 2.0, y, size, color, alpha);
    }

    pub fn text_width(&self, s: &str, size: f32) -> f32 {
        s.chars().map(|c| self.font.rasterize(c, size).0.advance_width).sum()
    }
}