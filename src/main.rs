mod game_state;
mod utils;
mod file_management;

use std::env;
use std::io::{Error, ErrorKind};
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use crate::game_state::GameState;



#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut state: GameState;
    let args: &Vec<String> = &env::args().collect();

    // Check if the user want to generate a new blank state file. 
    if args.len() >= 4 && args[1] == "new-blank" {
        let width = args[2].parse::<u32>().expect("Could not get width from user input.");
        let height = args[3].parse::<u32>().expect("Could not get height from user input.");
        state = GameState::new_blank(width, height);
        file_management::store_state(&state.to_state_string()).expect("Failed to store newly generated state file.");
        
    // Check if the user provided any input, if so run that Config.
    } else if let Ok(config) = Config::from_args(args) {
        set_window_size(config.width, config.height);
        state = GameState::new_rand_filled(config.width, config.height, config.chance_alive)
            .expect("Failed to create game state");
        
        if config.save_state {
            file_management::store_state(&state.to_state_string())
                .expect("Failed to store the game state to disk.");
        }
        
    // If no Config was provided via environment arguments, check if a state file exists and load it.
    } else if let Ok(file_state) = file_management::load_state() {
        state = file_state;
        set_window_size(state.width(), state.height());
        
    // If no correct input was provided, run a default configuration.    
    } else {
        let dimension = 500;
        set_window_size(dimension, dimension);
        state = GameState::new_rand_filled(dimension, dimension, 0.5)
            .expect("Failed to create game state");
    }
    
    
    
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
    pub save_state: bool,
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
                        save_state: false,
                })
            },
            5 => {
                let width = args[1].parse::<u32>().expect("Invalid width provided");
                let height = args[2].parse::<u32>().expect("Invalid height provided");
                let chance_alive = args[3].parse::<f64>().expect("Invalid chance_alive provided");
                let save_state = args[4].parse::<bool>().expect("Not clear if state should be saved.");

                Ok(Self { width, height, chance_alive, save_state, })
            },
            _ => {
                Err(Error::new(
                    ErrorKind::InvalidInput, 
                    "Could not create a run configuration from the user input."
                ))
            }
        }
    }
}
