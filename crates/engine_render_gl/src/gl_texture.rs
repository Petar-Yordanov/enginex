use engine_render::{TextureDesc, TextureFilter, TextureWrap};
use glow::HasContext;
use std::error::Error;

type EngineResult<T> = Result<T, Box<dyn Error>>;

pub struct GlTexture {
    raw: glow::NativeTexture,
    desc: TextureDesc,
}

impl GlTexture {
    pub fn from_rgba8(
        gl: &glow::Context,
        desc: TextureDesc,
        pixels: &[u8],
    ) -> EngineResult<Self> {
        let raw = unsafe {
            gl.create_texture()
                .map_err(|error| format!("Failed to create OpenGL texture: {error}"))?
        };

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(raw));

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                gl_wrap(desc.wrap_x) as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                gl_wrap(desc.wrap_y) as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                gl_filter(desc.min_filter) as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                gl_filter(desc.mag_filter) as i32,
            );

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                desc.width as i32,
                desc.height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(Some(pixels)),
            );

            gl.generate_mipmap(glow::TEXTURE_2D);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }

        Ok(Self { raw, desc })
    }

    pub fn bind_to_unit(&self, gl: &glow::Context, unit: u32) {
        unsafe {
            gl.active_texture(glow::TEXTURE0 + unit);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.raw));
        }
    }

    pub fn desc(&self) -> TextureDesc {
        self.desc
    }

    pub fn delete(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_texture(self.raw);
        }
    }
}

fn gl_filter(filter: TextureFilter) -> u32 {
    match filter {
        TextureFilter::Nearest => glow::NEAREST,
        TextureFilter::Linear => glow::LINEAR,
    }
}

fn gl_wrap(wrap: TextureWrap) -> u32 {
    match wrap {
        TextureWrap::Repeat => glow::REPEAT,
        TextureWrap::ClampToEdge => glow::CLAMP_TO_EDGE,
    }
}