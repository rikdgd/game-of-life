use std::io::ErrorKind;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use crate::utils;


const ALIVE_CHAR: char = 'O';
const DEAD_CHAR: char = '.';


pub struct GameState {
    width: u32,
    height: u32,
    cells: Vec<Vec<Cell>>,
}
impl GameState {
    #[allow(unused)]
    pub fn new_blank(width: u32, height: u32) -> Self {
        let mut cells = Vec::new();
        for y in 0..height as i32 {
            let mut cell_row: Vec<Cell> = Vec::new();
            for x in 0..width as i32 {
                cell_row.push(
                    Cell::new(false, Location::new(x, y))
                );
            }
            cells.push(cell_row);
        }

        Self {
            cells,
            width,
            height,
        }
    }

    pub fn new_rand_filled(width: u32, height: u32, chance_alive: f64) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&chance_alive) {
            return Err("Please enter a chance from 0.0 to 1.0 (included)".to_string());
        }

        let mut cells = Vec::new();
        let mut rng = thread_rng();

        for y in 0..height as i32 {
            let mut cell_row: Vec<Cell> = Vec::new();
            for x in 0..width as i32 {
                cell_row.push(
                    Cell::new(rng.gen_bool(chance_alive), Location::new(x, y))
                );
            }
            cells.push(cell_row);
        }
        Ok(Self { 
            cells,
            width,
            height,
        })
    }


    
    pub fn from_state_string(char_string: String) -> std::io::Result<Self> {
        let mut cells: Vec<Vec<Cell>> = Vec::new();
        
        let str_rows: Vec<&str> = char_string.split('\n').collect();
        let mut height = str_rows.len();
        let width = utils::count_chars_in_str(str_rows[0], ',') + 1;
        
        for (y, row) in str_rows.iter().enumerate() {
            if row.is_empty() {
                height -= 1;
                continue;
            }
            
            let mut cell_row: Vec<Cell> = Vec::new();
            
            let mut current_x_pos: u32 = 0;
            for char in row.chars() {
                let cell = match char {
                    ALIVE_CHAR => {
                        Some(Cell {
                            is_alive: true,
                            location: Location::new(current_x_pos as i32, y as i32),
                        })
                    }, 
                    DEAD_CHAR => {
                        Some(Cell {
                            is_alive: false,
                            location: Location::new(current_x_pos as i32, y as i32),
                        })
                    },
                    _ => {
                        None
                    }
                };
                
                if let Some(cell) = cell {
                    cell_row.push(cell);
                    current_x_pos += 1;
                }
            }
            
            if cell_row.len() != width {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidInput, 
                    "The given state string was incorrect"
                ))
            }
            
            cells.push(cell_row);
        }
        
        Ok(Self {
            cells,
            width: width as u32,
            height: height as u32,
        })
    }
    
    pub fn to_state_string(&self) -> String {
        let mut char_string = String::new();
        
        for row in &self.cells {
            let mut char_row: Vec<char> = Vec::new();
            for cell in row {
                let cell_char = match cell.is_alive {
                    true => ALIVE_CHAR,
                    false => DEAD_CHAR,
                };
                char_row.push(cell_char);
            }
            char_string.push_str(&format!("{:?}\n", char_row));
        }
        
        char_string
    }
    
    pub fn get_cells(&self) -> &Vec<Vec<Cell>> {
        &self.cells
    }

    pub fn update(&mut self) {
        let updated_cells: Vec<Vec<Cell>> = self.cells
            .par_iter()
            .map(|row| {
                row.par_iter()
                    .map(|cell| self.update_cell(cell))
                    .collect()
            })
            .collect();

        self.cells = updated_cells;
    }
    
    fn update_cell(&self, cell: &Cell) -> Cell {
        let mut updated_cell = cell.clone();
        let alive_count = self.get_surrounding_living_cells(&updated_cell.location);

        if alive_count <= 1 || (4..=8).contains(&alive_count) {
            updated_cell.is_alive = false;
        } else if alive_count == 3 {
            updated_cell.is_alive = true;
        } else if alive_count >= 9 {
            panic!("There shouldn't be more than 8 surrounding cells to begin with...");
        }

        updated_cell
    }

    pub fn get_cell_by_location(&self, location: &Location) -> Option<&Cell> {
        // The application doesn't have pixels on negative indexes, 
        // so asking to get one on a negative location is always unsuccessful, 
        // since we map cells to pixels.
        if location.x < 0 || location.y < 0 {
            return None;
        }
        
        if location.x as u32 > self.width || location.y as u32 > self.height {
            return None;
        }
        
        if let Some(cell_row) = self.cells.get(location.y as usize) {
            if let Some(cell) = cell_row.get(location.x as usize) {
                return Some(cell);
            }
        }
        
        None
    }

    pub fn set_cell_by_location(&mut self, location: &Location, new_cell: Cell) -> std::io::Result<()> {
        // The application doesn't have pixels on negative indexes, 
        // so asking to get one on a negative location is always unsuccessful, 
        // since we map cells to pixels.
        if location.x < 0 || location.y < 0 {
            return Err(std::io::Error::new(
                ErrorKind::InvalidInput, 
                "Tried to set pixel on negative index."
            ));
        }

        if location.x as u32 > self.width || location.y as u32 > self.height {
            return Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                "Tried to set pixel on outside of image range."
            ));
        }

        if let Some(cell_row) = self.cells.get_mut(location.y as usize) {
            if let Some(cell) = cell_row.get_mut(location.x as usize) {
                *cell = new_cell;
                return Ok(());
            }
        }

        Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            "Something went wrong when trying to set the pixel with the given input."
        ))
    }

    fn get_surrounding_locations(location: &Location) -> [Option<Location>; 8] {
        let mut location_list = Vec::new();
        for x_pos_diff in -1..=1 {
            for y_pos_diff in -1..=1 {
                if x_pos_diff == 0 && y_pos_diff == 0 {
                    continue;
                }

                location_list.push(Location::new(
                    location.x + x_pos_diff,
                    location.y + y_pos_diff
                ));
            }
        }

        let mut location_buffer: [Option<Location>; 8] = [None; 8];
        for (i, location) in location_list.iter().enumerate() {
            if i > 7 {
                panic!("There are somehow more than 8 surrounding locations, what did I mess up...");
            }
            location_buffer[i] = Some(*location);
        }
        
        location_buffer
    }
    
    fn get_surrounding_living_cells(&self, location: &Location) -> u8 {
        let surrounding_locations = GameState::get_surrounding_locations(location);
        let mut alive_counter: u8 = 0;
        
        for location in surrounding_locations {
            let location = &location.expect("Faulty location.");
            let cell = match self.get_cell_by_location(location) {
                Some(cell) => {
                    cell.clone()
                },
                None => {
                    // If the cell has a location outside the field, assume it's dead.
                    Cell::new(false, *location)
                },
            };
                
            if cell.is_alive {
                alive_counter += 1;
            }
        }
        
        alive_counter
    }
    pub fn set_cells(&mut self, cells: Vec<Vec<Cell>>) {
        // TODO: adjust width and height
        self.cells = cells;
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub is_alive: bool,
    pub location: Location,
}

impl Cell {
    pub fn new(is_alive: bool, location: Location) -> Self {
        Self {
            is_alive,
            location,
        }
    }

    pub fn from_char(char: char, location: Location) -> Option<Self> {
        match char {
            ALIVE_CHAR => {
                Some(Self { is_alive: true, location })
            },
            DEAD_CHAR => {
                Some(Self { is_alive: false, location })
            }
            _ => {
                None
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    x: i32,
    y: i32,
}
impl Location {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}



#[cfg(test)]
mod test {
    use crate::game_state::{Cell, GameState, Location};

    #[test]
    fn get_surrounding_cell_locations_test() {
        let location = Location::new(20, 55);
        let negative_location = Location::new(-88, -1);
        
        let expected_cell: [Option<Location>; 8] = [
            Some(Location::new(19, 54)),
            Some(Location::new(19, 55)),
            Some(Location::new(19, 56)),
            Some(Location::new(20, 54)),
            Some(Location::new(20, 56)),
            Some(Location::new(21, 54)),
            Some(Location::new(21, 55)),
            Some(Location::new(21, 56)),
        ];

        let expected_cell_negative_pos: [Option<Location>; 8] = [
            Some(Location::new(-89, -2)),
            Some(Location::new(-89, -1)),
            Some(Location::new(-89, 0)),
            Some(Location::new(-88, -2)),
            Some(Location::new(-88, 0)),
            Some(Location::new(-87, -2)),
            Some(Location::new(-87, -1)),
            Some(Location::new(-87, 0)),
        ];
        
        
        let cell_res = GameState::get_surrounding_locations(&location);
        let cell_negative_pos_res = GameState::get_surrounding_locations(&negative_location);
        
        
        assert_eq!(cell_res, expected_cell);
        assert_eq!(cell_negative_pos_res, expected_cell_negative_pos);
    }

    #[test]
    fn from_state_string_test() {
        let state_string = r#"['O', '.', '.']
['O', 'O', 'O']
['O', '.', 'O']
"#;

        let expected_cells = vec![
            vec![ // row 1
                Cell::new(true, Location::new(0, 0)),
                Cell::new(false, Location::new(1, 0)),
                Cell::new(false, Location::new(2, 0))
            ],
            vec![ // row 2
                Cell::new(true, Location::new(0, 1)),
                Cell::new(true, Location::new(1, 1)),
                Cell::new(true, Location::new(2, 1))
            ],
            vec![ // row 3
                Cell::new(true, Location::new(0, 2)),
                Cell::new(false, Location::new(1, 2)),
                Cell::new(true, Location::new(2, 2))
            ],
        ];
        let mut expected_state = GameState::new_blank(3, 3);
        expected_state.set_cells(expected_cells);

        let result_state = GameState::from_state_string(state_string.to_string()).unwrap();

        assert_eq!(result_state.cells, expected_state.cells);
    }

    #[test]
    fn to_state_string_test() {
        let expected_state_string = r#"['O', '.', '.']
['O', 'O', 'O']
['O', '.', 'O']
"#;

        let cells = vec![
            vec![ // row 1
                  Cell::new(true, Location::new(0, 0)),
                  Cell::new(false, Location::new(1, 0)),
                  Cell::new(false, Location::new(2, 0))
            ],
            vec![ // row 2
                  Cell::new(true, Location::new(0, 1)),
                  Cell::new(true, Location::new(1, 1)),
                  Cell::new(true, Location::new(2, 1))
            ],
            vec![ // row 3
                  Cell::new(true, Location::new(0, 2)),
                  Cell::new(false, Location::new(1, 2)),
                  Cell::new(true, Location::new(2, 2))
            ],
        ];
        let mut state = GameState::new_blank(3, 3);
        state.set_cells(cells);
        
        
        let result_state_string = state.to_state_string();

        assert_eq!(result_state_string, expected_state_string);
    }
}
