use std::error::Error;
use image::io::Reader as ImageReader;
use image::{ImageBuffer, Pixel, Rgb, RgbImage};
use crate::game_state::{Cell, GameState};
use crate::game_state::Location;


pub fn generate_blank_state_image(width: u32, height: u32) -> Result<(), Box<dyn Error>> {
    let mut image: RgbImage = ImageBuffer::new(width, height);
    for (_, _, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgb([0, 0, 0]);
    }

    image.save("./state-image.png")?;
    Ok(())
}

pub fn load_state_from_image(image_path: &str) -> Result<GameState, Box<dyn Error>> {
    let image = ImageReader::open(image_path)?.decode()?;
    let mut state = GameState::new_blank(image.width(), image.height());

    let pixel_enumerator = image
        .as_rgb8().expect("Could not convert image to rgb8 format.")
        .enumerate_pixels();
    for (x, y, pixel) in pixel_enumerator
    {
        if pixel.to_rgb() == Rgb([255, 255, 255]) {
            let location = Location::new(x as i32, y as i32);
            state.set_cell_by_location(&location, Cell::new(true, location))?;
        }
    }
        
    Ok(state)
}
