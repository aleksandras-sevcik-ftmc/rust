// src/services/ascii_service.rs
/**
* ASCII conversion business logic
* Responsibilities:
* - Image to ASCII conversion
* - Character mapping
* - Image processing
* - Output formatting
*/
use image::{DynamicImage, GenericImageView, Rgba, imageops};
use anyhow::Error as AnyhowError;
use crate::models::ascii_art::AsciiArt;

pub struct AsciiService {
    charset: Vec<char>,    // Characters for ASCII conversion
    max_width: u32,        // Maximum output width
}

impl AsciiService {
    pub fn new() -> Self {
        Self {
            charset: vec!['@', '%', '#', '*', '+', '=', '-', ':', '.', ' '],
            max_width: 100,
        }
    }

    // Main conversion method returning AsciiArt model
    pub fn image_to_ascii(&self, img: &DynamicImage) -> Result<AsciiArt, AnyhowError> {
        let processed = self.process_image(img)?;
        let ascii_vec = self.map_pixels_to_ascii(&processed);
        let content = self.format_output(&ascii_vec);
        
        AsciiArt::new(
            content,
            processed.width() as usize,
            processed.height() as usize
        )
    }

    // Process and resize image - now using AnyhowError
    fn process_image(&self, image: &DynamicImage) -> Result<DynamicImage, AnyhowError> {
        let aspect_ratio = image.height() as f32 / image.width() as f32;
        let new_width = self.max_width;
        let new_height = (new_width as f32 * aspect_ratio * 1.0) as u32;
        
        Ok(image.resize(new_width, new_height, imageops::FilterType::Lanczos3))
    }

    // Convert image pixels to ASCII characters
    fn map_pixels_to_ascii(&self, image: &DynamicImage) -> Vec<Vec<char>> {
        let (width, height) = image.dimensions();
        let mut ascii = Vec::with_capacity(height as usize);
        
        for y in 0..height {
            let mut row = Vec::with_capacity(width as usize);
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                let brightness = self.calculate_brightness(&pixel);
                let char_idx = ((brightness / 255.0) * (self.charset.len() - 1) as f32) as usize;
                row.push(self.charset[char_idx]);
            }
            ascii.push(row);
        }
        
        ascii
    }

    // Calculate pixel brightness using RGB weights
    fn calculate_brightness(&self, pixel: &Rgba<u8>) -> f32 {
        pixel[0] as f32 * 0.299 + 
        pixel[1] as f32 * 0.587 + 
        pixel[2] as f32 * 0.114
    }

    // Convert 2D char vector to string with newlines
    fn format_output(&self, ascii: &Vec<Vec<char>>) -> String {
        ascii.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}