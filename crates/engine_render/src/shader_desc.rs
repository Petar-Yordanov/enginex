#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderStage {
    Vertex,
    Fragment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShaderSource<'a> {
    pub stage: ShaderStage,
    pub source: &'a str,
}

impl<'a> ShaderSource<'a> {
    pub const fn vertex(source: &'a str) -> Self {
        Self {
            stage: ShaderStage::Vertex,
            source,
        }
    }

    pub const fn fragment(source: &'a str) -> Self {
        Self {
            stage: ShaderStage::Fragment,
            source,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShaderDesc<'a> {
    pub vertex: ShaderSource<'a>,
    pub fragment: ShaderSource<'a>,
}

impl<'a> ShaderDesc<'a> {
    pub const fn new(vertex_source: &'a str, fragment_source: &'a str) -> Self {
        Self {
            vertex: ShaderSource::vertex(vertex_source),
            fragment: ShaderSource::fragment(fragment_source),
        }
    }
}