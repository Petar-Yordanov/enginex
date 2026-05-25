use glow::HasContext;
use glutin::config::{ConfigTemplateBuilder, GlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext, Version,
};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{
    GlSurface, Surface, SurfaceAttributesBuilder, SwapInterval, WindowSurface,
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use std::error::Error;
use std::ffi::CString;
use std::num::NonZeroU32;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowAttributes};

type EngineResult<T> = Result<T, Box<dyn Error>>;

pub struct GlWindow {
    window: Window,
    gl: glow::Context,
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext,
}

impl GlWindow {
    pub fn new(event_loop: &EventLoop<()>) -> EngineResult<Self> {
        let window_attributes = WindowAttributes::default()
            .with_title("enginex")
            .with_inner_size(PhysicalSize::new(1280, 720));

        let config_template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_depth_size(24)
            .with_stencil_size(8);

        let display_builder =
            DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        let (window, gl_config) =
            display_builder.build(event_loop, config_template, |configs| {
                configs
                    .max_by_key(|config| config.num_samples())
                    .expect("No OpenGL configurations were found")
            })?;

        let window = window.expect("Window creation failed");

        let raw_window_handle = window.window_handle()?.as_raw();
        let gl_display = gl_config.display();

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(Some(raw_window_handle));

        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(Some(raw_window_handle));

        let not_current_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .or_else(|_| gl_display.create_context(&gl_config, &fallback_context_attributes))?
        };

        let initial_size = window.inner_size();

        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            non_zero_u32(initial_size.width),
            non_zero_u32(initial_size.height),
        );

        let surface = unsafe { gl_display.create_window_surface(&gl_config, &surface_attributes)? };

        let context = not_current_context.make_current(&surface)?;

        let gl = unsafe {
            glow::Context::from_loader_function(|symbol| {
                let symbol =
                    CString::new(symbol).expect("OpenGL symbol contained an unexpected null byte");

                gl_display.get_proc_address(&symbol) as *const _
            })
        };

        let _ = surface.set_swap_interval(&context, SwapInterval::Wait(non_zero_u32(1)));

        let gl_window = Self {
            window,
            gl,
            surface,
            context,
        };

        gl_window.update_viewport(initial_size);

        Ok(gl_window)
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn gl(&self) -> &glow::Context {
        &self.gl
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.surface.resize(
            &self.context,
            non_zero_u32(size.width),
            non_zero_u32(size.height),
        );

        self.update_viewport(size);
    }

    pub fn clear(&self) {
        unsafe {
            self.gl.clear_color(0.08, 0.10, 0.14, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn swap_buffers(&self) -> EngineResult<()> {
        self.surface.swap_buffers(&self.context)?;
        Ok(())
    }

    fn update_viewport(&self, size: PhysicalSize<u32>) {
        unsafe {
            self.gl.viewport(
                0,
                0,
                size.width.max(1) as i32,
                size.height.max(1) as i32,
            );
        }
    }
}

fn non_zero_u32(value: u32) -> NonZeroU32 {
    NonZeroU32::new(value.max(1)).expect("value was clamped to be non-zero")
}