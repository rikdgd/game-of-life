use std::error::Error;
use image::io::Reader as ImageReader;
use image::{ImageBuffer, Pixel, Rgb, RgbImage};
use crate::game_state::{Cell, GameState};
use crate::game_state::Location;



const STATE_IMAGE_PATH: &str = "./state-image.png";


pub fn generate_blank_state_image(width: u32, height: u32) -> Result<(), Box<dyn Error>> {
    let mut image: RgbImage = ImageBuffer::new(width, height);
    for (_, _, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    image.save(STATE_IMAGE_PATH)?;
    Ok(())
}

pub fn load_state_from_image() -> Result<GameState, Box<dyn Error>> {
    let image = ImageReader::open(STATE_IMAGE_PATH)?.decode()?;
    let mut state = GameState::new_blank(image.width(), image.height());

    let pixel_enumerator = image
        .as_rgb8().expect("Could not convert image to rgb8 format.")
        .enumerate_pixels();
    for (x, y, pixel) in pixel_enumerator
    {
        if pixel.to_rgb() == Rgb([0, 0, 0]) {
            let location = Location::new(x as i32, y as i32);
            state.set_cell_by_location(&location, Cell::new(true, location))?;
        }
    }
        
    Ok(state)
}
