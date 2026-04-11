// main.rs — Nova Browser Shell
 
mod anim;
mod tabs;
mod ui;
mod input;
mod renderer;
 
use std::num::NonZeroU32;
use std::sync::Arc;
 
use winit::application::ApplicationHandler;
use winit::event::{
    ElementState, MouseButton, MouseScrollDelta, WindowEvent,
};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey, ModifiersState};
use winit::window::{Window, WindowId};
 
use softbuffer::{Context, Surface};
use tiny_skia::Pixmap;
 
use tabs::TabManager;
use ui::UiState;
use input::Action;
use renderer::Renderer;
 
// ── App ───────────────────────────────────────────────────────────────────────
 
struct Nova {
    window:    Option<Arc<Window>>,
    surface:   Option<Surface<Arc<Window>, Arc<Window>>>,
    context:   Option<Context<Arc<Window>>>,
    pixmap:    Option<Pixmap>,
    renderer:  Option<Renderer>,
    tabs:      TabManager,
    ui:        UiState,
    modifiers: ModifiersState,
    mouse_x:   f32,
    mouse_y:   f32,
}
 
impl Nova {
    fn new() -> Self {
        Self {
            window:   None,
            surface:  None,
            context:  None,
            pixmap:   None,
            renderer: None,
            tabs:     TabManager::new(),
            ui:       UiState::new(1280, 800),
            modifiers: ModifiersState::empty(),
            mouse_x:  0.0,
            mouse_y:  0.0,
        }
    }
 
    fn handle(&mut self, action: Action) {
        match action {
            Action::CtrlHeld           => self.ui.tab_strip.on_ctrl_pressed(),
            Action::CtrlReleased       => self.ui.tab_strip.on_ctrl_released(),
            Action::ScrollTabs(d)      => self.tabs.scroll(d),
            Action::OpenIsland         => self.ui.island.open(),
            Action::CloseIsland        => self.ui.island.close(),
            Action::IslandType(c)      => self.ui.island.type_char(c),
            Action::IslandBackspace    => self.ui.island.backspace(),
            Action::IslandSubmit       => {
                let url = TabManager::resolve(&self.ui.island.input.clone());
                // TODO: navigate active tab to url
                self.ui.island.close();
            }
            Action::NewTab             => self.tabs.new_tab(),
            Action::CloseTab(i)        => self.tabs.close(i),
            Action::SwitchTab(i)       => self.tabs.active = i,
            Action::ToggleSettings     => {
                if self.ui.settings.is_open() {
                    self.ui.settings.tween.play_backward();
                } else {
                    self.ui.settings.tween.play_forward();
                }
            }
            Action::Resize(w, h)       => {
                self.ui.win_w = w;
                self.ui.win_h = h;
                self.pixmap   = Pixmap::new(w, h);
                self.renderer = Some(Renderer::new(w, h));
            }
            Action::Quit               => {}
        }
    }
 
    fn settings_btn_rect(&self) -> (f32, f32, f32, f32) {
        let h = self.ui.win_h as f32;
        (16.0, h - 40.0, 28.0, 28.0)
    }
 
    fn render(&mut self) {
        let Some(pixmap)   = self.pixmap.as_mut()   else { return };
        let Some(renderer) = self.renderer.as_ref()  else { return };
        let Some(surface)  = self.surface.as_mut()   else { return };
        let Some(window)   = self.window.as_ref()    else { return };
 
        renderer.draw(pixmap, &self.ui, &self.tabs);
 
        let size = window.inner_size();
        let w    = size.width;
        let h    = size.height;
        if w == 0 || h == 0 { return; }
 
        if let (Ok(mut buf), Some(_nzw), Some(_nzh)) = (
            surface.buffer_mut(),
            NonZeroU32::new(w),
            NonZeroU32::new(h),
        ) {
            let data = pixmap.data();
            for (i, pixel) in buf.iter_mut().enumerate() {
                let base = i * 4;
                if base + 3 >= data.len() { break; }
                // tiny-skia is RGBA, softbuffer wants 0xRRGGBB
                let r = data[base]     as u32;
                let g = data[base + 1] as u32;
                let b = data[base + 2] as u32;
                *pixel = (r << 16) | (g << 8) | b;
            }
            let _ = buf.present();
        }
    }
}
 
// ── winit handler ─────────────────────────────────────────────────────────────
 
impl ApplicationHandler for Nova {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() { return; }
 
        let attrs = Window::default_attributes()
            .with_title("Nova")
            .with_inner_size(winit::dpi::LogicalSize::new(1280u32, 800u32));
 
        let window = Arc::new(event_loop.create_window(attrs).unwrap());
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();
 
        let size     = window.inner_size();
        let w        = size.width.max(1);
        let h        = size.height.max(1);
        let pixmap   = Pixmap::new(w, h).unwrap();
        let renderer = Renderer::new(w, h);
 
        self.ui       = UiState::new(w, h);
        self.window   = Some(window);
        self.context  = Some(context);
        self.surface  = Some(surface);
        self.pixmap   = Some(pixmap);
        self.renderer = Some(renderer);
 
        event_loop.set_control_flow(ControlFlow::Poll);
    }
 
    fn window_event(&mut self, event_loop: &ActiveEventLoop,
                    _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
 
            WindowEvent::Resized(size) => {
                let w = size.width.max(1);
                let h = size.height.max(1);
                if let Some(surface) = &mut self.surface {
                    let _ = surface.resize(
                        NonZeroU32::new(w).unwrap(),
                        NonZeroU32::new(h).unwrap(),
                    );
                }
                self.handle(Action::Resize(w, h));
            }
 
            WindowEvent::ModifiersChanged(mods) => {
                let was = self.modifiers.control_key();
                self.modifiers = mods.state();
                let is  = self.modifiers.control_key();
                if !was && is  { self.handle(Action::CtrlHeld);     }
                if  was && !is { self.handle(Action::CtrlReleased); }
            }
 
            WindowEvent::MouseWheel { delta, .. } => {
                let lines = match delta {
                    MouseScrollDelta::LineDelta(_, y)  => y,
                    MouseScrollDelta::PixelDelta(p)    => p.y as f32 / 40.0,
                };
                if self.modifiers.control_key() {
                    self.handle(Action::ScrollTabs(lines));
                }
            }
 
            WindowEvent::MouseInput { button, state: ElementState::Pressed, .. } => {
                match button {
                    MouseButton::Middle => {
                        if self.ui.island.is_open() {
                            self.handle(Action::CloseIsland);
                        } else {
                            self.handle(Action::OpenIsland);
                        }
                    }
                    MouseButton::Left => {
                        let (bx, by, bw, bh) = self.settings_btn_rect();
                        let mx = self.mouse_x;
                        let my = self.mouse_y;
                        if mx >= bx && mx <= bx + bw && my >= by && my <= by + bh {
                            self.handle(Action::ToggleSettings);
                        }
                        // Close island if clicking outside it
                        if self.ui.island.is_open() {
                            self.handle(Action::CloseIsland);
                        }
                    }
                    _ => {}
                }
            }
 
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_x = position.x as f32;
                self.mouse_y = position.y as f32;
 
                // Hover detection for settings button
                let (bx, by, bw, bh) = self.settings_btn_rect();
                self.ui.settings_btn_hovered =
                    self.mouse_x >= bx && self.mouse_x <= bx + bw &&
                    self.mouse_y >= by && self.mouse_y <= by + bh;
            }
 
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                if key_event.state != ElementState::Pressed { return; }
                let PhysicalKey::Code(code) = key_event.physical_key else { return };
 
                if self.ui.island.is_open() {
                    match code {
                        KeyCode::Escape => self.handle(Action::CloseIsland),
                        KeyCode::Enter  => self.handle(Action::IslandSubmit),
                        KeyCode::Backspace => self.handle(Action::IslandBackspace),
                        _ => {
                            // Type characters
                            if let Some(text) = &key_event.text {
                                for c in text.chars() {
                                    if !c.is_control() {
                                        self.handle(Action::IslandType(c));
                                    }
                                }
                            }
                        }
                    }
                } else {
                    match code {
                        KeyCode::KeyT if self.modifiers.control_key() => {
                            self.handle(Action::NewTab);
                        }
                        KeyCode::KeyW if self.modifiers.control_key() => {
                            let idx = self.tabs.active;
                            self.handle(Action::CloseTab(idx));
                        }
                        _ => {}
                    }
                }
            }
 
            WindowEvent::RedrawRequested => {
                self.ui.tick();
                self.render();
                if let Some(w) = &self.window { w.request_redraw(); }
            }
 
            _ => {}
        }
    }
 
    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.ui.tick();
        if let Some(w) = &self.window { w.request_redraw(); }
    }
}
 
// ── Entry point ───────────────────────────────────────────────────────────────
 
fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app    = Nova::new();
    event_loop.run_app(&mut app).unwrap();
}