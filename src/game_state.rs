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

    pub fn get_cell_by_location(&self, location: &Location) -> Option<Cell> {
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
                return Some(cell.to_owned());
            }
        }
        
        None
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

    pub fn get_surrounding_cell_locations(&self) -> [Option<Location>; 8] {
        let mut location_list = Vec::new();
        for x_pos_diff in -1..=1 {
            for y_pos_diff in -1..=1 {
                if x_pos_diff == 0 && y_pos_diff == 0 {
                    continue;
                }

                location_list.push(Location::new(
                    self.location.x + x_pos_diff,
                    self.location.y + y_pos_diff
                ));
            }
        }

        let mut location_buffer: [Option<Location>; 8] = [None; 8];
        for (i, location) in location_list.iter().enumerate() {
            if i > 7 {
                panic!("There are somehow more than 8 surrounding locations...");
            }
            location_buffer[i] = Some(*location);
        }


        location_buffer
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
    use crate::game_state::{Cell, Location};

    #[test]
    fn get_surrounding_cell_locations_test() {
        let cell = Cell::new(true, Location::new(20, 55));
        let cell_negative_pos = Cell::new(true, Location::new(-88, -1));
        
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
        
        
        let cell_res = cell.get_surrounding_cell_locations();
        let cell_negative_pos_res = cell_negative_pos.get_surrounding_cell_locations();
        
        
        assert_eq!(cell_res, expected_cell);
        assert_eq!(cell_negative_pos_res, expected_cell_negative_pos);
    }
}
