use super::{assets::load_minimal_2d, GameMode, OUTPUT_FORMAT, RENDER_CONTEXT, TickResult};
use egui::FontDefinitions;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::*;
use std::iter;
use std::time::Instant;
use winit::event::Event::*;
use winit::event_loop::ControlFlow;

const INITIAL_WIDTH: u32 = 1024;
const INITIAL_HEIGHT: u32 = 768;

/// A custom event type for the winit app.
enum Event {
    RequestRedraw,
}

/// This is the repaint signal type that egui needs for requesting a repaint from another thread.
/// It sends the custom RequestRedraw event to the winit event loop.
struct ExampleRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>);

impl epi::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    }
}

pub fn run() {
    let event_loop = winit::event_loop::EventLoop::with_user_event();
    let window = winit::window::WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title("egui-wgpu_winit example")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
        })
        .build(&event_loop)
        .unwrap();

    super::init_render_context(&window);

    let size = window.inner_size();

    // We use the egui_winit_platform crate as the platform.
    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: size.width as u32,
        physical_height: size.height as u32,
        scale_factor: window.scale_factor(),
        font_definitions: FontDefinitions::default(),
        style: Default::default(),
    });

    // We use the egui_wgpu_backend crate as the render backend.
    let mut egui_rpass = RenderPass::new(
        &RENDER_CONTEXT.read().as_ref().unwrap().device,
        OUTPUT_FORMAT,
    );

    // Pre-Initialize game modes
    load_minimal_2d();
    let mut game_modes: [Box<dyn GameMode>; 3] = [
        Box::new(crate::display::Loader::new()),
        Box::new(crate::display::MainMenu::new()),
        Box::new(crate::display::WorldGen::new()),
    ];

    for gm in game_modes.iter_mut() {
        gm.pre_init();
    }

    let mut current_mode = 0;
    game_modes[current_mode].init();
    game_modes[current_mode].activate();

    // Get started
    let start_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        // Pass the winit events to the platform integration.
        platform.handle_event(&event);

        match event {
            RedrawRequested(..) => {
                platform.update_time(start_time.elapsed().as_secs_f64());

                let output_frame = match RENDER_CONTEXT
                    .read()
                    .as_ref()
                    .unwrap()
                    .swap_chain
                    .get_current_frame()
                {
                    Ok(frame) => frame,
                    Err(e) => {
                        eprintln!("Dropped frame with error: {}", e);
                        return;
                    }
                };

                // Begin to draw the UI frame.
                platform.begin_frame();

                // Game modes
                let tick_result = game_modes[current_mode].tick(&platform.context(), &output_frame.output);

                // End the UI frame. We could now handle the output and draw the UI with the backend.
                let (_output, paint_commands) = platform.end_frame();
                let paint_jobs = platform.context().tessellate(paint_commands);

                let mut rcl = RENDER_CONTEXT.write();
                let rc = rcl.as_mut().unwrap();

                let mut encoder =
                    rc.device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("encoder"),
                        });

                // Upload all resources for the GPU.
                let screen_descriptor = ScreenDescriptor {
                    physical_width: rc.swap_chain_descriptor.width,
                    physical_height: rc.swap_chain_descriptor.height,
                    scale_factor: window.scale_factor() as f32,
                };
                egui_rpass.update_texture(&rc.device, &rc.queue, &platform.context().texture());
                egui_rpass.update_user_textures(&rc.device, &rc.queue);
                egui_rpass.update_buffers(
                    &mut rc.device,
                    &mut rc.queue,
                    &paint_jobs,
                    &screen_descriptor,
                );

                // Record all render passes.
                egui_rpass.execute(
                    &mut encoder,
                    &output_frame.output.view,
                    &paint_jobs,
                    &screen_descriptor,
                    None,
                );

                // Submit the commands.
                rc.queue.submit(iter::once(encoder.finish()));
                *control_flow = ControlFlow::Poll;

                // Handle mode changes
                match tick_result {
                    TickResult::MainMenu => {
                        game_modes[current_mode].deactivate();
                        game_modes[1].activate();
                        current_mode = 1;
                    }
                    TickResult::WorldGen => {
                        game_modes[current_mode].deactivate();
                        game_modes[2].activate();
                        current_mode = 2;
                    }
                    TickResult::Quit => {
                        *control_flow = ControlFlow::Exit;
                    }
                    TickResult::Continue => {}
                }
            }
            MainEventsCleared | UserEvent(Event::RequestRedraw) => {
                window.request_redraw();
            }
            WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(size) => {
                    let mut rcl = RENDER_CONTEXT.write();
                    let rc = rcl.as_mut().unwrap();
                    rc.swap_chain_descriptor.width = size.width;
                    rc.swap_chain_descriptor.height = size.height;
                    rc.swap_chain = rc
                        .device
                        .create_swap_chain(&rc.surface, &rc.swap_chain_descriptor);
                }
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            _ => (),
        }
    });
}
