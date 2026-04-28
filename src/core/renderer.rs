#![allow(dead_code)]
/// author: "Triley"
/// description: "This module defines the Renderer struct and related types for managing rendering operations."

// Built-in imports
use std::sync::{LazyLock, Mutex};

// Third-party imports
use gl;
use glfw::{
    self,
    Action,
    Context,
    Glfw,
    Key,
    fail_on_errors,
    *
};

// Local imports
use crate::util::logging::{Logger, error, info, warning, SystemType};
use crate::core::shader::{Shader, Buffer, BufferType, VertexArray, buffer_data, clear_color};

static RENDERER_LOGGER: LazyLock<Mutex<Logger>> = LazyLock::new(|| Mutex::new(Logger::new(Some("renderer"), Some(SystemType::SYSTEM))));

type Vertex = [f32; 8];
type TriIndex = [u32; 3];


const VERTICES: [Vertex; 4] = [
        [  0.5,  0.5, 0.0, 1.0, 0.5, 0.0, 1.0, 1.0],     // top right
        [  0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0],    // bottom right
        [ -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],    // bottom left
        [ -0.5,  0.5, 0.0, 1.0, 0.0, 0.5, 0.0, 1.0]     // top left
    ];

#[allow(dead_code)]
const INDICES: [TriIndex; 2] = [
    [0, 1, 3], // first triangle
    [1, 2, 3]  // second triangle
];

enum RendererError {
    InitializationFailed,
    ViewportSetupFailed,
    ClearFailed,
    RenderFailed,
}

enum RendererResult<T> {
    Ok(T),
    Err(RendererError),
}

enum Resolution {
    R720p,
    R1080p,
    R1440p,
    R4K,
}

enum RenderAPI {
    OpenGL,
    Vulkan,
    DirectX,
    WGPU,
}

enum WindowMode {
    Windowed,
    Borderless,
    Fullscreen,
}

enum Dimension {
    D2 = gl::TEXTURE_2D as isize,
    D3 = gl::TEXTURE_3D as isize,
}

struct Renderer {
    window: glfw::PWindow,
    glfw: Glfw,
    event_receiver: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    render_api: RenderAPI,
    window_mode: WindowMode,
}

impl Renderer {
    // pub fn new() -> Self {
    //     Self {
    //         window: None,
    //         glfw: None,
    //         event_receiver: None,
    //         render_api: RenderAPI::OpenGL,
    //         window_mode: WindowMode::Windowed,
    //     }
    // }

    pub fn initialize(&self) {
        // Placeholder for OpenGL initialization logic
    }

    fn setup_window(&mut self) {
        #[cfg(target_os = "linux")]
        {
        //std::env::set_var("GLFW_PLATFORM", "wayland");
        if glfw::Platform::Wayland.is_supported() {
            println!("Wayland supported, using Wayland.");
        } else {
            println!("Wayland not supported, falling back to X11.");
        }
        }

        let mut glfw: Glfw = glfw::init(fail_on_errors!()).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        self.glfw = glfw;

        let (mut window, event_receiver): (glfw::PWindow, glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) = self.glfw
            .create_window(800, 600, "Arcadia Engine", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);
        window.set_close_polling(true);

        self.window = window;
        self.event_receiver = event_receiver;

    }

    fn setup_open_gl(&mut self) {
        // Placeholder for setting up OpenGL context and state
        gl::load_with(|s| {
        match self.window.get_proc_address(s) {
            Some(f) => f as *const _,
            None => std::ptr::null(),
        }});

        let vao: VertexArray = VertexArray::new().expect("Failed to create Vertex Array Object");
        vao.bind();

        let vbo: Buffer = Buffer::new().expect("Failed to create Vertex Buffer Object");
        vbo.bind(BufferType::ArrayBuffer);
        buffer_data(
            BufferType::ArrayBuffer,
            bytemuck::cast_slice(&VERTICES),
            gl::STATIC_DRAW);


        let ebo: Buffer = Buffer::new().expect("Failed to create Element Buffer Object");
        ebo.bind(BufferType::ElementArrayBuffer);
        buffer_data(
            BufferType::ElementArrayBuffer,
            bytemuck::cast_slice(&INDICES),
            gl::STATIC_DRAW);

        clear_color(0.1, 0.3, 0.3, 1.0);

    }

    fn setup_vulkan(&self) {
        todo!("Implement Vulkan setup logic");
        // Placeholder for setting up Vulkan context and state
    }

    fn setup_direct_x(&self) {
        todo!("Implement DirectX setup logic");
        // Placeholder for setting up DirectX context and state
    }

    fn setup_wgpu(&self) {
        todo!("Implement WGPU setup logic");
        // Placeholder for setting up WGPU context and state
    }

    /// Sets the viewport dimensions for rendering. Logs an error if the operation fails.
    pub fn set_viewport(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }

    pub fn clear(&self) {
        // Placeholder for clearing the screen
    }

    pub fn render(&self) {
        // Placeholder for rendering logic
    }

}