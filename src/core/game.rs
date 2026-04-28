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
use crate::core::actor::Actor;
use crate::core::camera_actor::CameraActor;
use crate::core::renderer::Renderer;




pub struct Game {
    // This is a placeholder struct for the main game logic of the Arcadia Engine.
    // The Game struct will likely include methods for initializing the game, updating game state, and rendering the game each frame.
    // We may also want to include support for managing game entities, handling input, and implementing game mechanics.

    renderer: Option<Renderer>,
    is_running: bool,
    tick_count: u64,
    actors: Vec<Actor>,
    peading_actors: Vec<Actor>,
    updating_actors: bool,
    camera_actor: Option<CameraActor>,
}

impl Game{
    pub fn new() -> Self {
        Self {
            tick_count: 0,
            renderer: None,
            is_running: true,
            actors: Vec::new(),
            peading_actors: Vec::new(),
            updating_actors: false,
            camera_actor: None,
        }
    }

    pub fn initialize(&mut self) {
        // Placeholder for game initialization logic
        // This is where we would initialize the game state, load resources, and set up the renderer.
        self.renderer = Some(Renderer::new());
        if let Some(renderer) = &mut self.renderer {
            renderer.initialize();
        }
    }

    pub fn update(&mut self) {
        // Placeholder for game update logic
        // This is where we would update the game state each frame, including handling input, updating entities, and processing game mechanics.
    }

    pub fn render(&self) {
        // Placeholder for game rendering logic
        // This is where we would issue draw calls to the renderer to render the current game state to the screen.
    }


    pub fn get_tick_count(&self) -> u64 {
        self.tick_count
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn get_renderer(&mut self) -> &mut Renderer {
        self.renderer.as_mut().expect("Renderer is not initialized")
    }
}