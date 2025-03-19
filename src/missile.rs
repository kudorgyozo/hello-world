use raylib::prelude::*;

pub const MISSILE_SPEED: f32 = 450.0;
pub const MISSILE_SPEED_ENEMY: f32 = 50.0;
pub const TARGET_RADIUS: f32 = 6.0;

pub struct Missile {
    pub position: Vector2,
    velocity: Vector2,
    target: Vector2,
    start_position: Vector2, // Store the initial launch position
}

impl Missile {
    pub fn new(start: Vector2, target: Vector2, enemy: bool) -> Self {
        let direction = (target - start).normalized()
            * if !enemy {
                MISSILE_SPEED
            } else {
                MISSILE_SPEED_ENEMY
            };
        Self {
            position: start,
            velocity: direction,
            target,
            start_position: start, // Save the starting position
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        self.position += self.velocity * delta_time;
        (self.position - self.target).length() >= TARGET_RADIUS
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        // Draw a line from start to the current position
        d.draw_line_v(self.start_position, self.position, Color::WHITE);

        d.draw_circle_v(self.position, 3.0, Color::RED);
    }
}
