// src/models/ascii_art.rs
/**
* ASCII art data model
* Responsibilities:
* - Define ASCII art structure
* - Manage ASCII art properties
* - Input validation
* - Data transformation
*/
// Import serialization traits and error handling
use serde::{Deserialize, Serialize};
use std::error::Error;

// Define AsciiArt struct with serialization support
#[derive(Debug, Serialize, Deserialize)]
pub struct AsciiArt {
    pub content: String,   // ASCII art content
    pub width: usize,      // Width in characters
    pub height: usize,     // Height in characters
    pub char_count: usize, // Total character count
}

impl AsciiArt {
    // Constructor with validation
    pub fn new(content: String, width: usize, height: usize) -> Result<Self, Box<dyn Error>> {
        if width * height == 0 {
            // Check for valid dimensions
            return Err("Invalid dimensions".into());
        }

        let ascii = Self {
            // Create instance
            content,
            width,
            height,
            char_count: width * height,
        };
        Ok(ascii) // Return if valid
    }
}