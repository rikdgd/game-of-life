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
        if chance_alive > 1.0 || chance_alive < 0.0 {
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
        todo!()
    }

    pub fn get_nearby_cell_count(&self, cell: Cell) -> u8 {
        for row in &self.cells {
            for cell in row {

            }
        }

        todo!()
    }
    
    fn get_surrounding_cell_locations(&self, cell: &Cell) -> [Option<Location>; 8] {
        let mut location_list = Vec::new();
        for x_pos_diff in -1..=1 {
            for y_pos_diff in -1..=1 {
                if x_pos_diff == 0 && y_pos_diff == 0 {
                    continue;
                }
                
                location_list.push(Location::new(
                    cell.location.x + x_pos_diff, 
                    cell.location.y + y_pos_diff
                ));
            }
        }
        
        let mut location_buffer: [Option<Location>; 8] = [None; 8];
        for (i, location) in location_list.iter().enumerate() {
            if i > 7 {
                panic!("There are somehow more than 8 surrounding locations...");
            }
            location_buffer[i] = Some(location.clone());
        }


        location_buffer
    }
}

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
    
    pub fn get_surrounding_cells(&self) -> [Option<Cell>; 8] {
        

        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Location {
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
        // Arrange
        const WIDTH: i32 = 500;
        const HEIGHT: i32 = 500;
        let state = GameState::new_blank(WIDTH as u32, HEIGHT as u32);
        
        let cell = Cell::new(true, Location::new(20, 55));
        let top_edge_cell = Cell::new(true, Location::new(451, 0));
        let bottom_edge_cell = Cell::new(true, Location::new(451, HEIGHT - 1));
        let far_left_cell = Cell::new(true, Location::new(0, 32));
        let far_right_cell = Cell::new(true, Location::new(WIDTH - 1, 60));
        let top_left_cell = Cell::new(true, Location::new(0, 0));
        let bottom_right_cell = Cell::new(true, Location::new(WIDTH - 1, HEIGHT - 1));
        
        let expected_cell: [Option<Location>; 8] = [
            Some(Location::new(19, 54)),
            Some(Location::new(20, 54)),
            Some(Location::new(21, 54)),
            Some(Location::new(19, 55)),
            Some(Location::new(21, 55)),
            Some(Location::new(19, 56)),
            Some(Location::new(20, 56)),
            Some(Location::new(21, 56)),
        ];
        
        
        // Act
        let cell_res = state.get_surrounding_cell_locations(&cell);
        
        
        // Assert
        assert_eq!(cell_res, expected_cell);
        
        todo!()
    }
}
