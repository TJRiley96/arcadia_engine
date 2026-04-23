#![allow(dead_code)]
use gl::types::*;
use glfw::Glfw;
use std::sync::{LazyLock, Mutex};

use crate::util::logging::{
    Logger,
    SystemType};

static SHADER_LOGGER: LazyLock<Mutex<Logger>> = LazyLock::new(|| Mutex::new(Logger::new(Some("shader"), Some(SystemType::SUBSYSTEM))));


pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
    }
}

/// Basic wrapper for a [Vertex Array
/// Object](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Array_Object).
pub struct VertexArray(pub GLuint);
impl VertexArray {
    pub fn new() -> Option<Self> {
        let mut vao: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        if vao != 0 {
            SHADER_LOGGER.lock().unwrap().add_info("Vertex Array Object created successfully.");
            Some(Self(vao))
        } else {
            SHADER_LOGGER.lock().unwrap().add_error("Failed to create Vertex Array Object.");
            None
        }

    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.0);
        }
        SHADER_LOGGER.lock().unwrap().add_info("Vertex Array Object bound.");
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
        SHADER_LOGGER.lock().unwrap().add_info("Vertex Array Object unbound.");
    }
}


pub enum BufferType {
    ArrayBuffer = gl::ARRAY_BUFFER as isize,
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER as isize,
}

pub struct Buffer(pub GLuint);
impl Buffer {
    pub fn new() -> Option<Self> {
        let mut vbo: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        if vbo != 0 {
            SHADER_LOGGER.lock().unwrap().add_info("Buffer created successfully.");
            Some(Self(vbo))
        } else {
            SHADER_LOGGER.lock().unwrap().add_error("Failed to create Buffer.");
            None
        }
    }

    pub fn bind(&self, target: BufferType) {
        unsafe {
            gl::BindBuffer(target as GLenum, self.0);
        }
        SHADER_LOGGER.lock().unwrap().add_info("Buffer bound.");
    }

    pub fn unbind(target: BufferType) {
        unsafe {
            gl::BindBuffer(target as GLenum, 0);
        }
        SHADER_LOGGER.lock().unwrap().add_info("Buffer unbound.");
    }
}

pub fn buffer_data(target: BufferType, data: &[u8], usage: GLenum) {
    unsafe {
        gl::BufferData(
            target as GLenum,
            data.len().try_into().unwrap(),
            data.as_ptr().cast(),
            usage
        );
    }
    SHADER_LOGGER.lock().unwrap().add_info("Buffer data set.");
}


pub enum ShaderType {
    VertexShader = gl::VERTEX_SHADER as isize,
    FragmentShader = gl::FRAGMENT_SHADER as isize,
    GeometryShader = gl::GEOMETRY_SHADER as isize,
}


/// Basic wrapper for OpenGL shader objects, with integrated logging for debugging purposes.
pub struct Shader(pub GLuint);
impl Shader {
    pub fn new(shader_type: ShaderType) -> Option<Self> {
        let shader_id = unsafe { gl::CreateShader(shader_type as GLenum) };
        if shader_id != 0 {
            SHADER_LOGGER.lock().unwrap().add_info("Shader created successfully.");
            Some(Self(shader_id))
        } else {
            SHADER_LOGGER.lock().unwrap().add_error("Failed to create Shader.");
            None
        }
    }

    // Set the shader source code and compile it, logging any errors that occur.
    pub fn set_source(&self, source: &str) {
        unsafe {
            gl::ShaderSource(
                self.0,
                1,
                &(source.as_bytes().as_ptr().cast()),
                &(source.len() as GLint)
            );
        }
        SHADER_LOGGER.lock().unwrap().add_info("Shader source set.");
    }


    pub fn compile(&self){
        unsafe {
            gl::CompileShader(self.0);
        }
        // Check for compilation errors
        if self.compile_success() {
            SHADER_LOGGER.lock().unwrap().add_info("Shader compiled successfully.");
        } else {
            self.get_info_log()
        }
    }

    pub fn compile_success(&self) -> bool {
        let mut success: i32 = 0;
        unsafe {
            gl::GetShaderiv(self.0, gl::COMPILE_STATUS, &mut success);
        }
        success != 0
    }

    /// Retrieves the shader compilation info log and logs any errors.
    pub fn get_info_log(&self) {
        let mut info_log: [u8; 512] = [0; 512];
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(self.0, gl::INFO_LOG_LENGTH, &mut len);
            gl::GetShaderInfoLog(
                self.0,
                512,
                &mut len,
                info_log.as_mut_ptr() as *mut gl::types::GLchar
            );
            SHADER_LOGGER.lock().unwrap().add_error(format!("Shader compilation failed: {}", std::str::from_utf8(&info_log).unwrap()).as_str());
        }
    }


    pub fn delete(&self) {
        unsafe {
            gl::DeleteShader(self.0);
        }
        SHADER_LOGGER.lock().unwrap().add_info("Shader deleted.");
    }

    pub fn from_source(source: &str, shader_type: ShaderType) -> Result<Self, String> {
        let shader = Self::new(shader_type)
        .ok_or_else(|| "Failed to create shader.".to_string())?;
        shader.set_source(source);
        shader.compile();
        if shader.compile_success() {
            Ok(shader)
        } else {
            shader.delete();
            Err("Shader compilation failed.".to_string())
        }
    }
}

/// Basic wrapper for OpenGL shader program objects, with integrated logging for debugging purposes.
pub struct ShaderProgram(pub GLuint);
impl ShaderProgram {
    pub fn new() -> Option<Self> {
        let program_id = unsafe { gl::CreateProgram() };
        if program_id != 0 {
            SHADER_LOGGER.lock().unwrap().add_info("Shader Program created successfully.");
            Some(Self(program_id))
        } else {
            SHADER_LOGGER.lock().unwrap().add_error("Failed to create Shader Program.");
            None
        }
    }

    pub fn attach_shader(&self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.0, shader.0);
        }
        SHADER_LOGGER.lock().unwrap().add_info("Shader attached to program.");
    }

    pub fn link_program(&self) {
        unsafe {
            gl::LinkProgram(self.0);
        }
        // Check for linking errors

        if self.link_success() {
            SHADER_LOGGER.lock().unwrap().add_info("Shader Program linked successfully.");
        } else {
            self.get_info_log();
        }
    }

    pub fn link_success(&self) -> bool {
        let mut success: i32 = 0;
        unsafe {
            gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut success);
        }
        success != 0
    }

    pub fn get_info_log(&self) {
        let mut info_log: [u8; 512] = [0; 512];
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut len);
            gl::GetProgramInfoLog(
                self.0,
                512,
                &mut len,
                info_log.as_mut_ptr() as *mut gl::types::GLchar
            );
            SHADER_LOGGER.lock().unwrap().add_error(format!("Shader Program linking failed: {}", std::str::from_utf8(&info_log).unwrap()).as_str());
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteProgram(self.0);
        }
        SHADER_LOGGER.lock().unwrap().add_info("Shader Program deleted.");
    }

    /// Creates a shader program from vertex and fragment shader source code, compiling and linking them together. Logs any errors that occur during the process.
    pub fn from_shaders(vert: &str, frag: &str) -> Result<Self, String> {
        let program = Self::new()
        .ok_or_else(|| "Failed to create shader program.".to_string())?;

        let vertex_shader = Shader::from_source(vert, ShaderType::VertexShader)
        .map_err(
            |e| format!("Vertex Shader compilation failed: {}", e))?;
        let fragment_shader = Shader::from_source(frag, ShaderType::FragmentShader)
        .map_err(
            |e| format!("Fragment Shader compilation failed: {}", e))?;

        program.attach_shader(&vertex_shader);
        program.attach_shader(&fragment_shader);
        program.link_program();
        vertex_shader.delete();
        fragment_shader.delete();

        if program.link_success() {
            Ok(program)
        } else {
            program.delete();
            Err("Shader Program linking failed.".to_string())
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> Option<GLint> {
        let c_name = std::ffi::CString::new(name).unwrap();
        let location = unsafe {
            gl::GetUniformLocation(self.0, c_name.as_ptr())
        };
        if location != -1 {
            Some(location)
        } else {
            SHADER_LOGGER.lock().unwrap().add_warning(format!("Uniform '{}' not found in shader program.", name).as_str());
            None
        }
    }

    // Set uniform values for the shader program. These functions can be expanded to support more types as needed.
    pub fn set_uniform_1f(&self, name: &str, value: f32) {
        if let Some(location) = self.get_uniform_location(name) {
            unsafe {
                gl::Uniform1f(location, value);
            }
        }
    }

    pub fn set_uniform_1i(&self, name: &str, value: i32) {
        if let Some(location) = self.get_uniform_location(name) {
            unsafe {
                gl::Uniform1i(location, value);
            }
        }
    }

    pub fn set_uniform_3f(&self, name: &str, v0: f32, v1: f32, v2: f32) {
        if let Some(location) = self.get_uniform_location(name) {
            unsafe {
                gl::Uniform3f(location, v0, v1, v2);
            }
        }
    }

    pub fn set_uniform_4f(&self, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        if let Some(location) = self.get_uniform_location(name) {
            unsafe {
                gl::Uniform4f(location, v0, v1, v2, v3);
            }
        }
    }

    pub fn set_uniform_4fv(&self, name: &str, count: GLsizei, value: &[f32]) {
        if let Some(location) = self.get_uniform_location(name) {
            unsafe {
                gl::Uniform4fv(location, count, value.as_ptr());
            }
        }
    }

    pub fn set_uniform_matrix4fv(&self, name: &str, count: GLsizei, transpose: GLboolean, value: &[f32]) {
        if let Some(location) = self.get_uniform_location(name) {
            unsafe {
                gl::UniformMatrix4fv(location, count, transpose, value.as_ptr());
            }
        }
    }
}

pub enum PolygonMode {
    Fill = gl::FILL as isize,
    Line = gl::LINE as isize,
    Point = gl::POINT as isize,
}

pub fn set_polygon_mode(mode: PolygonMode) {
    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, mode as GLenum);
    }
}

pub fn get_polygon_mode() -> PolygonMode {
    let mut mode: i32 = 0;
    unsafe {
        gl::GetIntegerv(gl::POLYGON_MODE, &mut mode);
    }
    match mode as GLenum {
        gl::FILL => PolygonMode::Fill,
        gl::LINE => PolygonMode::Line,
        gl::POINT => PolygonMode::Point,
        _ => {
            SHADER_LOGGER.lock().unwrap().add_warning(format!("Unknown polygon mode: {}", mode).as_str());
            PolygonMode::Fill
        }
    }
}

pub fn get_gl_time(glfw: &Glfw) -> f64 {
    glfw.get_time() as f64
}



