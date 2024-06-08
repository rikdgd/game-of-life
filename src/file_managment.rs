use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Write};
use crate::game_state::GameState;



const STATE_FILE_PATH: &str = "./game-state.txt";


pub fn create_blank_state(width: u32, height: u32) -> io::Result<()> {
    let new_state = GameState::new_blank(width, height);
    let mut state_file = OpenOptions::new().create(true).write(true).open(STATE_FILE_PATH)?;

    state_file.set_len(0)?;
    state_file.write_all(new_state.to_state_string().as_bytes())?;

    Ok(())
}

pub fn store_state(game_state: &str) -> io::Result<()> {
    let mut state_file = OpenOptions::new().create(true).write(true).open(STATE_FILE_PATH)?;

    state_file.set_len(0)?;
    state_file.write_all(game_state.as_bytes())?;

    Ok(())
}

pub fn load_state() -> io::Result<String> {
    let mut state_file = OpenOptions::new().read(true).open(STATE_FILE_PATH)?;
    let mut state_buffer = String::new();
    state_file.read_to_string(&mut state_buffer)?;

    Ok(state_buffer)
}
