use futures_lite::future::block_on;
use lazy_static::*;
use parking_lot::RwLock;
use wgpu::{Adapter, Device, Instance, Queue, Surface, SwapChain, SwapChainDescriptor};
use winit::window::Window;

pub const OUTPUT_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

lazy_static! {
    pub static ref RENDER_CONTEXT: RwLock<Option<RenderContext>> = RwLock::new(None);
}

pub struct RenderContext {
    pub instance: Instance,
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub swap_chain_descriptor: SwapChainDescriptor,
    pub swap_chain: SwapChain,
}

impl RenderContext {
    pub fn init_render_context(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        }))
        .expect("Unable to create WGPU adapter. Sorry.");

        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ))
        .expect("Error creating device and queue in WGPU.");

        let size = window.inner_size();
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: OUTPUT_FORMAT,
            width: size.width as u32,
            height: size.height as u32,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            swap_chain_descriptor: sc_desc,
            swap_chain,
        }
    }
}

pub fn init_render_context(window: &Window) {
    let mut rc = RENDER_CONTEXT.write();
    *rc = Some(RenderContext::init_render_context(window));
}
