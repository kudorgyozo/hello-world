mod missile;
mod explosion;
use missile::*;
use explosion::*;
use raylib::prelude::*;
use rand::Rng;

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
pub const ENEMY_SPAWN_INTERVAL: f32 = 2.0; // Spawn enemy every 1.5 seconds

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Missile Command")
        .build();

    let launch_pos = Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT);
    let mut player_missiles: Vec<Missile> = Vec::new();
    let mut enemy_missiles: Vec<Missile> = Vec::new();
    let mut explosions: Vec<Explosion> = Vec::new();
    let mut enemy_spawn_timer = 0.0;

    while !rl.window_should_close() {


        let delta_time = rl.get_frame_time();
        enemy_spawn_timer += delta_time;

        // Player launches missile on click
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let target = rl.get_mouse_position();
            player_missiles.push(Missile::new(launch_pos, target, false));
        }

        // Update player missiles
        player_missiles.retain_mut(|m| {
            if !m.update(delta_time) {
                explosions.push(Explosion::new(m.position));
                false
            } else {
                true
            }
        });

        // Spawn enemy missiles at intervals
        if enemy_spawn_timer >= ENEMY_SPAWN_INTERVAL {
            enemy_spawn_timer = 0.0;
            let mut rng = rand::rng();
            let x = rng.random_range(50.0..SCREEN_WIDTH - 50.0); // Random X at top
            let target_x = rng.random_range(100.0..SCREEN_WIDTH - 100.0); // Random landing zone
            let enemy_start = Vector2::new(x, 0.0);
            let enemy_target = Vector2::new(target_x, SCREEN_HEIGHT);
            enemy_missiles.push(Missile::new(enemy_start, enemy_target, true));
        }

        enemy_missiles.retain_mut(|m| {
            // Check collision with explosions
            let hit = explosions.iter().any(|e| {
                (m.position - e.position).length() < e.radius
            });

            if hit {
                false // Remove the enemy missile
            } else if !m.update(delta_time) {
                false
            } else {
                true
            }
        });

        // Update explosions
        explosions.retain_mut(|e| e.update(delta_time));

        // Drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Draw missiles & explosions
        for missile in &player_missiles {
            missile.draw(&mut d);
        }
        for missile in &enemy_missiles {
            missile.draw(&mut d);
        }
        for explosion in &explosions {
            explosion.draw(&mut d);
        }


    }
}