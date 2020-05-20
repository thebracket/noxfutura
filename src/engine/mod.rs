pub mod support;
pub mod shader;
mod shaders;
pub use shader::Shader;
mod imgui_glutin;
mod texture;
pub use texture::Texture;
pub mod glerror;
pub mod vertex_buffer;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use parking_lot::Mutex;
use imgui::*;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;
use crate::modes::Program;
use crate::opengl::gl_error;

lazy_static! {
    pub(crate) static ref GL: Mutex<Backend> = Mutex::new(Backend::new());
}

pub struct Backend {
    pub gl : Option<support::gl::Gles2>
}

impl Backend {
    pub fn new() -> Self {
        Backend { gl : None }
    }
}

pub fn main_loop() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Nox Futura - One Day, I Can Dream.");

    let windowed_context =
        ContextBuilder::new()
            .with_gl(glutin::GlRequest::Latest)
            .with_gl_profile(glutin::GlProfile::Core)
            .build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    println!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );

    {
        let gl = support::load(&windowed_context.context());
        let mut glock = GL.lock();
        glock.gl = Some(gl.gl);
    }
    let shader = shaders::load();

    // IMGUI
    let mut imgui = Context::create();
    let mut platform = WinitPlatform::init(&mut imgui);
    {
        platform.attach_window(imgui.io_mut(), windowed_context.window(), HiDpiMode::Rounded);
    }
    let io = imgui.io_mut();
    let sz = windowed_context.window().inner_size();
    io.display_size = [ sz.width as f32, sz.height as f32 ];
    imgui.set_ini_filename(None);

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("../../resources/mplus-1p-regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges: FontGlyphRanges::japanese(),
                ..FontConfig::default()
            }),
        },
    ]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
    let renderer = imgui_glutin::Renderer::new(&mut imgui);
    // END IMGUI

    // Program Setup
    let mut game = Program::new();
    game.init(GL.lock().gl.as_ref().unwrap());

    let mut last_frame = Instant::now();

    el.run(move |event, _, control_flow| {
        //println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(_) => last_frame = imgui.io_mut().update_delta_time(last_frame),
            Event::MainEventsCleared => {
                platform
                    .prepare_frame(imgui.io_mut(), windowed_context.window())
                    .expect("Failed to prepare frame");
                    windowed_context.window().request_redraw();
            }
            Event::LoopDestroyed => {},
            Event::RedrawRequested(_) => {
                shader.activate_selflock();
                //gl.draw_frame([1.0, 0.5, 0.7, 1.0]);

                let ui = imgui.frame();
                render(&mut game, &ui);

                platform.prepare_render(&ui, windowed_context.window());

                renderer
                    .render(ui);

                windowed_context.swap_buffers().unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                platform.handle_event(imgui.io_mut(), windowed_context.window(), &event);
            }
        }
    });
}

fn render(game: &mut Program, imgui: &imgui::Ui) {
    let glock = GL.lock();
    let gl = glock.gl.as_ref().unwrap();
    unsafe {
        gl_error(gl);
        gl.ClearColor(0.0, 0.0, 0.0, 0.0);
        gl.Clear(support::gl::COLOR_BUFFER_BIT);
    }
    gl_error(gl);
    game.tick(imgui, gl);
    gl_error(gl);
}