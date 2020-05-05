use imgui::*;
use imgui_wgpu::Renderer;
use imgui_winit_support;
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
mod shader;
mod vertex_buffer;
pub use vertex_buffer::VertexBuffer;
mod camera;
mod context;
pub mod texture;
pub use context::Context;
pub mod pipelines;
pub mod renderpass;
pub mod uniforms;

async fn run(event_loop: EventLoop<()>, window: Window, swapchain_format: wgpu::TextureFormat) {
    let size = window.inner_size();
    let surface = wgpu::Surface::create(&window);

    let adapter = wgpu::Adapter::request(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            compatible_surface: Some(&surface),
        },
        wgpu::BackendBit::PRIMARY,
    )
    .await
    .unwrap();

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        })
        .await;

    let mut context = Context::new(adapter, device, queue, size, surface, swapchain_format);

    let depth_id = context.register_depth_texture("depth_texture");

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = context.device.create_swap_chain(&context.surface, &sc_desc);

    // IMGUI
    let mut hidpi_factor = 1.0;
    let mut imgui = imgui::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Default,
    );
    imgui.set_ini_filename(None);

    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(imgui::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    let mut renderer = Renderer::new(
        &mut imgui,
        &context.device,
        &mut context.queue,
        sc_desc.format,
        None,
    );

    let mut last_frame = Instant::now();

    let mut last_cursor = None;
    // END IMGUI

    let mut program = crate::modes::Program::new();
    program.init(&mut context);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { scale_factor, .. },
                ..
            } => {
                hidpi_factor = scale_factor;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                sc_desc.width = size.width;
                sc_desc.height = size.height;
                swap_chain = context.device.create_swap_chain(&context.surface, &sc_desc);
                context.textures[depth_id] =
                    texture::Texture::create_depth_texture(&context.device, size, "depth_texture");
            }
            Event::RedrawRequested(_) => {
                let frame = renderpass::get_frame(&mut swap_chain);

                last_frame = imgui.io_mut().update_delta_time(last_frame);
                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                let ui = imgui.frame();

                let should_continue = program.tick(&mut context, &frame, depth_id, &ui);
                if !should_continue {
                    *control_flow = ControlFlow::Exit;
                }

                // ImGui
                {
                    let mut encoder: wgpu::CommandEncoder = context
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    if last_cursor != Some(ui.mouse_cursor()) {
                        last_cursor = Some(ui.mouse_cursor());
                        platform.prepare_render(&ui, &window);
                    }
                    renderer
                        .render(ui.render(), &mut context.device, &mut encoder, &frame.view)
                        .expect("Rendering failed");

                    context.queue.submit(&[encoder.finish()]);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
        platform.handle_event(imgui.io_mut(), &window, &event);
    });
}

pub fn main_loop() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    // Temporarily avoid srgb formats for the swapchain on the web
    futures::executor::block_on(run(event_loop, window, wgpu::TextureFormat::Bgra8UnormSrgb));
}
