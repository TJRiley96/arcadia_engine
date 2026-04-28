#![allow(dead_code)]

// Built-in imports
use std::path::Path;

// Third-party imports
use gl;
use image;


// Local imports
use crate::util::logging::{error, info, warning, SystemType};


const BORDER_COLOR: [f32; 4] = [1.0, 1.0, 0.0, 1.0];


pub struct Texture {
    id: u32,
    width: i32,
    height: i32,
}

impl Texture {

    pub fn new() -> Self {
        Self {
            id: 0, // Placeholder for texture ID
            width: 0, // Placeholder for texture width
            height: 0, // Placeholder for texture height
        }
    }

    /// Loads a texture from the specified file path. Logs an error if the file does not exist or if loading fails.
    pub fn load(&mut self, path: &str){
        match self.file_exists(path) {
            true => info(format!("Texture found: {}", path).as_str(), "Texture", SystemType::ASSET),
            false => error(format!("Texture file not found: {}", path).as_str(), "Texture", SystemType::ASSET),
        }

        let img = image::ImageReader::open(path)
        .expect("Failed to open texture file.")
        .decode()
        .expect("Failed to decode texture image.")
        .flipv()
        .into_rgba8();

        unsafe {

            gl::GenTextures(1, &mut self.id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, BORDER_COLOR.as_ptr());

            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_raw().as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_raw().as_ptr() as *const _,
            );

            self.width = img.width() as i32;
            self.height = img.height() as i32;

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

    }

    /// Binds the texture to the specified texture unit. Logs an error if the operation fails.
    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    /// Unbinds the texture from the current texture unit. Logs an error if the operation fails.


    /// Deletes the texture from GPU memory. Logs an error if the operation fails.
    pub fn unload(&self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
        info(format!("Unloading texture with ID: {}", self.id).as_str(), "Texture", SystemType::ASSET);
    }

    /// Returns the width of the texture.
    pub fn get_width(&self) -> i32 {
        self.width
    }

    /// Returns the height of the texture.
    pub fn get_height(&self) -> i32 {
        self.height
    }

    /// Checks if a file exists at the given path. Returns true if the file exists, false otherwise.
    fn file_exists(&self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }
}