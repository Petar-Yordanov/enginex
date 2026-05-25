use crate::gl_texture::GlTexture;
use engine_render::{TextureDesc, TextureHandle};
use image::ImageReader;
use std::error::Error;
use std::path::Path;

type EngineResult<T> = Result<T, Box<dyn Error>>;

pub struct Texture {
    handle: TextureHandle,
    texture: GlTexture,
}

impl Texture {
    pub fn from_file(
        gl: &glow::Context,
        handle: TextureHandle,
        path: impl AsRef<Path>,
    ) -> EngineResult<Self> {
        let path = path.as_ref();

        let image = ImageReader::open(path)
            .map_err(|error| format!("Failed to open texture '{}': {error}", path.display()))?
            .decode()
            .map_err(|error| format!("Failed to decode texture '{}': {error}", path.display()))?
            .flipv()
            .into_rgba8();

        let desc = TextureDesc::rgba8(image.width(), image.height());
        let texture = GlTexture::from_rgba8(gl, desc, image.as_raw())?;

        Ok(Self { handle, texture })
    }

    pub fn handle(&self) -> TextureHandle {
        self.handle
    }

    pub fn bind_to_unit(&self, gl: &glow::Context, unit: u32) {
        self.texture.bind_to_unit(gl, unit);
    }

    pub fn delete(&self, gl: &glow::Context) {
        self.texture.delete(gl);
    }
}