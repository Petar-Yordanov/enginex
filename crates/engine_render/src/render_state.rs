#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    None,
    Alpha,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthMode {
    Disabled,
    ReadWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    None,
    Back,
    Front,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderState {
    pub blend: BlendMode,
    pub depth: DepthMode,
    pub cull: CullMode,
}

impl RenderState {
    pub const ALPHA_2D: Self = Self {
        blend: BlendMode::Alpha,
        depth: DepthMode::Disabled,
        cull: CullMode::None,
    };
}

impl Default for RenderState {
    fn default() -> Self {
        Self::ALPHA_2D
    }
}