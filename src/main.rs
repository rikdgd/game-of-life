mod game_state;
mod utils;
mod file_managment;

use std::env;
use std::io::ErrorKind;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use crate::game_state::GameState;




#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let config = Config::from_args(&env::args().collect()).unwrap();

    set_window_size(config.width, config.height);
    let mut state = GameState::new_rand_filled(config.width, config.height, config.chance_alive)
        .expect("Failed to create game state");
    
    let state_str = state.to_state_string();
    println!("{state_str}");
    
    let test = GameState::from_state_string(state_str).unwrap();
    println!("{:?}", test.get_cells());
    
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
    pub fn from_args(args: &Vec<String>) -> std::io::Result<Self> {
        match args.len() {
            4 => {
                let width = args[1].parse::<u32>().expect("Invalid width provided");
                let height = args[2].parse::<u32>().expect("Invalid height provided");
                let chance_alive = args[3].parse::<f64>().expect("Invalid chance_alive provided");

                Ok(Self {
                    width,
                    height,
                    chance_alive,
                })
            },
            _ => {
                Err(std::io::Error::new(ErrorKind::InvalidInput, "Could not generate a config with the given input."))
            }
        }
    }
}