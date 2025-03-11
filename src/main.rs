mod missile;
mod explosion;
use missile::*;
use explosion::*;
use raylib::prelude::*;

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Missile Command")
        .build();

    let mut missiles: Vec<Missile> = Vec::with_capacity(100);
    let mut explosions: Vec<Explosion> = Vec::with_capacity(200);
    let launch_pos = Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT); // Bottom center

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let target = rl.get_mouse_position();
            missiles.push(Missile::new(launch_pos, target));
        }

        let delta_time = rl.get_frame_time();

        // Update missiles and create explosions when they reach target
        missiles.retain_mut(|m| {
            if !m.update(delta_time) {
                explosions.push(Explosion::new(m.position));
                return false; // Remove missile
            }
            true
        });

        // Update explosions and remove finished ones
        explosions.retain_mut(|e| e.update(delta_time));

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Draw missiles
        for missile in &missiles {
            missile.draw(&mut d);
        }

        // Draw explosions
        for explosion in &explosions {
            explosion.draw(&mut d);
        }
    }
}
