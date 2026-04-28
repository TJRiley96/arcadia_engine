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
use crate::core::actor::ActorTrait;


struct CameraActor {

    // Placeholder for camera actor properties (e.g., position, rotation, field of view, etc.)
    id: u32,
    name: String,
    components: Vec<Components>,
    state: State,

    world_transform: Matrix4,
    position: Vector3,
    rotation: Quaternion,
    scale: f32,
}

impl ActorTrait for CameraActor {
    fn update(&mut self, delta_time: f32) {
        // Placeholder for updating the camera actor each frame (e.g., handling input, moving the camera, etc.)
    }

}