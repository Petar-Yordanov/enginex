use glow::HasContext;
use std::error::Error;

type EngineResult<T> = Result<T, Box<dyn Error>>;

pub struct GlBuffer {
    raw: glow::NativeBuffer,
    target: u32,
}

impl GlBuffer {
    pub fn new_static(
        gl: &glow::Context,
        target: u32,
        bytes: &[u8],
    ) -> EngineResult<Self> {
        let raw = unsafe {
            gl.create_buffer()
                .map_err(|error| format!("Failed to create OpenGL buffer: {error}"))?
        };

        unsafe {
            gl.bind_buffer(target, Some(raw));
            gl.buffer_data_u8_slice(target, bytes, glow::STATIC_DRAW);
            gl.bind_buffer(target, None);
        }

        Ok(Self { raw, target })
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_buffer(self.target, Some(self.raw));
        }
    }

    pub fn unbind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_buffer(self.target, None);
        }
    }

    pub fn delete(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_buffer(self.raw);
        }
    }
}