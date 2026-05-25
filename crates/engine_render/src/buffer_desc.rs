#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferUsage {
    Vertex,
    Index,
    Uniform,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferMemoryUsage {
    Static,
    Dynamic,
    Stream,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferDesc {
    pub size_bytes: usize,
    pub usage: BufferUsage,
    pub memory_usage: BufferMemoryUsage,
}

impl BufferDesc {
    pub const fn new(size_bytes: usize, usage: BufferUsage, memory_usage: BufferMemoryUsage) -> Self {
        Self {
            size_bytes,
            usage,
            memory_usage,
        }
    }
}