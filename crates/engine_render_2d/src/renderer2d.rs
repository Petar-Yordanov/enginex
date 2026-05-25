use crate::SpriteInstance;
use engine_render::TextureHandle;
use engine_render_gl::gl_handles::GlHandleAllocator;
use engine_render_gl::{Quad, Shader, Texture};
use glam::Mat4;
use glow::HasContext;
use std::error::Error;
use std::path::Path;

type EngineResult<T> = Result<T, Box<dyn Error>>;

pub struct Renderer2D {
    handles: GlHandleAllocator,
    shader: Shader,
    quad: Quad,
    textures: Vec<Texture>,
}

impl Renderer2D {
    pub fn new(gl: &glow::Context) -> EngineResult<Self> {
        unsafe {
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        }

        let shader = Shader::new(gl, SPRITE_VERTEX_SHADER, SPRITE_FRAGMENT_SHADER)?;
        let quad = Quad::new(gl)?;

        Ok(Self {
            handles: GlHandleAllocator::new(),
            shader,
            quad,
            textures: Vec::new(),
        })
    }

    pub fn load_texture(
        &mut self,
        gl: &glow::Context,
        path: impl AsRef<Path>,
    ) -> EngineResult<TextureHandle> {
        let handle = self.handles.alloc_texture();
        let texture = Texture::from_file(gl, handle, path)?;
        self.textures.push(texture);

        Ok(handle)
    }

    pub fn render(
        &self,
        gl: &glow::Context,
        sprites: &[SpriteInstance],
        window_width: u32,
        window_height: u32,
    ) {
        let aspect = window_width as f32 / window_height.max(1) as f32;

        let projection = Mat4::orthographic_rh_gl(
            -aspect,
            aspect,
            -1.0,
            1.0,
            -1.0,
            1.0,
        );

        self.shader.bind(gl);
        self.shader.set_mat4(gl, "u_projection", projection);
        self.shader.set_i32(gl, "u_texture", 0);

        let mut sorted_sprites: Vec<&SpriteInstance> = sprites.iter().collect();
        sorted_sprites.sort_by_key(|sprite| sprite.layer);

        for sprite in sorted_sprites {
            let Some(texture) = self.texture(sprite.texture) else {
                continue;
            };

            self.shader
                .set_mat4(gl, "u_transform", sprite.transform.matrix());
            self.shader.set_color(gl, "u_tint", sprite.tint);

            texture.bind_to_unit(gl, 0);
            self.quad.draw(gl);
        }
    }

    pub fn delete(&self, gl: &glow::Context) {
        self.shader.delete(gl);
        self.quad.delete(gl);

        for texture in &self.textures {
            texture.delete(gl);
        }
    }

    fn texture(&self, handle: TextureHandle) -> Option<&Texture> {
        self.textures
            .iter()
            .find(|texture| texture.handle() == handle)
    }
}

const SPRITE_VERTEX_SHADER: &str = r#"#version 330 core

layout (location = 0) in vec3 a_position;
layout (location = 1) in vec2 a_uv;

uniform mat4 u_projection;
uniform mat4 u_transform;

out vec2 v_uv;

void main()
{
    v_uv = a_uv;
    gl_Position = u_projection * u_transform * vec4(a_position, 1.0);
}
"#;

const SPRITE_FRAGMENT_SHADER: &str = r#"#version 330 core

in vec2 v_uv;

uniform sampler2D u_texture;
uniform vec4 u_tint;

out vec4 frag_color;

void main()
{
    frag_color = texture(u_texture, v_uv) * u_tint;
}
"#;