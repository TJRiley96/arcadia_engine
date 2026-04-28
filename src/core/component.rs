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
use crate::ComponentTrait;
use crate::core::actor::Actor;
use crate::core::components::mesh_component::MeshComponent;



pub enum Components {
    MeshComponent(Component<MeshComponent>),
    SpriteRenderer,
    Collider,
    // Add more component types as needed
}

impl Components {
    pub fn get_component_name(&self) -> &'static str {
        match self {
            Components::MeshComponent(comp) => comp.get_component_name(),
            Components::SpriteRenderer => "SpriteRenderer",
            Components::Collider => "Collider",
        }
    }
}

struct  Component<T: ComponentTrait> {
    // This is a placeholder struct for components that can be attached to entities in the Arcadia Engine.
    // Components will likely include things like Transform, SpriteRenderer, Collider, etc.
    // Each component will have its own data and behavior, and can be added or removed from entities as needed.
    data: T,
    owner_entity_id: Actor,
    update_order: i32,
}

impl<T: ComponentTrait> Component<T> {
    pub fn new(data: T, owner_entity_id: Actor, update_order: i32) -> Self {
        Self {
            data,
            owner_entity_id,
            update_order,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // This is a placeholder method for updating the component each frame.
        // The actual implementation will depend on the specific type of component and its behavior.
        self.data.update(delta_time);
    }

    pub fn get_component_name(&self) -> &'static str {
        self.data.get_component_name()
    }

    pub fn process_input(&mut self, input: u8) {
        // This is a placeholder method for processing input for the component.
        // The actual implementation will depend on the specific type of component and how it interacts with input.
        self.data.process_input(input);
    }

    pub fn on_update_world_transform(&mut self) {
        // This is a placeholder method for handling updates to the world transform of the component.
        // The actual implementation will depend on the specific type of component and how it interacts with the world transform.
        self.data.on_update_world_tranform();
    }

    pub fn get_update_order(&self) -> i32 {
        self.update_order
    }


}