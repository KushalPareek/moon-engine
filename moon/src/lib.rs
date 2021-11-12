mod utils;
pub mod shader;
pub mod texture;
pub mod camera;
pub mod input;
pub mod transform;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use nalgebra::{Matrix4, Vector3};
use web_sys::HtmlCanvasElement as Canvas;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::{WebGlUniformLocation, HtmlImageElement};

pub use shader::create_shader;
pub use shader::create_program;
pub use texture::create_texture;
pub use camera::Camera;
pub use input::InputManager;
pub use transform::Transform;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn check_gl_error(gl: &GL) -> bool {
    let mut found_error = false;
    let mut gl_error = gl.get_error();
    while gl_error != GL::NO_ERROR {
        println!("OpenGL Error {}", gl_error);
        found_error = true;
        gl_error = gl.get_error();
    }
    found_error
}

/// The `Vertex` struct
/// 
/// The `Vertex` struct holds the data that will be later sent to WebGL in a `GL::ARRAY_BUFFER`.
/// It consists of position and color vectors, and UV co-ordinates.
#[repr(C)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
    uv: [f32; 2],
    normal: [f32; 3],
}


pub fn get_gl_context() -> Result<GL, String> {
    set_panic_hook();
    let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
    let canvas: Canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<Canvas>().unwrap();
    let context: GL = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<GL>().unwrap();
    Ok(context)
}

#[wasm_bindgen]
pub struct Application {
    gl: GL,
    camera: Camera,
    input: InputManager,
    u_time: Option<WebGlUniformLocation>,
    u_texture_0: Option<WebGlUniformLocation>,
    u_model_matrix: Option<WebGlUniformLocation>,
    u_view_matrix: Option<WebGlUniformLocation>,
    u_projection_matrix: Option<WebGlUniformLocation>,
    u_camera_position: Option<WebGlUniformLocation>,
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            gl: get_gl_context().unwrap(),
            camera: Camera::new(),
            input: InputManager::new(),
            u_time: None,
            u_texture_0: None,
            u_model_matrix: None,
            u_view_matrix: None,
            u_projection_matrix: None,
            u_camera_position: None,
        }
    }
    
    #[wasm_bindgen]
    pub fn init(&mut self) {
        let gl = &self.gl;
        gl.clear_color(0.0, 0.11, 0.2, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT|GL::DEPTH_BUFFER_BIT);

        let vertex_shader = create_shader(gl, GL::VERTEX_SHADER, include_str!("..\\res\\shader\\default.vert.glsl")).expect("Could not create Vertex Shader!");
        let fragment_shader = create_shader(gl, GL::FRAGMENT_SHADER, include_str!("..\\res\\shader\\default.frag.glsl")).expect("Could not create Fragment Shader!");
        
        let program = create_program(&gl, &vertex_shader, &fragment_shader).expect("Failed while creating Program!");
        gl.use_program(Some(&program));
        
        gl.delete_shader(Some(&vertex_shader));
        gl.delete_shader(Some(&fragment_shader));

        let u_time = gl.get_uniform_location(&program, "uTime");
        self.u_time = u_time;
        let u_texture_0 = gl.get_uniform_location(&program, "uTex0");
        let u_texture_1 = gl.get_uniform_location(&program, "uTex1");
        

        let u_model_matrix = gl.get_uniform_location(&program, "uModel");
        self.u_model_matrix = u_model_matrix;
        let u_view_matrix = gl.get_uniform_location(&program, "uView");
        self.u_view_matrix = u_view_matrix;
        let u_projection_matrix = gl.get_uniform_location(&program, "uProj");
        self.u_projection_matrix = u_projection_matrix;

        let u_camera_position = gl.get_uniform_location(&program, "uCamPos");
        self.u_camera_position = u_camera_position;

        let position_attrib_location = gl.get_attrib_location(&program, "aPosition");
        let vcolor_attrib_location = gl.get_attrib_location(&program, "aColor");
        let uv_attrib_location = gl.get_attrib_location(&program, "aTexCoord");
        let normal_attrib_location = gl.get_attrib_location(&program, "aNormal");
        
        let vao = gl.create_vertex_array().expect("Could not create Vertex Array Object.");
        gl.bind_vertex_array(Some(&vao));
        
        let vbo = gl.create_buffer().expect("Could not create Buffer Object.");
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));

        let ibo = gl.create_buffer().expect("Could not create Index Buffer Object.");
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ibo));
        
        let vertices = vec![
            // Bottom Side
            Vertex {
                position: [-0.5, 0.0, 0.5],
                color: [0.7, 0.3, 0.7],
                uv: [0.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.0, -0.5],
                color: [0.5, 0.2, 0.0],
                uv: [0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.0, -0.5],
                color: [0.8, 0.6, 0.0],
                uv: [1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.0, 0.5],
                color: [0.0, 0.4, 0.8],
                uv: [1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
        ];
        let indices : [u8; 6] = [
            0, 1, 2,
            0, 2, 3,
            ];
        let u8_slice = unsafe {
            std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8, vertices.len()*std::mem::size_of::<Vertex>()
            )
        };
        gl.buffer_data_with_u8_array(GL::ARRAY_BUFFER, u8_slice, GL::STATIC_DRAW);
        gl.buffer_data_with_u8_array(GL::ELEMENT_ARRAY_BUFFER, &indices, GL::STATIC_DRAW);
    
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 11 * 4, 0);
        gl.vertex_attrib_pointer_with_i32(1, 3, GL::FLOAT, false, 11 * 4, 12);
        gl.vertex_attrib_pointer_with_i32(2, 2, GL::FLOAT, false, 11 * 4, 24);
        gl.vertex_attrib_pointer_with_i32(3, 3, GL::FLOAT, false, 11 * 4, 32);

        gl.enable_vertex_attrib_array(position_attrib_location as u32);
        gl.enable_vertex_attrib_array(vcolor_attrib_location as u32);
        gl.enable_vertex_attrib_array(uv_attrib_location as u32);
        gl.enable_vertex_attrib_array(normal_attrib_location as u32);

        let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
        let img1 = document.get_element_by_id("texture0").unwrap().dyn_into::<HtmlImageElement>().unwrap();
        let _texture_alb = create_texture(gl, &img1, 0).expect("Failed to create Texture");
        let img2 = document.get_element_by_id("texture1").unwrap().dyn_into::<HtmlImageElement>().unwrap();
        let _texture_spec = create_texture(gl, &img2, 1).expect("Failed to create Texture");
        
        let initial_camera_position: Vector3<f32> = -Vector3::z()*2.0 - Vector3::y()*0.5;
        self.camera = Camera::with_position(initial_camera_position);
        let model: Matrix4<f32> = Matrix4::identity();
        gl.uniform1i(u_texture_0.as_ref(), 0);
        gl.uniform1i(u_texture_1.as_ref(), 0);
        gl.uniform_matrix4fv_with_f32_array(self.u_model_matrix.as_ref(), false, model.as_slice());
        gl.uniform_matrix4fv_with_f32_array(self.u_view_matrix.as_ref(), false, self.camera.transform.matrix());
        gl.uniform_matrix4fv_with_f32_array(self.u_projection_matrix.as_ref(), false, self.camera.projection());
        gl.enable(GL::DEPTH_TEST);
    }
    
    #[wasm_bindgen]
    pub fn resize(&mut self, width: f32, height: f32) {
        self.camera.set_width_and_height(width, height);
        self.gl.viewport(0, 0, width as i32, height as i32);
        self.gl.uniform_matrix4fv_with_f32_array(self.u_projection_matrix.as_ref(), false, self.camera.projection());
    }

    #[wasm_bindgen]
    pub fn input(&mut self, key_code: u8, is_down: bool) {
        if is_down {
            self.input.key_down(key_code);
        } else {
            self.input.key_up(key_code);
        }
    }

    #[wasm_bindgen]
    pub fn mouse_move(&mut self, mouse_x: f32, mouse_y: f32) {
        self.input.mouse_x = mouse_x / self.camera.width;
        self.input.mouse_y = mouse_y / self.camera.height;
    }
    
    #[wasm_bindgen]
    pub fn render(&mut self, delta_time: u32) {
        let sensitivity = 0.05;
        let gl = &self.gl;
        if self.input.get_key_state('W' as u8) {
            self.camera.transform.translate(&(Vector3::z() * sensitivity));
        }
        if self.input.get_key_state('A' as u8) {
            self.camera.transform.translate(&(Vector3::x() * sensitivity));
        }
        if self.input.get_key_state('S' as u8) {
            self.camera.transform.translate(&(-Vector3::z() * sensitivity));
        }
        if self.input.get_key_state('D' as u8) {
            self.camera.transform.translate(&(-Vector3::x() * sensitivity));
        }
        if self.input.get_key_state('Q' as u8) {
            self.camera.transform.translate(&(Vector3::y() * sensitivity));
        }
        if self.input.get_key_state('E' as u8) {
            self.camera.transform.translate(&(-Vector3::y() * sensitivity));
        }
        gl.clear(GL::COLOR_BUFFER_BIT|GL::DEPTH_BUFFER_BIT);
        gl.uniform3fv_with_f32_array(self.u_camera_position.as_ref(), self.camera.transform.get_position());
        gl.uniform_matrix4fv_with_f32_array(self.u_view_matrix.as_ref(), false, self.camera.transform.matrix());
        gl.uniform1f(self.u_time.as_ref(), delta_time as f32 * 0.001);
        gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_BYTE, 0);
    }
}