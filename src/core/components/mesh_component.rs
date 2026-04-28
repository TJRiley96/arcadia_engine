/// Description
///
/// author: "Triley"
/// copyright: "Copyright (c) 2024 Triley. All rights reserved."
/// credits: ["Triley"]
/// maintainer: "Triley"
/// version: "0.1.0"


// Built-in imports


// Third-party imports


// Local imports
use crate::ComponentTrait;

//#[derive(ComponentTrait)]
pub struct MeshComponent {
    // This is a placeholder struct for a mesh component that can be attached to entities in the Arcadia Engine.
    // A mesh component will likely include data such as vertex positions, normals, UV coordinates, and indices,
    // as well as methods for rendering the mesh using OpenGL.
    // We may also want to include support for loading mesh data from files, such as OBJ or FBX files.
}

impl ComponentTrait for MeshComponent {
    fn update(&mut self, delta_time: f32) {
        // This is a placeholder method for updating the mesh component each frame.
        // The actual implementation will depend on the specific behavior of the mesh component and how it interacts with other components and systems in the engine.
    }

    fn get_component_name(&self) -> &'static str {
        "MeshComponent"
    }


    fn process_input(&mut self, input: u8) {
        // This is a placeholder method for processing input for the mesh component.
        // The actual implementation will depend on the specific behavior of the mesh component and how it interacts with input.
    }

    fn on_update_world_tranform(&mut self) {
        // This is a placeholder method for handling updates to the world transform of the mesh component.
        // The actual implementation will depend on the specific behavior of the mesh component and how it interacts with the world transform.
    }

}