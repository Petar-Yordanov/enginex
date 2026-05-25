#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    Rgba8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFilter {
    Nearest,
    Linear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureWrap {
    Repeat,
    ClampToEdge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureDesc {
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub min_filter: TextureFilter,
    pub mag_filter: TextureFilter,
    pub wrap_x: TextureWrap,
    pub wrap_y: TextureWrap,
}

impl TextureDesc {
    pub const fn rgba8(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            format: TextureFormat::Rgba8,
            min_filter: TextureFilter::Nearest,
            mag_filter: TextureFilter::Nearest,
            wrap_x: TextureWrap::Repeat,
            wrap_y: TextureWrap::Repeat,
        }
    }
}