// anim.rs — Animation math

use std::time::Instant;

pub fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn spring_out(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t >= 1.0 { return 1.0; }
    let p = 0.3_f32;
    let s = p / 4.0;
    (2.0_f32).powf(-10.0 * t)
        * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin()
        * -1.0
        + 1.0
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

#[derive(Clone)]
pub struct Tween {
    pub start:    Option<Instant>,
    pub duration: f32,
    pub forward:  bool,
    pub value:    f32,
}

impl Tween {
    pub fn new(duration: f32) -> Self {
        Self { start: None, duration, forward: false, value: 0.0 }
    }

    pub fn play_forward(&mut self) {
        if self.value >= 1.0 && self.forward && self.start.is_none() { return; }
        self.forward = true;
        let done = self.value * self.duration;
        self.start = Some(Instant::now()
            - std::time::Duration::from_secs_f32(done));
    }

    pub fn play_backward(&mut self) {
        if self.value <= 0.0 && !self.forward && self.start.is_none() { return; }
        self.forward = false;
        let done = (1.0 - self.value) * self.duration;
        self.start = Some(Instant::now()
            - std::time::Duration::from_secs_f32(done));
    }

    pub fn tick(&mut self) {
        let Some(start) = self.start else { return };
        let t = (start.elapsed().as_secs_f32() / self.duration).clamp(0.0, 1.0);
        self.value = if self.forward { t } else { 1.0 - t };
        if t >= 1.0 { self.start = None; }
    }

    pub fn eased(&self)     -> f32  { smoothstep(self.value) }
    pub fn spring(&self)    -> f32  { spring_out(self.value) }
    pub fn is_done(&self)   -> bool { self.start.is_none() }
    pub fn is_active(&self) -> bool { self.value > 0.001 }
}