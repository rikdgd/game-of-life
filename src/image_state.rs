use std::error::Error;
use image::{ImageBuffer, Rgb, RgbImage};


pub fn generate_blank_state_image(width: u32, height: u32) -> Result<(), Box<dyn Error>> {
    let mut image: RgbImage = ImageBuffer::new(width, height);
    for (_, _, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgb([0, 0, 0]);
    }

    image.save("./state-image.png")?;
    Ok(())
}
