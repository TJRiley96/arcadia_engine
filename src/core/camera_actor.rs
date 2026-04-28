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
use crate::core::component::Components;
use crate::util::math::{matrix::Matrix4, quaternion::Quaternion, vector::Vector3};


enum State {
    Active,
    Paused,
    Dead,
}

pub struct CameraActor {

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