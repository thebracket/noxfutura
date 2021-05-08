use futures::executor::block_on;
use parking_lot::RwLock;
use wgpu::{
    Adapter, BackendBit, Device, Instance, Queue, Surface, SwapChain, SwapChainDescriptor,
    TextureFormat,
};
use winit::{dpi::PhysicalSize, window::Window};
mod init;
use init::{create_swap_chain, get_adapter, get_device_and_queue};

const SWAPCHAIN_FORMAT: TextureFormat = TextureFormat::Bgra8UnormSrgb;

pub struct RenderContext {
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub size: PhysicalSize<u32>,
    pub surface: Surface,
    pub swapchain_format: TextureFormat,
    pub swapchain_desc: SwapChainDescriptor,
    pub swap_chain: SwapChain,
}

impl RenderContext {
    pub(crate) fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = Instance::new(BackendBit::VULKAN);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = block_on(get_adapter(&instance, &surface));
        let (device, queue) = block_on(get_device_and_queue(&adapter));
        let (sc_desc, swap_chain) = create_swap_chain(&size, &device, &surface);

        Self {
            adapter,
            device,
            queue,
            size,
            surface,
            swapchain_format: SWAPCHAIN_FORMAT,
            swapchain_desc: sc_desc,
            swap_chain,
        }
    }
}

lazy_static! {
    pub static ref RENDER_CONTEXT: RwLock<Option<RenderContext>> = RwLock::new(None);
}

pub(crate) fn init_render_context(window: &Window) {
    let mut rc = RENDER_CONTEXT.write();
    *rc = Some(RenderContext::new(window));
}
