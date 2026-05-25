use glow::HasContext;
use std::error::Error;

type EngineResult<T> = Result<T, Box<dyn Error>>;

pub struct GlShader {
    raw: glow::NativeShader,
}

impl GlShader {
    pub fn compile(
        gl: &glow::Context,
        shader_type: u32,
        source: &str,
    ) -> EngineResult<Self> {
        let raw = unsafe {
            let shader = gl
                .create_shader(shader_type)
                .map_err(|error| format!("Failed to create OpenGL shader: {error}"))?;

            gl.shader_source(shader, source);
            gl.compile_shader(shader);

            if !gl.get_shader_compile_status(shader) {
                let log = gl.get_shader_info_log(shader);
                gl.delete_shader(shader);

                return Err(format!("OpenGL shader compile failed:\n{log}").into());
            }

            shader
        };

        Ok(Self { raw })
    }

    pub fn raw(&self) -> glow::NativeShader {
        self.raw
    }

    pub fn delete(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_shader(self.raw);
        }
    }
}