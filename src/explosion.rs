use raylib::prelude::*;

pub const EXPLOSION_DURATION: f32 = 2.5; // Total explosion duration
pub const EXPLOSION_RADIUS_MAX: f32 = 35.0;   // Max explosion size
pub const EXPLOSION_RADIUS_MIN: f32 = 5.0;   // Max explosion size

pub struct Explosion {
    pub position: Vector2,
    pub radius: f32,
    timer: f32,
}

impl Explosion {
    pub fn new(position: Vector2) -> Self {
        Self { position, timer: EXPLOSION_DURATION, radius: EXPLOSION_RADIUS_MIN }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {

        let progress = 1.0 - (self.timer / EXPLOSION_DURATION); // 0 → 1 over time

        if progress < 0.5 {
            // Expanding phase (first half)
            //                              (start, end, progress: 0->1)
            self.radius = raylib::math::lerp(EXPLOSION_RADIUS_MIN, EXPLOSION_RADIUS_MAX, progress * 2.0);
        } else {
            // Shrinking phase (second half)
            self.radius = raylib::math::lerp(EXPLOSION_RADIUS_MAX, EXPLOSION_RADIUS_MIN, (progress - 0.5) * 2.0);
        };

        self.timer -= delta_time;
        self.timer > 0.0 // Explosion is active while timer > 0
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {


        d.draw_circle_v(self.position, self.radius, Color::YELLOW);
    }
}