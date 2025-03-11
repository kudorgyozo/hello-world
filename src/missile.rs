use raylib::prelude::*;

pub const MISSILE_SPEED: f32 = 320.0;
pub const TARGET_RADIUS: f32 = 5.0;

pub struct Missile {
    pub position: Vector2,
    velocity: Vector2,
    target: Vector2,
}

impl Missile {
    pub fn new(start: Vector2, target: Vector2) -> Self {
        let direction = (target - start).normalized() * MISSILE_SPEED;
        Self {
            position: start,
            velocity: direction,
            target,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        self.position += self.velocity * delta_time;
        (self.position - self.target).length() >= TARGET_RADIUS
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.position, 5.0, Color::RED);
    }
}
