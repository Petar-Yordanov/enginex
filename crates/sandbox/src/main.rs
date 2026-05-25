mod gl_context;

use engine_math::Transform;
use engine_render::Color;
use engine_render_2d::{Renderer2D, SpriteInstance};
use gl_context::GlWindow;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::Instant;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};

type EngineResult<T> = Result<T, Box<dyn Error>>;

fn main() -> EngineResult<()> {
    let event_loop = EventLoop::new()?;

    let mut gl_window = GlWindow::new(&event_loop)?;
    let mut renderer_2d = Renderer2D::new(gl_window.gl())?;

    let texture_path = demo_texture_path()?;
    let checker_texture = renderer_2d.load_texture(gl_window.gl(), texture_path)?;

    let start_time = Instant::now();

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run(move |event, event_loop| {
        match event {
            Event::WindowEvent { window_id, event }
                if window_id == gl_window.window().id() =>
            {
                match event {
                    WindowEvent::CloseRequested => {
                        renderer_2d.delete(gl_window.gl());
                        event_loop.exit();
                    }

                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                state: ElementState::Pressed,
                                repeat: false,
                                ..
                            },
                        ..
                    } => {
                        renderer_2d.delete(gl_window.gl());
                        event_loop.exit();
                    }

                    WindowEvent::Resized(size) => {
                        gl_window.resize(size);
                    }

                    WindowEvent::RedrawRequested => {
                        gl_window.clear();

                        let elapsed = start_time.elapsed().as_secs_f32();
                        let sprites = demo_sprites(checker_texture, elapsed);
                        let window_size = gl_window.window().inner_size();

                        renderer_2d.render(
                            gl_window.gl(),
                            &sprites,
                            window_size.width,
                            window_size.height,
                        );

                        if let Err(error) = gl_window.swap_buffers() {
                            eprintln!("Failed to swap OpenGL buffers: {error}");
                            renderer_2d.delete(gl_window.gl());
                            event_loop.exit();
                        }
                    }

                    _ => {}
                }
            }

            Event::AboutToWait => {
                gl_window.window().request_redraw();
            }

            _ => {}
        }
    })?;

    Ok(())
}

fn demo_texture_path() -> EngineResult<PathBuf> {
    let texture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("checker.png");

    if !texture_path.exists() {
        return Err(format!("Demo texture not found: {}", texture_path.display()).into());
    }

    Ok(texture_path)
}

fn demo_sprites(
    checker_texture: engine_render::TextureHandle,
    elapsed: f32,
) -> Vec<SpriteInstance> {
    vec![
        SpriteInstance::new(
            checker_texture,
            Transform::from_xy(-0.65, 0.25)
                .with_scale_xy(0.35, 0.35)
                .with_rotation_z(elapsed * 1.2),
        )
        .with_tint(Color::WHITE)
        .with_layer(2),

        SpriteInstance::new(
            checker_texture,
            Transform::from_xy(0.0, 0.0)
                .with_scale_xy(0.5, 0.5)
                .with_rotation_z(-elapsed * 0.7),
        )
        .with_tint(Color::rgba(0.4, 1.0, 0.65, 0.85))
        .with_layer(1),

        SpriteInstance::new(
            checker_texture,
            Transform::from_xy(0.65, -0.25)
                .with_scale_xy(0.3, 0.3)
                .with_rotation_z(elapsed * 1.8),
        )
        .with_tint(Color::rgba(0.6, 0.75, 1.0, 1.0))
        .with_layer(3),

        SpriteInstance::new(
            checker_texture,
            Transform::from_xy(0.0, -0.55)
                .with_scale_xy(0.9, 0.18)
                .with_rotation_z(0.0),
        )
        .with_tint(Color::rgba(1.0, 0.9, 0.25, 0.65))
        .with_layer(0),
    ]
}