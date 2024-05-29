mod game_state;

use macroquad::miniquad::window::{screen_size, set_window_size};
use macroquad::prelude::*;
use crate::game_state::GameState;



const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

#[macroquad::main("BasicShapes")]
async fn main() {
    set_window_size(WIDTH, HEIGHT);
    let state = GameState::new_rand_filled(WIDTH, HEIGHT, 0.5)
        .expect("Failed to create game state");
    
    loop {
        clear_background(WHITE);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // 
        // draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);

        draw_rectangle(0.0, 0.0, 1.0, 1.0, BLACK);
        
        let test = format!("{}, {}", screen_width(), screen_height());
        draw_text(&test, 20.0, 20.0, 30.0, DARKGRAY);
        
        next_frame().await
    }
}