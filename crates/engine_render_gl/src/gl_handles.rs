use engine_render::{BufferHandle, RenderTargetHandle, ShaderHandle, TextureHandle};

#[derive(Debug, Default)]
pub struct GlHandleAllocator {
    next_buffer: u32,
    next_texture: u32,
    next_shader: u32,
    next_render_target: u32,
}

impl GlHandleAllocator {
    pub fn new() -> Self {
        Self {
            next_buffer: 1,
            next_texture: 1,
            next_shader: 1,
            next_render_target: 1,
        }
    }

    pub fn alloc_buffer(&mut self) -> BufferHandle {
        let handle = BufferHandle(self.next_buffer);
        self.next_buffer += 1;
        handle
    }

    pub fn alloc_texture(&mut self) -> TextureHandle {
        let handle = TextureHandle(self.next_texture);
        self.next_texture += 1;
        handle
    }

    pub fn alloc_shader(&mut self) -> ShaderHandle {
        let handle = ShaderHandle(self.next_shader);
        self.next_shader += 1;
        handle
    }

    pub fn alloc_render_target(&mut self) -> RenderTargetHandle {
        let handle = RenderTargetHandle(self.next_render_target);
        self.next_render_target += 1;
        handle
    }
}