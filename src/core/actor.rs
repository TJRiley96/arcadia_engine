#![allow(dead_code)]

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
use crate::{core::component::Components, util::math::{matrix::Matrix4, quaternion::Quaternion, vector::Vector3}};

enum State {
    Active,
    Paused,
    Dead,
}

pub struct Actor {
    id: u32,
    name: String,
    components: Vec<Components>,
    state: State,

    world_transform: Matrix4,
    position: Vector3,
    rotation: Quaternion,
    scale: f32,

}

impl Actor {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            components: Vec::new(),
            state: State::Active,
            world_transform: Matrix4::identity(),
            position: Vector3::zero(),
            rotation: Quaternion::identity(),
            scale: 1.0,
        }
    }

    pub fn add_component(&mut self, component: Components) {
        self.components.push(component);
    }

    pub fn remove_component(&mut self, component_type: &str) {
        self.components.retain(|c| c.get_component_name() != component_type);
    }

    pub fn get_component(&self, component_type: &str) -> Option<&Components> {
        self.components.iter().find(|c| c.get_component_name() == component_type)
    }
}


pub trait ActorTrait {
    fn update(&mut self, delta_time: f32);
    fn update_actor(&mut self, delta_time: f32);
    fn process_input(&mut self, input: u8);
    fn process_actor_input(&mut self, input: u8);
    fn get_id(&self) -> u32;
    fn get_name(&self) -> &str;
    fn get_state(&self) -> &State;
    fn set_state(&mut self, state: State);
    fn get_world_transform(&self) -> &Matrix4;
    fn get_position(&self) -> &Vector3;
    fn set_position(&mut self, position: Vector3);
    fn get_rotation(&self) -> &Quaternion;
    fn set_rotation(&mut self, rotation: Quaternion);
    fn get_scale(&self) -> f32;
    fn set_scale(&mut self, scale: f32);
    fn compute_world_transform(&mut self);
    fn add_component(&mut self, component: Components);
    fn remove_component(&mut self, component_type: &str);
    fn update_components(&mut self, delta_time: f32);
}