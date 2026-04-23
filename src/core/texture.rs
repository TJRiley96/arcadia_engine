#![allow(dead_code)]

// Built-in imports
use std::path::Path;

// Third-party imports


// Local imports
use crate::util::logging::{error, info, warning, SystemType};

pub struct Texture {
    id: u32,
    width: i32,
    height: i32,
}

impl Texture {
    pub fn load_from_file(_path: &str) -> Self {
        // Load image data from file and create OpenGL texture
        // This is a placeholder implementation
        Self {
            id: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn load(path: &str){
        match Texture::file_exists_debug(path) {
            true => info(format!("Texture found: {}", path).as_str(), "Texture", SystemType::SUBSYSTEM),
            false => error(format!("Texture file not found: {}", path).as_str(), "Texture", SystemType::SUBSYSTEM),
        }
    }

    fn file_exists_debug(path: &str) -> bool {
        std::path::Path::new(path).exists()
    }

    fn file_exists(&self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }
}