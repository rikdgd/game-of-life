mod game_state;

use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use crate::game_state::GameState;
use std::env;




#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let config = Config::from_args(&env::args().collect());

    set_window_size(config.width, config.height);
    let mut state = GameState::new_rand_filled(config.width, config.height, config.chance_alive)
        .expect("Failed to create game state");
    
    loop {
        state.update();
        clear_background(WHITE);
        
        for (y, cell_row) in state.get_cells().iter().enumerate() {
            for (x, cell) in cell_row.iter().enumerate() {
                let color = match cell.is_alive {
                    true => BLACK,
                    false => WHITE,
                };
                draw_rectangle(x as f32, y as f32, 1.0, 1.0, color);
            }
        }
        
        next_frame().await
    }
}

struct Config {
    pub width: u32,
    pub height: u32,
    pub chance_alive: f64,
}
impl Config {
    #[allow(clippy::ptr_arg)]
    pub fn from_args(args: &Vec<String>) -> Self {
        match args.len() {
            4 => {
                let width = args[1].parse::<u32>().expect("Invalid width provided");
                let height = args[2].parse::<u32>().expect("Invalid height provided");
                let chance_alive = args[3].parse::<f64>().expect("Invalid chance_alive provided");

                Config {
                    width,
                    height,
                    chance_alive,
                }
            },
            _ => {
                panic!("Invalid amount of arguments provided.");
            }
        }
    }
}