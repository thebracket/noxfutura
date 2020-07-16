use imgui::*;
use imgui_wgpu::Renderer;
use imgui_winit_support;
pub use nox_wgpu_utils::*;
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn run(event_loop: EventLoop<()>, window: Window, swapchain_format: wgpu::TextureFormat) {
    use futures::executor::block_on;

    let size = window.inner_size();
    let surface = wgpu::Surface::create(&window);

    let adapter = block_on(wgpu::Adapter::request(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        },
        wgpu::BackendBit::VULKAN,
    ))
    .unwrap();

    let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: true,
        },
        limits: wgpu::Limits::default(),
    }));

    let mut ctx = DEVICE_CONTEXT.write();
    *ctx = Some(nox_wgpu_utils::Context::new(
        adapter,
        device,
        queue,
        size,
        surface,
        swapchain_format,
    ));
    let context = ctx.as_mut().unwrap();

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
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("../../resources/fontawesome-webfont.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                glyph_ranges: FontGlyphRanges::from_slice(&[0xf000, 0xf2e0, 0]),
                ..FontConfig::default()
            }),
        },
    ]);

    let mut renderer = Renderer::new(
        &mut imgui,
        &context.device,
        &mut context.queue,
        sc_desc.format,
        None,
    );
    std::mem::drop(ctx);

    let mut last_frame = Instant::now();

    let mut last_cursor = None;
    // END IMGUI

    let mut program = crate::modes::Program::new();
    program.init();

    let mut keycode: Option<winit::event::VirtualKeyCode> = None;
    let mut mouse_world_pos = (0usize, 0usize, 0usize);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::NewEvents(_) => last_frame = imgui.io_mut().update_delta_time(last_frame),
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
                let mut context_lock = DEVICE_CONTEXT.write();
                let context = context_lock.as_mut().unwrap();
                sc_desc.width = size.width;
                sc_desc.height = size.height;
                swap_chain = context.device.create_swap_chain(&context.surface, &sc_desc);
                context.textures[depth_id] =
                    texture::Texture::create_depth_texture(&context.device, size, "depth_texture");
                context.size = size;
                std::mem::drop(context_lock);
                program.on_resize();
            }
            Event::RedrawRequested(_) => {
                let frame = renderpass::get_frame(&mut swap_chain);

                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                let mouse_position = imgui.io().mouse_pos;
                let ui = imgui.frame();

                let should_continue =
                    program.tick(&frame, depth_id, &ui, keycode, &mouse_world_pos);
                if !should_continue {
                    *control_flow = ControlFlow::Exit;
                }
                keycode = None;

                // Mouse buffer insanity
                if let Some(buf) = program.get_mouse_buffer() {
                    let mut context_lock = DEVICE_CONTEXT.write();
                    let context = context_lock.as_mut().unwrap();

                    let mx = mouse_position[0] as u32;
                    let my = mouse_position[1] as u32;
                    let mouse_index = (my * context.size.width) + mx;

                    let size = 4 * std::mem::size_of::<f32>() as u64;
                    let mbuf_pixel = mouse_index as u64 * size;

                    let future = buf.map_read(mbuf_pixel, size);
                    context.device.poll(wgpu::Maintain::Wait);
                    let mapping = futures::executor::block_on(future);
                    if let Ok(mapping) = mapping {
                        unsafe {
                            mapping
                                .as_slice()
                                .align_to::<[f32; 4]>()
                                .1
                                .iter()
                                //.skip(mouse_index as usize)
                                .take(1)
                                .for_each(|f| {
                                    mouse_world_pos = (
                                        f32::floor(f[0]) as usize,
                                        f32::floor(f[2]) as usize,
                                        f32::floor(f[1]) as usize,
                                    );
                                });
                        }
                    }
                }

                // ImGui
                {
                    let mut ctx = DEVICE_CONTEXT.write();
                    let context = ctx.as_mut().unwrap();
                    let mut encoder: wgpu::CommandEncoder = context
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    if last_cursor != Some(ui.mouse_cursor()) {
                        last_cursor = Some(ui.mouse_cursor());
                    }
                    platform.prepare_render(&ui, &window);
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
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(virtual_keycode),
                                state: winit::event::ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                keycode = Some(virtual_keycode);
            }
            _ => {}
        }
        platform.handle_event(imgui.io_mut(), &window, &event);
    });
}

pub fn main_loop() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    // Temporarily avoid srgb formats for the swapchain on the web
    //futures::executor::block_on(run(event_loop, window, wgpu::TextureFormat::Bgra8UnormSrgb));
    run(event_loop, window, wgpu::TextureFormat::Bgra8UnormSrgb);
}
