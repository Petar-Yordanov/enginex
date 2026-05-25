use crate::gl_buffer::GlBuffer;
use glow::HasContext;
use std::error::Error;

type EngineResult<T> = Result<T, Box<dyn Error>>;

pub struct Quad {
    vao: glow::NativeVertexArray,
    vbo: GlBuffer,
    ebo: GlBuffer,
}

impl Quad {
    pub fn new(gl: &glow::Context) -> EngineResult<Self> {
        #[rustfmt::skip]
        let vertices: [f32; 20] = [
            // position          // uv
             0.5,  0.5, 0.0,    1.0, 1.0,
             0.5, -0.5, 0.0,    1.0, 0.0,
            -0.5, -0.5, 0.0,    0.0, 0.0,
            -0.5,  0.5, 0.0,    0.0, 1.0,
        ];

        let indices: [u32; 6] = [
            0, 1, 3,
            1, 2, 3,
        ];

        let vao = unsafe {
            gl.create_vertex_array()
                .map_err(|error| format!("Failed to create quad VAO: {error}"))?
        };

        let vbo = GlBuffer::new_static(
            gl,
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&vertices),
        )?;

        let ebo = GlBuffer::new_static(
            gl,
            glow::ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(&indices),
        )?;

        unsafe {
            gl.bind_vertex_array(Some(vao));

            vbo.bind(gl);
            ebo.bind(gl);

            let stride = 5 * size_of::<f32>() as i32;

            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);

            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(
                1,
                2,
                glow::FLOAT,
                false,
                stride,
                3 * size_of::<f32>() as i32,
            );

            gl.bind_vertex_array(None);
            vbo.unbind(gl);
        }

        Ok(Self { vao, vbo, ebo })
    }

    pub fn draw(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }
    }

    pub fn delete(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_vertex_array(self.vao);
        }

        self.vbo.delete(gl);
        self.ebo.delete(gl);
    }
}