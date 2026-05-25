use engine_math::Transform;
use engine_render::{Color, Rect, TextureHandle};

#[derive(Debug, Clone)]
pub struct SpriteInstance {
    pub texture: TextureHandle,
    pub transform: Transform,
    pub tint: Color,
    pub source_rect: Option<Rect>,
    pub layer: i32,
}

impl SpriteInstance {
    pub fn new(texture: TextureHandle, transform: Transform) -> Self {
        Self {
            texture,
            transform,
            tint: Color::WHITE,
            source_rect: None,
            layer: 0,
        }
    }

    pub fn with_tint(mut self, tint: Color) -> Self {
        self.tint = tint;
        self
    }

    pub fn with_source_rect(mut self, source_rect: Rect) -> Self {
        self.source_rect = Some(source_rect);
        self
    }

    pub fn with_layer(mut self, layer: i32) -> Self {
        self.layer = layer;
        self
    }
}