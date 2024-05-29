use rand::{thread_rng, Rng};


pub struct GameState {
    width: u32,
    height: u32,
    pub cells: Vec<Vec<Cell>>,
}
impl GameState {
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
            width,
            height,
            cells,
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

    pub fn update(&mut self) {
        let mut updated_state: Vec<Vec<Cell>> = Vec::new();
        
        for cell_row in &self.cells {
            for cell in cell_row {
                
            }
        }
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
            let cell = self.get_cell_by_location(&location.expect("Faulty location.")).unwrap();
            if cell.is_alive {
                alive_counter += 1;
            }
        }

        alive_counter
    }
}

#[derive(Clone, Debug)]
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
    use crate::game_state::{GameState, Location};

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
}
