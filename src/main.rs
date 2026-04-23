// Built-in imports

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
use image;

// Local imports
mod util;
mod core;
use crate::core::shader::{Buffer};
use crate::util::{
    color::Color32,
    logging::Logger,
    logging::SystemType,
    logging::*
};
use crate::core::{
    shader::*,
    texture::Texture,
};

const WINDOW_TITLE: &str = "Rust OpenGL";

type Vertex = [f32; 8];
type TriIndex = [u32; 3];

type VertexColor = [f32; 8];

#[allow(dead_code)]
const USE_TEST_SHADER: bool = true;

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

#[allow(dead_code)]
const TRIANGLE_VERTICES: [VertexColor; 3] = [
    [ 0.0,  0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0],     // top
    [ 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],     // bottom right
    [-0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.5, 1.0]      // bottom left
];

#[allow(dead_code)]
const TRIANGLE_INDICES: [TriIndex; 1] = [
    [0, 1, 2]
];

#[allow(dead_code)]
const TEXTURE_COORDS: [[f32; 2]; 3] = [// top right
    [1.0, 0.0], // bottom right
    [0.0, 0.0], // bottom left
    [0.5, 1.0]  // top-center
];

const BORDER_COLOR: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

const TEST_TEXTURE_PATH: &str = "C:/Users/triley/Projects/rust/test/rust_opengl/src/assets/textures/Grid_UV_64PX.png";
const WALL_TEXTURE_PATH: &str = "C:/Users/triley/Projects/rust/test/rust_opengl/src/assets/textures/wall.jpg";

const TEST_ASSET_PATH: &str = "./src/assets/textures/Grid_UV_64PX.png";

fn main() {

    // This is a sanity check to ensure that the log directory is properly set up.
    sanity_check();
    Texture::load(TEST_ASSET_PATH);

    info("System Initialize", "root", SystemType::ROOT);

    let vert_shader_code: &str = include_str!("./assets/shaders/test_vertex_shader.vert");
    let frag_shader_code: &str = include_str!("./assets/shaders/test_fragment_shader.frag");
    let temp_path = std::path::Path::new(TEST_TEXTURE_PATH);
    if !temp_path.exists() {
        panic!("Texture file not found at path: {}", TEST_TEXTURE_PATH);
    }
    let img = image::ImageReader::open(temp_path)
        .expect("Failed to open texture file.")
        .decode()
        .expect("Failed to decode texture image.")
        .flipv()
        .into_rgba8();

    let temp_path = std::path::Path::new(WALL_TEXTURE_PATH);
    if !temp_path.exists() {
        panic!("Texture file not found at path: {}", WALL_TEXTURE_PATH);
    }
    let img2 = image::open(temp_path).expect("Failed to load texture image.").flipv().into_rgba8();

    let mut _logger: Logger = Logger::new(None, Some(SystemType::ROOT));
    _logger.init();
    _logger.add_info("Logger created.");


    // Init window
    #[cfg(target_os = "linux")]
    {
        std::env::set_var("GLFW_PLATFORM", "wayland");
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


    #[cfg(target_os = "windows")]
    {

        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    }



    // TODO: Handle window creation failure more gracefully.

    let (mut win, events) = glfw
        .create_window(800, 600, WINDOW_TITLE, glfw::WindowMode::Windowed)
        .expect("Couldn't make a window and context");

    win.make_current();
    win.set_key_polling(true);
    win.set_close_polling(true);

    gl::load_with(|s| {
        match win.get_proc_address(s) {
            Some(f) => f as *const _,
            None => std::ptr::null(),
        }});

    _logger.add_info("Load gl functions");


    let mut color_vector: Color32 = Color32{
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0
    };

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

    let mut texture1: u32 = 0;

    unsafe {
        gl::Viewport(0, 0, 800,600);

        gl::GenTextures(1, &mut texture1);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture1);

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
            img2.width() as i32,
            img2.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img2.as_raw().as_ptr() as *const _,
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
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    let mut texture2: u32 = 0;

    unsafe {
        gl::Viewport(0, 0, 800,600);

        gl::GenTextures(1, &mut texture2);
        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, texture2);

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
            img2.width() as i32,
            img2.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img2.as_raw().as_ptr() as *const _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    let shader_program = ShaderProgram::from_shaders(vert_shader_code, frag_shader_code).expect("Failed to create shader program");

    shader_program.use_program();

    //let color_loc: i32 = get_uniform_location(&shader_program, "ourColor").expect("Failed to get uniform location for 'ourColor'");
    //set_uniform_4fv(color_loc, 1, &color_vector.as_array());

    unsafe{

        // position
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<VertexColor>().try_into().unwrap(),
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        // color
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<VertexColor>().try_into().unwrap(),
            size_of::<[f32; 3]>() as *const _,
        );
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            size_of::<VertexColor>().try_into().unwrap(),
            size_of::<[f32; 6]>() as *const _,
        );
        gl::EnableVertexAttribArray(2);

        let mut n_attributes: gl::types::GLint = 0;
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut n_attributes);
        _logger.add_info(format!("Max vertex attributes supported: {}", n_attributes).as_str());
        let _ = n_attributes;
    }

    shader_program.set_uniform_1i("texture1", 0);
    shader_program.set_uniform_1i("texture2", 1);

    set_polygon_mode(PolygonMode::Fill);

    'main_loop: loop {


        if win.should_close(){ break 'main_loop; }
        // handle events this frame
        process_input(&mut win, &events);

        shader_program.use_program();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
        color_vector.rainbow_step_color(get_gl_time(&glfw));




        // now the events are clear
        win.swap_buffers();
        glfw.poll_events();
        // Here's where we could change the world state and draw.

    }
    _logger.add_info("Exiting program.");

}


pub fn process_input(win: &mut PWindow, events: &GlfwReceiver<(f64, WindowEvent)>){
    for (_,event) in glfw::flush_messages(&events){
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                win.set_should_close(true);},
            glfw::WindowEvent::Key(Key::F5, _, Action::Press, _) => {
                set_polygon_mode(PolygonMode::Fill);},
            glfw::WindowEvent::Key(Key::F6, _, Action::Release, _) => {
                set_polygon_mode(PolygonMode::Line);},
            glfw::WindowEvent::Key(Key::F7, _, Action::Press, _) => {
                set_polygon_mode(PolygonMode::Point);},
            glfw::WindowEvent::Close => {
                win.set_should_close(true);},
            _ => (),
        }
    }
}