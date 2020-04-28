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
use vertex_buffer::VertexBuffer;
mod texture;

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

    let (mut device, mut queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        })
        .await;

    let demo_shader = shader::Shader::from_source(&device, "resources/shaders/shader.vert", "resources/shaders/shader.frag");

    let mut vertex_buffer = VertexBuffer::<f32>::new(&[ 3, 2 ]);
    vertex_buffer.add_slice(&[-0.0868241,  0.49240386,  0.0,  0.4131759, 0.00759614]);
    vertex_buffer.add_slice(&[-0.49513406, 0.06958647,  0.0,  0.0048659444, 0.43041354]);
    vertex_buffer.add_slice(&[-0.21918549, -0.44939706, 0.0,  0.28081453, 0.949397057]);
    vertex_buffer.add_slice(&[0.35966998,  -0.3473291,  0.0,  0.85967, 0.84732911]);
    vertex_buffer.add_slice(&[0.44147372,   0.2347359,  0.0,  0.9414737, 0.2652641]);

    let mut index_buffer = VertexBuffer::<u16>::new(&[1]);
    index_buffer.add_slice(&[0, 1, 4]);
    index_buffer.add_slice(&[1, 2, 4]);
    index_buffer.add_slice(&[2, 3, 4]);

    // Textures
    let diffuse_bytes = include_bytes!("../../resources/avon-and-guards.png");
    let (diffuse_texture, cmd_buffer) = texture::Texture::from_bytes(
        &device, 
        diffuse_bytes, 
        "../../resources/avon-and-guards.png"
    ).unwrap();
    queue.submit(&[cmd_buffer]);

    let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        bindings: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::SampledTexture {
                    multisampled: false,
                    dimension: wgpu::TextureViewDimension::D2,
                    component_type: wgpu::TextureComponentType::Uint,
                },
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler {
                    comparison: false,
                },
            },
        ],
        label: Some("texture_bind_group_layout"),
    });

    let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        bindings: &[
            wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
            },
            wgpu::Binding {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
            }
        ],
        label: Some("diffuse_bind_group"),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&texture_bind_group_layout],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &demo_shader.vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &demo_shader.fs_module,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::None,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[wgpu::ColorStateDescriptor {
            format: swapchain_format,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[
                vertex_buffer.descriptor()
            ],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });
    vertex_buffer.build(&device, wgpu::BufferUsage::VERTEX);
    index_buffer.build(&device, wgpu::BufferUsage::INDEX);

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);    

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

    let mut renderer = Renderer::new(&mut imgui, &device, &mut queue, sc_desc.format, None);

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
                swap_chain = device.create_swap_chain(&surface, &sc_desc);
            }
            Event::RedrawRequested(_) => {
                // Base
                let frame = swap_chain
                    .get_next_texture()
                    .expect("Timeout when acquiring next swap chain texture");
                {
                    let mut encoder = device
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
                            depth_stencil_attachment: None,
                        });
                        rpass.set_pipeline(&render_pipeline);
                        rpass.set_bind_group(0, &diffuse_bind_group, &[]);
                        rpass.set_vertex_buffer(0, &vertex_buffer.buffer.as_ref().unwrap(), 0, 0);
                        rpass.set_index_buffer(&index_buffer.buffer.as_ref().unwrap(), 0, 0);
                        rpass.draw_indexed(0..index_buffer.len(), 0, 0..1);
                    }

                    queue.submit(&[encoder.finish()]);
                }

                // ImGui
                {
                    let delta_s = last_frame.elapsed();
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

                        let window = imgui::Window::new(im_str!("Hello too"));
                        window
                            .size([400.0, 200.0], Condition::FirstUseEver)
                            .position([400.0, 200.0], Condition::FirstUseEver)
                            .build(&ui, || {
                                ui.text(im_str!("Frametime: {:?}", delta_s));
                            });
                    }

                    let mut encoder: wgpu::CommandEncoder = device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    if last_cursor != Some(ui.mouse_cursor()) {
                        last_cursor = Some(ui.mouse_cursor());
                        platform.prepare_render(&ui, &window);
                    }
                    renderer
                        .render(ui.render(), &mut device, &mut encoder, &frame.view)
                        .expect("Rendering failed");

                    queue.submit(&[encoder.finish()]);
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
