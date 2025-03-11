mod structs;
mod missile;

use raylib::prelude::*;

use structs::*;
pub const SCREEN_WIDTH:i32 = 800;
pub const SCREEN_HEIGHT:i32 = 600;

fn main() {
    
    //let mut player: Player = Player { pos: SCREEN_WIDTH as f32 / 2.0 };
    //let ball: Ball = Ball { pos: Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT / 2) as f32 }, vel: Vector2 { x: 0., y: 0. }};


    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Missile command")
        .build();
     
    while !rl.window_should_close() {

        //let frame_time: f32 = rl.get_frame_time();
        //player.update(&mut rl);


        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);
            
            //player.draw(&mut d);
            //d.draw_rectangle(player.pos.round() as i32 - PLAYER_WIDTH / 2, SCREEN_HEIGHT - PLAYER_HEIGHT - PLAYER_HEIGHT / 2, PLAYER_WIDTH, PLAYER_HEIGHT, Color::RED);
        }
    }
}