use crate::gl_shader::GlShader;
use engine_render::Color;
use glam::Mat4;
use glow::HasContext;
use std::error::Error;

type EngineResult<T> = Result<T, Box<dyn Error>>;

pub struct Shader {
    program: glow::NativeProgram,
}

impl Shader {
    pub fn new(
        gl: &glow::Context,
        vertex_source: &str,
        fragment_source: &str,
    ) -> EngineResult<Self> {
        let vertex_shader = GlShader::compile(gl, glow::VERTEX_SHADER, vertex_source)?;
        let fragment_shader = GlShader::compile(gl, glow::FRAGMENT_SHADER, fragment_source)?;

        let program = unsafe {
            let program = gl
                .create_program()
                .map_err(|error| format!("Failed to create OpenGL shader program: {error}"))?;

            gl.attach_shader(program, vertex_shader.raw());
            gl.attach_shader(program, fragment_shader.raw());
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                let log = gl.get_program_info_log(program);

                gl.detach_shader(program, vertex_shader.raw());
                gl.detach_shader(program, fragment_shader.raw());

                vertex_shader.delete(gl);
                fragment_shader.delete(gl);

                gl.delete_program(program);

                return Err(format!("OpenGL shader program link failed:\n{log}").into());
            }

            gl.detach_shader(program, vertex_shader.raw());
            gl.detach_shader(program, fragment_shader.raw());

            vertex_shader.delete(gl);
            fragment_shader.delete(gl);

            program
        };

        Ok(Self { program })
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }

    pub fn set_i32(&self, gl: &glow::Context, name: &str, value: i32) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_1_i32(location.as_ref(), value);
        }
    }

    pub fn set_color(&self, gl: &glow::Context, name: &str, color: Color) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_4_f32(location.as_ref(), color.r, color.g, color.b, color.a);
        }
    }

    pub fn set_mat4(&self, gl: &glow::Context, name: &str, value: Mat4) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_matrix_4_f32_slice(location.as_ref(), false, &value.to_cols_array());
        }
    }

    pub fn delete(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.program);
        }
    }
}