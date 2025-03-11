use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;
use raylib::prelude::*;

pub const PLAYER_WIDTH: i32 = 100;
pub const PLAYER_HEIGHT: i32 = 20;
pub const PLAYER_SPEED: i32 = 200;

pub const BALL_RADIUS: f32 = 10.0;
pub const BALL_SPEED: f32 = 200.0; // Initial speed

pub struct Player {
    pub pos: f32,
}

impl Player {
    pub fn update(&mut self, rl: &mut RaylibHandle) {
        let frame_time: f32 = rl.get_frame_time();
        // Handle player movement
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
            self.pos -= PLAYER_SPEED as f32 * frame_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
            self.pos += PLAYER_SPEED as f32 * frame_time;
        }

        // Keep player within screen bounds
        if self.pos < (PLAYER_WIDTH / 2) as f32 {
            self.pos = (PLAYER_WIDTH / 2) as f32;
        }

        if self.pos > (SCREEN_WIDTH - PLAYER_WIDTH / 2) as f32 {
            self.pos = (SCREEN_WIDTH - PLAYER_WIDTH / 2) as f32;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            self.pos.round() as i32 - PLAYER_WIDTH / 2,
            SCREEN_HEIGHT - PLAYER_HEIGHT - PLAYER_HEIGHT / 2,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
            Color::RED,
        );
    }
}

pub struct Ball {
    pos: Vector2,
    vel: Vector2,
}

impl Ball {
    fn update(&mut self, delta_time: f32) {
        // Move the ball
        self.pos.x += self.vel.x * delta_time;
        self.pos.y += self.vel.y * delta_time;

        // Bounce off walls
        if self.pos.x - BALL_RADIUS <= 0.0 || self.pos.x + BALL_RADIUS >= SCREEN_WIDTH as f32 {
            self.vel.x = -self.vel.x;
        }
        if self.pos.y - BALL_RADIUS <= 0.0 || self.pos.y + BALL_RADIUS >= SCREEN_HEIGHT as f32 {
            self.vel.y = -self.vel.y;
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.pos, BALL_RADIUS, Color::BLUE);
    }
}