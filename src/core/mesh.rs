/// Mesh struct and related functionality for the Arcadia Engine.
///
/// author: "Triley"
/// copyright: "Copyright (c) 2024 Triley. All rights reserved."
/// credits: ["Triley"]
/// maintainer: "Triley"
/// version: "0.1.0"


// Built-in imports


// Third-party imports


// Local imports
use crate::core::shader::{VertexArray};
use crate::core::texture::Texture;
use crate::util::logging::{error, info, warning, SystemType};




pub struct Mesh {
    // Placeholder for mesh data (e.g., vertex buffer, index buffer, etc.)
    textures: Vec<Texture>, // List of texture file paths associated with this mesh
    vertex_array: VertexArray, // Vertex array object for this mesh
    shader_name: String, // Name of the shader associated with this mesh
    radius: f32, // Bounding sphere radius for the mesh
    specular_intensity: f32, // Specular intensity for lighting calculations
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            vertex_array: VertexArray::new().expect("Failed to create Vertex Array Object"),
            shader_name: String::new(),
            radius: 0.0,
            specular_intensity: 100.0,
        }
    }

    pub fn load(&mut self, path: &str) {
        // Implement mesh loading logic here (e.g., parse OBJ file)

        let f_check: bool = self.file_exists(path);
        if !f_check {

            return;
        }
    }

    pub fn unload(&mut self) {
        // Implement mesh unloading logic here (e.g., free GPU resources)
    }

    pub fn get_vertex_array(&self) -> &VertexArray {
        // Return a reference to the vertex array associated with this mesh
        &self.vertex_array
    }

    pub fn get_texture(&self, index: usize) -> Option<&Texture> {
        // Return a reference to the texture at the specified index, if it exists
        self.textures.get(index)
    }

    pub fn get_shader_name(&self) -> &str {
        // Return the name of the shader associated with this mesh
        &self.shader_name
    }

    pub fn get_radius(&self) -> f32 {
        // Return the bounding sphere radius for this mesh
        self.radius
    }

    pub fn get_specular_intensity(&self) -> f32 {
        // Return the specular intensity for lighting calculations
        self.specular_intensity
    }


    fn load_obj(&mut self, path: &str) -> VertexArray{
        // Implement OBJ file parsing and loading logic here\
        unimplemented!("OBJ file loading not implemented yet");
    }

    fn file_exists(&self, path: &str) -> bool {
        if std::fs::metadata(path).is_ok()
        {
            info(&format!("Mesh file found: {}", path), "Mesh", SystemType::ASSET);
            true
        } else {
            error(&format!("Mesh file not found: {}", path), "Mesh", SystemType::ASSET);
            false
        }
    }
}