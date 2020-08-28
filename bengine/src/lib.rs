mod game;
mod imgui_wgpu;
mod init;
mod textures;
mod core;
mod shaders;
mod buffers;
mod layouts;

pub use game::BEngineGame;
use imgui::Context;
use imgui_wgpu::Renderer;
use imgui_winit_support::WinitPlatform;
pub use init::WgpuInit;
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
pub use crate::core::Core;
pub use crate::textures::Textures;
pub use crate::shaders::Shaders;
pub use crate::shaders::ShaderType;
pub use crate::buffers::Buffers;
pub use crate::core::Initializer;

use crate::layouts::simple_texture_bg_layout;
use crate::layouts::simple_texture_bg;

pub mod gui {
    pub use imgui::*;
}

pub mod gpu {
    pub use wgpu::*;
}

fn bootstrap(title: &str) -> (
    EventLoop<()>,
    Window,
    WgpuInit,
    usize,
    Context,
    Renderer,
    f64,
    WinitPlatform,
    Textures,
    Shaders,
    Buffers
) {
    let event_loop = EventLoop::new();
    let mut window = Window::new(&event_loop).unwrap();
    window.set_title(title);
    let mut device_info = init::initialize_wgpu(&window);
    let mut textures = Textures::new();
    let depth_texture = textures.register_new_depth_texture(&device_info.device, device_info.size, "depth");
    let imgui_renderer = init::initialize_imgui(
        &window,
        &device_info.device,
        &mut device_info.queue,
        &device_info.swapchain_desc,
    );
    let shaders = Shaders::new();
    let buffers = Buffers::new();

    (
        event_loop,
        window,
        device_info,
        depth_texture,
        imgui_renderer.0,
        imgui_renderer.1,
        imgui_renderer.2,
        imgui_renderer.3,
        textures,
        shaders,
        buffers
    )
}

pub fn run<P: 'static>(mut program: P, title: &str)
where
    P: BEngineGame,
{
    let (
        event_loop,
        window,
        mut device_info,
        mut depth_texture,
        mut imgui,
        mut imgui_renderer,
        mut hidpi_factor,
        mut platform,
        mut textures,
        mut shaders,
        mut buffers
    ) = bootstrap(title);

    let mut last_frame = Instant::now();
    let mut last_cursor = None;
    let mut keycode: Option<winit::event::VirtualKeyCode> = None;
    let mut mouse_world_pos = (0usize, 0usize, 0usize);

    let mut initializer = Initializer::new(
        &device_info.device,
        &device_info.queue,
        &mut textures,
        &mut shaders,
        &mut buffers,
        device_info.swapchain_format
    );

    program.init(&mut initializer);

    let clear_color = wgpu::Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    };

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
                device_info.swapchain_desc.width = size.width;
                device_info.swapchain_desc.height = size.height;
                device_info.swap_chain = device_info
                    .device
                    .create_swap_chain(&device_info.surface, &device_info.swapchain_desc);
                textures.replace_depth_texture(depth_texture, &device_info.device, device_info.size, "depth");
                device_info.size = size;
                // TODO: program.on_resize();
            }
            Event::RedrawRequested(_) => {
                let frame = device_info
                    .swap_chain
                    .get_current_frame()
                    .expect("Frame failed");

                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                let mouse_position = imgui.io().mouse_pos;
                let ui = imgui.frame();

                let mut core = Core{
                    imgui: &ui,
                    textures: &mut textures,
                    frame: &frame,
                    device: &device_info.device,
                    buffers: &mut buffers,
                    queue: &mut device_info.queue
                };
                let should_continue = program.tick(&mut core);
                if !should_continue {
                    *control_flow = ControlFlow::Exit;
                }
                /*let should_continue = program.tick(&frame, depth_id, &ui, keycode, &mouse_world_pos);
                if !should_continue {
                    *control_flow = ControlFlow::Exit;
                }*/
                keycode = None;

                // Mouse buffer insanity
                /*
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
                }*/

                {
                    use imgui::*;
                    let title = format!(
                        "FPS: {:.0}. ### FPS",
                        ui.io().framerate
                    );
                    let title_tmp = ImString::new(title);
                    let window = imgui::Window::new(&title_tmp);
                    window
                        .collapsed(true, Condition::FirstUseEver)
                        .size([100.0, 100.0], Condition::FirstUseEver)
                        .movable(true)
                        .position([0.0, device_info.size.height as f32 - 20.0], Condition::FirstUseEver)
                        .build(&ui, || {});
                }

                // ImGui
                {
                    let mut encoder: wgpu::CommandEncoder = device_info
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    if last_cursor != Some(ui.mouse_cursor()) {
                        last_cursor = Some(ui.mouse_cursor());
                    }
                    platform.prepare_render(&ui, &window);

                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.output.view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });

                    imgui_renderer
                        .render(
                            ui.render(),
                            &mut device_info.queue,
                            &device_info.device,
                            &mut rpass,
                        )
                        .expect("Rendering failed");
                    std::mem::drop(rpass);

                    device_info.queue.submit(Some(encoder.finish()));
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
