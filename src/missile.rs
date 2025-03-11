use raylib::prelude::*;

pub const MISSILE_SPEED:f32 = 10.0;

struct Missile {
    position: Vector2,
    velocity: Vector2,
    active: bool,
}

impl Missile {
    fn new(start: Vector2, target: Vector2) -> Self {
        let direction = (target - start).normalize(); // Get unit direction
        Self {
            position: start,
            velocity: direction * speed,
            active: true,
        }
    }

    fn update(&mut self) {
        if self.active {
            self.position += self.velocity;
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.active {
            d.draw_circle_v(self.position, 5.0, Color::RED);
        }
    }
}