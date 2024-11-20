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
use anyhow::Error as AnyhowError;

#[derive(Debug, Serialize, Deserialize)]
pub struct AsciiArt {
    pub content: String,   // ASCII art content
    pub width: usize,      // Width in characters
    pub height: usize,     // Height in characters
    pub char_count: usize, // Total character count
}

impl AsciiArt {
    // Changed return type to use AnyhowError
    pub fn new(content: String, width: usize, height: usize) -> Result<Self, AnyhowError> {
        if width * height == 0 {
            // Use anyhow's bail! macro for early returns with errors
            anyhow::bail!("Invalid dimensions");
        }

        let ascii = Self {
            content,
            width,
            height,
            char_count: width * height,
        };
        Ok(ascii)
    }
}