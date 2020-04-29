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
mod texture;
use camera::Camera;
mod context;
pub use context::Context;
mod uniforms;
use uniforms::UniformBlock;
mod pipelines;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Uniforms {
    view_proj: ultraviolet::mat::Mat4,
    rot_angle: f32,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}
impl UniformBlock for Uniforms {}

impl Uniforms {
    fn new() -> Self {
        Self {
            view_proj: ultraviolet::mat::Mat4::identity(),
            rot_angle: 0.0,
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix();
        self.rot_angle += 0.01;
    }
}

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

    let mut context = Context::new(adapter, device, queue, size, surface);

    let shader_id = context.register_shader(
        "resources/shaders/shader.vert",
        "resources/shaders/shader.frag",
    );

    let world = crate::worldmap::WorldMap::new();
    let mut vertex_buffer = world.build_vertex_buffer();

    let depth_id = context.register_depth_texture("depth_texture");

    let camera = Camera::new(size.width, size.height);
    let mut uniforms = Uniforms::new();
    uniforms.update_view_proj(&camera);

    let (uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
        uniforms.create_buffer_layout_and_group(&context, 0, "some_uniforms");

    let pipeline_layout = context.create_pipeline_layout(&[&uniform_bind_group_layout]);

    let render_pipeline = pipelines::RenderPipelineBuilder::new(swapchain_format)
        .layout(&pipeline_layout)
        .vf_shader(&context, shader_id)
        .depth_buffer()
        .vertex_state(wgpu::IndexFormat::Uint16, &[vertex_buffer.descriptor()])
        .build(&context.device);

    vertex_buffer.build(&context.device, wgpu::BufferUsage::VERTEX);

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
                uniforms.update_view_proj(&camera);
                uniforms.update_buffer(&context, &uniform_buffer);

                // Base
                let frame = swap_chain
                    .get_next_texture()
                    .expect("Timeout when acquiring next swap chain texture");
                {
                    let mut encoder = context
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    {
                        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                                attachment: &frame.view,
                                resolve_target: None,
                                load_op: wgpu::LoadOp::Clear,
                                store_op: wgpu::StoreOp::Store,
                                clear_color: wgpu::Color::BLACK,
                            }],
                            depth_stencil_attachment: Some(
                                wgpu::RenderPassDepthStencilAttachmentDescriptor {
                                    attachment: &context.textures[depth_id].view,
                                    depth_load_op: wgpu::LoadOp::Clear,
                                    depth_store_op: wgpu::StoreOp::Store,
                                    clear_depth: 1.0,
                                    stencil_load_op: wgpu::LoadOp::Clear,
                                    stencil_store_op: wgpu::StoreOp::Store,
                                    clear_stencil: 0,
                                },
                            ),
                        });
                        rpass.set_pipeline(&render_pipeline);
                        //rpass.set_bind_group(0, &diffuse_bind_group, &[]);
                        rpass.set_bind_group(0, &uniform_bind_group, &[]);
                        rpass.set_vertex_buffer(0, &vertex_buffer.buffer.as_ref().unwrap(), 0, 0);
                        //rpass.set_index_buffer(&index_buffer.buffer.as_ref().unwrap(), 0, 0);
                        //rpass.draw_indexed(0..index_buffer.len(), 0, 0..1);
                        rpass.draw(0..vertex_buffer.len(), 0..1);
                    }

                    context.queue.submit(&[encoder.finish()]);
                }

                // ImGui
                {
                    //let delta_s = last_frame.elapsed();
                    last_frame = imgui.io_mut().update_delta_time(last_frame);

                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("Failed to prepare frame");
                    let ui = imgui.frame();

                    {
                        let window = imgui::Window::new(im_str!("Hello world"));
                        window
                            .size([300.0, 100.0], Condition::FirstUseEver)
                            .build(&ui, || {
                                ui.text(im_str!("Hello world!"));
                                ui.text(im_str!("This...is...imgui-rs on WGPU!"));
                                ui.separator();
                                let mouse_pos = ui.io().mouse_pos;
                                ui.text(im_str!(
                                    "Mouse Position: ({:.1},{:.1})",
                                    mouse_pos[0],
                                    mouse_pos[1]
                                ));
                            });

                        /*let window = imgui::Window::new(im_str!("Hello too"));
                        window
                            .size([400.0, 200.0], Condition::FirstUseEver)
                            .position([400.0, 200.0], Condition::FirstUseEver)
                            .build(&ui, || {
                                ui.text(im_str!("Frametime: {:?}", delta_s));
                            });
                        */
                    }

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

                //queue.submit(&[encoder.finish()]);
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
