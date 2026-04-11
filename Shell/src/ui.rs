// ui.rs — UI overlay state
 
use crate::anim::Tween;
use std::time::Instant;
 
// ── Settings Panel ────────────────────────────────────────────────────────────
pub struct Settings {
    pub tween:       Tween,
    pub active_tab:  SettingsSection,
}
 
#[derive(Clone, Copy, PartialEq)]
pub enum SettingsSection { General, Privacy, Appearance }
 
impl Settings {
    pub fn new() -> Self {
        Self { tween: Tween::new(0.25), active_tab: SettingsSection::General }
    }
    pub fn is_open(&self) -> bool { self.tween.is_active() }
}
 
// ── Search Island ─────────────────────────────────────────────────────────────
pub struct Island {
    pub tween:       Tween,
    pub input:       String,
    pub cursor_on:   bool,
    pub cursor_tick: Instant,
}
 
impl Island {
    pub fn new() -> Self {
        Self {
            tween:       Tween::new(0.18),
            input:       String::new(),
            cursor_on:   true,
            cursor_tick: Instant::now(),
        }
    }
 
    pub fn is_open(&self) -> bool { self.tween.is_active() }
 
    pub fn open(&mut self) {
        self.input.clear();
        self.cursor_on   = true;
        self.cursor_tick = Instant::now();
        self.tween.play_forward();
    }
 
    pub fn close(&mut self) {
        self.tween.play_backward();
    }
 
    pub fn type_char(&mut self, c: char) {
        self.input.push(c);
    }
 
    pub fn backspace(&mut self) {
        self.input.pop();
    }
 
    pub fn tick_cursor(&mut self) {
        if self.cursor_tick.elapsed().as_millis() > 530 {
            self.cursor_on   = !self.cursor_on;
            self.cursor_tick = Instant::now();
        }
    }
}
 
// ── Tab Strip ─────────────────────────────────────────────────────────────────
pub struct TabStrip {
    pub tween:            Tween,
    pub ctrl_held:        bool,
    pub hide_after:       Option<Instant>,
}
 
impl TabStrip {
    pub fn new() -> Self {
        Self {
            tween:      Tween::new(0.20),
            ctrl_held:  false,
            hide_after: None,
        }
    }
 
    pub fn is_visible(&self) -> bool { self.tween.is_active() }
 
    pub fn on_ctrl_pressed(&mut self) {
        self.ctrl_held  = true;
        self.hide_after = None;
        self.tween.play_forward();
    }
 
    pub fn on_ctrl_released(&mut self) {
        self.ctrl_held  = false;
        self.hide_after = Some(Instant::now());
    }
 
    pub fn tick(&mut self) {
        if let Some(t) = self.hide_after {
            if t.elapsed().as_millis() > 600 {
                self.hide_after = None;
                self.tween.play_backward();
            }
        }
    }
}
 
// ── Master UI State ───────────────────────────────────────────────────────────
pub struct UiState {
    pub tab_strip:    TabStrip,
    pub island:       Island,
    pub settings:     Settings,
    pub win_w:        u32,
    pub win_h:        u32,
    pub settings_btn_hovered: bool,
}
 
impl UiState {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            tab_strip:   TabStrip::new(),
            island:      Island::new(),
            settings:    Settings::new(),
            win_w:       w,
            win_h:       h,
            settings_btn_hovered: false,
        }
    }
 
    pub fn tick(&mut self) {
        self.tab_strip.tween.tick();
        self.tab_strip.tick();
        self.island.tween.tick();
        self.island.tick_cursor();
        self.settings.tween.tick();
    }
 
    pub fn needs_redraw(&self) -> bool {
        !self.tab_strip.tween.is_done() ||
        !self.island.tween.is_done()    ||
        !self.settings.tween.is_done()  ||
        self.island.is_open()            // cursor blink
    }
}