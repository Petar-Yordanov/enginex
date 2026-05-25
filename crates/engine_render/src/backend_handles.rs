#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferHandle(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureHandle(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderHandle(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderTargetHandle(pub u32);

impl BufferHandle {
    pub const INVALID: Self = Self(0);
}

impl TextureHandle {
    pub const INVALID: Self = Self(0);
}

impl ShaderHandle {
    pub const INVALID: Self = Self(0);
}

impl RenderTargetHandle {
    pub const INVALID: Self = Self(0);
}